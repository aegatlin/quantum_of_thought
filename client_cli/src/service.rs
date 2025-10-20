use crate::storage::{FileSystemStorage, Storage};
use directories::ProjectDirs;
use std::collections::HashMap;

// Simple view struct for Note data
#[derive(Clone, Debug)]
pub struct Note {
    pub id: String,
    pub content: String,
}

pub struct NoteService {
    notes: HashMap<String, crdt_note::Note>,
    storage: FileSystemStorage,
}

impl NoteService {
    pub fn new() -> Result<Self, String> {
        // Determine storage path using ProjectDirs
        let proj_dirs =
            ProjectDirs::from("", "", "qot").ok_or("Failed to determine storage directory")?;
        let base_path = proj_dirs.data_dir().to_path_buf();

        let storage = FileSystemStorage::new(base_path).map_err(|e| format!("{}", e))?;

        Ok(Self {
            notes: HashMap::new(),
            storage,
        })
    }

    pub fn create(&mut self, content: &str) -> Result<Note, String> {
        // Create note using crdt_note
        let crdt_note = crdt_note::Note::new(content);
        let note_id = crdt_note.id();
        let note_content = crdt_note.content();

        // Validate the note was created successfully
        if note_id.is_empty() {
            return Err("Failed to create note".to_string());
        }

        // Persist to storage
        let bytes = crdt_note::Note::into(&crdt_note);
        self.storage
            .set(&note_id, &bytes)
            .map_err(|e| format!("{}", e))?;

        // Store in memory
        let crdt_note_copy = crdt_note::Note::from(&bytes);
        self.notes.insert(note_id.clone(), crdt_note_copy);

        Ok(Note {
            id: note_id,
            content: note_content,
        })
    }

    pub fn list(&mut self) -> Result<Vec<Note>, String> {
        let mut uuids = self.storage.list().map_err(|e| format!("{}", e))?;

        // Sort by UUIDv7 timestamp (oldest first)
        uuids.sort();

        let mut note_list = Vec::new();
        for uuid in uuids {
            if let Some(bytes) = self.storage.get(&uuid).map_err(|e| format!("{}", e))? {
                // Deserialize from storage
                let crdt_note = crdt_note::Note::from(&bytes);
                let note_id = crdt_note.id();
                let note_content = crdt_note.content();

                // Store in memory cache
                self.notes.insert(note_id.clone(), crdt_note);

                note_list.push(Note {
                    id: note_id,
                    content: note_content,
                });
            }
        }

        Ok(note_list)
    }

    pub fn delete_by_index(&mut self, index: usize) -> Result<String, String> {
        // Get current sorted list
        let notes = self.list()?;

        // Check if index is valid (1-based)
        if index == 0 || index > notes.len() {
            return Err(format!("Index {} out of range (1-{})", index, notes.len()));
        }

        // Get the note at the given index (convert to 0-based)
        let note = &notes[index - 1];
        let note_id = note.id.clone();
        let note_content = note.content.clone();

        // Delete from storage
        self.storage
            .delete(&note_id)
            .map_err(|e| format!("{}", e))?;

        Ok(note_content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;

    #[test]
    fn test_list_returns_notes_sorted_by_creation_time() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage = FileSystemStorage::new(temp_dir.path().to_path_buf()).unwrap();

        let mut service = NoteService {
            notes: HashMap::new(),
            storage,
        };

        // Create first note
        let note1 = service.create("First note").unwrap();

        // Small delay to ensure different timestamps
        sleep(Duration::from_millis(10));

        // Create second note
        let note2 = service.create("Second note").unwrap();

        // Small delay to ensure different timestamps
        sleep(Duration::from_millis(10));

        // Create third note
        let note3 = service.create("Third note").unwrap();

        // List all notes
        let notes = service.list().unwrap();

        // Verify we have all three notes
        assert_eq!(notes.len(), 3);

        // Verify they're in chronological order (oldest first)
        assert_eq!(notes[0].id, note1.id);
        assert_eq!(notes[0].content, "First note");

        assert_eq!(notes[1].id, note2.id);
        assert_eq!(notes[1].content, "Second note");

        assert_eq!(notes[2].id, note3.id);
        assert_eq!(notes[2].content, "Third note");
    }

    #[test]
    fn test_delete_by_index() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage = FileSystemStorage::new(temp_dir.path().to_path_buf()).unwrap();

        let mut service = NoteService {
            notes: HashMap::new(),
            storage,
        };

        // Create three notes
        service.create("First note").unwrap();
        sleep(Duration::from_millis(10));
        service.create("Second note").unwrap();
        sleep(Duration::from_millis(10));
        service.create("Third note").unwrap();

        // Verify we have 3 notes
        assert_eq!(service.list().unwrap().len(), 3);

        // Delete the second note (index 2)
        let deleted_content = service.delete_by_index(2).unwrap();
        assert_eq!(deleted_content, "Second note");

        // Verify we now have 2 notes
        let notes = service.list().unwrap();
        assert_eq!(notes.len(), 2);
        assert_eq!(notes[0].content, "First note");
        assert_eq!(notes[1].content, "Third note");
    }

    #[test]
    fn test_delete_by_index_out_of_range() {
        let temp_dir = tempfile::tempdir().unwrap();
        let storage = FileSystemStorage::new(temp_dir.path().to_path_buf()).unwrap();

        let mut service = NoteService {
            notes: HashMap::new(),
            storage,
        };

        // Create one note
        service.create("Only note").unwrap();

        // Try to delete index 0 (invalid)
        let result = service.delete_by_index(0);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("out of range"));

        // Try to delete index 2 (out of range)
        let result = service.delete_by_index(2);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("out of range"));

        // Verify note is still there
        assert_eq!(service.list().unwrap().len(), 1);
    }
}
