use conrod_core::{Ui, UiBuilder};
use glium::glutin::{
    dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder, ContextBuilder,
};
use glium::Display;
use yaml_rust::{yaml, Yaml, YamlLoader};

use crate::error::{ensure, Result, YaguiError};

pub struct App {
    ui: Ui,
    display: Display,
}

impl App {
    pub fn from_yaml(yaml: &str) -> Result<Self> {
        let mut docs = YamlLoader::load_from_str(yaml)?;
        ensure!(!docs.is_empty(), YaguiError::InvalidYaml);
        dbg!(docs.pop().unwrap());

        let el = EventLoop::new();
        let wb = WindowBuilder::new()
            .with_title("Hello!")
            .with_inner_size(LogicalSize::new(200.0, 100.0));
        let cb = ContextBuilder::new();
        let display = Display::new(wb, cb, &el)?;
        let ui = UiBuilder::new([100.0, 100.0]).build();
        Ok(App { ui, display })
    }

    pub fn run(&self) -> ! {
        let mut _frame = self.display.draw();
        loop {}
    }
}
