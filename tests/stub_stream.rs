#![allow(dead_code)]

use std::io::{self, Cursor, Read, Write};

pub struct StubStream<'a> {
    read: Cursor<&'a [u8]>,
    pub written: Vec<u8>,
}

impl<'a> StubStream<'a> {
    pub fn new(response: &'a [u8]) -> Self {
        Self {
            read: Cursor::new(response),
            written: Vec::new(),
        }
    }
}

impl Read for StubStream<'_> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.read.read(buf)
    }
}

impl Write for StubStream<'_> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.written.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub fn json_response(status_line: &str, body: &str) -> Vec<u8> {
    http_response(
        status_line,
        &[
            ("Connection", "keep-alive"),
            ("Content-Type", "application/json"),
        ],
        body.as_bytes(),
    )
}

pub fn empty_response(status_line: &str) -> Vec<u8> {
    http_response(status_line, &[("Connection", "close")], &[])
}

fn http_response(status_line: &str, headers: &[(&str, &str)], body: &[u8]) -> Vec<u8> {
    let mut response = Vec::new();
    response.extend_from_slice(status_line.as_bytes());
    response.extend_from_slice(b"\r\n");

    for (name, value) in headers {
        response.extend_from_slice(name.as_bytes());
        response.extend_from_slice(b": ");
        response.extend_from_slice(value.as_bytes());
        response.extend_from_slice(b"\r\n");
    }

    response.extend_from_slice(format!("Content-Length: {}\r\n\r\n", body.len()).as_bytes());
    response.extend_from_slice(body);
    response
}
