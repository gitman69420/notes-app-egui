mod close_dialog_box;
use close_dialog_box::exit_confirmation;

mod notes_list_panel;


pub struct Note {
    idx: usize,
    title: String,
    note: String
}

impl Default for MyNotesApp {
    fn default() -> Self {
        Self {
            notes: vec![Note{
                idx: 1,
                title: "My first note".to_string(), 
                note: "Welcome to my first note, I'm glad to have you all here".to_string()
            },
            Note{
                idx: 2,
                title: "My second note".to_string(), 
                note: "Welcome to my second note, I think I'm getting better at this".to_string()
            }
            ],
            current_idx: 1,
            show_confirmation_dialog: false,
            allowed_to_close: false
        }
    }
}

struct MyNotesApp {
    notes: Vec<Note>,
    current_idx: usize,
    show_confirmation_dialog: bool,
    allowed_to_close: bool
}

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(750., 700.)),
        ..Default::default()
    };

    eframe::run_native("Notes!", options, Box::new(|_cc| Box::<MyNotesApp>::default())).unwrap();
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
        notes_list_panel::list(ctx, frame, &self.notes, &mut self.current_idx);

        // main panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                // ui.horizontal(|ui| {
                //     ui.label(format!("Welcome!"));
                //     ui.label(format!("There are my Notes!"));
                // });

                ui.add(
                        egui::Label::new(
                                egui::RichText::new(
                                        format!("{}",self.current_idx)
                                )
                        )
                    );

            });
        });
    }
}

