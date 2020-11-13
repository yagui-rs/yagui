use conrod_core::{image::Map, text::Font, Ui, UiBuilder};
use conrod_core::text::font::Id as FontId;
use conrod_glium::Renderer;
use glium::glutin::{dpi::*, event::*, event_loop::*, window::*, ContextBuilder};
use glium::{texture::Texture2d, Display, Surface};
use std::collections::HashMap;

use crate::config::Config;
use crate::error::Result;
use crate::gui::Gui;

pub struct App {
    event_loop: EventLoop<()>,
    display: Display,
    image_map: Map<Texture2d>,
    renderer: Renderer,
    ui: Ui,
    config: Config,
    fonts: HashMap<&'static str, FontId>,
}

impl App {
    pub fn from_yaml(yaml: &str) -> Result<Self> {
        let config = Config::from_yaml(yaml)?;
        let width = config.required_f64("App.width")?;
        let height = config.required_f64("App.height")?;
        let title = config.value_str("App.title");

        let event_loop = EventLoop::new();
        let wb = WindowBuilder::new()
            .with_title(title.unwrap_or("App"))
            .with_inner_size(LogicalSize::new(width, height));
        let cb = ContextBuilder::new().with_vsync(true).with_multisampling(4);
        let display = Display::new(wb, cb, &event_loop)?;
        let renderer = Renderer::new(&display)?;

        let mut app = App {
            event_loop,
            display,
            image_map: Map::<Texture2d>::new(),
            renderer,
            ui: UiBuilder::new([width, height]).build(),
            config,
            fonts: HashMap::new(),
        };

        app.add_font("regular", include_bytes!("fonts/Roboto-Regular.ttf"))?;
        app.add_font("bold", include_bytes!("fonts/Roboto-Bold.ttf"))?;
        app.add_font("italic", include_bytes!("fonts/Roboto-Italic.ttf"))?;
        app.add_font("bold_italic", include_bytes!("fonts/Roboto-BlackItalic.ttf"))?;

        app.ui.theme.font_id = Some(app.fonts["regular"]);

        Ok(app)
    }

    pub fn add_font(&mut self, key: &'static str, font: &'static [u8]) -> Result<()> {
        let id = self.ui.fonts.insert(Font::from_bytes(font)?);
        self.fonts.insert(key, id);
        Ok(())
    }

    pub fn run(self) -> Result<()> {
        let Self {
            event_loop,
            display,
            mut ui,
            image_map,
            mut renderer,
            config,
            fonts,
        } = self;

        let mut redraw = true;
        event_loop.run(move |event, _, control_flow| {
            match &event {
                Event::RedrawRequested(_) => {
                    let primitives = ui.draw();
                    renderer.fill(&display, primitives, &image_map);
                    let mut target = display.draw();
                    target.clear_color(1.0, 1.0, 1.0, 1.0);
                    renderer.draw(&display, &mut target, &image_map).unwrap();
                    target.finish().unwrap();
                }
                Event::MainEventsCleared => {
                    if redraw {
                        redraw = false;

                        let mut gui = Gui::new(&mut ui, &fonts, &config);
                        if gui.setup() {
                            display.gl_window().window().request_redraw();
                        }
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
}

conrod_winit::v023_conversion_fns!();
