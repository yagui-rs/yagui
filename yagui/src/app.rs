use conrod_core::{image::Map, Ui, UiBuilder};
use conrod_glium::Renderer;
use glium::glutin::{dpi::*, event::*, event_loop::*, window::*, ContextBuilder};
use glium::{Display, Surface, texture::Texture2d};

use crate::error::Result;
use crate::yaml_helper::YamlHelper;

pub struct App {
    event_loop: EventLoop<()>,
    display: Display,
    ui: Ui,
    image_map: Map<Texture2d>,
    renderer: Renderer,
}

impl<'a> App {
    pub fn from_yaml(yaml: &str) -> Result<Self> {
        let helper = YamlHelper::new(yaml)?;
        let width = helper.required_f64("App.width")?;
        let height = helper.required_f64("App.height")?;
        let title = helper.value_str("App.title");

        let event_loop = EventLoop::new();
        let wb = WindowBuilder::new()
            .with_inner_size(LogicalSize::new(width, height))
            .with_title(title.unwrap_or("App"));
        let cb = ContextBuilder::new();
        let display = Display::new(wb, cb, &event_loop)?;
        let ui = UiBuilder::new([width, height]).build();
        let image_map = Map::<Texture2d>::new();
        let renderer = Renderer::new(&display)?;
        Ok(App {
            event_loop,
            display,
            ui,
            image_map,
            renderer,
        })
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
                    target.clear_color(0.0, 0.0, 0.0, 1.0);
                    renderer.draw(&display, &mut target, &image_map).unwrap();
                    target.finish().unwrap();
                }
                Event::MainEventsCleared => {
                    if redraw {
                        redraw = false;
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
}

conrod_winit::v023_conversion_fns!();
