use worth_store::physical_runtime::{
    PhysicalSignalAspectRole, PhysicalSignalSettlementOutcome, PhysicalWorkEffectFate,
    PhysicalWorkOperationFamily, PhysicalWorkRecoveryDisposition, PhysicalWorkSignalFamily,
};
use worth_store_physical_backend::{BackendTargetProfile, CapabilityEvidenceClass};

use super::super::work_reconciliation::PhysicalWorkBackendRoleEvidence;
use super::{
    PhysicalWorkCausalRouteEvidence, PhysicalWorkReconciliationEvidence,
    PhysicalWorkReconciliationRecordEvidence, PhysicalWorkSignalBindingEvidence,
    PhysicalWorkSignalLineageEvidence, PhysicalWorkTerminalFateEvidence,
};

pub(super) fn emit(evidence: &PhysicalWorkReconciliationEvidence) {
    println!(
        "BOUNDED_RESIDENCY_WORK_RECONCILIATION {} {} {} {} {} {} {} {} {} {} {} {} {}",
        evidence.causal_overflow,
        evidence.terminal_overflow,
        evidence.safe_evidence_elided,
        evidence.faults,
        evidence.source_loads,
        evidence.exact_writebacks,
        evidence.identified_metadata_reads,
        evidence.identified_positioned_reads,
        evidence.identified_positioned_writes,
        evidence.settled_terminal_fates,
        evidence.continued_terminal_fates,
        evidence.signal_bindings.len(),
        evidence.records.len(),
    );
    for binding in &evidence.signal_bindings {
        emit_signal_binding(binding);
    }
    for record in &evidence.records {
        emit_record(record);
        emit_route(record.operation, &record.route);
    }
}

fn emit_signal_binding(binding: &PhysicalWorkSignalBindingEvidence) {
    println!(
        "BOUNDED_RESIDENCY_SIGNAL_BINDING {} {} {} {} {} {} {} {}",
        hex(&binding.digest),
        binding.aspect_key,
        signal_role(binding.role),
        binding
            .families
            .contains(PhysicalWorkSignalFamily::ReadFault),
        binding
            .families
            .contains(PhysicalWorkSignalFamily::ExactWriteback),
        binding
            .families
            .contains(PhysicalWorkSignalFamily::Publication),
        binding
            .families
            .contains(PhysicalWorkSignalFamily::Lifecycle),
        binding.partition.as_deref().unwrap_or("none"),
    );
}

const fn signal_role(role: PhysicalSignalAspectRole) -> &'static str {
    match role {
        PhysicalSignalAspectRole::Dependency => "dependency",
        PhysicalSignalAspectRole::Output => "output",
        PhysicalSignalAspectRole::DependencyAndOutput => "dependency-and-output",
    }
}

fn emit_route(operation: u64, route: &PhysicalWorkCausalRouteEvidence) {
    println!(
        "BOUNDED_RESIDENCY_WORK_ROUTE {} {} {} {} {} {} {} {} {} {} {} {}",
        operation,
        lineage(route.signal),
        route.predecessor.map_or_else(|| "none".to_owned(), lineage),
        route.signal_attempt,
        signal_family(route.signal_family),
        hex(&route.signal_binding),
        scheduler_profile(route.scheduler_profile),
        scheduler_evidence_class(route.scheduler_evidence_class),
        route.scheduler_grouped_writes,
        route.scheduler_primary_requirement,
        route.scheduler_secondary_present,
        signal_settlement(route.signal_settlement),
    );
}

fn lineage(lineage: PhysicalWorkSignalLineageEvidence) -> String {
    format!(
        "{}:{}:{}:{}",
        lineage.request, lineage.generation, lineage.branch, lineage.restore_epoch
    )
}

const fn signal_family(family: PhysicalWorkSignalFamily) -> &'static str {
    match family {
        PhysicalWorkSignalFamily::ReadFault => "read-fault",
        PhysicalWorkSignalFamily::ExactWriteback => "exact-writeback",
        PhysicalWorkSignalFamily::Publication => "publication",
        PhysicalWorkSignalFamily::Lifecycle => "lifecycle",
        PhysicalWorkSignalFamily::WalAppend => "wal-append",
        PhysicalWorkSignalFamily::DurabilityBarrier => "durability-barrier",
        PhysicalWorkSignalFamily::CheckpointCapture => "checkpoint-capture",
        PhysicalWorkSignalFamily::RootPublication => "root-publication",
    }
}

const fn scheduler_profile(profile: BackendTargetProfile) -> &'static str {
    match profile {
        BackendTargetProfile::SimulatedStrictDurable => "simulated-strict-durable",
        BackendTargetProfile::PosixFileFsyncDirSync => "posix-file-fsync-dir-sync",
        BackendTargetProfile::WindowsFlushFileBuffers => "windows-flush-file-buffers",
        BackendTargetProfile::MmapFlushNotDurabilityCertified => {
            "mmap-flush-not-durability-certified"
        }
        BackendTargetProfile::AdversarialLostFlush => "adversarial-lost-flush",
        BackendTargetProfile::AdversarialReorderedFlush => "adversarial-reordered-flush",
    }
}

const fn scheduler_evidence_class(class: CapabilityEvidenceClass) -> &'static str {
    match class {
        CapabilityEvidenceClass::DeclaredByConfig => "declared-by-config",
        CapabilityEvidenceClass::ObservedByProbe => "observed-by-probe",
        CapabilityEvidenceClass::EstablishedByFilesystemAdmission => {
            "established-by-filesystem-admission"
        }
        CapabilityEvidenceClass::ExternallyGuaranteed => "externally-guaranteed",
        CapabilityEvidenceClass::UnverifiableAssumption => "unverifiable-assumption",
        CapabilityEvidenceClass::CertifiedBackendProfile => "certified-backend-profile",
    }
}

const fn signal_settlement(settlement: PhysicalSignalSettlementOutcome) -> &'static str {
    match settlement {
        PhysicalSignalSettlementOutcome::Committed => "committed",
        PhysicalSignalSettlementOutcome::ReconciledFromPhysicalTruth => {
            "reconciled-from-physical-truth"
        }
        PhysicalSignalSettlementOutcome::DerivedStateUnavailable => "derived-state-unavailable",
    }
}

fn emit_record(record: &PhysicalWorkReconciliationRecordEvidence) {
    println!(
        "BOUNDED_RESIDENCY_WORK_RECORD {} {} {} {} {} {} {} {} {} {}",
        hex(&record.store.bytes()),
        record.runtime.get(),
        record.generation.get(),
        record.operation,
        family(record.family),
        record.backend_operation,
        media_role(record.backend_role),
        effect_fate(record.effect_fate),
        recovery(record.recovery),
        terminal(record.terminal),
    );
}

const fn media_role(role: PhysicalWorkBackendRoleEvidence) -> &'static str {
    match role {
        PhysicalWorkBackendRoleEvidence::CreateNew => "create-new",
        PhysicalWorkBackendRoleEvidence::PositionedRead => "positioned-read",
        PhysicalWorkBackendRoleEvidence::PositionedWrite => "positioned-write",
        PhysicalWorkBackendRoleEvidence::ReadMetadata => "read-metadata",
        PhysicalWorkBackendRoleEvidence::SynchronizeFileState => "synchronize-file-state",
        PhysicalWorkBackendRoleEvidence::SynchronizeDirectoryPublication => {
            "synchronize-directory-publication"
        }
        PhysicalWorkBackendRoleEvidence::AtomicReplace => "atomic-replace",
    }
}

const fn family(family: PhysicalWorkOperationFamily) -> &'static str {
    match family {
        PhysicalWorkOperationFamily::ArtifactMetadataRead => "artifact-metadata-read",
        PhysicalWorkOperationFamily::ArtifactRangeRead => "artifact-range-read",
        PhysicalWorkOperationFamily::ArtifactRangeWrite => "artifact-range-write",
        PhysicalWorkOperationFamily::ArtifactPublication => "artifact-publication",
        PhysicalWorkOperationFamily::WalAppend => "wal-append",
        PhysicalWorkOperationFamily::DurabilityBarrier => "durability-barrier",
    }
}

const fn effect_fate(fate: PhysicalWorkEffectFate) -> &'static str {
    match fate {
        PhysicalWorkEffectFate::ReadCompleted => "read-completed",
        PhysicalWorkEffectFate::WriteCompleted => "write-completed",
        PhysicalWorkEffectFate::PublicationCompleted => "publication-completed",
        PhysicalWorkEffectFate::ProvenNoEffect => "proven-no-effect",
        PhysicalWorkEffectFate::ReadIncomplete => "read-incomplete",
        PhysicalWorkEffectFate::WrittenButSchedulerRejected => "scheduler-rejected",
        PhysicalWorkEffectFate::Indeterminate => "indeterminate",
        PhysicalWorkEffectFate::StaleOrForeignOutcome => "stale-or-foreign",
    }
}

const fn recovery(recovery: PhysicalWorkRecoveryDisposition) -> &'static str {
    match recovery {
        PhysicalWorkRecoveryDisposition::NoEffect => "no-effect",
        PhysicalWorkRecoveryDisposition::RetryExact => "retry-exact",
        PhysicalWorkRecoveryDisposition::ContinueSettlement => "continue-settlement",
        PhysicalWorkRecoveryDisposition::InspectionRequired => "inspection-required",
    }
}

const fn terminal(terminal: PhysicalWorkTerminalFateEvidence) -> &'static str {
    match terminal {
        PhysicalWorkTerminalFateEvidence::Settled => "settled",
        PhysicalWorkTerminalFateEvidence::ContinuedAfterConsumerCancellation => {
            "continued-after-consumer-cancellation"
        }
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
