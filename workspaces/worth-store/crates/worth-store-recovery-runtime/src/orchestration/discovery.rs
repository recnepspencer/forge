use worth_store::physical_runtime::{RecoveryDiscoveryByteLimitScope, RecoveryDiscoveryFailure};
use worth_store_physical_format::{CheckpointStreamDecodeDenial, VerifiedCheckpointStream};
use worth_store_recovery_physics::PhysicalBootstrapFallbackAnchor;
use worth_store_recovery_physics::{
    PhysicalRecoveryResidue, PhysicalRootSlotObservation, PhysicalWalArtifactCorruption,
    PhysicalWalSegmentCandidate,
};

use crate::entry::{
    AdmittedPlatformAuthority, PhysicalRecoveryBlockEvidence,
    PhysicalRecoveryBlockKind as PhysicalRecoveryBlock, PhysicalRecoveryLimitDimension,
    PhysicalRecoveryLimitFailure, PhysicalRecoveryMediaObservationFailure,
    PhysicalRecoverySourceDenial,
};

use super::{ManifestFactsDiscovery, RecoveryCoordination};

mod observation;

use observation::observe_all;

pub(crate) struct DiscoveryMaterial {
    pub(crate) authority: AdmittedPlatformAuthority,
    pub(crate) coordination: RecoveryCoordination,
    pub(crate) current: PhysicalRootSlotObservation,
    pub(crate) previous: PhysicalRootSlotObservation,
    pub(crate) bootstrap: BootstrapDiscovery,
    pub(crate) current_manifest_facts: ManifestFactsDiscovery,
    pub(crate) previous_manifest_facts: ManifestFactsDiscovery,
    pub(crate) checkpoint: CheckpointDiscovery,
    pub(crate) wal: WalDiscovery,
    pub(crate) residue: Vec<PhysicalRecoveryResidue>,
    pub(crate) root_protocol_denials: Vec<PhysicalRecoverySourceDenial>,
    pub(crate) counters: crate::progression::PhysicalRecoveryDiscoveryCounters,
}

pub(crate) enum CheckpointDiscovery {
    Absent,
    Rejected(CheckpointStreamDecodeDenial),
    Admitted(VerifiedCheckpointStream),
}

pub(crate) enum BootstrapDiscovery {
    NotRequired,
    Absent,
    Rejected(crate::integrity_ingress::RecoveryIntegrityIngressRejection),
    Admitted(PhysicalBootstrapFallbackAnchor),
}

pub(crate) struct WalDiscovery {
    pub(crate) candidates: Vec<PhysicalWalSegmentCandidate>,
    pub(crate) rejected: bool,
    scanned_frames: u64,
    valid_frames: u64,
    valid_bytes: u64,
    observed_bytes: u64,
    torn_suffix_frames: u64,
    torn_suffix_bytes: u64,
    corruption_denials: u64,
    scanned_segments: u64,
    valid_segments: u64,
    pub(crate) corruptions: Vec<PhysicalWalArtifactCorruption>,
}

impl WalDiscovery {
    pub(crate) fn into_selection_parts(
        self,
    ) -> (
        Vec<PhysicalWalSegmentCandidate>,
        bool,
        Vec<PhysicalWalArtifactCorruption>,
    ) {
        (self.candidates, self.rejected, self.corruptions)
    }
}

pub(super) struct DiscoveryFailure {
    kind: PhysicalRecoveryBlock,
    limit: Option<PhysicalRecoveryLimitFailure>,
    source_denials: Vec<PhysicalRecoverySourceDenial>,
    integrity_trace: crate::integrity_ingress::RecoveryIntegrityIngressTrace,
}

impl DiscoveryFailure {
    pub(super) fn with_root_protocol_denials(
        mut self,
        denials: &[PhysicalRecoverySourceDenial],
    ) -> Self {
        let mut combined = denials.to_vec();
        combined.append(&mut self.source_denials);
        self.source_denials = combined;
        self
    }

    pub(super) fn with_integrity_trace(
        mut self,
        trace: crate::integrity_ingress::RecoveryIntegrityIngressTrace,
    ) -> Self {
        self.integrity_trace.append(trace);
        self
    }
}

impl From<PhysicalRecoveryBlock> for DiscoveryFailure {
    fn from(kind: PhysicalRecoveryBlock) -> Self {
        Self {
            kind,
            limit: None,
            source_denials: Vec::new(),
            integrity_trace: crate::integrity_ingress::RecoveryIntegrityIngressTrace::new(),
        }
    }
}

pub(crate) fn discover_sources(
    authority: AdmittedPlatformAuthority,
    coordination: RecoveryCoordination,
) -> Result<
    DiscoveryMaterial,
    (
        AdmittedPlatformAuthority,
        RecoveryCoordination,
        PhysicalRecoveryBlock,
        PhysicalRecoveryBlockEvidence,
    ),
> {
    let limits = authority.limits;
    let declaration = limits.declaration();
    if declaration.selector_candidates < 2 {
        return Err((
            authority,
            coordination,
            PhysicalRecoveryBlock::DiscoveryLimit,
            limit_evidence(
                PhysicalRecoveryLimitDimension::SelectorCandidates,
                2,
                declaration.selector_candidates,
            ),
        ));
    }
    let maximum_manifest_blocks = declaration
        .manifest_entries
        .checked_mul(2)
        .and_then(|value| value.checked_add(2));
    let maximum_entries = match maximum_manifest_blocks.and_then(|blocks| {
        5_u64
            .checked_add(declaration.wal_segments)
            .and_then(|entries| entries.checked_add(blocks))
    }) {
        Some(limit) => limit,
        None => {
            return Err((
                authority,
                coordination,
                PhysicalRecoveryBlock::DiscoveryLimit,
                PhysicalRecoveryBlockEvidence::default(),
            ));
        }
    };
    let AdmittedPlatformAuthority {
        media,
        session,
        _world_binding,
        limits,
        record_format,
    } = authority;
    let mut discovery = media
        .bounded_discovery(maximum_entries, declaration.observation_bytes)
        .expect("a nonzero admitted discovery limit constructs a bounded observer");
    let mut counters = crate::progression::PhysicalRecoveryDiscoveryCounters::default();
    let result = observe_all(&mut discovery, limits, record_format, &mut counters);
    counters.bytes_observed = discovery.counters().bytes_read;
    counters.wal_entries = discovery.counters().directory_entries_observed;
    counters.wal_bytes = discovery.counters().wal_bytes_read;
    let media = discovery.finish();
    let authority = AdmittedPlatformAuthority {
        media,
        session,
        _world_binding,
        limits,
        record_format,
    };
    match result {
        Ok(observed) => Ok(DiscoveryMaterial {
            authority,
            coordination,
            current: observed.current,
            previous: observed.previous,
            bootstrap: observed.bootstrap,
            current_manifest_facts: observed.current_manifest_facts,
            previous_manifest_facts: observed.previous_manifest_facts,
            checkpoint: observed.checkpoint,
            wal: observed.wal,
            residue: observed.residue,
            root_protocol_denials: observed.root_protocol_denials,
            counters,
        }),
        Err(failure) => {
            let DiscoveryFailure {
                kind,
                limit,
                source_denials,
                integrity_trace,
            } = failure;
            Err((
                authority,
                coordination,
                kind,
                PhysicalRecoveryBlockEvidence {
                    counters,
                    limit,
                    artifact: Some(discovery_artifact_context(kind).to_owned()),
                    source_denials,
                    integrity_trace,
                    ..PhysicalRecoveryBlockEvidence::default()
                },
            ))
        }
    }
}

pub(super) fn map_discovery_failure(
    failure: RecoveryDiscoveryFailure,
    entry_dimension: PhysicalRecoveryLimitDimension,
    byte_dimension: PhysicalRecoveryLimitDimension,
) -> DiscoveryFailure {
    match failure {
        RecoveryDiscoveryFailure::EntryLimitExceeded { observed, admitted } => DiscoveryFailure {
            kind: PhysicalRecoveryBlock::DiscoveryLimit,
            limit: Some(PhysicalRecoveryLimitFailure {
                dimension: entry_dimension,
                observed,
                admitted,
            }),
            source_denials: Vec::new(),
            integrity_trace: crate::integrity_ingress::RecoveryIntegrityIngressTrace::new(),
        },
        RecoveryDiscoveryFailure::ByteLimitExceeded {
            observed,
            admitted,
            scope,
        } => DiscoveryFailure {
            kind: PhysicalRecoveryBlock::DiscoveryLimit,
            limit: Some(PhysicalRecoveryLimitFailure {
                dimension: match scope {
                    RecoveryDiscoveryByteLimitScope::Observation => {
                        PhysicalRecoveryLimitDimension::ObservationBytes
                    }
                    RecoveryDiscoveryByteLimitScope::Requested => byte_dimension,
                },
                observed,
                admitted,
            }),
            source_denials: Vec::new(),
            integrity_trace: crate::integrity_ingress::RecoveryIntegrityIngressTrace::new(),
        },
        RecoveryDiscoveryFailure::Media { artifact, failure } => DiscoveryFailure {
            kind: PhysicalRecoveryBlock::MediaObservation,
            limit: None,
            source_denials: vec![PhysicalRecoverySourceDenial::MediaObservation {
                artifact,
                failure: PhysicalRecoveryMediaObservationFailure::Backend {
                    kind: failure.kind(),
                    io_kind: failure.io_kind(),
                },
            }],
            integrity_trace: crate::integrity_ingress::RecoveryIntegrityIngressTrace::new(),
        },
        RecoveryDiscoveryFailure::InvalidAddress { artifact } => DiscoveryFailure {
            kind: PhysicalRecoveryBlock::MediaObservation,
            limit: None,
            source_denials: vec![PhysicalRecoverySourceDenial::MediaObservation {
                artifact,
                failure: PhysicalRecoveryMediaObservationFailure::InvalidAddress,
            }],
            integrity_trace: crate::integrity_ingress::RecoveryIntegrityIngressTrace::new(),
        },
    }
}

pub(super) fn map_cumulative_discovery_failure(
    failure: RecoveryDiscoveryFailure,
    entry_dimension: PhysicalRecoveryLimitDimension,
    byte_dimension: PhysicalRecoveryLimitDimension,
    admitted_bytes: u64,
    remaining_bytes: u64,
) -> DiscoveryFailure {
    let mut mapped = map_discovery_failure(failure, entry_dimension, byte_dimension);
    let Some(limit) = mapped.limit.as_mut() else {
        return mapped;
    };
    if limit.dimension == byte_dimension {
        limit.observed = admitted_bytes
            .saturating_sub(remaining_bytes)
            .saturating_add(limit.observed);
        limit.admitted = admitted_bytes;
    }
    mapped
}

fn limit_evidence(
    dimension: PhysicalRecoveryLimitDimension,
    observed: u64,
    admitted: u64,
) -> PhysicalRecoveryBlockEvidence {
    PhysicalRecoveryBlockEvidence {
        limit: Some(PhysicalRecoveryLimitFailure {
            dimension,
            observed,
            admitted,
        }),
        ..PhysicalRecoveryBlockEvidence::default()
    }
}

pub(super) fn discovery_limit(
    dimension: PhysicalRecoveryLimitDimension,
    observed: u64,
    admitted: u64,
) -> DiscoveryFailure {
    DiscoveryFailure {
        kind: PhysicalRecoveryBlock::DiscoveryLimit,
        limit: Some(PhysicalRecoveryLimitFailure {
            dimension,
            observed,
            admitted,
        }),
        source_denials: Vec::new(),
        integrity_trace: crate::integrity_ingress::RecoveryIntegrityIngressTrace::new(),
    }
}

fn discovery_artifact_context(kind: PhysicalRecoveryBlock) -> &'static str {
    match kind {
        PhysicalRecoveryBlock::DiscoveryLimit => "bounded recovery-media observation",
        PhysicalRecoveryBlock::MediaObservation => "recovery-media artifact",
        PhysicalRecoveryBlock::RootProtocol => "records/root selectors",
        PhysicalRecoveryBlock::Checkpoint => "families/checkpoint.current",
        PhysicalRecoveryBlock::WalInventory => "families/wal",
        PhysicalRecoveryBlock::SourceSelection => "persisted-source cut",
        PhysicalRecoveryBlock::BindingFreshness => "selected checkpoint binding freshness",
        PhysicalRecoveryBlock::PageAdmission => "manifest-addressed page or extent",
        PhysicalRecoveryBlock::OperationReconciliation => "operation-fate evidence",
        PhysicalRecoveryBlock::RedoPlanning => "canonical redo plan",
        PhysicalRecoveryBlock::Staging => "closed recovery staging generation",
        PhysicalRecoveryBlock::Publication => "recovered-root publication",
    }
}
