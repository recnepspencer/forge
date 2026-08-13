//! Shared composition-guard test body.
//!
//! Included by a thin `#[path]` shim in each workspace so that **`cargo test`
//! enforces the guards**, not only CI and the git hook. That matters because an
//! agent loop frequently edits and runs tests without ever committing, so a
//! pre-commit hook never fires for it — but every agent runs `cargo test`.
//!
//! The analysis itself lives in `scripts/ci/check_composition_advisories.py`,
//! the single source of truth shared with CI, `.githooks/pre-commit`, and
//! edit-time agent hooks. This file only invokes it.

use std::path::{Path, PathBuf};
use std::process::Command;

fn repository_root() -> PathBuf {
    // Walk up from the crate manifest until the guard script is visible. Each
    // workspace sits at a different depth, so a fixed `../..` would be a lie
    // that happens to work from one of them.
    let mut dir: PathBuf = env!("CARGO_MANIFEST_DIR").into();
    loop {
        if dir
            .join("scripts/ci/check_composition_advisories.py")
            .is_file()
        {
            return dir;
        }
        if !dir.pop() {
            panic!("composition guard script not found above CARGO_MANIFEST_DIR");
        }
    }
}

fn python_command(root: &Path) -> Option<Command> {
    // `python3` is the CI spelling. On Windows it is usually a Microsoft Store
    // alias stub that prints help and exits 0 — which would silently pass this
    // test. Probe with real code so a stub cannot masquerade as an interpreter.
    #[cfg(windows)]
    let candidates = ["python", "py", "python3"];
    #[cfg(not(windows))]
    let candidates = ["python3", "python", "py"];
    for candidate in candidates {
        let probe = Command::new(candidate)
            .args(["-c", "import sys; sys.exit(7)"])
            .current_dir(root)
            .status();
        if matches!(probe, Ok(status) if status.code() == Some(7)) {
            let mut command = Command::new(candidate);
            command.current_dir(root);
            return Some(command);
        }
    }
    None
}

#[test]
fn composition_guards_hold_for_this_workspace() {
    let root = repository_root();
    let Some(mut command) = python_command(&root) else {
        // Failing rather than skipping is deliberate. A guard that quietly
        // does not run is the failure mode this whole mechanism exists to
        // remove — the same shape as a green report from a target that was
        // never executed.
        panic!(
            "no working Python 3 interpreter found; the composition guards could not run. \
             Install Python 3 (CI uses `python3`) rather than treating this as a skip."
        );
    };

    // `dirty` scope, deliberately: a change is judged on what it touches, never
    // on inherited debt. Gating `cargo test` on the whole tree would teach the
    // fastest fix — bulk-allowlisting — which is worse than no guard.
    //
    // The full-tree gate lives in CI ("Composition guards", workspace scope) and
    // in `scripts/ci/check_workspace_rust_line_caps.sh`. This target is the
    // in-loop signal, not the ceiling.
    let output = command
        .arg("scripts/ci/check_composition_advisories.py")
        .arg("dirty")
        .output()
        .expect("composition guard script should be executable");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Advisories are reported, never fatal: they are a design signal, not a
    // gate. Only the hard 400-line cap fails the build, and the script owns
    // that distinction so every surface agrees on it.
    let advisories: Vec<&str> = stdout
        .lines()
        .filter(|line| line.starts_with("ADVISORY:"))
        .collect();
    if !advisories.is_empty() {
        println!(
            "composition advisories ({}); worst first:\n{}",
            advisories.len(),
            advisories
                .iter()
                .take(20)
                .copied()
                .collect::<Vec<_>>()
                .join("\n")
        );
    }

    assert!(
        output.status.success(),
        "composition guards failed:\n{stdout}\n{stderr}"
    );
}
