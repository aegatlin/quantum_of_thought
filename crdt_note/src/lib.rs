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

        doc.put(ROOT, "id", &id);

        match doc.put_object(ROOT, "content", ObjType::Text) {
            Ok(ex_id) => {
                doc.update_text(&ex_id, content);
            }
            Err(_) => {}
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

    pub fn into(&self) -> Vec<u8> {
        self.doc.clone().save()
    }

    pub fn from(bytes: &[u8]) -> Self {
        match AutoCommit::load(bytes) {
            Ok(doc) => return Self { doc: doc },
            Err(_) => {
                return Self {
                    doc: AutoCommit::new(),
                };
            }
        }
    }
}
