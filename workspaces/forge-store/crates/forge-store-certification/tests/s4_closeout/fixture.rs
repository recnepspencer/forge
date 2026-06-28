#[path = "../s4_bounded_recovery_budget/memory_budget_fixture.rs"]
mod memory_budget_fixture;
#[allow(dead_code, unused_imports)]
#[path = "../s4_idempotent_redo_replay/redo_replay_fixture.rs"]
mod redo_replay_fixture;
#[allow(dead_code, unused_imports)]
#[path = "../s4_recovery_source_precedence/source_precedence_fixture.rs"]
mod source_precedence_fixture;

#[allow(dead_code)]
mod closeout_collectors;
#[allow(dead_code)]
mod crash_evidence;
mod executed_recovery;
mod foundational_evidence;
#[allow(dead_code)]
mod shortcut_evidence;

#[allow(unused_imports)]
pub use closeout_collectors::{
    certify_complete_closeout, complete_closeout_evidence, evidence_with_missing_crash_seam,
    evidence_with_missing_shortcut_rejection_denial, mixed_authority_closeout_denial,
    unbounded_closeout_denial,
};
#[allow(unused_imports)]
pub use crash_evidence::{
    missing_crash_scheduler_evidence_denial, same_process_runtime_report_denial,
};
#[allow(unused_imports)]
pub use shortcut_evidence::{unrelated_budget_shortcut_denial, unrelated_residue_shortcut_denial};
