use automerge::{transaction::Transactable, AutoCommit, ObjType, ReadDoc, ROOT};
use std::collections::HashMap;
use uuid::Uuid;

struct Note {
    doc: AutoCommit,
}

impl Note {
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

#[derive(Debug)]
pub enum NoteError {
    NotFound(String),
    ExtractionError(String),
    DeserializationError(String),
}

#[derive(Debug, Clone)]
pub struct NoteData {
    pub id: String,
    pub content: String,
}

impl TryFrom<&Note> for NoteData {
    type Error = NoteError;

    fn try_from(note: &Note) -> Result<Self, NoteError> {
        Ok(Self {
            id: note.id()?,
            content: note.content()?,
        })
    }
}

pub struct NoteStore {
    notes: HashMap<String, Note>,
}

impl NoteStore {
    pub fn new() -> Self {
        Self {
            notes: HashMap::new(),
        }
    }

    pub fn create(&mut self, content: &str) -> Result<NoteData, NoteError> {
        let note = Note::new(content)?;
        let note_data: NoteData = (&note).try_into()?;
        self.notes.insert(note_data.id.clone(), note);
        Ok(note_data)
    }

    pub fn get(&self, id: &str) -> Result<NoteData, NoteError> {
        match self.notes.get(id) {
            Some(note) => note.try_into(),
            None => Err(NoteError::NotFound(format!(
                "note not found with id: {}",
                id
            ))),
        }
    }

    pub fn to_bytes(&mut self, id: &str) -> Result<Vec<u8>, NoteError> {
        match self.notes.get_mut(id) {
            Some(note) => Ok(note.to_bytes()),
            None => Err(NoteError::NotFound(format!(
                "note not found with id: {}",
                id
            ))),
        }
    }

    pub fn from_bytes(&mut self, data: &[u8]) -> Result<NoteData, NoteError> {
        let note = Note::from_bytes(data)?;
        let note_data: NoteData = (&note).try_into()?;
        Ok(note_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note() {
        let expected_content = "hello, world";
        let result = Note::new(expected_content);
        assert!(result.is_ok());

        let note = result.unwrap();
        let content = note.content();
        assert!(content.is_ok_and(|c| c == expected_content));

        let id = note.id();
        assert!(id.is_ok_and(|i| i.len() > 0))
    }

    #[test]
    fn test_note_data() {
        let expected_content = "hello, world";
        let note = Note::new(expected_content).unwrap();

        let note_data: NoteData = (&note).try_into().unwrap();

        assert_eq!(note_data.content, expected_content);
        assert!(note_data.id.len() > 0);
    }

    #[test]
    fn test_note_store_create() {
        let expected_content = "hello, world";
        let mut note_store = NoteStore::new();

        let note = note_store.create(expected_content);
        assert!(note.is_ok_and(|n| n.content == expected_content));
    }

    #[test]
    fn test_note_store_get() {
        let expected_content = "hello, world";
        let mut note_store = NoteStore::new();
        let id = note_store.create(expected_content).unwrap().id;

        let note = note_store.get(&id);
        assert!(note.is_ok_and(|n| n.content == expected_content))
    }
}
