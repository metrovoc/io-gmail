use io_gmail::messages::list::{GmailMessagesList, GmailMessagesListResult};
use io_socket::runtimes::std_stream::handle;
use secrecy::SecretString;

mod stub_stream;

use stub_stream::{StubStream, json_response};

#[test]
fn lists_with_cjk_query() {
    let response = json_response(
        "HTTP/1.1 200 OK",
        r#"{"messages":[{"id":"abc","threadId":"t1"}],"nextPageToken":"PG2","resultSizeEstimate":1}"#,
    );
    let mut stream = StubStream::new(&response);
    let auth = SecretString::from("fake-token".to_string());
    let mut coroutine =
        GmailMessagesList::new(&auth, "me", Some("日本語"), &[], Some(25), None, false).unwrap();
    let mut arg = None;

    let response = loop {
        match coroutine.resume(arg.take()) {
            GmailMessagesListResult::Ok { response, .. } => break response,
            GmailMessagesListResult::Io { input } => {
                arg = Some(handle(&mut stream, input).unwrap())
            }
            GmailMessagesListResult::Err { err } => panic!("{err}"),
        }
    };

    assert_eq!(response.messages.len(), 1);
    assert_eq!(response.messages[0].id, "abc");

    let request = String::from_utf8_lossy(&stream.written);
    assert!(
        request.contains("q=%E6%97%A5%E6%9C%AC%E8%AA%9E"),
        "got: {request}"
    );
}

#[test]
fn surfaces_api_error_on_400() {
    let response = json_response(
        "HTTP/1.1 400 Bad Request",
        r#"{"error":{"code":400,"message":"Invalid query"}}"#,
    );
    let mut stream = StubStream::new(&response);
    let auth = SecretString::from("fake-token".to_string());
    let mut coroutine =
        GmailMessagesList::new(&auth, "me", Some("bad:query"), &[], None, None, false).unwrap();
    let mut arg = None;

    let err = loop {
        match coroutine.resume(arg.take()) {
            GmailMessagesListResult::Err { err } => break err.to_string(),
            GmailMessagesListResult::Io { input } => {
                arg = Some(handle(&mut stream, input).unwrap())
            }
            GmailMessagesListResult::Ok { .. } => panic!("expected error"),
        }
    };

    assert!(err.contains("HTTP 400"));
    assert!(err.contains("Invalid query"));
}
