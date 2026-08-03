use std::collections::HashSet;

use super::super::protocol::{
    BoundedResidencyMediaRole, BoundedResidencySchedulerEvidenceClass,
    BoundedResidencySchedulerProfile, BoundedResidencySignalFamily,
    BoundedResidencySignalSettlement, BoundedResidencyWorkEffectFate, BoundedResidencyWorkFamily,
    BoundedResidencyWorkReconciliationObservation, BoundedResidencyWorkRecovery,
};

#[path = "work_reconciliation/signal_basis.rs"]
mod signal_basis;
#[path = "work_reconciliation/digest.rs"]
mod work_reconciliation_digest;

#[cfg(test)]
#[path = "work_reconciliation/tests.rs"]
mod tests;

pub(super) fn digest(evidence: &BoundedResidencyWorkReconciliationObservation) -> [u8; 32] {
    work_reconciliation_digest::digest(evidence)
}

pub(super) fn verify(
    evidence: &BoundedResidencyWorkReconciliationObservation,
    store: [u8; 16],
    runtime: u64,
    generation: u64,
) -> Result<(), String> {
    require_complete_evidence(evidence)?;
    let expected = ExpectedRuntimeIdentity {
        store,
        runtime,
        generation,
    };
    let mut signal_bindings =
        signal_basis::InstalledSignalBindings::require(&evidence.signal_bindings)?;
    let mut identities = ObservedIdentities::with_capacity(evidence.records.len());
    let mut counts = ReconciledWorkCounts::default();
    for record in &evidence.records {
        verify_record(record, expected, &mut identities, &mut signal_bindings)?;
        counts.record(record)?;
    }
    signal_bindings.require_complete_native_use()?;
    require_exact_counts(evidence, counts)
}

fn require_complete_evidence(
    evidence: &BoundedResidencyWorkReconciliationObservation,
) -> Result<(), String> {
    if evidence.causal_overflow != 0 {
        return Err("physical work reconciliation causal evidence overflowed".to_owned());
    }
    if evidence.terminal_overflow != 0 {
        return Err("physical work reconciliation terminal evidence overflowed".to_owned());
    }
    if evidence.safe_evidence_elided != 0 {
        return Err("physical work reconciliation terminal evidence was elided".to_owned());
    }
    if evidence.records.is_empty() {
        return Err("physical work reconciliation emitted no media-reaching records".to_owned());
    }
    Ok(())
}

fn verify_record(
    record: &super::super::protocol::BoundedResidencyWorkRecordObservation,
    expected: ExpectedRuntimeIdentity,
    identities: &mut ObservedIdentities,
    signal_bindings: &mut signal_basis::InstalledSignalBindings<'_>,
) -> Result<(), String> {
    require_runtime_identity(record, expected)?;
    require_unique_identities(record, identities)?;
    verify_causal_route(record, identities, signal_bindings)
}

fn require_runtime_identity(
    record: &super::super::protocol::BoundedResidencyWorkRecordObservation,
    expected: ExpectedRuntimeIdentity,
) -> Result<(), String> {
    if record.store != expected.store
        || record.runtime != expected.runtime
        || record.generation != expected.generation
    {
        return Err("physical work reconciliation admitted a foreign runtime identity".to_owned());
    }
    Ok(())
}

fn require_unique_identities(
    record: &super::super::protocol::BoundedResidencyWorkRecordObservation,
    identities: &mut ObservedIdentities,
) -> Result<(), String> {
    if !identities.work.insert(record.operation) {
        return Err("physical work reconciliation duplicated a work identity".to_owned());
    }
    if !identities.backend.insert(record.backend_operation) {
        return Err(
            "physical work reconciliation duplicated a backend receipt identity".to_owned(),
        );
    }
    let route = record.route;
    if !identities.signal_attempts.insert((
        route.signal.request,
        route.signal.generation,
        route.signal_attempt,
    )) {
        return Err("physical work reconciliation duplicated a Signal attempt identity".to_owned());
    }
    Ok(())
}

fn verify_causal_route(
    record: &super::super::protocol::BoundedResidencyWorkRecordObservation,
    identities: &mut ObservedIdentities,
    signal_bindings: &mut signal_basis::InstalledSignalBindings<'_>,
) -> Result<(), String> {
    let route = record.route;
    let expected_family = match record.family {
        BoundedResidencyWorkFamily::ArtifactMetadataRead
        | BoundedResidencyWorkFamily::ArtifactRangeRead => BoundedResidencySignalFamily::ReadFault,
        BoundedResidencyWorkFamily::ArtifactRangeWrite => {
            BoundedResidencySignalFamily::ExactWriteback
        }
        BoundedResidencyWorkFamily::ArtifactPublication => {
            BoundedResidencySignalFamily::Publication
        }
    };
    if route.signal_family != expected_family
        || route.signal_binding.iter().all(|byte| *byte == 0)
        || route.signal_settlement == BoundedResidencySignalSettlement::DerivedStateUnavailable
        || route.scheduler_evidence_class
            == BoundedResidencySchedulerEvidenceClass::UnverifiableAssumption
    {
        return Err("physical work reconciliation admitted an inexact causal route".to_owned());
    }
    signal_bindings.require_record(record)?;
    match identities.scheduler_profile {
        None => identities.scheduler_profile = Some(route.scheduler_profile),
        Some(profile) if profile != route.scheduler_profile => {
            return Err("physical work reconciliation mixed scheduler backend profiles".to_owned());
        }
        Some(_) => {}
    }
    Ok(())
}

#[derive(Clone, Copy)]
struct ExpectedRuntimeIdentity {
    store: [u8; 16],
    runtime: u64,
    generation: u64,
}

struct ObservedIdentities {
    work: HashSet<u64>,
    backend: HashSet<u64>,
    signal_attempts: HashSet<(u64, u64, u64)>,
    scheduler_profile: Option<BoundedResidencySchedulerProfile>,
}

impl ObservedIdentities {
    fn with_capacity(capacity: usize) -> Self {
        Self {
            work: HashSet::with_capacity(capacity),
            backend: HashSet::with_capacity(capacity),
            signal_attempts: HashSet::with_capacity(capacity),
            scheduler_profile: None,
        }
    }
}

#[derive(Default)]
struct ReconciledWorkCounts {
    families: [u64; 4],
    metadata_reads: u64,
    positioned_reads: u64,
    positioned_writes: u64,
    settled_terminal_fates: u64,
    continued_terminal_fates: u64,
}

impl ReconciledWorkCounts {
    fn record(
        &mut self,
        record: &super::super::protocol::BoundedResidencyWorkRecordObservation,
    ) -> Result<(), String> {
        let family = match (record.family, record.effect_fate, record.recovery) {
            (
                BoundedResidencyWorkFamily::ArtifactMetadataRead,
                BoundedResidencyWorkEffectFate::ReadCompleted,
                BoundedResidencyWorkRecovery::NoEffect,
            ) => 0,
            (
                BoundedResidencyWorkFamily::ArtifactRangeRead,
                BoundedResidencyWorkEffectFate::ReadCompleted,
                BoundedResidencyWorkRecovery::NoEffect,
            ) => 1,
            (
                BoundedResidencyWorkFamily::ArtifactRangeWrite,
                BoundedResidencyWorkEffectFate::WriteCompleted,
                BoundedResidencyWorkRecovery::ContinueSettlement,
            ) => 2,
            (
                BoundedResidencyWorkFamily::ArtifactPublication,
                BoundedResidencyWorkEffectFate::PublicationCompleted,
                BoundedResidencyWorkRecovery::ContinueSettlement,
            ) => 3,
            _ => {
                return Err(
                    "physical work reconciliation admitted an inexact effect fate or recovery"
                        .to_owned(),
                );
            }
        };
        self.families[family] = self.families[family].saturating_add(1);
        match record.backend_role {
            BoundedResidencyMediaRole::ReadMetadata => {
                self.metadata_reads = self.metadata_reads.saturating_add(1);
            }
            BoundedResidencyMediaRole::PositionedRead => {
                self.positioned_reads = self.positioned_reads.saturating_add(1);
            }
            BoundedResidencyMediaRole::PositionedWrite => {
                self.positioned_writes = self.positioned_writes.saturating_add(1);
            }
            _ => {}
        }
        require_exact_backend_role(record.family, record.backend_role)?;
        match record.terminal {
            super::super::protocol::BoundedResidencyWorkTerminalFate::Settled => {
                self.settled_terminal_fates = self.settled_terminal_fates.saturating_add(1);
            }
            super::super::protocol::BoundedResidencyWorkTerminalFate::ContinuedAfterConsumerCancellation => {
                self.continued_terminal_fates =
                    self.continued_terminal_fates.saturating_add(1);
            }
        }
        Ok(())
    }
}

fn require_exact_counts(
    evidence: &BoundedResidencyWorkReconciliationObservation,
    counts: ReconciledWorkCounts,
) -> Result<(), String> {
    if evidence.faults != evidence.source_loads {
        return Err("physical work reconciliation fault/source-load count drifted".to_owned());
    }
    if counts.families[0] != evidence.identified_metadata_reads
        || counts.metadata_reads != evidence.identified_metadata_reads
    {
        return Err(
            "physical work reconciliation identified metadata-read topology drifted".to_owned(),
        );
    }
    if counts.families[1] != evidence.source_loads
        || counts.positioned_reads != evidence.identified_positioned_reads
    {
        return Err(
            "physical work reconciliation identified positioned-read topology drifted".to_owned(),
        );
    }
    if counts.families[2] != evidence.exact_writebacks {
        return Err(
            "physical work reconciliation exact writeback receipt count drifted".to_owned(),
        );
    }
    if counts.positioned_writes != evidence.identified_positioned_writes {
        return Err(
            "physical work reconciliation identified positioned-write topology drifted".to_owned(),
        );
    }
    if counts.settled_terminal_fates != evidence.settled_terminal_fates {
        return Err("physical work reconciliation settled terminal fate count drifted".to_owned());
    }
    if counts.continued_terminal_fates != evidence.continued_terminal_fates {
        return Err(
            "physical work reconciliation continued terminal fate count drifted".to_owned(),
        );
    }
    Ok(())
}

fn require_exact_backend_role(
    family: BoundedResidencyWorkFamily,
    role: BoundedResidencyMediaRole,
) -> Result<(), String> {
    let exact = match family {
        BoundedResidencyWorkFamily::ArtifactMetadataRead => {
            role == BoundedResidencyMediaRole::ReadMetadata
        }
        BoundedResidencyWorkFamily::ArtifactRangeRead => {
            role == BoundedResidencyMediaRole::PositionedRead
        }
        BoundedResidencyWorkFamily::ArtifactRangeWrite => {
            role == BoundedResidencyMediaRole::PositionedWrite
        }
        BoundedResidencyWorkFamily::ArtifactPublication => matches!(
            role,
            BoundedResidencyMediaRole::PositionedWrite
                | BoundedResidencyMediaRole::SynchronizeFileState
                | BoundedResidencyMediaRole::SynchronizeDirectoryPublication
                | BoundedResidencyMediaRole::AtomicReplace
        ),
    };
    if exact {
        Ok(())
    } else {
        Err("physical work reconciliation admitted an inexact backend media role".to_owned())
    }
}
