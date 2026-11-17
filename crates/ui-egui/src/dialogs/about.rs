use super::Dialog;

/// Help ▸ About The AllCraft Grid: the app, its contributors and the AI models that helped.
pub(super) fn about(ui: &mut egui::Ui, d: &mut Dialog) {
    ui.horizontal(|ui| {
        for tab in ["About", "Contributors", "Models"] {
            if ui.selectable_label(d.tab == tab, tab).clicked() {
                d.tab = tab.to_string();
            }
        }
    });
    ui.separator();
    match d.tab.as_str() {
        "Contributors" => crate::credits::contributors_ui(ui),
        "Models" => crate::credits::models_ui(ui),
        _ => {
            ui.heading("The AllCraft Grid");
            ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));
            ui.add_space(4.0);
            ui.label("A clean-room, open-source, Rust-native spreadsheet — part of The AllCraft Studio.");
            ui.hyperlink_to("The AllCraft Studio — the-allcraft.pages.dev", "https://the-allcraft.pages.dev");
            ui.hyperlink_to("GridCraft on GitHub", "https://github.com/thepixopik/gridcraft");
        }
    }
}
