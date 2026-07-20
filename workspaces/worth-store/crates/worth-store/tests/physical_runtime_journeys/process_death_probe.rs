use std::{
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    process::{Child, ChildStdout, Command, ExitStatus, Stdio},
    sync::mpsc,
    time::Duration,
};

use worth_store::physical_runtime::{PhysicalRuntimeAdmission, PhysicalStore};

const CHILD_MODE: &str = "WORTH_STORE_C3_PROCESS_DEATH_CHILD";
const CHILD_ROOT: &str = "WORTH_STORE_C3_PROCESS_DEATH_ROOT";
const ADMITTED_MARKER: &str = "C3_CHILD_ADMITTED explicit_closes=0";

pub fn run_child_mode() -> bool {
    if std::env::var_os(CHILD_MODE).is_none() {
        return false;
    }

    let root = PathBuf::from(std::env::var_os(CHILD_ROOT).expect("child root must be supplied"));
    let admission = PhysicalRuntimeAdmission::new(root).expect("child root must validate");
    let runtime = PhysicalStore::admit(admission).expect("child runtime must admit");
    println!(
        "C3_CHILD_ADMITTED explicit_closes={}",
        PhysicalStore::diagnostics().explicit_closes()
    );
    std::io::stdout()
        .flush()
        .expect("child admission marker must flush");
    let _live_authority = runtime;
    loop {
        std::thread::park();
    }
}

pub fn prove_process_death_emits_no_physical_residue(root: PathBuf) {
    assert!(!root.exists());
    let executable = std::env::current_exe().expect("test executable path must be available");
    let mut child = ProcessDeathChild::spawn(
        Command::new(executable)
            .args([
                "--exact",
                "runtime_authority_pressure_journey_keeps_observation_read_only_and_phase_scoped",
                "--nocapture",
            ])
            .env(CHILD_MODE, "1")
            .env(CHILD_ROOT, &root)
            .stdout(Stdio::piped()),
    );
    let stdout = child.take_stdout();
    let (transcript_sender, transcript_receiver) = mpsc::sync_channel(1);
    let reader = std::thread::spawn(move || {
        let mut transcript = Vec::new();
        for line in BufReader::new(stdout).lines() {
            let line = line.expect("child output must remain readable");
            let admitted = line == ADMITTED_MARKER;
            transcript.push(line);
            if admitted {
                break;
            }
        }
        transcript_sender
            .send(transcript)
            .expect("parent must await the child transcript");
    });
    let transcript = match transcript_receiver.recv_timeout(Duration::from_secs(10)) {
        Ok(transcript) => transcript,
        Err(error) => {
            drop(child);
            reader.join().expect("child output reader must stop");
            panic!("child did not reach admission before the watchdog expired: {error}");
        }
    };
    reader.join().expect("child output reader must not panic");
    assert!(
        transcript.iter().any(|line| line == ADMITTED_MARKER),
        "child exited before admission: {transcript:?}"
    );
    let status = child.kill_and_wait();
    assert!(!status.success());
    assert!(!root.exists());
}

struct ProcessDeathChild {
    child: Option<Child>,
}

impl ProcessDeathChild {
    fn spawn(command: &mut Command) -> Self {
        Self {
            child: Some(command.spawn().expect("process-death child must start")),
        }
    }

    fn take_stdout(&mut self) -> ChildStdout {
        self.child
            .as_mut()
            .and_then(|child| child.stdout.take())
            .expect("child stdout must be piped")
    }

    fn kill_and_wait(mut self) -> ExitStatus {
        let mut child = self
            .child
            .take()
            .expect("the process-death child must still be owned");
        child.kill().expect("the live child must be killable");
        child.wait().expect("the killed child must be reapable")
    }
}

impl Drop for ProcessDeathChild {
    fn drop(&mut self) {
        let Some(child) = self.child.as_mut() else {
            return;
        };
        let _ = child.kill();
        let _ = child.wait();
    }
}
