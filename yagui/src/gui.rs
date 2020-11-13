use conrod_core::{widget::*, Colorable, Labelable, Positionable, Ui};
use std::collections::HashMap;
use yaml_rust::{yaml::Hash, Yaml};

use crate::config::Config;

pub struct Gui<'a> {
    ui: &'a mut Ui,
    config: &'a Config,
    ids: HashMap<String, Id>,
}

impl<'a> Gui<'a> {
    pub fn new(ui: &'a mut Ui, config: &'a Config) -> Self {
        let ids = HashMap::new();
        Gui { ui, config, ids }
    }

    pub fn setup(&mut self) -> bool {
        if let Some(Yaml::Hash(ref widgets)) = self.config.value("App") {
            self.add_widgets(widgets);
        }
        true
    }

    pub fn get_id(&mut self, key: &str) -> Id {
        if !self.ids.contains_key(key) {
            self.ids
                .insert(key.to_string(), self.ui.widget_id_generator().next());
        }
        self.ids[key]
    }

    fn add_button(&mut self, button: &Hash) {
        println!("add button");
        let id = self.get_id("button");
        let ui = &mut self.ui.set_widgets();
        Button::new().label("This is a button").set(id, ui);
    }

    fn add_text(&mut self, text: &Hash) {
        println!("add text");
        let id = self.get_id("text");
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
