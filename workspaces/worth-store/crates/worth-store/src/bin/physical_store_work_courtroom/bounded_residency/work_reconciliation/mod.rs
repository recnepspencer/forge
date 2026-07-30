use std::collections::HashSet;

use worth_store::physical_runtime::{
    ClosedRuntime, LifecycleGeneration, PhysicalWorkEffectFate, PhysicalWorkObservation,
    PhysicalWorkOperationFamily, PhysicalWorkRecoveryDisposition, RuntimeIdentity,
    ServingShutdownOutcome,
};
use worth_store_physical_format::store_namespace::StableStoreIdentity;

mod backend_role;
mod causal_record;
mod causal_route;
mod ordinary_media_window;
mod signal_basis;
mod terminal_identity;

pub(super) use backend_role::PhysicalWorkBackendRoleEvidence;
pub(super) use causal_route::{PhysicalWorkCausalRouteEvidence, PhysicalWorkSignalLineageEvidence};
pub(super) use ordinary_media_window::PhysicalWorkReconciliationWindow;
pub(super) use signal_basis::PhysicalWorkSignalBindingEvidence;

#[derive(Debug)]
pub(super) struct PhysicalWorkReconciliationBasis {
    store: StableStoreIdentity,
    runtime: RuntimeIdentity,
    generation: LifecycleGeneration,
    faults: u64,
    source_loads: u64,
    exact_writebacks: u64,
    identified_metadata_reads: u64,
    identified_positioned_reads: u64,
    identified_positioned_writes: u64,
    signal_bindings: Box<[PhysicalWorkSignalBindingEvidence]>,
}

pub(super) struct PhysicalWorkReconciliationEvidence {
    pub(super) causal_overflow: u64,
    pub(super) terminal_overflow: u64,
    pub(super) safe_evidence_elided: u64,
    pub(super) faults: u64,
    pub(super) source_loads: u64,
    pub(super) exact_writebacks: u64,
    pub(super) identified_metadata_reads: u64,
    pub(super) identified_positioned_reads: u64,
    pub(super) identified_positioned_writes: u64,
    pub(super) settled_terminal_fates: u64,
    pub(super) continued_terminal_fates: u64,
    pub(super) signal_bindings: Box<[PhysicalWorkSignalBindingEvidence]>,
    pub(super) records: Box<[PhysicalWorkReconciliationRecordEvidence]>,
}

pub(super) struct PhysicalWorkReconciliationRecordEvidence {
    pub(super) store: StableStoreIdentity,
    pub(super) runtime: RuntimeIdentity,
    pub(super) generation: LifecycleGeneration,
    pub(super) operation: u64,
    pub(super) family: PhysicalWorkOperationFamily,
    pub(super) backend_operation: u64,
    pub(super) backend_role: PhysicalWorkBackendRoleEvidence,
    pub(super) effect_fate: PhysicalWorkEffectFate,
    pub(super) recovery: PhysicalWorkRecoveryDisposition,
    pub(super) route: causal_route::PhysicalWorkCausalRouteEvidence,
    pub(super) terminal: PhysicalWorkTerminalFateEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PhysicalWorkTerminalFateEvidence {
    Settled,
    ContinuedAfterConsumerCancellation,
}

pub(super) fn reconcile(
    basis: PhysicalWorkReconciliationBasis,
    observer: &PhysicalWorkObservation,
    close: &ServingShutdownOutcome<ClosedRuntime>,
) -> Result<PhysicalWorkReconciliationEvidence, String> {
    let causal_overflow = observer.causal().overflow();
    if causal_overflow != 0 {
        return Err(format!(
            "physical work causal reconciliation overflowed by {causal_overflow}"
        ));
    }
    let drain = close.work().drain();
    let settled_terminal_fates = drain.settled().len() as u64;
    let continued_terminal_fates = drain.continued_after_consumer_cancellation().len() as u64;
    let mut terminal = terminal_identity::TerminalIdentityIndex::from_drain(drain)?;
    let records = observer
        .causal()
        .records()
        .into_vec()
        .into_iter()
        .map(|record| causal_record::reconcile(&basis, record, &mut terminal))
        .collect::<Result<Vec<_>, _>>()?;
    terminal.require_consumed()?;
    require_unique_identities(&records)?;
    require_family_counts(&basis, &records)?;
    Ok(PhysicalWorkReconciliationEvidence {
        causal_overflow,
        terminal_overflow: drain.evidence_overflow(),
        safe_evidence_elided: drain.safe_evidence_elided(),
        faults: basis.faults,
        source_loads: basis.source_loads,
        exact_writebacks: basis.exact_writebacks,
        identified_metadata_reads: basis.identified_metadata_reads,
        identified_positioned_reads: basis.identified_positioned_reads,
        identified_positioned_writes: basis.identified_positioned_writes,
        settled_terminal_fates,
        continued_terminal_fates,
        signal_bindings: basis.signal_bindings,
        records: records.into_boxed_slice(),
    })
}

fn require_unique_identities(
    records: &[PhysicalWorkReconciliationRecordEvidence],
) -> Result<(), String> {
    let work = records
        .iter()
        .map(|record| record.operation)
        .collect::<HashSet<_>>();
    let backend = records
        .iter()
        .map(|record| record.backend_operation)
        .collect::<HashSet<_>>();
    let signal_attempts = records
        .iter()
        .map(|record| {
            (
                record.route.signal.request,
                record.route.signal.generation,
                record.route.signal_attempt,
            )
        })
        .collect::<HashSet<_>>();
    if work.len() != records.len()
        || backend.len() != records.len()
        || signal_attempts.len() != records.len()
    {
        return Err(
            "physical work, Signal attempt, or backend receipt identity was duplicated".to_owned(),
        );
    }
    Ok(())
}

fn require_family_counts(
    basis: &PhysicalWorkReconciliationBasis,
    records: &[PhysicalWorkReconciliationRecordEvidence],
) -> Result<(), String> {
    let count = |family| {
        records
            .iter()
            .filter(|record| record.family == family)
            .count() as u64
    };
    let range_writes = count(PhysicalWorkOperationFamily::ArtifactRangeWrite);
    let range_reads = count(PhysicalWorkOperationFamily::ArtifactRangeRead);
    let metadata_reads = count(PhysicalWorkOperationFamily::ArtifactMetadataRead);
    let role_count = |role| {
        records
            .iter()
            .filter(|record| record.backend_role == role)
            .count() as u64
    };
    let routed_metadata_reads = role_count(PhysicalWorkBackendRoleEvidence::ReadMetadata);
    let positioned_reads = role_count(PhysicalWorkBackendRoleEvidence::PositionedRead);
    let positioned_writes = role_count(PhysicalWorkBackendRoleEvidence::PositionedWrite);
    if basis.faults != basis.source_loads
        || range_reads != basis.source_loads
        || metadata_reads != basis.identified_metadata_reads
        || routed_metadata_reads != basis.identified_metadata_reads
        || positioned_reads != basis.identified_positioned_reads
        || range_writes != basis.exact_writebacks
        || positioned_writes != basis.identified_positioned_writes
    {
        return Err(format!(
            "physical work/media topology drifted: faults={}, loads={}, metadata={}, \
             routed_metadata={}, media_metadata={}, reads={}, routed_reads={}, media_reads={}, \
             receipts={}, writes={}, routed_writes={}, media_writes={}",
            basis.faults,
            basis.source_loads,
            metadata_reads,
            routed_metadata_reads,
            basis.identified_metadata_reads,
            range_reads,
            positioned_reads,
            basis.identified_positioned_reads,
            basis.exact_writebacks,
            range_writes,
            positioned_writes,
            basis.identified_positioned_writes,
        ));
    }
    Ok(())
}
