use std::collections::HashMap;

use crate::note_crdt::NoteCrdt;

mod note_crdt;
#[cfg(feature = "wasm")]
pub mod wasm;

#[derive(Debug)]
pub enum NoteError {
    NotFound(String),
    ExtractionError(String),
    DeserializationError(String),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "wasm", derive(serde::Serialize, serde::Deserialize))]
pub struct Note {
    pub id: String,
    pub content: String,
}

impl TryFrom<&NoteCrdt> for Note {
    type Error = NoteError;

    fn try_from(note_crdt: &NoteCrdt) -> Result<Self, NoteError> {
        Ok(Self {
            id: note_crdt.id()?,
            content: note_crdt.content()?,
        })
    }
}

pub struct Notes {
    note_crdts: HashMap<String, NoteCrdt>,
}

impl Notes {
    pub fn new() -> Self {
        Self {
            note_crdts: HashMap::new(),
        }
    }

    /// Creates a new `Note` from string content
    pub fn create(&mut self, content: &str) -> Result<Note, NoteError> {
        let note_crdt = NoteCrdt::new(content)?;
        let note: Note = (&note_crdt).try_into()?;
        self.note_crdts.insert(note.id.clone(), note_crdt);
        Ok(note)
    }

    /// Adds a new `Note` from bytes. These bytes likely come from a persistence
    /// layer.
    pub fn add(&self, data: &[u8]) -> Result<Note, NoteError> {
        let note_crdt = NoteCrdt::from_bytes(data)?;
        let note: Note = (&note_crdt).try_into()?;
        Ok(note)
    }

    /// Gets a `Note` by `id`
    pub fn get(&self, id: &str) -> Result<Note, NoteError> {
        match self.note_crdts.get(id) {
            Some(note_crdt) => note_crdt.try_into(),
            None => Err(NoteError::NotFound(format!(
                "note not found with id: {}",
                id
            ))),
        }
    }

    /// Gets the bytes representation of a `Note` by `id`.
    pub fn get_bytes(&mut self, id: &str) -> Result<Vec<u8>, NoteError> {
        match self.note_crdts.get_mut(id) {
            Some(note_crdt) => Ok(note_crdt.to_bytes()),
            None => Err(NoteError::NotFound(format!(
                "note not found with id: {}",
                id
            ))),
        }
    }

    /// Lists all notes in `Notes`.
    ///
    /// Internally the notes are stored as `NoteCrdt`s and so there is also a
    /// mapping of each `NoteCrdt` into `Note` via `.try_into()`.
    pub fn list(&self) -> Result<Vec<Note>, NoteError> {
        let mut notes: Vec<Note> = self
            .note_crdts
            .values()
            .map(|note_crdt| note_crdt.try_into())
            .collect::<Result<Vec<Note>, NoteError>>()?;

        // Sort by ID (UUIDv7 is chronologically sortable)
        notes.sort_by(|a, b| a.id.cmp(&b.id));

        Ok(notes)
    }

    // Deletes a `Note` by `id` from `Notes`.
    pub fn delete(&mut self, id: &str) -> Result<(), NoteError> {
        match self.note_crdts.remove(id) {
            Some(_) => Ok(()),
            None => Err(NoteError::NotFound(format!(
                "note not found with id: {}",
                id
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note() {
        let expected_content = "hello, world";
        let note_crdt = NoteCrdt::new(expected_content).unwrap();

        let note: Note = (&note_crdt).try_into().unwrap();

        assert_eq!(note.content, expected_content);
        assert!(note.id.len() > 0);
    }

    #[test]
    fn test_notes_create() {
        let expected_content = "hello, world";
        let mut notes = Notes::new();

        let note = notes.create(expected_content);
        assert!(note.is_ok_and(|n| n.content == expected_content));
    }

    #[test]
    fn test_notes_get() {
        let expected_content = "hello, world";
        let mut notes = Notes::new();
        let note_id = notes.create(expected_content).unwrap().id;

        let note = notes.get(&note_id);
        assert!(note.is_ok_and(|n| n.content == expected_content))
    }

    #[test]
    fn test_notes_list() {
        let mut notes = Notes::new();

        // List should be empty initially
        let empty_list = notes.list().unwrap();
        assert_eq!(empty_list.len(), 0);

        // Create three notes
        let note1 = notes.create("First note").unwrap();
        let note2 = notes.create("Second note").unwrap();
        let note3 = notes.create("Third note").unwrap();

        // List should contain all three notes
        let list = notes.list().unwrap();
        assert_eq!(list.len(), 3);

        // Notes should be sorted by ID (chronologically via UUIDv7)
        assert_eq!(list[0].id, note1.id);
        assert_eq!(list[0].content, "First note");

        assert_eq!(list[1].id, note2.id);
        assert_eq!(list[1].content, "Second note");

        assert_eq!(list[2].id, note3.id);
        assert_eq!(list[2].content, "Third note");
    }

    #[test]
    fn test_notes_delete() {
        let mut notes = Notes::new();

        // Create a note
        let note = notes.create("Test note").unwrap();
        let note_id = note.id.clone();

        // Verify it exists
        assert!(notes.get(&note_id).is_ok());
        assert_eq!(notes.list().unwrap().len(), 1);

        // Delete it
        let result = notes.delete(&note_id);
        assert!(result.is_ok());

        // Verify it's gone
        assert!(notes.get(&note_id).is_err());
        assert_eq!(notes.list().unwrap().len(), 0);

        // Deleting again should return error
        let result = notes.delete(&note_id);
        assert!(result.is_err());
        match result {
            Err(NoteError::NotFound(_)) => (),
            _ => panic!("Expected NotFound error"),
        }
    }
}
