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

    pub fn update(&self, new_content: &str) -> Self {
        let mut doc = self.doc.clone();

        if let Ok(Some((_, ex_id))) = doc.get(ROOT, "content") {
            match doc.update_text(&ex_id, new_content) {
                Ok(_) => Self { doc: doc },
                Err(_) => Note::empty(),
            }
        } else {
            Note::empty()
        }
    }

    pub fn merge(&self, other: &Note) -> Self {
        let mut doc = self.doc.clone();
        let mut other_doc = other.doc.clone();

        match doc.merge(&mut other_doc) {
            Ok(_) => Self { doc: doc },
            Err(_) => Note::empty(),
        }
    }

    pub fn into(&self) -> Vec<u8> {
        self.doc.clone().save()
    }

    pub fn from(bytes: &[u8]) -> Self {
        match AutoCommit::load(bytes) {
            Ok(doc) => Self { doc: doc },
            Err(_) => Note::empty(),
        }
    }

    fn empty() -> Self {
        Self {
            doc: AutoCommit::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_and_id_and_content() {
        let expected_content = "expected content!";
        let note = Note::new(expected_content);

        let id = note.id();
        assert!(Uuid::try_parse(&id).is_ok());

        let content = note.content();
        assert_eq!(&content, expected_content);
    }

    #[test]
    fn test_into_and_from() {
        let expected_content = "expected content!";
        let note1 = Note::new(expected_content);

        let bytes = Note::into(&note1);
        assert!(!bytes.is_empty(), "Serialized bytes should not be empty");

        let note2 = Note::from(&bytes);
        assert_eq!(note2.id(), note1.id());
        assert_eq!(note2.content(), note1.content());
        assert_eq!(note2.content(), expected_content);
    }

    #[test]
    fn test_update_and_merge() {
        let bytes = Note::into(&Note::new("one two three"));
        let mut note1 = Note::from(&bytes);
        let mut note2 = Note::from(&bytes);
        note1 = note1.update(&format!("cool {}", note1.content()));
        note2 = note2.update(&format!("{} wow", note2.content()));

        assert_eq!(note1.id(), note2.id());
        assert_eq!(note1.content(), "cool one two three");
        assert_eq!(note2.content(), "one two three wow");

        let note3 = note1.merge(&note2);
        assert_eq!(note3.id(), note1.id());
        assert_eq!(note3.content(), "cool one two three wow");
    }
}
