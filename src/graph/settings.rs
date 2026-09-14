use eframe::egui::{self, Context, Ui};
use serde::{Deserialize, Serialize};

use crate::t;

#[derive(Serialize, Deserialize)]
pub struct GraphicalSettings {
    pub dark_mode: bool,
}

impl Default for GraphicalSettings {
    fn default() -> Self {
        Self { dark_mode: true }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LanguageSettings {
    pub locale: String,
}

impl Default for LanguageSettings {
    fn default() -> Self {
        Self {
            locale: "en".to_string(),
        }
    }
}

impl GraphicalSettings {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.heading(t!("settings_graphical"));

        if ui
            .checkbox(&mut self.dark_mode, t!("settings_graphical_dark_mode"))
            .changed()
        {
            ui.ctx().set_visuals(if self.dark_mode {
                egui::Visuals::dark()
            } else {
                egui::Visuals::light()
            });
        }
    }
}

impl LanguageSettings {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.heading(t!("settings_language"));

        egui::ComboBox::from_label(t!("settings_language_select"))
            .selected_text(&self.locale)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.locale, "en".to_string(), "English");
                ui.selectable_value(&mut self.locale, "tr".to_string(), "Türkçe");
            });
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Default)]
pub enum SettingsCategory {
    #[default]
    Graphical,
    Language,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Settings {
    pub graphical: GraphicalSettings,
    pub language: LanguageSettings,

    #[serde(skip)]
    pub open: bool,
    #[serde(skip)]
    category: SettingsCategory,
}

impl Settings {
    pub fn ui(&mut self, ctx: &Context) {
        if self.open {
            egui::Modal::new(egui::Id::new("settings_modal")).show(ctx, |ui| {
                ui.set_width(500.0);
                ui.set_min_height(320.0);

                ui.heading(t!("settings"));
                ui.separator();

                ui.horizontal(|ui| {
                    ui.set_height(320.0);
                    ui.vertical(|ui| {
                        ui.set_width(120.0);
                        ui.set_min_height(ui.available_height());
                        ui.selectable_value(
                            &mut self.category,
                            SettingsCategory::Graphical,
                            t!("settings_graphical"),
                        );
                        ui.selectable_value(
                            &mut self.category,
                            SettingsCategory::Language,
                            t!("settings_language"),
                        );
                    });

                    ui.separator();

                    ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                        match self.category {
                            SettingsCategory::Graphical => self.graphical.ui(ui),
                            SettingsCategory::Language => self.language.ui(ui),
                        }
                    });
                });

                ui.separator();

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Max), |ui| {
                    if ui.button(t!("settings_close")).clicked() {
                        self.open = false;
                    }
                });
            });
        }
    }
}
