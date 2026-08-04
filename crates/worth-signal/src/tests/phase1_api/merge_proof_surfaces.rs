use super::source_corpus::{
    FACADE_SOURCE, GUIDED_SOURCE, MERGE_CANDIDATE_SCOPE_SOURCE, MERGE_COMPATIBILITY_DENIAL_SOURCE,
    MERGE_COMPATIBILITY_FACTS_SOURCE, MERGE_COMPATIBILITY_READMISSION_SOURCE,
    MERGE_COMPATIBILITY_WITNESS_SOURCE, MERGE_FOUNDATIONAL_SCOPE_SOURCE,
    MERGE_INSPECTION_ABSENCE_SOURCE, MERGE_INSPECTION_SUPPORT_ROWS_SOURCE,
    MERGE_INSPECTION_SUPPORT_WITNESS_SOURCE, MERGE_PLAN_SOURCE, MERGE_PROOF_SOURCE,
    MERGE_RESULT_SOURCE, MERGE_RUNTIME_CANDIDATES_SOURCE, MERGE_RUNTIME_PLAN_SOURCE,
    MERGE_RUNTIME_REQUEST_BOUNDARY_SOURCE, MERGE_SCOPED_PROOF_SOURCE,
    MERGE_STRATEGY_IDENTITY_SOURCE, MERGE_STRATEGY_WITNESS_SOURCE, RECORDER_SOURCE, REPLAY_SOURCE,
};

#[test]
fn merge_foundational_lowering_api_is_source_visible_and_planner_gated() {
    assert!(
        MERGE_FOUNDATIONAL_SCOPE_SOURCE.contains("pub struct LoweredFoundationalMergeRequest"),
        "phase-4 should define a proof-bearing lowered foundational merge request wrapper"
    );
    assert!(
        MERGE_FOUNDATIONAL_SCOPE_SOURCE.contains("pub fn lower_to_foundational_scope"),
        "phase-4 should expose one explicit lowering lane from normalized signal requests into foundational scope"
    );
    assert!(
        MERGE_FOUNDATIONAL_SCOPE_SOURCE.contains("FoundationalMergeScope::selected_nodes")
            && MERGE_FOUNDATIONAL_SCOPE_SOURCE.contains("FoundationalMergeScope::selected_aspects"),
        "phase-4 lowering should use the shared foundational scope constructors instead of a signal-local parallel ontology"
    );
    assert!(
        MERGE_RUNTIME_PLAN_SOURCE.contains("request: &LoweredFoundationalMergeRequest"),
        "merge planner and executor boundaries should consume the lowered foundational request proof"
    );
    assert!(
        MERGE_RUNTIME_REQUEST_BOUNDARY_SOURCE
            .contains("signal_scope_family_matches_foundational_family("),
        "phase-4 planner entry should verify foundational-family parity rather than silently trusting a parallel signal-local scope dialect"
    );
    assert!(
        MERGE_RUNTIME_REQUEST_BOUNDARY_SOURCE.contains("foundational_scope_lowering_count")
            && GUIDED_SOURCE.contains("lower_foundational_merge_request(&request)"),
        "phase-4 lowering should be measured at the runtime boundary and guided merge should route through that one counted lowering seam"
    );
}

#[test]
fn merge_candidate_scope_api_is_source_visible_and_planner_narrowing() {
    assert!(
        MERGE_CANDIDATE_SCOPE_SOURCE.contains("pub struct LoweredScopedMergeCandidateSet"),
        "phase-5 should define a proof-bearing scoped candidate artifact instead of narrowing candidates inline in the planner"
    );
    assert!(
        MERGE_CANDIDATE_SCOPE_SOURCE.contains("pub struct ScopedMergeCandidateBreadthSummary"),
        "phase-5 should preserve explicit requested/admitted/skipped/no-op/support breadth accounting"
    );
    assert!(
        MERGE_CANDIDATE_SCOPE_SOURCE.contains("pub fn lower(")
            && MERGE_CANDIDATE_SCOPE_SOURCE.contains("pub fn with_support_closure_nodes("),
        "phase-5 should expose one lowering seam for scoped candidates plus an explicit support-closure update path"
    );
    assert!(
        MERGE_RUNTIME_CANDIDATES_SOURCE.contains("LoweredScopedMergeCandidateSet::lower(")
            && MERGE_RUNTIME_CANDIDATES_SOURCE
                .contains("scoped_candidates.admitted_candidate_nodes().to_vec()"),
        "phase-5 planner should lower scoped candidates once and feed downstream planning from admitted candidates rather than raw source-journal breadth"
    );
    assert!(
        MERGE_PLAN_SOURCE.contains("scoped_candidates: LoweredScopedMergeCandidateSet")
            && MERGE_PLAN_SOURCE.contains("pub fn scoped_candidates(&self)"),
        "phase-5 merge plans should retain the scoped candidate proof artifact instead of dropping requested/skipped/support posture after planning"
    );
    assert!(
        MERGE_RUNTIME_CANDIDATES_SOURCE.contains("scoped_candidate_lowering_count")
            && MERGE_RUNTIME_CANDIDATES_SOURCE
                .contains("scoped_candidate_support_closure_breadth"),
        "phase-5 candidate lowering should be measured at the runtime boundary so breadth remains performance-visible"
    );
}

#[test]
fn merge_scoped_proof_api_is_source_visible_and_retained_across_plan_and_result() {
    assert!(
        MERGE_SCOPED_PROOF_SOURCE.contains("pub struct ScopedMergeProofPacket"),
        "phase-7 should define a dedicated retained scoped merge proof packet instead of leaking planner-local candidate structures across the proof boundary"
    );
    assert!(
        MERGE_SCOPED_PROOF_SOURCE.contains("from_request_and_candidates"),
        "phase-7 should derive retained scoped proof once from the admitted request and scoped candidate truth"
    );
    assert!(
        MERGE_PLAN_SOURCE.contains("scoped_merge_proof: ScopedMergeProofPacket")
            && MERGE_PLAN_SOURCE.contains("pub fn scoped_merge_proof(&self)"),
        "phase-7 plans should retain the scoped proof packet as a first-class artifact"
    );
    assert!(
        MERGE_RESULT_SOURCE.contains("pub scoped_merge_proof: ScopedMergeProofPacket"),
        "phase-7 results and execution summaries should retain the same scoped proof packet instead of degrading to counters-only merge summaries"
    );
    assert!(
        MERGE_PROOF_SOURCE.contains("pub scoped_merge_proof: ScopedMergeProofPacket")
            && MERGE_PROOF_SOURCE.contains("pub scoped_merge_proof: Option<ScopedMergeProofPacket>"),
        "phase-7 proof reports and replay proof inputs should carry the retained scoped proof packet directly instead of forcing replay to rediscover scope from ambient branch state"
    );
    assert!(
        REPLAY_SOURCE.contains("BranchMergeSummary")
            && REPLAY_SOURCE.contains("as_scoped_merge_proof")
            && RECORDER_SOURCE.contains("scoped_merge_proof: summary.scoped_merge_proof.clone()"),
        "phase-7 retained branch-merge replay history should carry the scoped proof packet directly instead of collapsing it into message-only diagnostics"
    );
}

#[test]
fn merge_strategy_witness_api_is_source_visible_and_retained_across_plan_result_and_replay() {
    assert!(
        MERGE_STRATEGY_WITNESS_SOURCE.contains("pub struct SignalMergeStrategyWitness"),
        "phase-9 should define a dedicated retained merge strategy witness instead of leaving strategy identity smeared across plan and result fields"
    );
    assert!(
        MERGE_STRATEGY_IDENTITY_SOURCE.contains("pub struct SignalMergeStrategyIdentity")
            && MERGE_STRATEGY_IDENTITY_SOURCE
                .contains("pub struct SignalInvalidationStrategyIdentity")
            && MERGE_STRATEGY_IDENTITY_SOURCE
                .contains("pub struct SignalDeliveryStrategyIdentity"),
        "phase-9 should keep merge, invalidation, and delivery posture distinguishable inside the retained strategy witness"
    );
    assert!(
        MERGE_PLAN_SOURCE.contains("strategy_witness: SignalMergeStrategyWitness")
            && MERGE_PLAN_SOURCE.contains("pub fn strategy_witness(&self)"),
        "phase-9 plans should retain the strategy witness as first-class proof instead of rediscovering it from selected semantics"
    );
    assert!(
        MERGE_RESULT_SOURCE.contains("pub strategy_witness: SignalMergeStrategyWitness"),
        "phase-9 execution summaries and results should retain the same strategy witness"
    );
    assert!(
        MERGE_PROOF_SOURCE.contains("pub strategy_witness: SignalMergeStrategyWitness")
            && MERGE_PROOF_SOURCE.contains("pub strategy_witness: Option<SignalMergeStrategyWitness>"),
        "phase-9 proof reports and replay proof inputs should carry the retained strategy witness directly"
    );
    assert!(
        REPLAY_SOURCE.contains("as_strategy_witness")
            && RECORDER_SOURCE.contains("strategy_witness: summary.strategy_witness.clone()"),
        "phase-9 retained replay history should carry the strategy witness directly instead of collapsing strategy posture into message text"
    );
}

#[test]
fn merge_compatibility_witness_api_is_source_visible_and_readmission_prepared() {
    assert!(
        MERGE_COMPATIBILITY_WITNESS_SOURCE.contains("pub struct SignalMergeCompatibilityWitness")
            && MERGE_COMPATIBILITY_WITNESS_SOURCE
                .contains("pub struct SignalMergeCompatibilityBasis"),
        "phase-10 should define a retained compatibility witness and explicit compatibility basis instead of leaving consumers to rediscover compatibility from three separate retained artifacts"
    );
    assert!(
        MERGE_COMPATIBILITY_FACTS_SOURCE
            .contains("pub struct SignalMergeCompatibilityFactInventory")
            && MERGE_COMPATIBILITY_FACTS_SOURCE.contains("from_retained"),
        "phase-10 should project one lower-authority fact inventory directly from retained branch basis, scoped proof, and strategy witness"
    );
    assert!(
        MERGE_COMPATIBILITY_DENIAL_SOURCE.contains("pub enum SignalMergeCompatibilityDenial")
            && MERGE_COMPATIBILITY_DENIAL_SOURCE.contains("StaleBranchBasis")
            && MERGE_COMPATIBILITY_DENIAL_SOURCE.contains("ScopedMergeProofMismatch")
            && MERGE_COMPATIBILITY_DENIAL_SOURCE.contains("StrategyWitnessMismatch"),
        "phase-10 should preserve stale, missing, and mismatched compatibility posture as typed denial instead of collapsing it into generic replay or merge failure text"
    );
    assert!(
        MERGE_COMPATIBILITY_READMISSION_SOURCE
            .contains("pub fn merge_result_compatibility_artifact(")
            && MERGE_COMPATIBILITY_READMISSION_SOURCE
                .contains("pub fn replay_merge_compatibility_artifact(")
            && MERGE_COMPATIBILITY_READMISSION_SOURCE
                .contains("pub fn readmit_merge_compatibility_artifact("),
        "phase-10 should expose one retained compatibility seam across merge result, replay, and explicit authority-backed readmission"
    );
    assert!(
        !MERGE_COMPATIBILITY_READMISSION_SOURCE
            .contains("pub fn merge_compatibility_artifact_from_parts("),
        "phase-10 should not expose a public raw parts seam that can mint compatibility-looking artifacts from caller-assembled retained fragments"
    );
    assert!(
        MERGE_COMPATIBILITY_READMISSION_SOURCE.contains("validate_branch_basis_artifact(")
            && MERGE_COMPATIBILITY_READMISSION_SOURCE.contains("readmit_with_authority("),
        "phase-10 compatibility should be anchored to current branch-basis validation and explicit worth-proof readmission rather than helper-local folklore progression"
    );
    assert!(
        MERGE_COMPATIBILITY_READMISSION_SOURCE.contains("merge_compatibility_build_count")
            && MERGE_COMPATIBILITY_READMISSION_SOURCE
                .contains("merge_compatibility_readmission_denial_count"),
        "phase-10 compatibility and readmission preparation should be measured at the boundary so retained-proof preparation stays performance-visible"
    );
    assert!(
        MERGE_RESULT_SOURCE.contains("pub compatibility_witness: SignalMergeCompatibilityWitness")
            && REPLAY_SOURCE.contains("as_compatibility_witness")
            && RECORDER_SOURCE
                .contains("compatibility_witness: summary.compatibility_witness.clone()"),
        "phase-10 compatibility witness should be retained on merge results and replay history instead of being rebuilt ad hoc from separate retained fragments"
    );
}

#[test]
fn merge_support_inspection_api_is_source_visible_and_support_ready() {
    assert!(
        MERGE_INSPECTION_SUPPORT_WITNESS_SOURCE
            .contains("pub struct SignalMergeSupportInspectionWitness"),
        "phase-11 should define one proof-bearing support inspection witness instead of leaving collaboration-critical posture split across ad hoc helpers"
    );
    assert!(
        MERGE_INSPECTION_SUPPORT_ROWS_SOURCE.contains("pub struct SignalBranchBasisInspectionRow")
            && MERGE_INSPECTION_SUPPORT_ROWS_SOURCE
                .contains("pub struct SignalScopedMergeInspectionRow")
            && MERGE_INSPECTION_SUPPORT_ROWS_SOURCE
                .contains("pub struct SignalStrategyInspectionRow")
            && MERGE_INSPECTION_SUPPORT_ROWS_SOURCE
                .contains("pub struct SignalCompatibilityInspectionRow"),
        "phase-11 should project separate summarized branch-basis, scope, strategy, and compatibility rows instead of flattening retained proof into one opaque support blob"
    );
    assert!(
        MERGE_INSPECTION_ABSENCE_SOURCE
            .contains("pub enum SignalMergeSupportInspectionAbsence"),
        "phase-11 inspection should type missing or incomplete retained proof instead of synthesizing fallback support rows"
    );
    assert!(
        MERGE_INSPECTION_SUPPORT_WITNESS_SOURCE.contains(
            "pub(crate) fn merge_support_inspection_from_retained_parts("
        ) && MERGE_INSPECTION_SUPPORT_WITNESS_SOURCE
            .contains("pub fn merge_result_support_inspection(")
            && MERGE_INSPECTION_SUPPORT_WITNESS_SOURCE
                .contains("pub fn merge_execution_summary_support_inspection(")
            && MERGE_INSPECTION_SUPPORT_WITNESS_SOURCE
            .contains("pub fn replay_merge_support_inspection("),
        "phase-11 should keep the raw retained-parts builder internal while exposing explicit result, execution-summary, and replay inspection lanes for real support consumption"
    );
    assert!(
        FACADE_SOURCE.contains("SignalMergeSupportInspectionWitness")
            && FACADE_SOURCE.contains("SignalMergeSupportReadinessPosture"),
        "facade runtime surfaces should expose the phase-11 support inspection witness and readiness summary"
    );
    assert!(
        MERGE_INSPECTION_SUPPORT_WITNESS_SOURCE.contains("compatibility_witness: Option<&SignalMergeCompatibilityWitness>")
            && MERGE_INSPECTION_SUPPORT_WITNESS_SOURCE.contains("ReplayDetailUnavailable"),
        "phase-11 support inspection should prefer retained compatibility witness when present and deny explicitly when replay does not retain merge support detail"
    );
}
