use automerge::{
    transaction::Transactable, AutoCommit, ObjType, ReadDoc, ScalarValue, Value, ROOT,
};
use std::{borrow::Cow, collections::HashMap};
use uuid::Uuid;

struct Note {
    /// Internally a UUIDv7 is used, but for FFI compatibility it is a `String`
    pub id: String,
    doc: AutoCommit,
}

impl Note {
    fn new(content: &str) -> Self {
        let id = Uuid::now_v7().to_string();
        let mut doc = AutoCommit::new();

        let _ = doc.put(ROOT, "id", &id);

        let content_text = doc
            .put_object(automerge::ROOT, "content", ObjType::Text)
            .unwrap();

        let _ = doc.update_text(&content_text, content);

        Self { id, doc }
    }

    fn extract_id(doc: &AutoCommit) -> String {
        let id = match doc.get(ROOT, "id") {
            Ok(it) => match it {
                Some((Value::Scalar(Cow::Owned(ScalarValue::Str(id))), _id_exid)) => id,
                _ => panic!("idk"),
            },
            Err(_err) => panic!("idk"),
        };

        id.to_string()
    }

    fn extract_content(&self) -> String {
        let content_exid = match self.doc.get(ROOT, "content") {
            Ok(it) => match it {
                Some((Value::Object(ObjType::Text), content)) => content,
                _ => panic!("idk"),
            },
            Err(_err) => panic!("idk"),
        };

        let content = self.doc.text(content_exid).unwrap();

        content
    }

    fn to_bytes(&mut self) -> Vec<u8> {
        self.doc.save()
    }

    /// &[u8] allegedly is the most friendly FFI input type. In JS it is a
    /// Uint8Array. In Swift it is `Data`. In Kotlin it is ByteArray. In
    /// Elixir/Erlang it is binary().
    fn from_bytes(data: &[u8]) -> Result<Self, String> {
        let doc = AutoCommit::load(data).map_err(|e| format!("Failed to load: {:?}", e))?;

        let id = Note::extract_id(&doc);

        Ok(Self { id, doc })
    }
}

#[derive(Debug, Clone)]
pub struct NoteData {
    pub id: String,
    pub content: String,
}

impl From<&Note> for NoteData {
    fn from(note: &Note) -> Self {
        NoteData {
            id: note.id.clone(),
            content: note.extract_content(),
        }
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

    pub fn create(&mut self, content: &str) -> String {
        let note = Note::new(content);
        let id = note.id.clone();
        self.notes.insert(id.clone(), note);
        id
    }

    pub fn get(&self, id: &str) -> Option<NoteData> {
        self.notes.get(id).map(|note| note.into())
    }

    pub fn to_bytes(&mut self, id: &str) -> Option<Vec<u8>> {
        self.notes.get_mut(id).map(|note| note.to_bytes())
    }

    pub fn from_bytes(&mut self, data: &[u8]) -> Result<String, String> {
        let note = Note::from_bytes(data)?;
        let id = note.id.clone();
        self.notes.insert(id.clone(), note);
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_note() {
        let expected_content = "hello, world";
        let mut note_store = NoteStore::new();
        let note_id = note_store.create(expected_content);
        assert!(!note_id.is_empty());

        let note = note_store.get(&note_id);
        assert!(note.is_some());

        let note_data = note.unwrap();
        assert_eq!(note_data.id, note_id);
        assert_eq!(note_data.content, expected_content);
    }
}
