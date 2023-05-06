mod close_dialog_box;
use close_dialog_box::exit_confirmation;

mod notes_list_panel;

mod my_notes_app;
use my_notes_app::{Note, MyNotesApp};

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(750., 700.)),
        ..Default::default()
    };

    eframe::run_native(
        "Notes!", 
        options, 
        Box::new(|_cc| Box::<MyNotesApp>::default())).unwrap();
}


impl eframe::App for MyNotesApp {

    fn on_close_event(&mut self) -> bool {
        self.show_confirmation_dialog = true;
        self.allowed_to_close
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        // exit confirmation window
        exit_confirmation(ctx, frame, &mut self.show_confirmation_dialog, &mut self.allowed_to_close);
        
        // for side panel
        notes_list_panel::sidebar(ctx, frame, &self.notes, &mut self.current_idx);

        // main panel
        egui::CentralPanel::default().show(ctx, |ui| {    

            // scroll for texts longer than the window height
            egui::ScrollArea::both().show(ui, |ui| {

                // for text area to fill the remaining area
                ui.add_sized(ui.available_size(),
                    // multiline TextEdit for editing text 
                    egui::TextEdit::multiline(
                        &mut self.notes[self.current_idx-1].note
                    )
                );
            });
        });
    }
}
