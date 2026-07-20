use std::io::{BufRead, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, Stdio};
use std::sync::mpsc::{self, Receiver};
use std::time::Duration;

const CHILD_TEST: &str = "child_dispatch::journey_child_role";
const EVENT_PREFIX: &str = "C4_EVENT ";

pub(super) struct Contender {
    child: Child,
    input: ChildStdin,
    events: Receiver<String>,
}

impl Contender {
    pub(super) fn spawn(root: &Path, index: usize) -> Self {
        let mut child = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", CHILD_TEST, "--nocapture"])
            .env("WORTH_STORE_C4_CHILD_ROLE", "mutation-contender")
            .env("WORTH_STORE_C4_CHILD_ROOT", root)
            .env("WORTH_STORE_C4_CONTENDER_INDEX", index.to_string())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();
        let input = child.stdin.take().unwrap();
        let output = child.stdout.take().unwrap();
        let (sender, events) = mpsc::channel();
        std::thread::spawn(move || {
            for line in std::io::BufReader::new(output).lines() {
                let line = line.unwrap();
                if let Some(event) = line.strip_prefix(EVENT_PREFIX) {
                    sender.send(event.to_owned()).unwrap();
                }
            }
        });
        Self {
            child,
            input,
            events,
        }
    }

    pub(super) fn send(&mut self, command: u8) {
        self.input.write_all(&[command]).unwrap();
        self.input.flush().unwrap();
    }

    pub(super) fn event(&self) -> String {
        self.events
            .recv_timeout(Duration::from_secs(30))
            .expect("contender failed to reach a process barrier")
    }

    pub(super) fn kill(&mut self) {
        self.child.kill().unwrap();
    }

    pub(super) fn wait(&mut self) -> std::process::ExitStatus {
        self.child.wait().unwrap()
    }
}
