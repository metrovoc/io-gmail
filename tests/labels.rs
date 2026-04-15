use io_gmail::labels::{
    create::{GmailLabelCreate, GmailLabelCreateResult},
    delete::{GmailLabelDelete, GmailLabelDeleteResult},
    list::{GmailLabelsList, GmailLabelsListResult},
    update::{GmailLabelUpdate, GmailLabelUpdateResult},
};
use io_socket::runtimes::std_stream::handle;
use secrecy::SecretString;

mod stub_stream;

use stub_stream::{StubStream, empty_response, json_response};

#[test]
fn lists_labels() {
    let response = json_response(
        "HTTP/1.1 200 OK",
        r#"{"labels":[{"id":"INBOX","name":"INBOX","type":"system"}]}"#,
    );
    let mut stream = StubStream::new(&response);
    let auth = SecretString::from("fake-token".to_string());
    let mut coroutine = GmailLabelsList::new(&auth, "me").unwrap();
    let mut arg = None;

    let response = loop {
        match coroutine.resume(arg.take()) {
            GmailLabelsListResult::Ok { response, .. } => break response,
            GmailLabelsListResult::Io { input } => arg = Some(handle(&mut stream, input).unwrap()),
            GmailLabelsListResult::Err { err } => panic!("{err}"),
        }
    };

    assert_eq!(response.labels.len(), 1);
    assert_eq!(response.labels[0].id, "INBOX");
}

#[test]
fn creates_label_with_default_visibilities() {
    let response = json_response(
        "HTTP/1.1 200 OK",
        r#"{"id":"Label_1","name":"todo","type":"user"}"#,
    );
    let mut stream = StubStream::new(&response);
    let auth = SecretString::from("fake-token".to_string());
    let mut coroutine = GmailLabelCreate::new(&auth, "me", "todo").unwrap();
    let mut arg = None;

    loop {
        match coroutine.resume(arg.take()) {
            GmailLabelCreateResult::Ok { response, .. } => {
                assert_eq!(response.id, "Label_1");
                break;
            }
            GmailLabelCreateResult::Io { input } => arg = Some(handle(&mut stream, input).unwrap()),
            GmailLabelCreateResult::Err { err } => panic!("{err}"),
        }
    }

    let request = String::from_utf8_lossy(&stream.written);
    assert!(
        request.contains("\"labelListVisibility\":\"labelShow\""),
        "got: {request}"
    );
    assert!(
        request.contains("\"messageListVisibility\":\"show\""),
        "got: {request}"
    );
}

#[test]
fn updates_label_with_patch() {
    let response = json_response(
        "HTTP/1.1 200 OK",
        r#"{"id":"Label_1","name":"renamed","type":"user"}"#,
    );
    let mut stream = StubStream::new(&response);
    let auth = SecretString::from("fake-token".to_string());
    let mut coroutine = GmailLabelUpdate::new(&auth, "me", "Label_1", "renamed").unwrap();
    let mut arg = None;

    loop {
        match coroutine.resume(arg.take()) {
            GmailLabelUpdateResult::Ok { response, .. } => {
                assert_eq!(response.name, "renamed");
                break;
            }
            GmailLabelUpdateResult::Io { input } => arg = Some(handle(&mut stream, input).unwrap()),
            GmailLabelUpdateResult::Err { err } => panic!("{err}"),
        }
    }

    let request = String::from_utf8_lossy(&stream.written);
    assert!(
        request.starts_with("PATCH /gmail/v1/users/me/labels/Label_1"),
        "got: {request}"
    );
}

#[test]
fn deletes_label() {
    let response = empty_response("HTTP/1.1 204 No Content");
    let mut stream = StubStream::new(&response);
    let auth = SecretString::from("fake-token".to_string());
    let mut coroutine = GmailLabelDelete::new(&auth, "me", "Label_1").unwrap();
    let mut arg = None;

    loop {
        match coroutine.resume(arg.take()) {
            GmailLabelDeleteResult::Ok { .. } => break,
            GmailLabelDeleteResult::Io { input } => arg = Some(handle(&mut stream, input).unwrap()),
            GmailLabelDeleteResult::Err { err } => panic!("{err}"),
        }
    }

    let request = String::from_utf8_lossy(&stream.written);
    assert!(
        request.starts_with("DELETE /gmail/v1/users/me/labels/Label_1"),
        "got: {request}"
    );
}
