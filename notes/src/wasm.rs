use crate::Notes;
use wasm_bindgen::prelude::*;

/// WASM wrapper around the Notes library
#[wasm_bindgen]
pub struct WasmNotes {
    inner: Notes,
}

#[wasm_bindgen]
impl WasmNotes {
    /// Create a new WasmNotes instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: Notes::new(),
        }
    }

    /// Create a new note with the given content
    /// Returns a JsValue containing the note (id and content)
    pub fn create(&mut self, content: &str) -> Result<JsValue, JsValue> {
        let note = self
            .inner
            .create(content)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

        serde_wasm_bindgen::to_value(&note).map_err(|e| JsValue::from_str(&format!("{}", e)))
    }

    /// List all notes
    /// Returns a JsValue containing an array of notes
    pub fn list(&self) -> Result<JsValue, JsValue> {
        let notes = self
            .inner
            .list()
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

        serde_wasm_bindgen::to_value(&notes).map_err(|e| JsValue::from_str(&format!("{}", e)))
    }

    /// Get a specific note by ID
    /// Returns a JsValue containing the note (id and content)
    pub fn get(&mut self, id: &str) -> Result<JsValue, JsValue> {
        let note = self
            .inner
            .get(id)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

        serde_wasm_bindgen::to_value(&note).map_err(|e| JsValue::from_str(&format!("{}", e)))
    }

    /// Get the serialized bytes for a specific note by ID
    /// Returns a Uint8Array containing the automerge bytes
    pub fn get_bytes(&mut self, id: &str) -> Result<Vec<u8>, JsValue> {
        self.inner
            .get_bytes(id)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    /// Add a note from serialized bytes
    /// Returns a JsValue containing the loaded note (id and content)
    pub fn add(&mut self, data: &[u8]) -> Result<JsValue, JsValue> {
        let note = self
            .inner
            .add(data)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;

        serde_wasm_bindgen::to_value(&note).map_err(|e| JsValue::from_str(&format!("{}", e)))
    }

    /// Delete a note by ID
    pub fn delete(&mut self, id: &str) -> Result<(), JsValue> {
        self.inner
            .delete(id)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }
}

#[cfg(all(test, target_arch = "wasm32"))]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_wasm_notes_can_be_created() {
        let _notes = WasmNotes::new();
        // If we get here without panicking, the test passes
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn test_create_note_returns_valid_js_value() {
        let mut notes = WasmNotes::new();
        let result = notes.create("test content");

        // Should succeed and return a JsValue
        assert!(result.is_ok());

        let js_value = result.unwrap();

        // Should be an object (not undefined, null, etc.)
        assert!(js_value.is_object());
    }

    #[wasm_bindgen_test]
    fn test_list_returns_array() {
        let notes = WasmNotes::new();
        let result = notes.list();

        assert!(result.is_ok());

        let js_value = result.unwrap();

        // Should be an array
        assert!(js_sys::Array::is_array(&js_value));
    }
}
