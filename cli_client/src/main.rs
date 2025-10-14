use clap::Parser;
use notes::{Note, NoteError, Notes};

#[derive(Parser)]
#[command(name = "qot")]
#[command(about = "Quantum of Thought - A note-taking CLI")]
struct Cli {
    /// The command or note content
    #[arg(trailing_var_arg = true)]
    args: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    let mut note_service = NoteService::new();

    if cli.args.is_empty() {
        eprintln!("Usage: qot <content> or qot list");
        std::process::exit(1);
    }

    let first_arg = &cli.args[0];

    if first_arg == "list" {
        // For now, just show that list works - we'll add persistence next
        println!("No notes yet. Create one with: qot get milk");
    } else {
        // Everything is note content
        let content = cli.args.join(" ");

        match note_service.create(&content) {
            Ok(note) => {
                println!("Created note: {}", note.content);
            }
            Err(e) => {
                eprintln!("Error creating note: {:?}", e);
                std::process::exit(1);
            }
        }
    }
}

struct NoteService {
    notes: Notes,
}

impl NoteService {
    fn new() -> Self {
        Self {
            notes: Notes::new(),
        }
    }

    fn create(&mut self, content: &str) -> Result<Note, NoteError> {
        let note = self.notes.create(content)?;
        Ok(note)
    }
}
