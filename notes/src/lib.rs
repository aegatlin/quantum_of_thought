use automerge::{transaction::Transactable, AutoCommit, ObjType, ReadDoc, ROOT};
use std::collections::HashMap;
use uuid::Uuid;

struct NoteCrdt {
    doc: AutoCommit,
}

#[derive(Debug)]
pub enum NoteError {
    NotFound(String),
    ExtractionError(String),
    DeserializationError(String),
}

impl NoteCrdt {
    fn new(content: &str) -> Result<Self, NoteError> {
        let id = Uuid::now_v7().to_string();
        let mut doc = AutoCommit::new();

        let _ = doc.put(ROOT, "id", &id);

        let content_text = doc
            .put_object(automerge::ROOT, "content", ObjType::Text)
            .unwrap();

        let _ = doc.update_text(&content_text, content);

        Ok(Self { doc })
    }

    fn id(&self) -> Result<String, NoteError> {
        match self.doc.get(ROOT, "id") {
            Ok(res) => match res {
                Some((v, _)) => Ok(v.to_string()),
                None => Err(NoteError::ExtractionError(
                    "problem extracting id".to_string(),
                )),
            },
            Err(err) => Err(NoteError::ExtractionError(err.to_string())),
        }
    }

    fn content(&self) -> Result<String, NoteError> {
        match self.doc.get(ROOT, "content") {
            Ok(res) => match res {
                Some((_, exid)) => match self.doc.text(exid) {
                    Ok(s) => Ok(s),
                    Err(err) => Err(NoteError::ExtractionError(err.to_string())),
                },
                None => Err(NoteError::ExtractionError(
                    "problem extracting content".to_string(),
                )),
            },
            Err(err) => Err(NoteError::ExtractionError(err.to_string())),
        }
    }

    fn to_bytes(&mut self) -> Vec<u8> {
        self.doc.save()
    }

    /// &[u8] allegedly is the most friendly FFI input type. In JS it is a
    /// Uint8Array. In Swift it is `Data`. In Kotlin it is ByteArray. In
    /// Elixir/Erlang it is binary().
    fn from_bytes(data: &[u8]) -> Result<Self, NoteError> {
        match AutoCommit::load(data) {
            Ok(doc) => Ok(Self { doc }),
            Err(automerge_error) => {
                Err(NoteError::DeserializationError(automerge_error.to_string()))
            }
        }
    }
}

#[derive(Debug, Clone)]
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

    pub fn create(&mut self, content: &str) -> Result<Note, NoteError> {
        let note_crdt = NoteCrdt::new(content)?;
        let note: Note = (&note_crdt).try_into()?;
        self.note_crdts.insert(note.id.clone(), note_crdt);
        Ok(note)
    }

    pub fn get(&self, id: &str) -> Result<Note, NoteError> {
        match self.note_crdts.get(id) {
            Some(note_crdt) => note_crdt.try_into(),
            None => Err(NoteError::NotFound(format!(
                "note not found with id: {}",
                id
            ))),
        }
    }

    pub fn to_bytes(&mut self, id: &str) -> Result<Vec<u8>, NoteError> {
        match self.note_crdts.get_mut(id) {
            Some(note_crdt) => Ok(note_crdt.to_bytes()),
            None => Err(NoteError::NotFound(format!(
                "note not found with id: {}",
                id
            ))),
        }
    }

    pub fn from_bytes(&mut self, data: &[u8]) -> Result<Note, NoteError> {
        let note_crdt = NoteCrdt::from_bytes(data)?;
        let note: Note = (&note_crdt).try_into()?;
        Ok(note)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_crdt() {
        let expected_content = "hello, world";
        let result = NoteCrdt::new(expected_content);
        assert!(result.is_ok());

        let note_crdt = result.unwrap();
        let content = note_crdt.content();
        assert!(content.is_ok_and(|c| c == expected_content));

        let id = note_crdt.id();
        assert!(id.is_ok_and(|i| i.len() > 0))
    }

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
}
