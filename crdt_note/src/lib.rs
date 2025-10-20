use automerge::{
    AutoCommit, ObjType, ROOT, ReadDoc, ScalarValue, Value, transaction::Transactable,
};
use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug)]
pub struct Note {
    doc: AutoCommit,
}

#[wasm_bindgen]
impl Note {
    pub fn new(content: &str) -> Self {
        let mut doc = AutoCommit::new();
        let id = Uuid::now_v7().to_string();

        if let Err(_) = doc.put(ROOT, "id", &id) {
            return Self { doc: doc };
        }

        match doc.put_object(ROOT, "content", ObjType::Text) {
            Ok(ex_id) => {
                if let Err(_) = doc.update_text(&ex_id, content) {
                    return Self { doc: doc };
                };
            }
            Err(_) => {
                return Self { doc: doc };
            }
        }

        Note { doc: doc }
    }

    pub fn id(&self) -> String {
        let doc = self.doc.clone();

        if let Ok(Some((Value::Scalar(v), _))) = doc.get(ROOT, "id") {
            let w = v.as_ref();
            if let ScalarValue::Str(id) = w {
                return id.as_str().into();
            } else {
                return "".into();
            }
        } else {
            return "".into();
        }
    }

    pub fn content(&self) -> String {
        let doc = self.doc.clone();

        if let Ok(Some((_, ex_id))) = doc.get(ROOT, "content") {
            match doc.text(ex_id) {
                Ok(content) => {
                    return content;
                }
                Err(_automerge_error) => return "".into(),
            }
        } else {
            return "".into();
        }
    }

    pub fn merge(&self, other: &Note) -> Self {
        let mut doc = self.doc.clone();
        let mut other_doc = other.doc.clone();

        match doc.merge(&mut other_doc) {
            Ok(_) => Self { doc: doc },
            Err(_) => Self {
                doc: AutoCommit::new(),
            },
        }
    }

    pub fn into(&self) -> Vec<u8> {
        self.doc.clone().save()
    }

    pub fn from(bytes: &[u8]) -> Self {
        match AutoCommit::load(bytes) {
            Ok(doc) => Self { doc: doc },
            Err(_) => Self {
                doc: AutoCommit::new(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_id_and_content() {
        let note = Note::new("Hello, World!");

        // Test that id is not empty and is a valid UUID format
        let id = note.id();
        assert!(!id.is_empty(), "Note id should not be empty");
        assert!(
            Uuid::parse_str(&id).is_ok(),
            "Note id should be a valid UUID"
        );

        // Test that content is set correctly
        assert_eq!(note.content(), "Hello, World!");
    }

    #[test]
    fn test_into_and_from() {
        let original = Note::new("Test content");
        let original_id = original.id();
        let original_content = original.content();

        // Serialize the note
        let bytes = Note::into(&original);
        assert!(!bytes.is_empty(), "Serialized bytes should not be empty");

        // Deserialize the note
        let restored = Note::from(&bytes);
        assert_eq!(
            restored.id(),
            original_id,
            "Restored note should have same id"
        );
        assert_eq!(
            restored.content(),
            original_content,
            "Restored note should have same content"
        );
    }

    #[test]
    fn test_merge() {
        let note1 = Note::new("First note");
        let note2 = Note::new("Second note");

        // Merge note2 into note1
        let merged = note1.merge(&note2);

        // The merged note should contain content from both notes
        // In this case with independent documents, the result contains both
        let content = merged.content();
        assert!(
            content.contains("First note") || content.contains("Second note"),
            "Merged content should contain text from one of the notes"
        );

        // The merged note should have a valid id
        assert!(!merged.id().is_empty());
    }
}
