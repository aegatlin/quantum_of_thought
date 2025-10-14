use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_no_args_shows_usage() {
    Command::cargo_bin("qot")
        .unwrap()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn test_create_note_with_single_word() {
    Command::cargo_bin("qot")
        .unwrap()
        .arg("hello")
        .assert()
        .success()
        .stdout(predicate::str::contains("Created note"))
        .stdout(predicate::str::contains("hello"));
}

#[test]
fn test_create_note_with_multiple_words() {
    Command::cargo_bin("qot")
        .unwrap()
        .args(&["get", "milk"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Created note"))
        .stdout(predicate::str::contains("get milk"));
}

#[test]
fn test_list_shows_placeholder() {
    Command::cargo_bin("qot")
        .unwrap()
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("No notes yet"));
}

#[test]
fn test_create_note_with_special_characters() {
    Command::cargo_bin("qot")
        .unwrap()
        .args(&["buy", "eggs", "&", "milk"])
        .assert()
        .success()
        .stdout(predicate::str::contains("buy eggs & milk"));
}
