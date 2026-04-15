use io_gmail::{
    messages::get::{GmailMessageGet, GmailMessageGetResult},
    types::message::{MessageFormat, decode_raw},
};
use io_socket::runtimes::std_stream::handle;
use secrecy::SecretString;

mod stub_stream;

use stub_stream::{StubStream, json_response};

#[test]
fn gets_raw_message_and_decodes_it() {
    let response = json_response(
        "HTTP/1.1 200 OK",
        r#"{"id":"abc","raw":"U3ViamVjdDogdGVzdA0KDQpoZWxsbw"}"#,
    );
    let mut stream = StubStream::new(&response);
    let auth = SecretString::from("fake-token".to_string());
    let mut coroutine = GmailMessageGet::new(&auth, "me", "abc", MessageFormat::Raw, &[]).unwrap();
    let mut arg = None;

    let response = loop {
        match coroutine.resume(arg.take()) {
            GmailMessageGetResult::Ok { response, .. } => break response,
            GmailMessageGetResult::Io { input } => arg = Some(handle(&mut stream, input).unwrap()),
            GmailMessageGetResult::Err { err } => panic!("{err}"),
        }
    };

    assert_eq!(
        decode_raw(response.raw.as_deref().unwrap()).unwrap(),
        b"Subject: test\r\n\r\nhello"
    );
}

#[test]
fn repeats_metadata_headers() {
    let response = json_response(
        "HTTP/1.1 200 OK",
        r#"{"id":"abc","payload":{"headers":[]}}"#,
    );
    let mut stream = StubStream::new(&response);
    let auth = SecretString::from("fake-token".to_string());
    let mut coroutine = GmailMessageGet::new(
        &auth,
        "me",
        "abc",
        MessageFormat::Metadata,
        &["Message-ID", "Subject"],
    )
    .unwrap();
    let mut arg = None;

    loop {
        match coroutine.resume(arg.take()) {
            GmailMessageGetResult::Ok { .. } => break,
            GmailMessageGetResult::Io { input } => arg = Some(handle(&mut stream, input).unwrap()),
            GmailMessageGetResult::Err { err } => panic!("{err}"),
        }
    }

    let request = String::from_utf8_lossy(&stream.written);
    assert!(
        request.contains("metadataHeaders=Message-ID"),
        "got: {request}"
    );
    assert!(
        request.contains("metadataHeaders=Subject"),
        "got: {request}"
    );
}
