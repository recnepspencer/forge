use std::path::Path;

use super::workspace_source::read;
use crate::workspace_root;

const PIN_LIFECYCLE: &str =
    "crates/worth-store-buffer-pool/src/physical_residency/pool/pin_lifecycle.rs";
const SPECULATION: &str =
    "crates/worth-store-buffer-pool/src/physical_residency/pool/operation_accounting.rs";
const WRITEBACK: &str =
    "crates/worth-store-buffer-pool/src/physical_residency/pool/writeback_claim.rs";
const EXACT_WAITER: &str =
    "crates/worth-store-buffer-pool/src/physical_residency/pool/frame_admission/mod.rs";
const BOUNDED_WAITER: &str =
    "crates/worth-store-buffer-pool/src/physical_residency/pool/bounded_frame_admission/join.rs";

struct CleanupSources<'source> {
    pin: (&'source Path, &'source str),
    speculation: (&'source Path, &'source str),
    writeback: (&'source Path, &'source str),
    exact_waiter: (&'source Path, &'source str),
    bounded_waiter: (&'source Path, &'source str),
}

#[test]
fn post_close_handle_cleanup_remains_exact() {
    let root = workspace_root();
    let pin_path = root.join(PIN_LIFECYCLE);
    let speculation_path = root.join(SPECULATION);
    let writeback_path = root.join(WRITEBACK);
    let exact_waiter_path = root.join(EXACT_WAITER);
    let bounded_waiter_path = root.join(BOUNDED_WAITER);
    let pin = read(&pin_path).expect("read pin lifecycle");
    let speculation = read(&speculation_path).expect("read speculative accounting");
    let writeback = read(&writeback_path).expect("read writeback claims");
    let exact_waiter = read(&exact_waiter_path).expect("read exact waiter lifecycle");
    let bounded_waiter = read(&bounded_waiter_path).expect("read bounded waiter lifecycle");

    inspect_shutdown_cleanup(CleanupSources {
        pin: (&pin_path, &pin),
        speculation: (&speculation_path, &speculation),
        writeback: (&writeback_path, &writeback),
        exact_waiter: (&exact_waiter_path, &exact_waiter),
        bounded_waiter: (&bounded_waiter_path, &bounded_waiter),
    })
    .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn shutdown_cleanup_gate_kills_pin_suppression_mutant() {
    let denial = inspect_shutdown_cleanup(CleanupSources {
        pin: (
            Path::new("pin.rs"),
            "fn release_pin() { if state.closed { return; } }",
        ),
        speculation: (Path::new("speculation.rs"), honest_speculation()),
        writeback: (Path::new("writeback.rs"), honest_writeback()),
        exact_waiter: (Path::new("exact.rs"), honest_exact_waiter()),
        bounded_waiter: (Path::new("bounded.rs"), honest_bounded_waiter()),
    })
    .expect_err("post-close pin suppression mutant must be denied");
    assert!(denial.contains("release_pin"));
}

#[test]
fn shutdown_cleanup_gate_kills_speculation_suppression_mutant() {
    let denial = inspect_shutdown_cleanup(CleanupSources {
        pin: (Path::new("pin.rs"), honest_pin()),
        speculation: (
            Path::new("speculation.rs"),
            "fn release_speculative() { if state.closed { return; } }",
        ),
        writeback: (Path::new("writeback.rs"), honest_writeback()),
        exact_waiter: (Path::new("exact.rs"), honest_exact_waiter()),
        bounded_waiter: (Path::new("bounded.rs"), honest_bounded_waiter()),
    })
    .expect_err("post-close speculation suppression mutant must be denied");
    assert!(denial.contains("release_speculative"));
}

#[test]
fn shutdown_cleanup_gate_kills_writeback_suppression_mutant() {
    let denial = inspect_shutdown_cleanup(CleanupSources {
        pin: (Path::new("pin.rs"), honest_pin()),
        speculation: (Path::new("speculation.rs"), honest_speculation()),
        writeback: (
            Path::new("writeback.rs"),
            "fn release_writeback_claim() { if state.closed { return; } }",
        ),
        exact_waiter: (Path::new("exact.rs"), honest_exact_waiter()),
        bounded_waiter: (Path::new("bounded.rs"), honest_bounded_waiter()),
    })
    .expect_err("post-close writeback suppression mutant must be denied");
    assert!(denial.contains("release_writeback_claim"));
}

#[test]
fn shutdown_cleanup_gate_kills_exact_waiter_drain_omission() {
    let denial = inspect_shutdown_cleanup(CleanupSources {
        pin: (Path::new("pin.rs"), honest_pin()),
        speculation: (Path::new("speculation.rs"), honest_speculation()),
        writeback: (Path::new("writeback.rs"), honest_writeback()),
        exact_waiter: (
            Path::new("exact.rs"),
            "fn release_loading_waiter() { state.append_evictable(key); }",
        ),
        bounded_waiter: (Path::new("bounded.rs"), honest_bounded_waiter()),
    })
    .expect_err("exact waiter post-close drain omission must be denied");
    assert!(denial.contains("release_loading_waiter"));
}

#[test]
fn shutdown_cleanup_gate_kills_bounded_waiter_drain_omission() {
    let denial = inspect_shutdown_cleanup(CleanupSources {
        pin: (Path::new("pin.rs"), honest_pin()),
        speculation: (Path::new("speculation.rs"), honest_speculation()),
        writeback: (Path::new("writeback.rs"), honest_writeback()),
        exact_waiter: (Path::new("exact.rs"), honest_exact_waiter()),
        bounded_waiter: (
            Path::new("bounded.rs"),
            "fn release_resident_bounded_waiter() { state.append_evictable(key); }",
        ),
    })
    .expect_err("bounded waiter post-close drain omission must be denied");
    assert!(denial.contains("release_resident_bounded_waiter"));
}

fn inspect_shutdown_cleanup(sources: CleanupSources<'_>) -> Result<(), String> {
    let pin_release = exact_release_body(sources.pin, "fn release_pin")?;
    if !pin_release.contains("drain_all_legal_clean_frames") {
        return Err(format!(
            "shutdown cleanup: release_pin must drain post-close clean frames through legal-victim authority in {}",
            sources.pin.0.display()
        ));
    }
    exact_release_body(sources.speculation, "fn release_speculative")?;
    exact_release_body(sources.writeback, "fn release_writeback_claim")?;
    require_post_close_drain(sources.exact_waiter, "fn release_loading_waiter")?;
    require_post_close_drain(sources.bounded_waiter, "fn release_resident_bounded_waiter")?;
    Ok(())
}

fn require_post_close_drain(source: (&Path, &str), signature: &str) -> Result<(), String> {
    let body = exact_release_body(source, signature)?;
    if !body.contains("drain_all_legal_clean_frames") {
        return Err(format!(
            "shutdown cleanup: `{signature}` must drain a final post-close clean pin in {}",
            source.0.display()
        ));
    }
    Ok(())
}

fn exact_release_body<'source>(
    source: (&Path, &'source str),
    signature: &str,
) -> Result<&'source str, String> {
    let body = delimited_body(source.1, signature).ok_or_else(|| {
        format!(
            "shutdown cleanup: `{signature}` missing in {}",
            source.0.display()
        )
    })?;
    let compact: String = body
        .chars()
        .filter(|character| !character.is_whitespace())
        .collect();
    if compact.contains("ifstate.closed{return;}") {
        return Err(format!(
            "shutdown cleanup: `{signature}` suppresses owned release after close in {}",
            source.0.display()
        ));
    }
    Ok(body)
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

fn honest_pin() -> &'static str {
    r#"
fn release_pin() {
    if state.closed {
        state.drain_all_legal_clean_frames();
    }
}
"#
}

fn honest_speculation() -> &'static str {
    "fn release_speculative() { state.accounting.release_speculative(kind, frames); }"
}

fn honest_writeback() -> &'static str {
    "fn release_writeback_claim() { state.accounting.release_writeback(count); }"
}

fn honest_exact_waiter() -> &'static str {
    "fn release_loading_waiter() { state.drain_all_legal_clean_frames(); }"
}

fn honest_bounded_waiter() -> &'static str {
    "fn release_resident_bounded_waiter() { state.drain_all_legal_clean_frames(); }"
}
