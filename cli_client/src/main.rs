mod service;
mod storage;

use clap::Parser;
use service::NoteService;

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
    } else if first_arg == "delete" {
        if cli.args.len() < 2 {
            eprintln!("Usage: qot delete <index>");
            std::process::exit(1);
        }

        let index_str = &cli.args[1];
        let index: usize = match index_str.parse() {
            Ok(i) => i,
            Err(_) => {
                eprintln!("Error: index must be a number");
                std::process::exit(1);
            }
        };

        match note_service.delete_by_index(index) {
            Ok(content) => {
                println!("Deleted: {}", content);
            }
            Err(e) => {
                eprintln!("Error deleting note: {}", e);
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
