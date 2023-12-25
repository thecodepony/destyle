pub fn custom() -> egui::style::Style {
    let mut style = egui::Style::default();
    style.spacing.scroll = egui::style::ScrollStyle::solid();
    style
}