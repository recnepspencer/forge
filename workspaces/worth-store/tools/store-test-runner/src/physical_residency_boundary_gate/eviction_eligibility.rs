use std::path::Path;

use super::workspace_source::read;
use crate::workspace_root;

const EVICTION_ELIGIBILITY: &str =
    "crates/worth-store-buffer-pool/src/physical_residency/pool/eviction/eligibility.rs";

#[test]
fn eviction_eligibility_excludes_every_governed_ineligible_state() {
    let source =
        read(&workspace_root().join(EVICTION_ELIGIBILITY)).expect("read eviction eligibility");
    inspect_eviction_eligibility(Path::new(EVICTION_ELIGIBILITY), &source)
        .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn eviction_gate_kills_the_pinned_victim_mutant_locally() {
    let mutant = r#"
pub(super) enum EvictionEligibility {
    Legal,
    Pinned,
    Dirty,
    Loading,
    CandidatePublication,
    WritebackClaimed,
}
fn eviction_eligibility(&self) -> EvictionEligibility {
    if self.writeback_claimed {
        EvictionEligibility::WritebackClaimed
    } else if self.dirty {
        EvictionEligibility::Dirty
    } else if self.loading_waiters != 0 {
        EvictionEligibility::Loading
    } else {
        match self.state {
            FrameState::Loading | FrameState::LoadFailed(_) => EvictionEligibility::Loading,
            FrameState::CandidateReserved => EvictionEligibility::CandidatePublication,
            FrameState::Resident(_) => EvictionEligibility::Legal,
        }
    }
}
fn is_evictable(&self) -> bool {
    self.eviction_eligibility() == EvictionEligibility::Legal
}
"#;
    let denial = inspect_eviction_eligibility(Path::new("controlled_mutant.rs"), mutant)
        .expect_err("pinned eviction mutant must be denied");
    assert!(denial.contains("pin exclusion"));
}

fn inspect_eviction_eligibility(path: &Path, source: &str) -> Result<(), String> {
    for (required, label) in [
        (
            "pub(super) enum EvictionEligibility",
            "closed eligibility type",
        ),
        ("if self.pins != 0", "pin exclusion"),
        ("EvictionEligibility::Pinned", "typed pin exclusion"),
        (
            "else if self.writeback_claimed",
            "writeback-claim exclusion",
        ),
        (
            "EvictionEligibility::WritebackClaimed",
            "typed writeback exclusion",
        ),
        ("else if self.dirty", "dirty exclusion"),
        ("EvictionEligibility::Dirty", "typed dirty exclusion"),
        (
            "else if self.loading_waiters != 0",
            "loading-waiter exclusion",
        ),
        (
            "FrameState::Loading | FrameState::LoadFailed(_)",
            "loading lifecycle exclusion",
        ),
        (
            "FrameState::CandidateReserved => EvictionEligibility::CandidatePublication",
            "candidate-publication exclusion",
        ),
        (
            "FrameState::Resident(_) => EvictionEligibility::Legal",
            "resident legal classification",
        ),
        (
            "self.eviction_eligibility() == EvictionEligibility::Legal",
            "exact legal delegation",
        ),
    ] {
        if !source.contains(required) {
            return Err(format!(
                "physical residency boundary: eviction classifier lost {label} in {}",
                path.display()
            ));
        }
    }
    Ok(())
}
