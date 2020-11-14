use conrod_core::text::font::Id as FontId;
use conrod_core::{widget::*, Colorable, Labelable, Positionable, Ui};
use std::collections::HashMap;

use crate::config::Config;

pub struct Gui {
    config: Config,
    fonts: HashMap<&'static str, FontId>,
    ids: HashMap<String, Id>,
}

impl Gui {
    pub fn new( config: Config, fonts: HashMap<&'static str, FontId>) -> Self {
        let ids = HashMap::new();
        Gui { config, fonts, ids }
    }

    pub fn update(&mut self, ui: &mut Ui) -> bool {
        if let Some(ref config) = self.config.sub("App") {
            self.add_widgets(ui, config);
        }
        ui.has_changed()
    }

    pub fn get_id(&mut self, key: &str, ui: &mut Ui) -> Id {
        if !self.ids.contains_key(key) {
            self.ids
                .insert(key.to_string(), ui.widget_id_generator().next());
        }
        self.ids[key]
    }

    fn add_button(&mut self, ui: &mut Ui, button: &Config) {
        println!("add button");
        let id = self.get_id("button", ui);
        let ui = &mut ui.set_widgets();
        Button::new().label("This is a button").set(id, ui);
    }

    fn add_text(&mut self, ui: &mut Ui, text: &Config) {
        println!("add text");
        let id = self.get_id("text", ui);
        let ui = &mut ui.set_widgets();
        Text::new(text.value_str("text").unwrap_or(""))
            .middle_of(ui.window)
            .color(conrod_core::color::BLACK)
            .font_size(32)
            .set(id, ui);
    }

    fn add_widgets(&mut self, ui: &mut Ui, config: &Config) {
        for k in config.keys() {
            match k {
                //"Button" => self.add_button(ui, &config.sub(k).unwrap()),
                "Text" => self.add_text(ui, &config.sub(k).unwrap()),
                _ => (),
            }
        }
    }
}
