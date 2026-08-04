use sha2::{Digest, Sha256};

use super::super::super::protocol::{
    BoundedResidencyMediaRole, BoundedResidencySchedulerEvidenceClass,
    BoundedResidencySchedulerProfile, BoundedResidencySignalAspectRole,
    BoundedResidencySignalFamily, BoundedResidencySignalSettlement, BoundedResidencyWorkEffectFate,
    BoundedResidencyWorkFamily, BoundedResidencyWorkReconciliationObservation,
    BoundedResidencyWorkRecovery, BoundedResidencyWorkTerminalFate,
};

pub(super) fn digest(evidence: &BoundedResidencyWorkReconciliationObservation) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"bounded-residency-work-reconciliation-v5");
    digest.update(evidence.causal_overflow.to_le_bytes());
    digest.update(evidence.terminal_overflow.to_le_bytes());
    digest.update(evidence.safe_evidence_elided.to_le_bytes());
    digest.update(evidence.faults.to_le_bytes());
    digest.update(evidence.source_loads.to_le_bytes());
    digest.update(evidence.exact_writebacks.to_le_bytes());
    digest.update(evidence.identified_metadata_reads.to_le_bytes());
    digest.update(evidence.identified_positioned_reads.to_le_bytes());
    digest.update(evidence.identified_positioned_writes.to_le_bytes());
    digest.update(evidence.settled_terminal_fates.to_le_bytes());
    digest.update(evidence.continued_terminal_fates.to_le_bytes());
    digest.update((evidence.signal_bindings.len() as u64).to_le_bytes());
    for binding in &evidence.signal_bindings {
        update_signal_binding(&mut digest, binding);
    }
    digest.update((evidence.records.len() as u64).to_le_bytes());
    for record in &evidence.records {
        update_record(&mut digest, record);
    }
    digest.finalize().into()
}

fn update_signal_binding(
    digest: &mut Sha256,
    binding: &super::super::super::protocol::BoundedResidencySignalBindingObservation,
) {
    digest.update(binding.digest);
    update_text(digest, &binding.aspect_key);
    digest.update([signal_role(binding.role)]);
    digest.update([
        u8::from(binding.families.read_fault),
        u8::from(binding.families.exact_writeback),
        u8::from(binding.families.publication),
        u8::from(binding.families.lifecycle),
        u8::from(binding.families.wal_append),
        u8::from(binding.families.durability_barrier),
        u8::from(binding.families.checkpoint_capture),
        u8::from(binding.families.root_publication),
        u8::from(binding.families.wal_reclamation),
    ]);
    if let Some(partition) = binding.partition.as_deref() {
        digest.update([1]);
        update_text(digest, partition);
    } else {
        digest.update([0]);
    }
}

fn update_record(
    digest: &mut Sha256,
    record: &super::super::super::protocol::BoundedResidencyWorkRecordObservation,
) {
    digest.update(record.store);
    digest.update(record.runtime.to_le_bytes());
    digest.update(record.generation.to_le_bytes());
    digest.update(record.operation.to_le_bytes());
    digest.update([family(record.family)]);
    digest.update(record.backend_operation.to_le_bytes());
    digest.update([media_role(record.backend_role)]);
    digest.update([effect_fate(record.effect_fate)]);
    digest.update([recovery(record.recovery)]);
    digest.update(record.route.signal.request.to_le_bytes());
    digest.update(record.route.signal.generation.to_le_bytes());
    digest.update(record.route.signal.branch.to_le_bytes());
    digest.update(record.route.signal.restore_epoch.to_le_bytes());
    if let Some(predecessor) = record.route.predecessor {
        digest.update([1]);
        digest.update(predecessor.request.to_le_bytes());
        digest.update(predecessor.generation.to_le_bytes());
        digest.update(predecessor.branch.to_le_bytes());
        digest.update(predecessor.restore_epoch.to_le_bytes());
    } else {
        digest.update([0]);
    }
    digest.update(record.route.signal_attempt.to_le_bytes());
    digest.update([signal_family(record.route.signal_family)]);
    digest.update(record.route.signal_binding);
    digest.update([scheduler_profile(record.route.scheduler_profile)]);
    digest.update([scheduler_evidence_class(
        record.route.scheduler_evidence_class,
    )]);
    digest.update(record.route.scheduler_grouped_writes.to_le_bytes());
    digest.update([record.route.scheduler_primary_requirement]);
    digest.update([u8::from(record.route.scheduler_secondary_present)]);
    digest.update([signal_settlement(record.route.signal_settlement)]);
    digest.update([terminal(record.terminal)]);
}

fn update_text(digest: &mut Sha256, value: &str) {
    digest.update((value.len() as u64).to_le_bytes());
    digest.update(value.as_bytes());
}

const fn signal_role(role: BoundedResidencySignalAspectRole) -> u8 {
    match role {
        BoundedResidencySignalAspectRole::Dependency => 1,
        BoundedResidencySignalAspectRole::Output => 2,
        BoundedResidencySignalAspectRole::DependencyAndOutput => 3,
    }
}

const fn media_role(role: BoundedResidencyMediaRole) -> u8 {
    match role {
        BoundedResidencyMediaRole::CreateNew => 1,
        BoundedResidencyMediaRole::PositionedRead => 2,
        BoundedResidencyMediaRole::PositionedWrite => 3,
        BoundedResidencyMediaRole::ReadMetadata => 4,
        BoundedResidencyMediaRole::SynchronizeFileState => 5,
        BoundedResidencyMediaRole::SynchronizeDirectoryPublication => 6,
        BoundedResidencyMediaRole::AtomicReplace => 7,
        BoundedResidencyMediaRole::Delete => 8,
    }
}

const fn signal_family(family: BoundedResidencySignalFamily) -> u8 {
    match family {
        BoundedResidencySignalFamily::ReadFault => 1,
        BoundedResidencySignalFamily::ExactWriteback => 2,
        BoundedResidencySignalFamily::Publication => 3,
        BoundedResidencySignalFamily::Lifecycle => 4,
        BoundedResidencySignalFamily::WalAppend => 5,
        BoundedResidencySignalFamily::DurabilityBarrier => 6,
        BoundedResidencySignalFamily::CheckpointCapture => 7,
        BoundedResidencySignalFamily::RootPublication => 8,
        BoundedResidencySignalFamily::WalReclamation => 9,
    }
}

const fn scheduler_profile(profile: BoundedResidencySchedulerProfile) -> u8 {
    match profile {
        BoundedResidencySchedulerProfile::SimulatedStrictDurable => 1,
        BoundedResidencySchedulerProfile::PosixFileFsyncDirSync => 2,
        BoundedResidencySchedulerProfile::WindowsFlushFileBuffers => 3,
        BoundedResidencySchedulerProfile::MmapFlushNotDurabilityCertified => 4,
        BoundedResidencySchedulerProfile::AdversarialLostFlush => 5,
        BoundedResidencySchedulerProfile::AdversarialReorderedFlush => 6,
    }
}

const fn scheduler_evidence_class(class: BoundedResidencySchedulerEvidenceClass) -> u8 {
    match class {
        BoundedResidencySchedulerEvidenceClass::DeclaredByConfig => 1,
        BoundedResidencySchedulerEvidenceClass::ObservedByProbe => 2,
        BoundedResidencySchedulerEvidenceClass::EstablishedByFilesystemAdmission => 3,
        BoundedResidencySchedulerEvidenceClass::ExternallyGuaranteed => 4,
        BoundedResidencySchedulerEvidenceClass::UnverifiableAssumption => 5,
        BoundedResidencySchedulerEvidenceClass::CertifiedBackendProfile => 6,
    }
}

const fn signal_settlement(settlement: BoundedResidencySignalSettlement) -> u8 {
    match settlement {
        BoundedResidencySignalSettlement::Committed => 1,
        BoundedResidencySignalSettlement::ReconciledFromPhysicalTruth => 2,
        BoundedResidencySignalSettlement::DerivedStateUnavailable => 3,
    }
}

const fn family(family: BoundedResidencyWorkFamily) -> u8 {
    match family {
        BoundedResidencyWorkFamily::ArtifactMetadataRead => 1,
        BoundedResidencyWorkFamily::ArtifactRangeRead => 2,
        BoundedResidencyWorkFamily::ArtifactRangeWrite => 3,
        BoundedResidencyWorkFamily::ArtifactPublication => 4,
        BoundedResidencyWorkFamily::WalAppend => 5,
        BoundedResidencyWorkFamily::DurabilityBarrier => 6,
        BoundedResidencyWorkFamily::CheckpointCapture => 7,
        BoundedResidencyWorkFamily::RootPublication => 8,
        BoundedResidencyWorkFamily::WalReclamation => 9,
    }
}

const fn effect_fate(fate: BoundedResidencyWorkEffectFate) -> u8 {
    match fate {
        BoundedResidencyWorkEffectFate::ReadCompleted => 1,
        BoundedResidencyWorkEffectFate::WriteCompleted => 2,
        BoundedResidencyWorkEffectFate::PublicationCompleted => 3,
        BoundedResidencyWorkEffectFate::CheckpointCompleted => 4,
        BoundedResidencyWorkEffectFate::WalReclamationCompleted => 5,
    }
}

const fn recovery(recovery: BoundedResidencyWorkRecovery) -> u8 {
    match recovery {
        BoundedResidencyWorkRecovery::NoEffect => 1,
        BoundedResidencyWorkRecovery::ContinueSettlement => 2,
    }
}

const fn terminal(terminal: BoundedResidencyWorkTerminalFate) -> u8 {
    match terminal {
        BoundedResidencyWorkTerminalFate::Settled => 1,
        BoundedResidencyWorkTerminalFate::ContinuedAfterConsumerCancellation => 2,
    }
}
