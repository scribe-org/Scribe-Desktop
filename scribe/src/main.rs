use std::process::{Child, Command};

fn main() {
    let listener_process = start_process("./target/debug/listener", "Event Listener");
    let gui_process = start_process("./target/debug/gui", "GUI Application");

    if let Some(mut p) = gui_process {
        let _ = p.wait();
    }

    if let Some(mut p) = listener_process {
        let _ = p.kill();
        let _ = p.wait();
    }
}

fn start_process(command: &str, _name: &str) -> Option<Child> {
    match Command::new(command).spawn() {
        Ok(child) => {
            // println!("Started {} successfully.", name);
            Some(child)
        }
        Err(_e) => {
            // eprintln!("Failed to start {}: {}", name, e);
            None
        }
    }
}
