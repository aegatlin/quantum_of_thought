mod storage;

use clap::Parser;
use directories::ProjectDirs;
use notes::{Note, Notes};
use storage::{FileSystemStorage, Storage};

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

    let mut note_service = match NoteService::new() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to initialize service: {}", e);
            std::process::exit(1);
        }
    };

    if cli.args.is_empty() {
        eprintln!("Usage: qot <content> or qot list");
        std::process::exit(1);
    }

    let first_arg = &cli.args[0];

    if first_arg == "list" {
        match note_service.list() {
            Ok(notes) => {
                if notes.is_empty() {
                    println!("No notes yet. Create one with: qot get milk");
                } else {
                    for note in notes {
                        println!("{}", note.content);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error listing notes: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        // Everything is note content
        let content = cli.args.join(" ");

        match note_service.create(&content) {
            Ok(note) => {
                println!("Created note: {}", note.content);
            }
            Err(e) => {
                eprintln!("Error creating note: {}", e);
                std::process::exit(1);
            }
        }
    }
}

struct NoteService {
    notes: Notes,
    storage: FileSystemStorage,
}

impl NoteService {
    fn new() -> Result<Self, String> {
        // Determine storage path using ProjectDirs
        let proj_dirs =
            ProjectDirs::from("", "", "qot").ok_or("Failed to determine storage directory")?;
        let base_path = proj_dirs.data_dir().to_path_buf();

        let storage = FileSystemStorage::new(base_path).map_err(|e| format!("{}", e))?;

        Ok(Self {
            notes: Notes::new(),
            storage,
        })
    }

    fn create(&mut self, content: &str) -> Result<Note, String> {
        // Create note in memory
        let note = self.notes.create(content).map_err(|e| format!("{:?}", e))?;

        // Persist to storage
        let bytes = self
            .notes
            .to_bytes(&note.id)
            .map_err(|e| format!("{:?}", e))?;

        self.storage
            .set(&note.id, &bytes)
            .map_err(|e| format!("{}", e))?;

        Ok(note)
    }

    fn list(&mut self) -> Result<Vec<Note>, String> {
        let uuids = self.storage.list().map_err(|e| format!("{}", e))?;

        let mut note_list = Vec::new();
        for uuid in uuids {
            if let Some(bytes) = self.storage.get(&uuid).map_err(|e| format!("{}", e))? {
                let note = self
                    .notes
                    .from_bytes(&bytes)
                    .map_err(|e| format!("{:?}", e))?;

                note_list.push(note);
            }
        }

        Ok(note_list)
    }
}
