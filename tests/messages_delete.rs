use io_gmail::messages::delete::{GmailMessageDelete, GmailMessageDeleteResult};
use io_socket::runtimes::std_stream::handle;
use secrecy::SecretString;

mod stub_stream;

use stub_stream::{StubStream, empty_response};

#[test]
fn deletes_message() {
    let response = empty_response("HTTP/1.1 204 No Content");
    let mut stream = StubStream::new(&response);
    let auth = SecretString::from("fake-token".to_string());
    let mut coroutine = GmailMessageDelete::new(&auth, "me", "abc").unwrap();
    let mut arg = None;

    loop {
        match coroutine.resume(arg.take()) {
            GmailMessageDeleteResult::Ok { .. } => break,
            GmailMessageDeleteResult::Io { input } => {
                arg = Some(handle(&mut stream, input).unwrap())
            }
            GmailMessageDeleteResult::Err { err } => panic!("{err}"),
        }
    }

    let request = String::from_utf8_lossy(&stream.written);
    assert!(
        request.starts_with("DELETE /gmail/v1/users/me/messages/abc"),
        "got: {request}"
    );
}
