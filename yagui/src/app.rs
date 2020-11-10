use conrod_core::{image::Map, widget::*, Colorable, Labelable, Positionable, Ui, UiBuilder};
use conrod_glium::Renderer;
use glium::glutin::{dpi::*, event::*, event_loop::*, window::*, ContextBuilder};
use glium::{texture::Texture2d, Display, Surface};
use yaml_rust::{yaml::Hash, Yaml};

use crate::error::Result;
use crate::yaml_helper::YamlHelper;

pub struct App {
    event_loop: EventLoop<()>,
    display: Display,
    image_map: Map<Texture2d>,
    renderer: Renderer,
    ui: Ui,
}

impl App {
    pub fn from_yaml(yaml: &str) -> Result<Self> {
        let helper = YamlHelper::from_yaml(yaml)?;
        let width = helper.required_f64("App.width")?;
        let height = helper.required_f64("App.height")?;
        let title = helper.value_str("App.title");

        let event_loop = EventLoop::new();
        let wb = WindowBuilder::new()
            .with_inner_size(LogicalSize::new(width, height))
            .with_title(title.unwrap_or("App"));
        let cb = ContextBuilder::new();
        let display = Display::new(wb, cb, &event_loop)?;
        let image_map = Map::<Texture2d>::new();
        let renderer = Renderer::new(&display)?;
        let ui = UiBuilder::new([width, height]).build();

        let mut app = App {
            event_loop,
            display,
            image_map,
            renderer,
            ui,
        };

        if let Yaml::Hash(ref widgets) = helper.required_value("App")? {
            app.add_widgets(widgets);
        }

        Ok(app)
    }

    pub fn run(self) -> Result<()> {
        let Self {
            event_loop,
            display,
            mut ui,
            image_map,
            mut renderer,
        } = self;

        let mut redraw = true;
        event_loop.run(move |event, _, control_flow| {
            match &event {
                Event::RedrawRequested(_) => {
                    let primitives = ui.draw();
                    renderer.fill(&display, primitives, &image_map);
                    let mut target = display.draw();
                    target.clear_color(1.0, 1.0, 1.0, 0.5);
                    renderer.draw(&display, &mut target, &image_map).unwrap();
                    target.finish().unwrap();
                }
                Event::MainEventsCleared => {
                    if redraw {
                        redraw = false;
                        // TODO: Set UI here
                        display.gl_window().window().request_redraw();
                    }
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                _ => (),
            }

            if let Some(event) = convert_event(&event, &display.gl_window().window()) {
                ui.handle_event(event);
                redraw = true;
            }
        });
    }

    fn add_button(&mut self, button: &Hash) {
        println!("add button");
        dbg!(&button);
        let id = self.ui.widget_id_generator().next();
        let ui = &mut self.ui.set_widgets();
        Button::new().label("This is a button").set(id, ui);
    }

    fn add_text(&mut self, text: &Hash) {
        println!("add text");
        dbg!(&text);
        let id = self.ui.widget_id_generator().next();
        let ui = &mut self.ui.set_widgets();
        Text::new("Yes!")
            .middle_of(ui.window)
            .color(conrod_core::color::BLACK)
            .font_size(32)
            .set(id, ui);
    }

    fn add_widgets(&mut self, widgets: &Hash) {
        for (k, v) in widgets {
            if let Yaml::String(name) = k {
                if let Yaml::Hash(widget) = v {
                    match name.as_str() {
                        "Button" => self.add_button(widget),
                        "Text" => self.add_text(widget),
                        _ => (),
                    }
                }
            }
        }
    }
}

conrod_winit::v023_conversion_fns!();
