use io_gmail::profile::{GmailProfileGet, GmailProfileGetResult};
use io_socket::runtimes::std_stream::handle;
use secrecy::SecretString;

mod stub_stream;

use stub_stream::{StubStream, json_response};

#[test]
fn gets_profile() {
    let response = json_response(
        "HTTP/1.1 200 OK",
        r#"{"emailAddress":"me@example.com","messagesTotal":12,"threadsTotal":8}"#,
    );
    let mut stream = StubStream::new(&response);
    let auth = SecretString::from("fake-token".to_string());
    let mut coroutine = GmailProfileGet::new(&auth, "me").unwrap();
    let mut arg = None;

    let response = loop {
        match coroutine.resume(arg.take()) {
            GmailProfileGetResult::Ok { response, .. } => break response,
            GmailProfileGetResult::Io { input } => arg = Some(handle(&mut stream, input).unwrap()),
            GmailProfileGetResult::Err { err } => panic!("{err}"),
        }
    };

    assert_eq!(response.email_address, "me@example.com");
    assert_eq!(response.messages_total, Some(12));
}
