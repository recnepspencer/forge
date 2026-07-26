use std::path::Path;

use super::workspace_source::read;
use crate::workspace_root;

const LOWER_SHUTDOWN: &str = "crates/worth-store-buffer-pool/src/physical_residency/observation.rs";
const STORE_LIFECYCLE: &str = "crates/worth-store/src/physical_runtime/instance/lifecycle.rs";

#[test]
fn shutdown_residue_reaches_the_store_terminal_posture() {
    let root = workspace_root();
    let lower_path = root.join(LOWER_SHUTDOWN);
    let store_path = root.join(STORE_LIFECYCLE);
    let lower = read(&lower_path).expect("read lower shutdown observation");
    let store = read(&store_path).expect("read Store shutdown lifecycle");

    inspect_shutdown_residue((&lower_path, &lower), (&store_path, &store))
        .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn shutdown_residue_gate_kills_omitted_cancellable_work_mutant() {
    let lower = r#"
pub const fn requires_inspection(self) -> bool {
    self.counters.dirty_frames() > 0
        || self.counters.active_writeback_claims() > 0
}
pub const fn has_cancellable_work_residue(self) -> bool {
    self.counters.active_operation_bytes() > 0
}
"#;
    let denial = inspect_shutdown_residue(
        (Path::new("observation.rs"), lower),
        (Path::new("lifecycle.rs"), honest_store_lifecycle()),
    )
    .expect_err("omitted cancellable-work mutant must be denied");
    assert!(denial.contains("cancellable work"));
}

#[test]
fn shutdown_residue_gate_kills_store_projection_mutant() {
    let store = r#"
fn release_media(self) {
    RecordServingTerminalObservation::new(
        self.health.requires_inspection() || !self.publication_residue.is_empty(),
    );
}
"#;
    let denial = inspect_shutdown_residue(
        (Path::new("observation.rs"), honest_lower_shutdown()),
        (Path::new("lifecycle.rs"), store),
    )
    .expect_err("Store projection mutant must be denied");
    assert!(denial.contains("Store terminal"));
}

fn inspect_shutdown_residue(lower: (&Path, &str), store: (&Path, &str)) -> Result<(), String> {
    let inspection = required_body(lower, "pub const fn requires_inspection")?;
    if !inspection.contains("self.has_cancellable_work_residue()") {
        return Err(format!(
            "shutdown residue: cancellable work must enter exhaustive inspection in {}",
            lower.0.display()
        ));
    }
    if !inspection.contains("active_writeback_claims() > 0") {
        return Err(format!(
            "shutdown residue: active writeback claims must enter exhaustive inspection in {}",
            lower.0.display()
        ));
    }
    if lower.1.contains("cancelled_read_work") {
        return Err(format!(
            "shutdown residue: scope-generic work cannot retain the read-only name in {}",
            lower.0.display()
        ));
    }

    let release = required_body(store, "fn release_media")?;
    if !release.contains("residency.requires_inspection()") {
        return Err(format!(
            "shutdown residue: exhaustive lower residue must reach the Store terminal in {}",
            store.0.display()
        ));
    }
    Ok(())
}

fn required_body<'source>(
    source: (&Path, &'source str),
    signature: &str,
) -> Result<&'source str, String> {
    delimited_body(source.1, signature).ok_or_else(|| {
        format!(
            "shutdown residue: `{signature}` missing in {}",
            source.0.display()
        )
    })
}

fn delimited_body<'source>(source: &'source str, signature: &str) -> Option<&'source str> {
    let start = source.find(signature)?;
    let tail = &source[start..];
    let body_start = tail.find('{')?;
    let mut depth = 0_u32;
    for (offset, byte) in tail[body_start..].bytes().enumerate() {
        match byte {
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&tail[body_start..=body_start + offset]);
                }
            }
            _ => {}
        }
    }
    None
}

fn honest_lower_shutdown() -> &'static str {
    r#"
pub const fn requires_inspection(self) -> bool {
    self.counters.dirty_frames() > 0
        || self.counters.active_writeback_claims() > 0
        || self.has_cancellable_work_residue()
}
pub const fn has_cancellable_work_residue(self) -> bool {
    self.counters.active_operation_bytes() > 0
}
"#
}

fn honest_store_lifecycle() -> &'static str {
    r#"
fn release_media(self) {
    RecordServingTerminalObservation::new(
        self.health.requires_inspection()
            || !self.publication_residue.is_empty()
            || residency.requires_inspection(),
    );
}
"#
}
