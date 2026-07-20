use std::io::{BufRead, Write};
use std::path::Path;
use std::process::Stdio;
use std::sync::mpsc;
use std::time::Duration;

const CHILD_TEST: &str = "child_dispatch::journey_child_role";
const EVENT_PREFIX: &str = "C4_EVENT ";

pub(super) fn run_abrupt_writer(root: &Path, case: &str) -> String {
    let mut child = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", CHILD_TEST, "--nocapture"])
        .env("WORTH_STORE_C4_CHILD_ROLE", "faulted-media-writer")
        .env("WORTH_STORE_C4_CHILD_ROOT", root)
        .env("WORTH_STORE_C4_FAULT_CASE", case)
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    let output = child.stdout.take().unwrap();
    let (sender, receiver) = mpsc::channel();
    std::thread::spawn(move || {
        for line in std::io::BufReader::new(output).lines() {
            let line = line.unwrap();
            if let Some(event) = line.strip_prefix(EVENT_PREFIX) {
                sender.send(event.to_owned()).unwrap();
            }
        }
    });
    let boundary = receiver.recv_timeout(Duration::from_secs(30)).unwrap();
    assert!(boundary.starts_with("fault-boundary;"));
    child.kill().unwrap();
    assert!(!child.wait().unwrap().success());
    boundary
}

pub(super) fn event(message: &str) {
    println!("{EVENT_PREFIX}{message}");
    std::io::stdout().flush().unwrap();
}
