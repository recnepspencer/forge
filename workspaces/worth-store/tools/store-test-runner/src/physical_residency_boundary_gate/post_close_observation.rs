use std::path::Path;

use super::workspace_source::read;
use crate::workspace_root;

const POOL: &str = "crates/worth-store-buffer-pool/src/physical_residency/pool.rs";
const OPERATION_ACCOUNTING: &str =
    "crates/worth-store-buffer-pool/src/physical_residency/pool/operation_accounting.rs";

#[test]
fn post_close_executed_transitions_remain_observable() {
    let root = workspace_root();
    let pool_path = root.join(POOL);
    let accounting_path = root.join(OPERATION_ACCOUNTING);
    let pool = read(&pool_path).expect("read pool denial accounting");
    let accounting = read(&accounting_path).expect("read pool copy accounting");

    inspect_post_close_observation((&pool_path, &pool), (&accounting_path, &accounting))
        .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn post_close_observation_gate_kills_denial_suppression_mutant() {
    let pool = "fn deny() { if !state.closed { state.accounting.deny(); } }";
    let denial = inspect_post_close_observation(
        (Path::new("pool.rs"), pool),
        (Path::new("accounting.rs"), honest_copy()),
    )
    .expect_err("post-close denial suppression mutant must be denied");
    assert!(denial.contains("denial"));
}

#[test]
fn post_close_observation_gate_kills_copy_suppression_mutant() {
    let accounting =
        "fn record_copy() { if state.closed { return; } state.accounting.record_copy(bytes); }";
    let denial = inspect_post_close_observation(
        (Path::new("pool.rs"), honest_denial()),
        (Path::new("accounting.rs"), accounting),
    )
    .expect_err("post-close copy suppression mutant must be denied");
    assert!(denial.contains("copy"));
}

fn inspect_post_close_observation(
    pool: (&Path, &str),
    accounting: (&Path, &str),
) -> Result<(), String> {
    let denial = required_body(pool, "fn deny")?;
    if !denial.contains("state.accounting.deny()") || denial.contains("state.closed") {
        return Err(format!(
            "post-close observation: executed denial is conditionally suppressed in {}",
            pool.0.display()
        ));
    }

    let copy = required_body(accounting, "fn record_copy")?;
    if !copy.contains("state.accounting.record_copy(bytes)") || copy.contains("state.closed") {
        return Err(format!(
            "post-close observation: executed copy is conditionally suppressed in {}",
            accounting.0.display()
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
            "post-close observation: `{signature}` missing in {}",
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

fn honest_denial() -> &'static str {
    "fn deny() { state.accounting.deny(); }"
}

fn honest_copy() -> &'static str {
    "fn record_copy() { state.accounting.record_copy(bytes); }"
}
