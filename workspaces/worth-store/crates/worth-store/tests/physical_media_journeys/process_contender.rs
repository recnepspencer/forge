use super::*;

const CHILD_TEST: &str = "process_contender::child_media_contender";

pub(super) fn run_contender(root: &Path, expectation: &str) {
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", CHILD_TEST, "--nocapture"])
        .env("WORTH_STORE_C4_CHILD_ROOT", root)
        .env("WORTH_STORE_C4_CHILD_EXPECTATION", expectation)
        .status()
        .unwrap();
    assert!(status.success(), "child contender failed: {expectation}");
}

pub(super) fn spawn_lease_holder(root: &Path) -> std::process::Child {
    use std::io::BufRead;

    let mut child = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", CHILD_TEST, "--nocapture"])
        .env("WORTH_STORE_C4_CHILD_ROOT", root)
        .env("WORTH_STORE_C4_CHILD_EXPECTATION", "hold")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let mut output = std::io::BufReader::new(child.stdout.take().unwrap());
    loop {
        let mut line = String::new();
        assert_ne!(output.read_line(&mut line).unwrap(), 0);
        if line.trim() == "media-lease-ready" {
            break;
        }
    }
    std::thread::spawn(move || {
        let _ = std::io::copy(&mut output, &mut std::io::sink());
    });
    child
}

#[test]
fn child_media_contender() {
    use std::io::{Read, Write};

    let Ok(root) = std::env::var("WORTH_STORE_C4_CHILD_ROOT") else {
        return;
    };
    let expectation = std::env::var("WORTH_STORE_C4_CHILD_EXPECTATION").unwrap();
    let runtime = admit_runtime(Path::new(&root));
    match runtime
        .try_admit_filesystem_media(media_admission())
        .into_raw()
    {
        TransitionOutcome::Success(media) if expectation == "success" => {
            assert!(matches!(media.close(), MediaShutdownOutcome::Released(_)));
        }
        TransitionOutcome::Deferred(deferred) if expectation == "deferred" => {
            deferred.into_runtime().abort();
        }
        TransitionOutcome::Success(_media) if expectation == "die" => std::process::exit(0),
        TransitionOutcome::Success(media) if expectation == "hold" => {
            println!("media-lease-ready");
            std::io::stdout().flush().unwrap();
            let mut release = [0_u8; 1];
            std::io::stdin().read_exact(&mut release).unwrap();
            assert!(matches!(media.close(), MediaShutdownOutcome::Released(_)));
        }
        _ => panic!("child observed the wrong ownership outcome"),
    }
}
