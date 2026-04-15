use io_gmail::messages::send::{GmailMessageSend, GmailMessageSendResult};
use io_socket::runtimes::std_stream::handle;
use secrecy::SecretString;

mod stub_stream;

use stub_stream::{StubStream, json_response};

#[test]
fn sends_base64url_encoded_message_body() {
    let response = json_response("HTTP/1.1 200 OK", r#"{"id":"abc","threadId":"t1"}"#);
    let mut stream = StubStream::new(&response);
    let auth = SecretString::from("fake-token".to_string());
    let mut coroutine = GmailMessageSend::new(&auth, "me", b"Subject: test\r\n\r\nhello").unwrap();
    let mut arg = None;

    let sent = loop {
        match coroutine.resume(arg.take()) {
            GmailMessageSendResult::Ok { response, .. } => break response,
            GmailMessageSendResult::Io { input } => arg = Some(handle(&mut stream, input).unwrap()),
            GmailMessageSendResult::Err { err } => panic!("{err}"),
        }
    };

    assert_eq!(sent.id, "abc");

    let request = String::from_utf8_lossy(&stream.written);
    assert!(
        request.contains("U3ViamVjdDogdGVzdA0KDQpoZWxsbw"),
        "got: {request}"
    );
}
