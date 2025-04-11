/* SPDX-License-Identifier: GPL-3.0-or-later */
use rdev::{listen, Event, EventType};
use scribe::allowed_keys;
use std::io::Write;
use std::net::TcpStream;

fn main() {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
}

fn callback(event: Event) {
    if let EventType::KeyPress(key) = event.event_type {
        if let Some(key) = allowed_keys(&key) {
            if let Ok(mut stream) = TcpStream::connect("127.0.0.1:7878") {
                let _ = stream.write_all(&[key as u8]);
            }
        }
    }
}
