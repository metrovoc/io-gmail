use io_gmail::messages::modify::{GmailMessageModify, GmailMessageModifyResult};
use io_socket::runtimes::std_stream::handle;
use secrecy::SecretString;

mod stub_stream;

use stub_stream::{StubStream, json_response};

#[test]
fn sends_add_and_remove_label_updates() {
    let response = json_response("HTTP/1.1 200 OK", r#"{"id":"abc","labelIds":["STARRED"]}"#);
    let mut stream = StubStream::new(&response);
    let auth = SecretString::from("fake-token".to_string());
    let add = vec!["STARRED".to_string()];
    let remove = vec!["UNREAD".to_string()];
    let mut coroutine = GmailMessageModify::new(&auth, "me", "abc", &add, &remove).unwrap();
    let mut arg = None;

    loop {
        match coroutine.resume(arg.take()) {
            GmailMessageModifyResult::Ok { response, .. } => {
                assert_eq!(response.id, "abc");
                break;
            }
            GmailMessageModifyResult::Io { input } => {
                arg = Some(handle(&mut stream, input).unwrap())
            }
            GmailMessageModifyResult::Err { err } => panic!("{err}"),
        }
    }

    let request = String::from_utf8_lossy(&stream.written);
    assert!(
        request.contains("\"addLabelIds\":[\"STARRED\"]"),
        "got: {request}"
    );
    assert!(
        request.contains("\"removeLabelIds\":[\"UNREAD\"]"),
        "got: {request}"
    );
}

#[test]
fn rejects_empty_modify_request() {
    let auth = SecretString::from("fake-token".to_string());
    let err = GmailMessageModify::new(&auth, "me", "abc", &[], &[])
        .err()
        .unwrap();
    assert!(err.to_string().contains("at least one label update"));
}
