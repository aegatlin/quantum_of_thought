mod service;
mod storage;

use clap::{CommandFactory, Parser, Subcommand};
use service::NoteService;

#[derive(Parser)]
#[command(name = "qot")]
#[command(about = "Quantum of Thought - A note capture CLI")]
#[command(after_help = "EXAMPLES:\n  \
    qot get milk          # Create a new note (implicit)\n  \
    qot add buy eggs      # Create a new note (explicit)\n  \
    qot list              # Show all notes\n  \
    qot delete 2          # Delete note #2")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Note content (when not using a subcommand)
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, hide = true)]
    content: Vec<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new note
    Add {
        /// The content of the note
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        content: Vec<String>,
    },
    /// List all notes with their indices
    List,
    /// Delete a note by its index number
    Delete {
        /// The index number shown in 'qot list' (e.g., 1, 2, 3)
        index: usize,
    },
}

fn main() {
    let cli = Cli::parse();

    let mut note_service = NoteService::new().unwrap_or_else(|e| {
        eprintln!("Failed to initialize service: {}", e);
        std::process::exit(1);
    });

    match cli.command {
        Some(Commands::Add { content }) => {
            if content.is_empty() {
                eprintln!("Error: note content cannot be empty");
                std::process::exit(1);
            }
            let note_content = content.join(" ");
            create_note(&mut note_service, &note_content);
        }
        Some(Commands::List) => {
            list_notes(&mut note_service);
        }
        Some(Commands::Delete { index }) => {
            delete_note(&mut note_service, index);
        }
        None => {
            // No subcommand - treat as implicit note creation
            if cli.content.is_empty() {
                // Show help when no arguments provided
                Cli::command().print_help().unwrap();
                std::process::exit(0);
            }
            let note_content = cli.content.join(" ");
            create_note(&mut note_service, &note_content);
        }
    }
}

fn create_note(note_service: &mut NoteService, content: &str) {
    match note_service.create(content) {
        Ok(note) => {
            println!("Created note: {}", note.content);
        }
        Err(e) => {
            eprintln!("Error creating note: {}", e);
            std::process::exit(1);
        }
    }
}

fn list_notes(note_service: &mut NoteService) {
    match note_service.list() {
        Ok(notes) => {
            if notes.is_empty() {
                println!("No notes yet. Create one with: qot get milk");
            } else {
                for (i, note) in notes.iter().enumerate() {
                    println!("{}. {}", i + 1, note.content);
                }
            }
        }
        Err(e) => {
            eprintln!("Error listing notes: {}", e);
            std::process::exit(1);
        }
    }
}

fn delete_note(note_service: &mut NoteService, index: usize) {
    match note_service.delete_by_index(index) {
        Ok(content) => {
            println!("Deleted: {}", content);
        }
        Err(e) => {
            eprintln!("Error deleting note: {}", e);
            std::process::exit(1);
        }
    }
}
