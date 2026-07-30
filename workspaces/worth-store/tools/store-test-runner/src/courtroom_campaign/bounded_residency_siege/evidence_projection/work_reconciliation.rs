use serde_json::{json, Value};

use super::super::protocol::{
    BoundedResidencyMediaRole, BoundedResidencySchedulerEvidenceClass,
    BoundedResidencySchedulerProfile, BoundedResidencySignalAspectRole,
    BoundedResidencySignalBindingObservation, BoundedResidencySignalFamily,
    BoundedResidencySignalSettlement, BoundedResidencyWorkEffectFate, BoundedResidencyWorkFamily,
    BoundedResidencyWorkReconciliationObservation, BoundedResidencyWorkRecordObservation,
    BoundedResidencyWorkRecovery, BoundedResidencyWorkTerminalFate,
};
use crate::physical_work_evidence::hex;

pub(super) fn value(evidence: &BoundedResidencyWorkReconciliationObservation) -> Value {
    json!({
        "causal_overflow": evidence.causal_overflow,
        "terminal_overflow": evidence.terminal_overflow,
        "safe_evidence_elided": evidence.safe_evidence_elided,
        "faults": evidence.faults,
        "source_loads": evidence.source_loads,
        "exact_writebacks": evidence.exact_writebacks,
        "identified_metadata_reads": evidence.identified_metadata_reads,
        "identified_positioned_reads": evidence.identified_positioned_reads,
        "identified_positioned_writes": evidence.identified_positioned_writes,
        "terminal_fates": {
            "settled": evidence.settled_terminal_fates,
            "continued_after_consumer_cancellation": evidence.continued_terminal_fates,
        },
        "signal_bindings": evidence
            .signal_bindings
            .iter()
            .map(signal_binding)
            .collect::<Vec<_>>(),
        "records": evidence.records.iter().map(record).collect::<Vec<_>>(),
    })
}

fn signal_binding(binding: &BoundedResidencySignalBindingObservation) -> Value {
    json!({
        "digest": hex(&binding.digest),
        "aspect_key": binding.aspect_key,
        "role": signal_role(binding.role),
        "families": {
            "read_fault": binding.families.read_fault,
            "exact_writeback": binding.families.exact_writeback,
            "publication": binding.families.publication,
            "lifecycle": binding.families.lifecycle,
        },
        "partition": binding.partition,
    })
}

const fn signal_role(role: BoundedResidencySignalAspectRole) -> &'static str {
    match role {
        BoundedResidencySignalAspectRole::Dependency => "dependency",
        BoundedResidencySignalAspectRole::Output => "output",
        BoundedResidencySignalAspectRole::DependencyAndOutput => "dependency-and-output",
    }
}

fn record(record: &BoundedResidencyWorkRecordObservation) -> Value {
    json!({
        "store": hex(&record.store),
        "runtime": record.runtime,
        "generation": record.generation,
        "operation": record.operation,
        "family": family(record.family),
        "backend_operation": record.backend_operation,
        "backend_role": media_role(record.backend_role),
        "effect_fate": effect_fate(record.effect_fate),
        "recovery": recovery(record.recovery),
        "route": {
            "signal": {
                "request": record.route.signal.request,
                "generation": record.route.signal.generation,
                "branch": record.route.signal.branch,
                "restore_epoch": record.route.signal.restore_epoch,
                "attempt": record.route.signal_attempt,
                "family": signal_family(record.route.signal_family),
                "binding": hex(&record.route.signal_binding),
            },
            "predecessor": record.route.predecessor.map(|predecessor| json!({
                "request": predecessor.request,
                "generation": predecessor.generation,
                "branch": predecessor.branch,
                "restore_epoch": predecessor.restore_epoch,
            })),
            "scheduler": {
                "profile": scheduler_profile(record.route.scheduler_profile),
                "evidence_class": scheduler_evidence_class(
                    record.route.scheduler_evidence_class
                ),
                "grouped_writes": record.route.scheduler_grouped_writes,
                "primary_requirement": record.route.scheduler_primary_requirement,
                "secondary_present": record.route.scheduler_secondary_present,
            },
            "signal_settlement": signal_settlement(record.route.signal_settlement),
        },
        "terminal": terminal(record.terminal),
    })
}

const fn media_role(role: BoundedResidencyMediaRole) -> &'static str {
    match role {
        BoundedResidencyMediaRole::CreateNew => "create-new",
        BoundedResidencyMediaRole::PositionedRead => "positioned-read",
        BoundedResidencyMediaRole::PositionedWrite => "positioned-write",
        BoundedResidencyMediaRole::ReadMetadata => "read-metadata",
        BoundedResidencyMediaRole::SynchronizeFileState => "synchronize-file-state",
        BoundedResidencyMediaRole::SynchronizeDirectoryPublication => {
            "synchronize-directory-publication"
        }
        BoundedResidencyMediaRole::AtomicReplace => "atomic-replace",
    }
}

const fn signal_family(family: BoundedResidencySignalFamily) -> &'static str {
    match family {
        BoundedResidencySignalFamily::ReadFault => "read-fault",
        BoundedResidencySignalFamily::ExactWriteback => "exact-writeback",
        BoundedResidencySignalFamily::Publication => "publication",
        BoundedResidencySignalFamily::Lifecycle => "lifecycle",
    }
}

const fn scheduler_profile(profile: BoundedResidencySchedulerProfile) -> &'static str {
    match profile {
        BoundedResidencySchedulerProfile::SimulatedStrictDurable => "simulated-strict-durable",
        BoundedResidencySchedulerProfile::PosixFileFsyncDirSync => "posix-file-fsync-dir-sync",
        BoundedResidencySchedulerProfile::WindowsFlushFileBuffers => "windows-flush-file-buffers",
        BoundedResidencySchedulerProfile::MmapFlushNotDurabilityCertified => {
            "mmap-flush-not-durability-certified"
        }
        BoundedResidencySchedulerProfile::AdversarialLostFlush => "adversarial-lost-flush",
        BoundedResidencySchedulerProfile::AdversarialReorderedFlush => {
            "adversarial-reordered-flush"
        }
    }
}

const fn scheduler_evidence_class(class: BoundedResidencySchedulerEvidenceClass) -> &'static str {
    match class {
        BoundedResidencySchedulerEvidenceClass::DeclaredByConfig => "declared-by-config",
        BoundedResidencySchedulerEvidenceClass::ObservedByProbe => "observed-by-probe",
        BoundedResidencySchedulerEvidenceClass::EstablishedByFilesystemAdmission => {
            "established-by-filesystem-admission"
        }
        BoundedResidencySchedulerEvidenceClass::ExternallyGuaranteed => "externally-guaranteed",
        BoundedResidencySchedulerEvidenceClass::UnverifiableAssumption => "unverifiable-assumption",
        BoundedResidencySchedulerEvidenceClass::CertifiedBackendProfile => {
            "certified-backend-profile"
        }
    }
}

const fn signal_settlement(settlement: BoundedResidencySignalSettlement) -> &'static str {
    match settlement {
        BoundedResidencySignalSettlement::Committed => "committed",
        BoundedResidencySignalSettlement::ReconciledFromPhysicalTruth => {
            "reconciled-from-physical-truth"
        }
        BoundedResidencySignalSettlement::DerivedStateUnavailable => "derived-state-unavailable",
    }
}

const fn family(family: BoundedResidencyWorkFamily) -> &'static str {
    match family {
        BoundedResidencyWorkFamily::ArtifactMetadataRead => "artifact-metadata-read",
        BoundedResidencyWorkFamily::ArtifactRangeRead => "artifact-range-read",
        BoundedResidencyWorkFamily::ArtifactRangeWrite => "artifact-range-write",
        BoundedResidencyWorkFamily::ArtifactPublication => "artifact-publication",
    }
}

const fn effect_fate(fate: BoundedResidencyWorkEffectFate) -> &'static str {
    match fate {
        BoundedResidencyWorkEffectFate::ReadCompleted => "read-completed",
        BoundedResidencyWorkEffectFate::WriteCompleted => "write-completed",
        BoundedResidencyWorkEffectFate::PublicationCompleted => "publication-completed",
    }
}

const fn recovery(recovery: BoundedResidencyWorkRecovery) -> &'static str {
    match recovery {
        BoundedResidencyWorkRecovery::NoEffect => "no-effect",
        BoundedResidencyWorkRecovery::ContinueSettlement => "continue-settlement",
    }
}

const fn terminal(terminal: BoundedResidencyWorkTerminalFate) -> &'static str {
    match terminal {
        BoundedResidencyWorkTerminalFate::Settled => "settled",
        BoundedResidencyWorkTerminalFate::ContinuedAfterConsumerCancellation => {
            "continued-after-consumer-cancellation"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::courtroom_campaign::bounded_residency_siege::protocol::BoundedResidencySignalFamilySet;

    #[test]
    fn projection_preserves_every_identity_receipt_fate_and_terminal_field() {
        let evidence = BoundedResidencyWorkReconciliationObservation {
            causal_overflow: 0,
            terminal_overflow: 0,
            safe_evidence_elided: 0,
            faults: 1,
            source_loads: 1,
            exact_writebacks: 0,
            identified_metadata_reads: 0,
            identified_positioned_reads: 1,
            identified_positioned_writes: 0,
            settled_terminal_fates: 1,
            continued_terminal_fates: 0,
            signal_bindings: vec![BoundedResidencySignalBindingObservation {
                digest: [7; 32],
                aspect_key: "store.physical.record.frame-read-basis".to_owned(),
                role: BoundedResidencySignalAspectRole::Dependency,
                families: BoundedResidencySignalFamilySet {
                    read_fault: true,
                    exact_writeback: false,
                    publication: false,
                    lifecycle: false,
                },
                partition: Some("store.physical.record.frame".to_owned()),
            }]
            .into_boxed_slice(),
            records: vec![BoundedResidencyWorkRecordObservation {
                store: [7; 16],
                runtime: 11,
                generation: 13,
                operation: 17,
                family: BoundedResidencyWorkFamily::ArtifactRangeRead,
                backend_operation: 19,
                backend_role: BoundedResidencyMediaRole::PositionedRead,
                effect_fate: BoundedResidencyWorkEffectFate::ReadCompleted,
                recovery: BoundedResidencyWorkRecovery::NoEffect,
                route: crate::courtroom_campaign::bounded_residency_siege::protocol::exact_route_fixture(
                    17,
                    BoundedResidencyWorkFamily::ArtifactRangeRead,
                    [7; 32],
                ),
                terminal: BoundedResidencyWorkTerminalFate::Settled,
            }]
            .into_boxed_slice(),
        };
        let encoded = value(&evidence);
        assert_eq!(encoded["faults"], 1);
        assert_eq!(encoded["identified_metadata_reads"], 0);
        assert_eq!(encoded["terminal_fates"]["settled"], 1);
        assert_eq!(
            encoded["signal_bindings"][0]["digest"],
            "0707070707070707070707070707070707070707070707070707070707070707"
        );
        assert_eq!(
            encoded["signal_bindings"][0]["aspect_key"],
            "store.physical.record.frame-read-basis"
        );
        assert_eq!(encoded["signal_bindings"][0]["role"], "dependency");
        assert_eq!(
            encoded["signal_bindings"][0]["families"]["read_fault"],
            true
        );
        assert_eq!(
            encoded["signal_bindings"][0]["families"]["exact_writeback"],
            false
        );
        assert_eq!(
            encoded["signal_bindings"][0]["families"]["publication"],
            false
        );
        assert_eq!(
            encoded["signal_bindings"][0]["families"]["lifecycle"],
            false
        );
        assert_eq!(
            encoded["signal_bindings"][0]["partition"],
            "store.physical.record.frame"
        );
        assert_eq!(
            encoded["terminal_fates"]["continued_after_consumer_cancellation"],
            0
        );
        assert_eq!(
            encoded["records"][0]["store"],
            "07070707070707070707070707070707"
        );
        assert_eq!(encoded["records"][0]["runtime"], 11);
        assert_eq!(encoded["records"][0]["generation"], 13);
        assert_eq!(encoded["records"][0]["operation"], 17);
        assert_eq!(encoded["records"][0]["family"], "artifact-range-read");
        assert_eq!(encoded["records"][0]["backend_operation"], 19);
        assert_eq!(encoded["records"][0]["backend_role"], "positioned-read");
        assert_eq!(encoded["records"][0]["effect_fate"], "read-completed");
        assert_eq!(encoded["records"][0]["recovery"], "no-effect");
        assert_eq!(encoded["records"][0]["terminal"], "settled");
    }
}
