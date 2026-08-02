use crate::courtroom_campaign::bounded_residency_siege::protocol::{
    exact_route_fixture, BoundedResidencyMediaRole, BoundedResidencySignalAspectRole,
    BoundedResidencySignalBindingObservation, BoundedResidencySignalFamily,
    BoundedResidencySignalFamilySet, BoundedResidencyWorkEffectFate, BoundedResidencyWorkFamily,
    BoundedResidencyWorkReconciliationObservation, BoundedResidencyWorkRecordObservation,
    BoundedResidencyWorkRecovery, BoundedResidencyWorkTerminalFate,
};

pub(super) const STORE: [u8; 16] = [9; 16];
pub(super) const RUNTIME: u64 = 11;
pub(super) const GENERATION: u64 = 13;
pub(super) const ROOT: [u8; 32] = [1; 32];
const ARTIFACT: [u8; 32] = [2; 32];
pub(super) const FRAME: [u8; 32] = [3; 32];
const SCAN: [u8; 32] = [4; 32];
const WRITEBACK: [u8; 32] = [5; 32];
const PUBLICATION: [u8; 32] = [6; 32];

pub(super) fn fixture() -> BoundedResidencyWorkReconciliationObservation {
    BoundedResidencyWorkReconciliationObservation {
        causal_overflow: 0,
        terminal_overflow: 0,
        safe_evidence_elided: 0,
        faults: 1,
        source_loads: 1,
        exact_writebacks: 1,
        identified_metadata_reads: 3,
        identified_positioned_reads: 1,
        identified_positioned_writes: 1,
        settled_terminal_fates: 6,
        continued_terminal_fates: 0,
        signal_bindings: signal_bindings(),
        records: vec![
            record(
                1,
                101,
                BoundedResidencyWorkFamily::ArtifactMetadataRead,
                BoundedResidencyWorkEffectFate::ReadCompleted,
                BoundedResidencyWorkRecovery::NoEffect,
                ROOT,
            ),
            record(
                2,
                102,
                BoundedResidencyWorkFamily::ArtifactMetadataRead,
                BoundedResidencyWorkEffectFate::ReadCompleted,
                BoundedResidencyWorkRecovery::NoEffect,
                ARTIFACT,
            ),
            record(
                3,
                103,
                BoundedResidencyWorkFamily::ArtifactRangeRead,
                BoundedResidencyWorkEffectFate::ReadCompleted,
                BoundedResidencyWorkRecovery::NoEffect,
                FRAME,
            ),
            record(
                4,
                104,
                BoundedResidencyWorkFamily::ArtifactMetadataRead,
                BoundedResidencyWorkEffectFate::ReadCompleted,
                BoundedResidencyWorkRecovery::NoEffect,
                SCAN,
            ),
            record(
                5,
                105,
                BoundedResidencyWorkFamily::ArtifactRangeWrite,
                BoundedResidencyWorkEffectFate::WriteCompleted,
                BoundedResidencyWorkRecovery::ContinueSettlement,
                WRITEBACK,
            ),
            record(
                6,
                106,
                BoundedResidencyWorkFamily::ArtifactPublication,
                BoundedResidencyWorkEffectFate::PublicationCompleted,
                BoundedResidencyWorkRecovery::ContinueSettlement,
                PUBLICATION,
            ),
        ]
        .into_boxed_slice(),
    }
}

fn record(
    operation: u64,
    backend_operation: u64,
    family: BoundedResidencyWorkFamily,
    effect_fate: BoundedResidencyWorkEffectFate,
    recovery: BoundedResidencyWorkRecovery,
    signal_binding: [u8; 32],
) -> BoundedResidencyWorkRecordObservation {
    BoundedResidencyWorkRecordObservation {
        store: STORE,
        runtime: RUNTIME,
        generation: GENERATION,
        operation,
        family,
        backend_operation,
        backend_role: match family {
            BoundedResidencyWorkFamily::ArtifactMetadataRead => {
                BoundedResidencyMediaRole::ReadMetadata
            }
            BoundedResidencyWorkFamily::ArtifactRangeRead => {
                BoundedResidencyMediaRole::PositionedRead
            }
            BoundedResidencyWorkFamily::ArtifactRangeWrite => {
                BoundedResidencyMediaRole::PositionedWrite
            }
            BoundedResidencyWorkFamily::ArtifactPublication => {
                BoundedResidencyMediaRole::SynchronizeFileState
            }
        },
        effect_fate,
        recovery,
        route: exact_route_fixture(operation, family, signal_binding),
        terminal: BoundedResidencyWorkTerminalFate::Settled,
    }
}

fn signal_bindings() -> Box<[BoundedResidencySignalBindingObservation]> {
    vec![
        read_binding(
            ROOT,
            "store.physical.record.root-read-basis",
            "store.physical.record.root",
        ),
        read_binding(
            ARTIFACT,
            "store.physical.record.artifact-read-basis",
            "store.physical.record.artifact",
        ),
        read_binding(
            FRAME,
            "store.physical.record.frame-read-basis",
            "store.physical.record.frame",
        ),
        read_binding(
            SCAN,
            "store.physical.record.scan-read-basis",
            "store.physical.record.scan",
        ),
        mutation_binding(
            WRITEBACK,
            "store.physical.record.frame-writeback-basis",
            BoundedResidencySignalFamily::ExactWriteback,
        ),
        mutation_binding(
            PUBLICATION,
            "store.physical.record.publication-basis",
            BoundedResidencySignalFamily::Publication,
        ),
    ]
    .into_boxed_slice()
}

fn read_binding(
    digest: [u8; 32],
    aspect_key: &str,
    partition: &str,
) -> BoundedResidencySignalBindingObservation {
    BoundedResidencySignalBindingObservation {
        digest,
        aspect_key: aspect_key.to_owned(),
        role: BoundedResidencySignalAspectRole::Dependency,
        families: family_set(BoundedResidencySignalFamily::ReadFault),
        partition: Some(partition.to_owned()),
    }
}

fn mutation_binding(
    digest: [u8; 32],
    aspect_key: &str,
    family: BoundedResidencySignalFamily,
) -> BoundedResidencySignalBindingObservation {
    BoundedResidencySignalBindingObservation {
        digest,
        aspect_key: aspect_key.to_owned(),
        role: BoundedResidencySignalAspectRole::DependencyAndOutput,
        families: family_set(family),
        partition: None,
    }
}

fn family_set(family: BoundedResidencySignalFamily) -> BoundedResidencySignalFamilySet {
    BoundedResidencySignalFamilySet {
        read_fault: family == BoundedResidencySignalFamily::ReadFault,
        exact_writeback: family == BoundedResidencySignalFamily::ExactWriteback,
        publication: family == BoundedResidencySignalFamily::Publication,
        lifecycle: family == BoundedResidencySignalFamily::Lifecycle,
        wal_append: false,
        durability_barrier: false,
        checkpoint_capture: false,
        root_publication: false,
        wal_reclamation: false,
    }
}
