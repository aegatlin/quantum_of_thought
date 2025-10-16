use automerge::{
    AutoCommit, ObjType, ROOT, ReadDoc, ScalarValue, Value, transaction::Transactable,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct Note {
    id: String,
    content: String,
    doc: AutoCommit,
    errors: Vec<NoteError>,
}

#[derive(Debug)]
pub enum NoteError {
    NotFound(String),
    ExtractionError(String),
    DeserializationError(String),
    CreationError(String),
    AutomergeError(String),
    MissingId(String),
    MissingContent(String),
}

pub fn new(content: &str) -> Note {
    let mut note = Note {
        id: Uuid::now_v7().to_string(),
        content: "".into(),
        doc: AutoCommit::new(),
        errors: vec![],
    };

    if let Err(automerge_error) = note.doc.put(ROOT, "id", &note.id) {
        note.errors
            .push(NoteError::AutomergeError(automerge_error.to_string()));
        return note;
    }

    match note.doc.put_object(ROOT, "content", ObjType::Text) {
        Ok(obj_id) => {
            if let Err(automerge_error) = note.doc.update_text(&obj_id, content) {
                note.errors
                    .push(NoteError::AutomergeError(automerge_error.to_string()));
                return note;
            }
            note.content = content.into();
        }
        Err(automerge_error) => {
            note.errors
                .push(NoteError::AutomergeError(automerge_error.to_string()));
            return note;
        }
    }

    note
}

pub fn into(note: &Note) -> Vec<u8> {
    note.doc.clone().save()
}

pub fn from(bytes: &[u8]) -> Note {
    let mut note = Note {
        id: "".into(),
        content: "".into(),
        doc: AutoCommit::new(),
        errors: vec![],
    };

    match AutoCommit::load(bytes) {
        Ok(doc) => {
            if let Ok(Some((Value::Scalar(v), _))) = doc.get(ROOT, "id") {
                let w = v.as_ref();
                if let ScalarValue::Str(id) = w {
                    note.id = id.as_str().into();
                } else {
                    note.errors.push(NoteError::MissingId("missing id".into()))
                }
            } else {
                note.errors.push(NoteError::MissingId("missing id".into()))
            }

            if let Ok(Some((_, ex_id))) = doc.get(ROOT, "content") {
                match doc.text(ex_id) {
                    Ok(content) => {
                        note.content = content;
                    }
                    Err(automerge_error) => note
                        .errors
                        .push(NoteError::AutomergeError(automerge_error.to_string())),
                }
            } else {
                note.errors
                    .push(NoteError::MissingContent("missing content".into()))
            }
        }
        Err(automerge_error) => {
            note.errors
                .push(NoteError::AutomergeError(automerge_error.to_string()));
        }
    }

    note
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let expected_content = "hello, world";

        let note = new(expected_content);

        assert!(note.errors.is_empty());
        assert_eq!(note.content, expected_content);
        assert!(!note.id.is_empty())
    }

    #[test]
    fn test_into() {
        let note = new("idk");

        let bytes = into(&note);

        assert!(!bytes.is_empty())
    }

    #[test]
    fn test_from() {
        let note = new("idk");
        let bytes = into(&note);

        let note2 = from(&bytes);

        assert!(note2.errors.is_empty());
        assert_eq!(note2.content, note.content);
        assert_eq!(note2.id, note.id);
    }
}
