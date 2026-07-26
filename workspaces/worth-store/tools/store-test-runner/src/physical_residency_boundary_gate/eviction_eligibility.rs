use std::path::Path;

use super::workspace_source::read;
use crate::workspace_root;

const POOL_STATE: &str = "crates/worth-store-buffer-pool/src/physical_residency/pool.rs";

#[test]
fn eviction_eligibility_excludes_every_governed_ineligible_state() {
    let source = read(&workspace_root().join(POOL_STATE)).expect("read physical pool state");
    inspect_eviction_eligibility(Path::new(POOL_STATE), &source)
        .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn eviction_gate_kills_the_pinned_victim_mutant_locally() {
    let mutant = r#"
fn is_evictable(&self) -> bool {
    !self.dirty
        && !self.writeback_claimed
        && matches!(&self.state, FrameState::Resident(_))
        && self.loading_waiters == 0
}
"#;
    let denial = inspect_eviction_eligibility(Path::new("controlled_mutant.rs"), mutant)
        .expect_err("pinned eviction mutant must be denied");
    assert!(denial.contains("pin exclusion"));
}

fn inspect_eviction_eligibility(path: &Path, source: &str) -> Result<(), String> {
    let start = source.find("fn is_evictable").ok_or_else(|| {
        format!(
            "physical residency boundary: eviction predicate missing in {}",
            path.display()
        )
    })?;
    let predicate = &source[start
        ..source[start..]
            .find("\n    }")
            .map_or(source.len(), |end| start + end)];
    for (required, label) in [
        ("self.pins == 0", "pin exclusion"),
        ("!self.dirty", "dirty exclusion"),
        ("!self.writeback_claimed", "writeback-claim exclusion"),
        ("FrameState::Resident", "nonresident exclusion"),
        ("self.loading_waiters == 0", "loading-waiter exclusion"),
    ] {
        if !predicate.contains(required) {
            return Err(format!(
                "physical residency boundary: eviction predicate lost {label} in {}",
                path.display()
            ));
        }
    }
    Ok(())
}
