use io_gmail::types::message::{decode_raw, encode_raw};

#[test]
fn decodes_padded_raw_messages() {
    assert_eq!(decode_raw("SGVsbG8=").unwrap(), b"Hello");
}

#[test]
fn decodes_unpadded_raw_messages() {
    assert_eq!(decode_raw("SGVsbG8").unwrap(), b"Hello");
}

#[test]
fn decodes_wrapped_raw_messages() {
    assert_eq!(decode_raw("SGVs\nbG8=\r\n").unwrap(), b"Hello");
}

#[test]
fn round_trips_raw_messages() {
    let raw = b"Subject: test\r\n\r\nhello";
    assert_eq!(decode_raw(&encode_raw(raw)).unwrap(), raw);
}
