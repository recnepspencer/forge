use super::wal_tail as wal_only_tail_fixture;
#[cfg(feature = "certification-world")]
use super::{
    reopened_artifact::reopened_redo_eligibility, source_precedence as source_precedence_fixture,
};

#[cfg(feature = "certification-world")]
use source_precedence_fixture::{
    checkpoint_base, checkpoint_base_for_root, wal_only_tail, wal_tail_for_checkpoint,
};
#[cfg(feature = "certification-world")]
use worth_store_physical_format::PhysicalReference;
use worth_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalSegmentId,
};
use worth_store_physical_integrity::{
    ManifestIntegrityAuthority, ManifestIntegrityInspectionRequest,
};
use worth_store_recovery_physics::{
    AdmittedRecoverySource, LogSequenceNumber, PageLsn, PageRedoDigestState, PageRedoEligibility,
    RecoveryCandidateDiscoveryTrace, RedoApplicationCursor, RedoApplicationPageFact,
    RedoRecordGrammar, RedoRecordGrammarDenial, RedoRecordGrammarDenialKind,
    RedoRecordIdempotenceBasis, RedoRecordIntegrityBinding, RedoRecordMaterializedForm,
    RedoRecordOperationForm, RedoRecordTargetGeneration, WalLsnRange,
    WalPrefixIntegrityObservation, WalPrefixObservationScan, WalSegmentGeneration, WalValidPrefix,
};
#[cfg(feature = "certification-world")]
use worth_store_recovery_physics::{RecoverySourceCandidate, RecoverySourcePrecedenceGraph};

pub fn assert_grammar_denial(
    result: Result<RedoRecordGrammar, RedoRecordGrammarDenial>,
    kind: RedoRecordGrammarDenialKind,
) {
    assert_eq!(result.unwrap_err().kind(), kind);
}

#[cfg(feature = "certification-world")]
pub fn checkpoint_plus_tail_source(start: u64, end: u64) -> AdmittedRecoverySource {
    let (checkpoint, receipt) = checkpoint_base(10, start, start - 1, 1);
    checkpoint_plus_tail_source_from_basis(checkpoint, receipt, end)
}

#[cfg(feature = "certification-world")]
pub fn checkpoint_plus_tail_source_for_root(
    start: u64,
    end: u64,
    root: PhysicalReference,
) -> AdmittedRecoverySource {
    let (checkpoint, receipt) = checkpoint_base_for_root(10, start, start - 1, 1, root);
    checkpoint_plus_tail_source_from_basis(checkpoint, receipt, end)
}

#[cfg(feature = "certification-world")]
fn checkpoint_plus_tail_source_from_basis(
    checkpoint: worth_store_recovery_physics::CheckpointBaseAdmission,
    receipt: worth_store_recovery_physics::CheckpointCutoverReceipt,
    end: u64,
) -> AdmittedRecoverySource {
    RecoverySourcePrecedenceGraph::new("strict-test-profile")
        .discover(RecoverySourceCandidate::checkpoint_base(checkpoint))
        .discover(RecoverySourceCandidate::wal_tail(wal_tail_for_checkpoint(
            &receipt, end, 2,
        )))
        .admit_sources()
}

#[cfg(feature = "certification-world")]
pub fn wal_only_source(start: u64, end: u64) -> AdmittedRecoverySource {
    RecoverySourcePrecedenceGraph::new("strict-test-profile")
        .discover(RecoverySourceCandidate::wal_tail(wal_only_tail(
            start, end, 1,
        )))
        .admit_sources()
}

pub fn trace(label: &str, order: u64) -> RecoveryCandidateDiscoveryTrace {
    RecoveryCandidateDiscoveryTrace::new("strict-test-profile", label, order)
}

pub fn wal_range(start: u64, end: u64) -> WalLsnRange {
    WalLsnRange::new(lsn(start), lsn(end)).unwrap()
}

pub fn valid_prefix<const N: usize>(
    source: &AdmittedRecoverySource,
    start: u64,
    end: u64,
    observations: [WalPrefixIntegrityObservation; N],
) -> WalValidPrefix {
    let scan = WalPrefixObservationScan::from_observations(observations.to_vec());
    WalValidPrefix::from_observation_scan(source, wal_generation(), wal_range(start, end), scan)
        .unwrap()
}

#[cfg(feature = "certification-world")]
pub fn redo_eligibility(current_lsn: u64, redo_lsn: u64) -> PageRedoEligibility {
    redo_eligibility_for_page(current_lsn, redo_lsn, 2)
}

#[cfg(feature = "certification-world")]
pub fn redo_eligibility_for_page(
    current_lsn: u64,
    redo_lsn: u64,
    page_value: u64,
) -> PageRedoEligibility {
    reopened_redo_eligibility(current_lsn, redo_lsn, page_value)
}

pub fn cursor(
    eligibility: &PageRedoEligibility,
    page_lsn_value: u64,
    digest: &str,
) -> RedoApplicationCursor {
    let page_generation = eligibility.page_generation();
    RedoApplicationCursor::new(vec![RedoApplicationPageFact::new(
        page_generation.page_id(),
        eligibility.clone(),
        PageRedoDigestState::new(page_generation, page_lsn(page_lsn_value), digest),
    )])
    .unwrap()
}

pub fn grammar_for(
    eligibility: &PageRedoEligibility,
    redo_lsn: u64,
    page_lsn_basis: PageLsn,
) -> Result<RedoRecordGrammar, RedoRecordGrammarDenial> {
    grammar_for_operation_digest(
        eligibility,
        redo_lsn,
        page_lsn_basis,
        &format!("op-{redo_lsn}"),
    )
}

pub fn grammar_for_operation_digest(
    eligibility: &PageRedoEligibility,
    redo_lsn: u64,
    page_lsn_basis: PageLsn,
    operation_digest: &str,
) -> Result<RedoRecordGrammar, RedoRecordGrammarDenial> {
    RedoRecordGrammar::from_materialized_record(RedoRecordMaterializedForm::new(
        eligibility.page_generation().page_id(),
        RedoRecordTargetGeneration::new(eligibility.page_generation()),
        lsn(redo_lsn),
        RedoRecordOperationForm::declared_digest(operation_digest),
        RedoRecordIntegrityBinding::declared_digest(format!("integrity-{redo_lsn}")),
        RedoRecordIdempotenceBasis::declared_digest(format!("idem-{redo_lsn}")),
        page_lsn_basis,
    ))
}

pub fn missing_generation(
    eligibility: &PageRedoEligibility,
) -> Result<RedoRecordGrammar, RedoRecordGrammarDenial> {
    RedoRecordGrammar::admit(
        Some(eligibility.page_generation().page_id()),
        None,
        Some(lsn(20)),
        Some(RedoRecordOperationForm::declared_digest("op")),
        Some(RedoRecordIntegrityBinding::declared_digest("integrity")),
        Some(RedoRecordIdempotenceBasis::declared_digest("idem")),
        Some(page_lsn(20)),
    )
}

pub fn missing_operation(
    eligibility: &PageRedoEligibility,
) -> Result<RedoRecordGrammar, RedoRecordGrammarDenial> {
    RedoRecordGrammar::admit(
        Some(eligibility.page_generation().page_id()),
        Some(RedoRecordTargetGeneration::new(
            eligibility.page_generation(),
        )),
        Some(lsn(20)),
        None,
        Some(RedoRecordIntegrityBinding::declared_digest("integrity")),
        Some(RedoRecordIdempotenceBasis::declared_digest("idem")),
        Some(page_lsn(20)),
    )
}

pub fn missing_integrity(
    eligibility: &PageRedoEligibility,
) -> Result<RedoRecordGrammar, RedoRecordGrammarDenial> {
    RedoRecordGrammar::admit(
        Some(eligibility.page_generation().page_id()),
        Some(RedoRecordTargetGeneration::new(
            eligibility.page_generation(),
        )),
        Some(lsn(20)),
        Some(RedoRecordOperationForm::declared_digest("op")),
        None,
        Some(RedoRecordIdempotenceBasis::declared_digest("idem")),
        Some(page_lsn(20)),
    )
}

pub fn missing_idempotence(
    eligibility: &PageRedoEligibility,
) -> Result<RedoRecordGrammar, RedoRecordGrammarDenial> {
    RedoRecordGrammar::admit(
        Some(eligibility.page_generation().page_id()),
        Some(RedoRecordTargetGeneration::new(
            eligibility.page_generation(),
        )),
        Some(lsn(20)),
        Some(RedoRecordOperationForm::declared_digest("op")),
        Some(RedoRecordIntegrityBinding::declared_digest("integrity")),
        None,
        Some(page_lsn(20)),
    )
}

pub fn missing_page_lsn(
    eligibility: &PageRedoEligibility,
) -> Result<RedoRecordGrammar, RedoRecordGrammarDenial> {
    RedoRecordGrammar::admit(
        Some(eligibility.page_generation().page_id()),
        Some(RedoRecordTargetGeneration::new(
            eligibility.page_generation(),
        )),
        Some(lsn(20)),
        Some(RedoRecordOperationForm::declared_digest("op")),
        Some(RedoRecordIntegrityBinding::declared_digest("integrity")),
        Some(RedoRecordIdempotenceBasis::declared_digest("idem")),
        None,
    )
}

pub fn frame(value: u64) -> WalPrefixIntegrityObservation {
    WalPrefixIntegrityObservation::from_vetted_wal_tail(
        &wal_only_tail_fixture::wal_only_tail_proof(wal_range(value, value + 1)),
        lsn(value),
        wal_generation(),
    )
    .unwrap()
}

pub fn stale_generation_frame(
    value: u64,
    segment_generation: WalSegmentGeneration,
) -> WalPrefixIntegrityObservation {
    WalPrefixIntegrityObservation::from_vetted_wal_tail(
        &wal_only_tail_fixture::wal_only_tail_proof(wal_range(value, value + 1)),
        lsn(value),
        segment_generation,
    )
    .unwrap()
}

pub fn torn_frame(value: u64) -> WalPrefixIntegrityObservation {
    WalPrefixIntegrityObservation::from_quarantined_wal_tail(
        &wal_only_tail_fixture::quarantined_torn_wal_tail_handoff(wal_range(value, value + 1)),
        lsn(value),
        wal_generation(),
    )
}

pub fn middle_corruption_frame(value: u64) -> WalPrefixIntegrityObservation {
    WalPrefixIntegrityObservation::from_recovery_blocking_damage(
        &wal_only_tail_fixture::recovery_blocking_wal_frame_damage(wal_range(value, value + 1)),
        lsn(value),
        wal_generation(),
    )
    .unwrap()
}

pub fn page_lsn(value: u64) -> PageLsn {
    PageLsn::from_lsn(lsn(value))
}

pub fn lsn(value: u64) -> LogSequenceNumber {
    LogSequenceNumber::new(value)
}

pub fn wal_generation() -> WalSegmentGeneration {
    WalSegmentGeneration::new(1).unwrap()
}

pub fn blocked_manifest_damage() -> worth_store_recovery_physics::RecoveryBlockedByIntegrityDamage {
    let denial = ManifestIntegrityAuthority::new()
        .inspect_manifest(ManifestIntegrityInspectionRequest::damaged_root(
            PhysicalGenerationAuthority::for_canonical_physical_format()
                .page_cell(
                    PhysicalSegmentId::from_raw(7).unwrap(),
                    PhysicalPageId::from_raw(7).unwrap(),
                )
                .with_page_generation(PhysicalGeneration::from_raw(7).unwrap())
                .owner(),
        ))
        .unwrap_err();
    worth_store_recovery_physics::RecoveryBlockedByIntegrityDamage::damaged_manifest_root(&denial)
}

pub fn recovery_blocking_torn_wal_frame_damage(
    range: worth_store_recovery_physics::WalLsnRange,
) -> worth_store_recovery_physics::RecoveryBlockedByIntegrityDamage {
    wal_only_tail_fixture::recovery_blocking_torn_wal_frame_damage(range)
}
