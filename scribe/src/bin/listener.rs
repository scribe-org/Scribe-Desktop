/* SPDX-License-Identifier: GPL-3.0-or-later */
use rdev::{listen, Event, EventType, Key};
use std::io::Write;
use std::net::TcpStream;

fn main() {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

fn send_char(c: char) {
    let mut buf = [0u8; 4];
    let encoded = c.encode_utf8(&mut buf);
    let bytes = encoded.as_bytes();
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:7878") {
        let mut packet = vec![bytes.len() as u8];
        packet.extend_from_slice(bytes);
        let _ = stream.write_all(&packet);
    }
}

fn callback(event: Event) {
    if let EventType::KeyPress(key) = &event.event_type {
        match key {
            Key::Backspace => send_char('\x08'),
            _ => {
                if let Some(name) = &event.name {
                    for c in name.chars() {
                        if !c.is_control() {
                            send_char(c);
                        }
                    }
                }
            }
        }
    }
}
