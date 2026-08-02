use super::super::super::read_repository_document;
use super::{contains_in_order, function_body, function_signature};

const SUBMISSION: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                          record_serving/publication/director/submission.rs";
const DISPATCH: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                        record_serving/publication/director/durable_data.rs";
const MANAGED_MUTATION: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                                record_serving/publication/director/managed_mutation.rs";
const CANDIDATE_CLEANUP: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                                 record_serving/publication/director/durable_data/\
                                 candidate_cleanup.rs";
const FAILURE_OUTCOME: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                               record_serving/publication/director/durable_data/\
                               failure_outcome.rs";
const EFFECT_PROGRESSION: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                                  record_serving/publication/director/durable_data/\
                                  effect_progression.rs";
const PAGE_BASIS: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                          durability/data/page_wal_basis.rs";
const PREPARED_PLAN: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                             durability/data/prepared_plan.rs";
const DISPATCHED: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                          durability/mutation/progression/data_dispatched.rs";
const SETTLED: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                       durability/mutation/progression/data_settled.rs";
const JOIN: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                    durability/data/writeback_join.rs";
const WRITEBACK: &str = "workspaces/worth-store/crates/worth-store/src/physical_runtime/\
                         record_serving/residency/dirty/writeback/execution.rs";
const CANDIDATE_EVIDENCE: &str =
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/\
     residency/candidate_frame_residency/write_evidence.rs";

#[test]
fn wal_bound_frames_and_exact_existing_artifact_effects_are_required_for_data_settlement() {
    inspect(&sources()).unwrap();
}

#[test]
fn data_gate_rejects_inversion_derived_basis_substitution_and_early_clean_mutants() {
    let source = sources();

    let mut inversion = source.clone();
    inversion.dispatch = inversion.dispatch.replace(
        "durable: WalDurablePhysicalMutation",
        "durable: WalAppendedPhysicalMutation",
    );
    assert!(inspect(&inversion).is_err());

    let mut caller_digest = source.clone();
    caller_digest.page_basis = caller_digest.page_basis.replace(
        "resulting_payload_digest: Sha256::digest(encoded_frame).into()",
        "resulting_payload_digest: [0; 32]",
    );
    assert!(inspect(&caller_digest).is_err());

    let mut substituted_basis = source.clone();
    substituted_basis.join = substituted_basis
        .join
        .replace("frame.basis() != effect.basis()", "false");
    assert!(inspect(&substituted_basis).is_err());

    let mut raw_settlement = source.clone();
    raw_settlement.settled = raw_settlement.settled.replace(
        "settlement: CompletionBoundPhysicalDataSettlement",
        "settlement: DataDispatchedPhysicalMutation",
    );
    assert!(inspect(&raw_settlement).is_err());

    let mut early_clean = source;
    early_clean.writeback = early_clean.writeback.replace(
        "let settled_success = settlement.effect_fate() == PhysicalWorkEffectFate::WriteCompleted",
        "let settled_success = true || settlement.effect_fate() == PhysicalWorkEffectFate::WriteCompleted",
    );
    assert!(inspect(&early_clean).is_err());
}

#[test]
fn data_gate_rejects_unsafe_candidate_cleanup_and_unproved_retry_mutants() {
    let source = sources();

    let mut materialized_target = source.clone();
    materialized_target.candidate_cleanup = materialized_target.candidate_cleanup.replace(
        "prior.is_materialized() || prior.identity() != target",
        "prior.identity() != target",
    );
    assert!(inspect(&materialized_target).is_err());

    let mut unrelated_target = source.clone();
    unrelated_target.candidate_cleanup = unrelated_target.candidate_cleanup.replace(
        "prior.is_materialized() || prior.identity() != target",
        "prior.is_materialized()",
    );
    assert!(inspect(&unrelated_target).is_err());

    let mut non_extent = source.clone();
    non_extent.candidate_cleanup = non_extent.candidate_cleanup.replace(
        "if !matches!(artifact, RecordArtifactFile::Extent { .. })",
        "if false",
    );
    assert!(inspect(&non_extent).is_err());

    let mut nondurable_delete = source.clone();
    nondurable_delete.candidate_cleanup = nondurable_delete.candidate_cleanup.replace(
        "tree.remove_file_durably(artifact)",
        "tree.remove_file(artifact)",
    );
    assert!(inspect(&nondurable_delete).is_err());

    let mut unproved_retry = source;
    unproved_retry.failure_outcome = unproved_retry.failure_outcome.replace(
        "cleanup_extent_candidate_data(media, &durable)",
        "Some(Vec::new())",
    );
    assert!(inspect(&unproved_retry).is_err());
}

#[derive(Clone)]
struct DataSources {
    submission: String,
    dispatch: String,
    managed_mutation: String,
    candidate_cleanup: String,
    failure_outcome: String,
    effect_progression: String,
    page_basis: String,
    prepared_plan: String,
    dispatched: String,
    settled: String,
    join: String,
    writeback: String,
    candidate_evidence: String,
}

fn sources() -> DataSources {
    DataSources {
        submission: read_repository_document(SUBMISSION).expect("read data dispatch facade"),
        dispatch: read_repository_document(DISPATCH).expect("read data dispatch owner"),
        managed_mutation: read_repository_document(MANAGED_MUTATION)
            .expect("read managed mutation driver"),
        candidate_cleanup: read_repository_document(CANDIDATE_CLEANUP)
            .expect("read candidate cleanup owner"),
        failure_outcome: read_repository_document(FAILURE_OUTCOME)
            .expect("read data failure projection"),
        effect_progression: read_repository_document(EFFECT_PROGRESSION)
            .expect("read data effect progression"),
        page_basis: read_repository_document(PAGE_BASIS).expect("read page WAL basis"),
        prepared_plan: read_repository_document(PREPARED_PLAN).expect("read prepared data plan"),
        dispatched: read_repository_document(DISPATCHED).expect("read dispatched progression"),
        settled: read_repository_document(SETTLED).expect("read settled progression"),
        join: read_repository_document(JOIN).expect("read exact data settlement join"),
        writeback: read_repository_document(WRITEBACK)
            .expect("read existing-artifact writeback execution"),
        candidate_evidence: read_repository_document(CANDIDATE_EVIDENCE)
            .expect("read candidate effect evidence"),
    }
}

fn inspect(source: &DataSources) -> Result<(), &'static str> {
    inspect_wal_before_data(
        &source.submission,
        &source.dispatch,
        &source.managed_mutation,
    )?;
    inspect_candidate_cleanup(&source.candidate_cleanup, &source.failure_outcome)?;
    inspect_page_basis(&source.page_basis, &source.prepared_plan)?;
    inspect_effect_sources(
        &source.dispatch,
        &source.effect_progression,
        &source.failure_outcome,
        &source.join,
        &source.candidate_evidence,
    )?;
    inspect_exact_join(&source.dispatched, &source.settled, &source.join)?;
    inspect_existing_artifact_cleaning(&source.writeback)?;
    Ok(())
}

fn inspect_candidate_cleanup(cleanup: &str, failure_outcome: &str) -> Result<(), &'static str> {
    let body = function_body(cleanup, "pub(super) fn cleanup_extent_candidate_data(")
        .ok_or("candidate cleanup owner is absent")?;
    for required in [
        "prior.is_materialized() || prior.identity() != target",
        "if !matches!(artifact, RecordArtifactFile::Extent { .. })",
        "tree.file_exists(artifact)",
        "tree.remove_file_durably(artifact)",
        "matches!(tree.file_exists(artifact), Ok(false))",
    ] {
        if !body.contains(required) {
            return Err("candidate cleanup lost an absence, identity, family, or durability guard");
        }
    }
    let retry = function_body(failure_outcome, "fn retry_after_cleaned_pressure(")
        .ok_or("cleaned pressure retry projection is absent")?;
    for required in [
        "failure.pressure(generation)",
        "cleanup_extent_candidate_data(media, &durable)",
        "let Some(pressure)",
        "let Some(deleted_artifacts)",
        "PhysicalDataDispatchOutcome::RetryableAfterCleanup",
        "PhysicalDataDispatchOutcome::Indeterminate",
    ] {
        if !retry.contains(required) {
            return Err("retryable pressure outcome escaped proved cleanup");
        }
    }
    Ok(())
}

fn inspect_wal_before_data(
    submission: &str,
    dispatch: &str,
    managed_mutation: &str,
) -> Result<(), &'static str> {
    if submission.contains("pub fn dispatch_wal_durable_data(") {
        return Err("ordinary callers can still drive data dispatch");
    }
    let managed = function_body(managed_mutation, "fn execute_managed_mutation(")
        .ok_or("managed mutation driver is absent")?;
    if !managed.contains("self.dispatch_wal_durable_data(durable)") {
        return Err("Store-managed mutation bypasses the semantic data owner");
    }
    let owner = function_signature(dispatch, "pub(super) fn dispatch_wal_durable_data(")
        .ok_or("data dispatch owner is absent")?;
    if !owner.contains("durable: WalDurablePhysicalMutation") {
        return Err("data dispatch owner accepts weaker WAL evidence");
    }
    for required in [
        "dispatch_admission_failure(&durable)",
        "identity.store_identity() != self.durability.store_identity()",
        "identity.runtime_identity() != self.durability.runtime_identity()",
        "durable.appended().reserved().signal_profile() != self.signal_profile",
    ] {
        if !dispatch.contains(required) {
            return Err("data dispatch lost stale or foreign authority admission");
        }
    }
    Ok(())
}

fn inspect_page_basis(page_basis: &str, prepared_plan: &str) -> Result<(), &'static str> {
    let signature = function_signature(page_basis, "fn from_encoded_frame")
        .ok_or("encoded-frame page WAL basis is absent")?;
    if signature.contains("digest") || !signature.contains("encoded_frame: &[u8]") {
        return Err("caller can supply the resulting frame digest");
    }
    let body = function_body(page_basis, "fn from_encoded_frame")
        .ok_or("encoded-frame page WAL basis body is absent")?;
    for required in [
        "strictly_advancing_result",
        "decode_data_frame_page_lsn(encoded_frame",
        "resulting_payload_digest: Sha256::digest(encoded_frame).into()",
    ] {
        if !body.contains(required) {
            return Err("page WAL basis is not derived from exact encoded bytes");
        }
    }
    for required in [
        "PageWalBasis::from_encoded_frame(",
        "PhysicalRedoTargetClaim::new(frame.target, basis.resulting_payload_digest())",
    ] {
        if !prepared_plan.contains(required) {
            return Err("WAL target claims disagree with the encoded frame basis");
        }
    }
    Ok(())
}

fn inspect_effect_sources(
    dispatch: &str,
    effect_progression: &str,
    failure_outcome: &str,
    join: &str,
    candidate_evidence: &str,
) -> Result<(), &'static str> {
    for source in [join, candidate_evidence] {
        if !source.contains("NewArtifact") || !source.contains("ExistingArtifactWriteback") {
            return Err("new and existing-artifact data effects collapsed into one authority");
        }
    }
    for required in [
        "effect_progression::DurableFrameDispatch::new",
        ".execute(durable, residency)",
    ] {
        if !dispatch.contains(required) {
            return Err("data dispatch bypasses its effect-progression owner");
        }
    }
    for required in [
        "write_new_candidate",
        "write_existing_artifact_candidate",
        "PhysicalDataEffectSettlement::from_candidate",
        "residency.require_complete()",
        "DataDispatchedPhysicalMutation::new",
        "PhysicalDataDispatchOutcome::Indeterminate",
    ] {
        if !effect_progression.contains(required) {
            return Err("data effect progression lost an exact effect or completion posture");
        }
    }
    for required in [
        "CandidateFrameFailurePosture::ProvenNoEffect",
        "CandidateFrameFailurePosture::UnsettledBeforeEffect",
        "CandidateFrameFailurePosture::EffectPossible",
        "PhysicalDataDispatchOutcome::Indeterminate",
    ] {
        if !failure_outcome.contains(required) {
            return Err("data failure projection lost an exact failure posture");
        }
    }
    Ok(())
}

fn inspect_exact_join(dispatched: &str, settled: &str, join: &str) -> Result<(), &'static str> {
    let settle = function_body(dispatched, "pub fn settle_exact_effects(")
        .ok_or("public exact-effect join is absent")?;
    if !settle.contains("join_dispatched_data(self)") {
        return Err("data-dispatched authority bypasses the exact join");
    }
    for predicate in [
        "expected.is_empty() || effects.is_empty()",
        "expected.len() != effects.len()",
        "frame.basis() != effect.basis()",
        "!range.contains(redo.lsn())",
        "effect.coordinate() != target",
        "effect.payload_digest() != frame.basis().resulting_payload_digest()",
        "!effect_has_completed_fate(effect)",
        "effect.effect_identity().is_none()",
        "!work.insert(effect.work_identity())",
        "effect.source() != expected_source",
        "CompletionBoundPhysicalDataSettlement(dispatched)",
    ] {
        if !join.contains(predicate) {
            return Err("exact data settlement lost a required identity or completion predicate");
        }
    }
    if !settled.contains("settlement: CompletionBoundPhysicalDataSettlement")
        || !settled.contains("pub(in crate::physical_runtime) fn new(")
    {
        return Err("raw dispatched data can construct settled authority");
    }
    Ok(())
}

fn inspect_existing_artifact_cleaning(source: &str) -> Result<(), &'static str> {
    let body = function_body(source, "fn execute(")
        .ok_or("existing-artifact writeback execution is absent")?;
    if !contains_in_order(
        body,
        &[
            "PhysicalWritebackSettlement::from_settled",
            "let settled_success = settlement.effect_fate() == PhysicalWorkEffectFate::WriteCompleted",
            "settlement.recovery() != PhysicalWorkRecoveryDisposition::InspectionRequired",
            "completion",
            ".publish_clean(",
            "PhysicalWritebackExecution::Clean(settlement)",
        ],
    ) {
        return Err("existing-artifact writeback can clean before exact completed settlement");
    }
    Ok(())
}
