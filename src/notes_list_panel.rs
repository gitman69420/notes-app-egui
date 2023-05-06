use crate::Note;

pub fn sidebar(ctx: &egui::Context, _frame: &mut eframe::Frame,  app_notes_ref:&Vec<Note>, current_idx: &mut usize) {

    egui::SidePanel::left("notes_list")
        .show_separator_line(true)
        .show(ctx, |ui| {
        
        egui::ScrollArea::vertical().id_source("sidebar").show(ui, |ui| {

            egui::CollapsingHeader::new("| Your Notes |").default_open(true).show(ui, |ui| {
                for note in app_notes_ref {
                    if ui.add(
                        egui::Label::new(
                            format!("{}", note.title)
                        )
                        .sense(egui::Sense::click())).clicked() {
                            *current_idx = note.idx;
                        };
                }
            });
            
            egui::CollapsingHeader::new("| Settings |").show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        if ui.button("X").clicked() {
                            _frame.close();
                        }
                        ui.label("Close");
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Settings").clicked() {
                            todo!();
                        }
                    });
                });
            });
        });    
        
    });
        
}