use automerge::{transaction::Transactable, AutoCommit, ObjType, ReadDoc, ROOT};
use uuid::Uuid;

use crate::NoteError;

pub(crate) struct NoteCrdt {
    doc: AutoCommit,
}

impl NoteCrdt {
    pub(crate) fn new(content: &str) -> Result<Self, NoteError> {
        let id = Uuid::now_v7().to_string();
        let mut doc = AutoCommit::new();

        let _ = doc.put(ROOT, "id", &id);

        let content_text = doc
            .put_object(automerge::ROOT, "content", ObjType::Text)
            .unwrap();

        let _ = doc.update_text(&content_text, content);

        Ok(Self { doc })
    }

    pub(crate) fn id(&self) -> Result<String, NoteError> {
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

    pub(crate) fn content(&self) -> Result<String, NoteError> {
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

    pub(crate) fn to_bytes(&mut self) -> Vec<u8> {
        self.doc.save()
    }

    /// &[u8] allegedly is the most friendly FFI input type. In JS it is a
    /// Uint8Array. In Swift it is `Data`. In Kotlin it is ByteArray. In
    /// Elixir/Erlang it is binary().
    pub(crate) fn from_bytes(data: &[u8]) -> Result<Self, NoteError> {
        match AutoCommit::load(data) {
            Ok(doc) => Ok(Self { doc }),
            Err(automerge_error) => {
                Err(NoteError::DeserializationError(automerge_error.to_string()))
            }
        }
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
}
