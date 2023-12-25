pub trait CustomWidgets {
    fn hyperlink_in_new_tab(&mut self, text: impl Into<egui::WidgetText>, url: impl ToString);
    fn fullscreen_button(&mut self);
    fn created_by_codepony(&mut self);
}
impl CustomWidgets for egui::Ui {
    fn hyperlink_in_new_tab(&mut self, text: impl Into<egui::WidgetText>, url: impl ToString) {
        self
        .add(egui::Hyperlink::from_label_and_url(text, url)
        .open_in_new_tab(true));
    }
    fn fullscreen_button(&mut self) {
        if cfg!(target_arch = "wasm32") { // never draw on web
            return;
        }
        let mut is_fullscreen = bool::default();
        self.ctx().input(|i| {
            is_fullscreen = i.viewport().fullscreen.unwrap_or_default();
        });
        if self.selectable_label(is_fullscreen, "Fullscreen").clicked() {
            self.ctx().send_viewport_cmd(egui::ViewportCommand::Fullscreen(!is_fullscreen));
        }
    }
    fn created_by_codepony(&mut self) {
        self.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            ui.label("created by ");
            ui.hyperlink_to("codepony", "https://github.com/thecodepony");
            ui.label(".");
        });
    }
}