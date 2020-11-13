use conrod_core::{widget::*, Colorable, Labelable, Positionable, Ui};
use conrod_core::text::font::Id as FontId;
use std::collections::HashMap;
use yaml_rust::{yaml::Hash, Yaml};

use crate::config::Config;

pub struct Gui<'a> {
    ui: &'a mut Ui,
    config: &'a Config,
    fonts: &'a HashMap<&'static str, FontId>,
    ids: HashMap<String, Id>,
}

impl<'a> Gui<'a> {
    pub fn new(ui: &'a mut Ui, fonts: &'a HashMap<&'static str, FontId>, config: &'a Config) -> Self {
        let ids = HashMap::new();
        Gui { ui, config, fonts, ids }
    }

    pub fn setup(&mut self) -> bool {
        if let Some(ref config) = self.config.sub("App") {
            self.add_widgets(config);
        }
        self.ui.has_changed()
    }

    pub fn get_id(&mut self, key: &str) -> Id {
        if !self.ids.contains_key(key) {
            self.ids
                .insert(key.to_string(), self.ui.widget_id_generator().next());
        }
        self.ids[key]
    }

    fn add_button(&mut self, button: &Config) {
        println!("add button");
        let id = self.get_id("button");
        let ui = &mut self.ui.set_widgets();
        Button::new().label("This is a button").set(id, ui);
    }

    fn add_text(&mut self, text: &Config) {
        println!("add text");
        let id = self.get_id("text");
        let ui = &mut self.ui.set_widgets();
        Text::new(text.value_str("text").unwrap_or(""))
            .middle_of(ui.window)
            .color(conrod_core::color::BLACK)
            .font_size(32)
            .set(id, ui);
    }

    fn add_widgets(&mut self, config: &Config) {
        for k in config.keys() {
            match k {
                "Button" => self.add_button(&config.sub(k).unwrap()),
                "Text" => self.add_text(&config.sub(k).unwrap()),
                _ => (),
            }
        }
    }
}
