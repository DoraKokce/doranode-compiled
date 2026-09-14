use std::{collections::HashMap, fs, io, path::PathBuf, sync::OnceLock};

use eframe::egui;

use crate::{
    graph::{settings::Settings, translations::Translations},
    t,
};
pub mod settings;
pub mod translations;

pub static TRANSLATIONS: OnceLock<Translations> = OnceLock::new();

fn bin_path() -> io::Result<PathBuf> {
    if cfg!(debug_assertions) {
        Ok(PathBuf::from(env!("CARGO_MANIFEST_DIR")))
    } else {
        std::env::current_exe()
    }
}

pub fn launch() {
    let bin_path = bin_path().expect("couldn't get bin path error:");
    let settings = fs::read_to_string(bin_path.join("settings.json"))
        .ok()
        .and_then(|s| serde_json::from_str::<Settings>(&s).ok())
        .unwrap_or_default();

    let locale_file = format!("{}.toml", settings.language.locale);
    let content = fs::read_to_string(bin_path.join("translations").join(&locale_file))
        .unwrap_or_else(|e| panic!("failed to read translations file {locale_file}: {e}"));

    let translations: HashMap<String, String> =
        toml::from_str(&content).expect("failed to parse translations toml");

    let translations = Translations(translations);

    TRANSLATIONS.set(translations).ok();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Doranode",
        native_options,
        Box::new(move |cc| Ok(Box::new(GraphEditor::new(cc, settings)))),
    )
    .expect("running eframe failed");
}

struct GraphEditor {
    settings: Settings,
}

impl GraphEditor {
    fn new(_: &eframe::CreationContext<'_>, settings: Settings) -> Self {
        Self { settings }
    }
}
impl eframe::App for GraphEditor {
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
        egui::Panel::top("menu_bar").show(ui, |ui| {
            ui.horizontal(|ui| {
                let but = ui.button(t!("settings"));
                if but.clicked() {
                    self.settings.open = true;
                }
            });
        });

        self.settings.ui(ui.ctx());
    }

    fn on_exit(&mut self) {
        let path = bin_path()
            .expect("couldn't get bin path")
            .join("settings.json");

        let json = serde_json::to_string_pretty(&self.settings)
            .expect("couldn't convert settings into json");
        std::fs::write(path, json).expect("couldn't write");
    }
}
