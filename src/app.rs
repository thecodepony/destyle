use egui_dock::{TabViewer, SurfaceIndex, NodeIndex};
use crate::widgets::CustomWidgets;
type Title = String;
enum RequestType {
    CreateTab(SurfaceIndex, NodeIndex),
    CloseTab(<FileViewer as TabViewer>::Tab),
}
#[derive(serde::Deserialize, serde::Serialize)]
struct FileViewer {
    #[serde(skip)]
    requests: Vec<RequestType>,
    buffers: std::collections::BTreeMap<Title, String>,
}
impl TabViewer for FileViewer {
    type Tab = Title;
    fn title(&mut self, title: &mut Title) -> egui::WidgetText {
        (&*title).into()
    }
    fn ui(&mut self, ui: &mut egui::Ui, title: &mut Title) {
        let Some(text_buffer) = self.buffers.get_mut(title) else {
            self.requests.push(RequestType::CloseTab(title.to_owned()));
            return;
        };
        ui.add_sized(
            ui.available_size_before_wrap(),
            egui::TextEdit::multiline(text_buffer)
            .margin(egui::Vec2::ZERO)
            .frame(false)
            .code_editor()
            .desired_width(f32::INFINITY)
        );
    }
    fn on_add(&mut self, surface: egui_dock::SurfaceIndex, node: egui_dock::NodeIndex) {
        self.requests.push(RequestType::CreateTab(surface, node));
    }
    fn on_close(&mut self, tab: &mut Self::Tab) -> bool {
        self.requests.push(RequestType::CloseTab(tab.to_owned()));
        true
    }
}
#[derive(serde::Deserialize, serde::Serialize)]
struct ShowPanel {
    top: bool,
    left: bool,
    right: bool,
    center: bool,
}
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Application {
    show_panel: ShowPanel,
    tree: egui_dock::DockState<String>,
    file_viewer: FileViewer,
}
impl Default for Application {
    fn default() -> Self {
        Self {
            show_panel: ShowPanel {
                top: true,
                left: true,
                right: true,
                center: true,
            },
            tree: egui_dock::DockState::new(Vec::default()),
            file_viewer: FileViewer {
                requests: vec![],
                // TODO: new tab if empty
                buffers: std::collections::BTreeMap::default(),
            },
        }
    }
}
impl Application {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // cc.egui_ctx.set_visuals
        // cc.egui_ctx.set_fonts
        cc.egui_ctx.set_style(crate::style::custom());
        // Load previous app state (if any).
        let Some(storage) = cc.storage else {
            return Self::default();
        };
        let Some(deserialized): Option<Application> = eframe::get_value(storage, eframe::APP_KEY) else {
            return Self::default();
        };
        // TODO: move test to Application.FileViewer.ui()
        for tab in deserialized.tree.iter_all_tabs() {
            if deserialized.file_viewer.buffers.get(tab.1).is_none() {
                log::warn!("Found destructive tree and buffers. App storage is reset.");
                return Self::default()
            }
        }
        deserialized
    }
    fn new_file(&mut self) {
        self.file_viewer.on_add(SurfaceIndex::main(), NodeIndex::root());
    }
    fn close_all_files_without_save(&mut self) {
        self.tree = egui_dock::DockState::new(Vec::default());
        self.file_viewer.buffers.clear();
    }
}
impl eframe::App for Application {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let is_web = cfg!(target_arch = "wasm32");
        if self.show_panel.top {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.add(egui::Button::new("New File").shortcut_text("Ctrl+N")).clicked() {
                    self.new_file();
                }
                if ui.add(egui::Button::new("Open File...").shortcut_text("Ctrl+O")).clicked() { todo!(); }
                ui.menu_button("Open Recent", |ui| {
                    if ui.button("Open Last Session").clicked() { todo!(); }
                });
                ui.separator();
                if ui.button("Save File").clicked() { todo!(); }
                if ui.button("Save File As...").clicked() { todo!(); }
                if ui.button("Save All Files").clicked() { todo!(); }
                if !is_web {
                    ui.separator();
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                }
                ui.separator();
                if ui.button("Close All Files").clicked() {
                    self.close_all_files_without_save();
                }
            });
            ui.menu_button("Imprint", |ui| {
                if ui.button("Open Imprint...").clicked() { todo!(); }
                if ui.button("Open Recent Imprint...").clicked() { todo!(); }
                ui.separator();
                if ui.button("Generate Imprint...").clicked() { todo!(); }
            });
            ui.menu_button("Edit", |ui| {
                if ui.button("Undo").clicked() { todo!(); }
                if ui.button("Redo").clicked() { todo!(); }
                ui.separator();
                if ui.button("Find").clicked() { todo!(); }
                if ui.button("Replace").clicked() { todo!(); }
            });
            ui.menu_button("View", |ui| {
                if ui.button("Left Side Panel").clicked() {
                    self.show_panel.left =! self.show_panel.left;
                }
                if ui.button("Right Side Panel").clicked() {
                    self.show_panel.right =! self.show_panel.right;
                }
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.fullscreen_button();
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });});}
        if self.show_panel.left { egui::SidePanel::left("Dictionary").resizable(true).show(ctx, |ui| {
            ui.heading("Dictionary");
            ui.separator();
            ui.hyperlink_in_new_tab("Link", "https://github.com/thecodepony/destyle/");
        });}
        if self.show_panel.right { egui::SidePanel::right("Rules").resizable(true).show(ctx, |ui| {
            ui.heading("Rules");
            ui.separator();
            ui.hyperlink_in_new_tab("Link", "https://github.com/thecodepony/destyle/");
        });}
        if self.show_panel.center {
            self.file_viewer.requests.drain(..).for_each(|request| { match request {
                RequestType::CreateTab(s, n) => {
                    let Some(title) = (1..).find_map(|x| {
                        let t = format!("Untitled-{}", x);
                        if self.tree.find_tab(&t).is_none() { Some(t) } else { None }
                    }) else {
                        return; // break match
                    };
                    self.file_viewer.buffers.insert(title.to_owned(), String::default());
                    self.tree.set_focused_node_and_surface((s, n));
                    self.tree.push_to_focused_leaf(title);
                },
                RequestType::CloseTab(t) => {
                    self.file_viewer.buffers.remove(&t);
                },
            }});
            egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(0.))
            .show(ctx, |ui| {
                egui_dock::DockArea::new(&mut self.tree)
                .show_add_buttons(true)
                .show_inside(ui, &mut self.file_viewer);
            });
        }
    }
}
