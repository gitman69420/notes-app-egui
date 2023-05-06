pub fn exit_confirmation(ctx: &egui::Context, _frame: &mut eframe::Frame, show_confirmation_dialog: &mut bool, allowed_to_close: &mut bool) {
    if *show_confirmation_dialog {
        // Show confirmation dialog:
        egui::Window::new("Are you sure you want to exit?")
            .collapsible(false)
            .resizable(false)
            .movable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0., 0.))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Cancel").clicked() {
                        *show_confirmation_dialog = false;
                    }

                    if ui.button("Yes!").clicked() {
                        *allowed_to_close = true;
                        _frame.close();
                    }
                });
            });
    }
}