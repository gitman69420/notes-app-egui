use crate::Note;

pub fn list(ctx: &egui::Context, _frame: &mut eframe::Frame,  app_notes_ref:&Vec<Note>, current_idx: &mut usize) {
    egui::SidePanel::left("notes_list").show(ctx, |ui| {
            
        egui::ScrollArea::vertical().show(ui, |ui| {

            egui::CollapsingHeader::new("| Your Notes |").show(ui, |ui| {
                for note in app_notes_ref {
                    if ui.add(egui::Label::new(format!("{}", note.title)).sense(egui::Sense::click())).clicked() {
                        println!("Note [{}] clicked", note.idx);
                        *current_idx = note.idx;
                };
            }
            });
        
            egui::CollapsingHeader::new("| Settings |").show(ui, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("X").clicked() {
                        _frame.close();
                    }
                    ui.label("Close");
                })
            })
        });
    });
}