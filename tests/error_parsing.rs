use io_gmail::parse_api_error;

#[test]
fn parses_error_envelope() {
    let (status, message) =
        parse_api_error(400, br#"{"error":{"code":401,"message":"bad token"}}"#);
    assert_eq!(status, 401);
    assert_eq!(message, "bad token");
}

#[test]
fn falls_back_when_message_missing() {
    let (status, message) = parse_api_error(403, br#"{"error":{"code":403}}"#);
    assert_eq!(status, 403);
    assert_eq!(message, "unknown Gmail API error");
}

#[test]
fn handles_empty_body() {
    let (status, message) = parse_api_error(500, b"");
    assert_eq!(status, 500);
    assert_eq!(message, "unknown Gmail API error");
}

#[test]
fn handles_non_json_body() {
    let (status, message) = parse_api_error(502, b"upstream failure");
    assert_eq!(status, 502);
    assert_eq!(message, "upstream failure");
}

#[test]
fn prefers_nested_status_when_present() {
    let (status, message) =
        parse_api_error(500, br#"{"error":{"code":429,"message":"slow down"}}"#);
    assert_eq!(status, 429);
    assert_eq!(message, "slow down");
}
