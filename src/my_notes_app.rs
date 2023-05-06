pub struct Note {
    pub idx: usize,
    pub title: String,
    pub note: String
}

pub struct MyNotesApp {
    pub notes: Vec<Note>,
    pub current_idx: usize,
    pub show_confirmation_dialog: bool,
    pub allowed_to_close: bool
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


