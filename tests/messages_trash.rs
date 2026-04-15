use io_gmail::messages::trash::{
    GmailMessageTrash, GmailMessageTrashResult, GmailMessageUntrash, GmailMessageUntrashResult,
};
use io_socket::runtimes::std_stream::handle;
use secrecy::SecretString;

mod stub_stream;

use stub_stream::{StubStream, json_response};

#[test]
fn posts_trash_endpoint() {
    let response = json_response("HTTP/1.1 200 OK", r#"{"id":"abc"}"#);
    let mut stream = StubStream::new(&response);
    let auth = SecretString::from("fake-token".to_string());
    let mut coroutine = GmailMessageTrash::new(&auth, "me", "abc").unwrap();
    let mut arg = None;

    loop {
        match coroutine.resume(arg.take()) {
            GmailMessageTrashResult::Ok { response, .. } => {
                assert_eq!(response.id, "abc");
                break;
            }
            GmailMessageTrashResult::Io { input } => {
                arg = Some(handle(&mut stream, input).unwrap())
            }
            GmailMessageTrashResult::Err { err } => panic!("{err}"),
        }
    }

    let request = String::from_utf8_lossy(&stream.written);
    assert!(
        request.starts_with("POST /gmail/v1/users/me/messages/abc/trash"),
        "got: {request}"
    );
}

#[test]
fn posts_untrash_endpoint() {
    let response = json_response("HTTP/1.1 200 OK", r#"{"id":"abc"}"#);
    let mut stream = StubStream::new(&response);
    let auth = SecretString::from("fake-token".to_string());
    let mut coroutine = GmailMessageUntrash::new(&auth, "me", "abc").unwrap();
    let mut arg = None;

    loop {
        match coroutine.resume(arg.take()) {
            GmailMessageUntrashResult::Ok { response, .. } => {
                assert_eq!(response.id, "abc");
                break;
            }
            GmailMessageUntrashResult::Io { input } => {
                arg = Some(handle(&mut stream, input).unwrap())
            }
            GmailMessageUntrashResult::Err { err } => panic!("{err}"),
        }
    }

    let request = String::from_utf8_lossy(&stream.written);
    assert!(
        request.starts_with("POST /gmail/v1/users/me/messages/abc/untrash"),
        "got: {request}"
    );
}
