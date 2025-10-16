use automerge::{transaction::Transactable, AutoCommit, ObjType, ReadDoc, ROOT};
use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Note {
    doc: AutoCommit,
}

#[wasm_bindgen]
impl Note {
    #[wasm_bindgen(constructor)]
    pub fn new(content: &str) -> Result<Note, JsValue> {
        let id = Uuid::now_v7().to_string();
        let mut doc = AutoCommit::new();

        doc.put(ROOT, "id", &id)
            .map_err(|e| JsValue::from_str(&format!("Failed to set id: {:?}", e)))?;

        let content_text = doc
            .put_object(ROOT, "content", ObjType::Text)
            .map_err(|e| JsValue::from_str(&format!("Failed to create content: {:?}", e)))?;

        doc.update_text(&content_text, content)
            .map_err(|e| JsValue::from_str(&format!("Failed to set content: {:?}", e)))?;

        Ok(Self { doc })
    }

    pub fn id(&self) -> Result<String, JsValue> {
        match self.doc.get(ROOT, "id") {
            Ok(Some((v, _))) => Ok(v.to_string()),
            Ok(None) => Err(JsValue::from_str("id not found")),
            Err(e) => Err(JsValue::from_str(&format!("Failed to get id: {:?}", e))),
        }
    }

    pub fn content(&self) -> Result<String, JsValue> {
        match self.doc.get(ROOT, "content") {
            Ok(Some((_, exid))) => self
                .doc
                .text(exid)
                .map_err(|e| JsValue::from_str(&format!("Failed to read text: {:?}", e))),
            Ok(None) => Err(JsValue::from_str("content not found")),
            Err(e) => Err(JsValue::from_str(&format!(
                "Failed to get content: {:?}",
                e
            ))),
        }
    }

    pub fn into_bytes(&mut self) -> Vec<u8> {
        self.doc.save()
    }

    pub fn from_bytes(data: &[u8]) -> Result<Note, JsValue> {
        AutoCommit::load(data)
            .map(|doc| Self { doc })
            .map_err(|e| JsValue::from_str(&format!("Failed to load: {:?}", e)))
    }

    pub fn merge(&mut self, other_bytes: &[u8]) -> Result<(), JsValue> {
        let mut other = AutoCommit::load(other_bytes)
            .map_err(|e| JsValue::from_str(&format!("Failed to load: {:?}", e)))?;

        self.doc
            .merge(&mut other)
            .map_err(|e| JsValue::from_str(&format!("Failed to merge: {:?}", e)))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_creation() {
        let expected_content = "hello, world";
        let note = Note::new(expected_content);
        assert!(note.is_ok());

        let note = note.unwrap();
        let content = note.content();
        assert!(content.is_ok());
        assert_eq!(content.unwrap(), expected_content);

        let id = note.id();
        assert!(id.is_ok());
        assert!(id.unwrap().len() > 0);
    }

    #[test]
    fn test_note_id_is_unique() {
        let note1 = Note::new("first").unwrap();
        let note2 = Note::new("second").unwrap();

        let id1 = note1.id().unwrap();
        let id2 = note2.id().unwrap();

        assert_ne!(id1, id2);
    }

    #[test]
    fn test_note_serialization() {
        let expected_content = "test content";
        let mut note = Note::new(expected_content).unwrap();
        let original_id = note.id().unwrap();

        // Serialize
        let bytes = note.into_bytes();
        assert!(bytes.len() > 0);

        // Deserialize
        let restored_note = Note::from_bytes(&bytes);
        assert!(restored_note.is_ok());

        let restored_note = restored_note.unwrap();
        assert_eq!(restored_note.id().unwrap(), original_id);
        assert_eq!(restored_note.content().unwrap(), expected_content);
    }

    #[test]
    fn test_empty_content() {
        let note = Note::new("");
        assert!(note.is_ok());

        let note = note.unwrap();
        assert_eq!(note.content().unwrap(), "");
    }

    #[test]
    fn test_multiline_content() {
        let content = "line 1\nline 2\nline 3";
        let note = Note::new(content).unwrap();
        assert_eq!(note.content().unwrap(), content);
    }

    #[test]
    fn test_merge() {
        // Create two notes with the same ID (simulate same note on different devices)
        let mut note1 = Note::new("initial content").unwrap();
        let note1_id = note1.id().unwrap();

        // Serialize note1's initial state
        let note1_bytes = note1.into_bytes();

        // Create note2 from the same initial state (simulating a second device)
        let mut note2 = Note::from_bytes(&note1_bytes).unwrap();
        assert_eq!(note2.id().unwrap(), note1_id);

        // Both notes should have the same content initially
        assert_eq!(note1.content().unwrap(), "initial content");
        assert_eq!(note2.content().unwrap(), "initial content");

        // Serialize note2's state
        let note2_bytes = note2.into_bytes();

        // Merge note2 into note1
        let result = note1.merge(&note2_bytes);
        assert!(result.is_ok());

        // After merging identical states, content should be unchanged
        assert_eq!(note1.content().unwrap(), "initial content");
        assert_eq!(note1.id().unwrap(), note1_id);
    }
}
