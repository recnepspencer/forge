use worth_store::physical_runtime::{BoundedRecoveryFilesystemDiscovery, ObservedWalArtifact};
use worth_store_physical_format::{
    inspect_checkpoint_stream, PhysicalRecordFormatDeclaration, BOOTSTRAP_CATALOG_BYTES,
};
use worth_store_recovery_physics::{
    inspect_physical_wal_artifacts, InspectedPhysicalWalArtifacts, PhysicalRecoveryResidue,
    PhysicalRecoveryResidueKind, PhysicalRootSelectorDenial, PhysicalRootSlotObservation,
};

use crate::entry::{
    PhysicalRecoveryBlockKind as PhysicalRecoveryBlock, PhysicalRecoveryLimitDimension,
    PhysicalRecoveryLimits, PhysicalRecoverySourceDenial,
};
use crate::integrity_ingress::{
    admit_observed_bootstrap_catalog, IntegrityAdmittedRecoveryArtifact,
    RecoveryIntegrityIngressCounters, RecoveryIntegrityIngressRejection,
};
use crate::progression::PhysicalRecoveryDiscoveryCounters;

use super::super::manifest_facts::{observe_manifest_facts, ManifestObservationBudget};
use super::super::ManifestFactsDiscovery;
use super::{
    discovery_limit, map_discovery_failure, BootstrapDiscovery, CheckpointDiscovery,
    DiscoveryFailure, WalDiscovery,
};

mod counters;
mod root_observation;

use counters::{record_checkpoint_counters, record_wal_counters};
use root_observation::{observe_root_slots, RootObservations};

pub(super) struct ObservedSources {
    pub(super) current: PhysicalRootSlotObservation,
    pub(super) previous: PhysicalRootSlotObservation,
    pub(super) bootstrap: BootstrapDiscovery,
    pub(super) current_manifest_facts: ManifestFactsDiscovery,
    pub(super) previous_manifest_facts: ManifestFactsDiscovery,
    pub(super) checkpoint: CheckpointDiscovery,
    pub(super) wal: WalDiscovery,
    pub(super) residue: Vec<PhysicalRecoveryResidue>,
    pub(super) root_protocol_denials: Vec<PhysicalRecoverySourceDenial>,
}

pub(super) fn observe_all(
    discovery: &mut BoundedRecoveryFilesystemDiscovery,
    limits: PhysicalRecoveryLimits,
    record_format: PhysicalRecordFormatDeclaration,
    counters: &mut PhysicalRecoveryDiscoveryCounters,
) -> Result<ObservedSources, DiscoveryFailure> {
    let declaration = limits.declaration();
    let mut roots = observe_root_slots(discovery, limits, record_format, counters)?;
    let root_protocol_denials = roots.denials.clone();
    let preserve_root_denials =
        |failure: DiscoveryFailure| failure.with_root_protocol_denials(&root_protocol_denials);
    let bootstrap = observe_fallback_anchor(discovery, &roots, record_format, counters)
        .map_err(&preserve_root_denials)?;
    let (current_manifest_facts, previous_manifest_facts) =
        observe_root_manifest_facts(discovery, limits, &mut roots, counters)
            .map_err(&preserve_root_denials)?;
    let preserve_manifest_observations = |failure| {
        preserve_post_manifest_failure(
            failure,
            &root_protocol_denials,
            &current_manifest_facts,
            &previous_manifest_facts,
        )
    };
    let checkpoint =
        observe_checkpoint(discovery, limits).map_err(&preserve_manifest_observations)?;
    record_checkpoint_counters(counters, &checkpoint);
    let (wal, residue, wal_entries) =
        observe_wal(discovery, limits).map_err(&preserve_manifest_observations)?;
    record_wal_counters(counters, &wal, &residue, wal_entries);
    if discovery.counters().bytes_read > declaration.observation_bytes {
        return Err(preserve_manifest_observations(discovery_limit(
            PhysicalRecoveryLimitDimension::ObservationBytes,
            discovery.counters().bytes_read,
            declaration.observation_bytes,
        )));
    }
    Ok(ObservedSources {
        current: roots.current,
        previous: roots.previous,
        bootstrap,
        current_manifest_facts,
        previous_manifest_facts,
        checkpoint,
        wal,
        residue,
        root_protocol_denials,
    })
}

fn preserve_post_manifest_failure(
    failure: DiscoveryFailure,
    root_protocol_denials: &[PhysicalRecoverySourceDenial],
    current: &ManifestFactsDiscovery,
    previous: &ManifestFactsDiscovery,
) -> DiscoveryFailure {
    failure
        .with_root_protocol_denials(root_protocol_denials)
        .with_integrity_trace(current.integrity_trace().clone())
        .with_integrity_trace(previous.integrity_trace().clone())
}

fn observe_fallback_anchor(
    discovery: &mut BoundedRecoveryFilesystemDiscovery,
    roots: &RootObservations,
    record_format: PhysicalRecordFormatDeclaration,
    counters: &mut PhysicalRecoveryDiscoveryCounters,
) -> Result<BootstrapDiscovery, DiscoveryFailure> {
    if !current_requires_fallback_anchor(&roots.current)
        || !matches!(roots.previous, PhysicalRootSlotObservation::Candidate(_))
    {
        return Ok(BootstrapDiscovery::NotRequired);
    }
    let artifact = discovery
        .read_bootstrap_catalog(BOOTSTRAP_CATALOG_BYTES as u64)
        .map_err(map_selector_discovery_failure)?;
    let mut ingress_counters = RecoveryIntegrityIngressCounters::default();
    let attempt = admit_observed_bootstrap_catalog(
        &artifact,
        discovery.store_identity(),
        record_format,
        &mut ingress_counters,
    );
    let discovery = match attempt.into_outcome() {
        Ok(IntegrityAdmittedRecoveryArtifact::BootstrapCatalog(admitted)) => {
            let projection = admitted.project(&mut ingress_counters);
            BootstrapDiscovery::Admitted(
                worth_store_recovery_physics::PhysicalBootstrapFallbackAnchor::from_integrity_projection(
                    admitted.scope().store_identity(),
                    projection.record_format,
                    projection.current_root_generation,
                ),
            )
        }
        Ok(_) => unreachable!("bootstrap ingress routes only the bootstrap family"),
        Err(RecoveryIntegrityIngressRejection::Absent) => BootstrapDiscovery::Absent,
        Err(rejection) => BootstrapDiscovery::Rejected(rejection),
    };
    record_bootstrap_counters(counters, ingress_counters);
    Ok(discovery)
}

fn record_bootstrap_counters(
    counters: &mut PhysicalRecoveryDiscoveryCounters,
    ingress: RecoveryIntegrityIngressCounters,
) {
    counters.bootstrap_integrity_attempts = ingress.attempted;
    counters.bootstrap_integrity_admissions = ingress.admitted;
    counters.bootstrap_absent = ingress.rejected_absent;
    counters.bootstrap_integrity_rejections =
        ingress.attempted - ingress.admitted - ingress.rejected_absent;
    counters.bootstrap_owner_projections = ingress.owner_projection_entries;
    counters.bootstrap_owner_decoder_entries = ingress.owner_decoder_entries;
}

fn current_requires_fallback_anchor(current: &PhysicalRootSlotObservation) -> bool {
    matches!(
        current,
        PhysicalRootSlotObservation::SelectorRejected(
            PhysicalRootSelectorDenial::Integrity | PhysicalRootSelectorDenial::AuthorityMismatch
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selector_conflict_never_requests_fallback_anchor_io() {
        assert!(!current_requires_fallback_anchor(
            &PhysicalRootSlotObservation::SelectorRejected(PhysicalRootSelectorDenial::Conflict)
        ));
        assert!(current_requires_fallback_anchor(
            &PhysicalRootSlotObservation::SelectorRejected(
                PhysicalRootSelectorDenial::AuthorityMismatch
            )
        ));
    }
}

fn map_selector_discovery_failure(
    failure: worth_store::physical_runtime::RecoveryDiscoveryFailure,
) -> DiscoveryFailure {
    map_discovery_failure(
        failure,
        PhysicalRecoveryLimitDimension::ObservationBytes,
        PhysicalRecoveryLimitDimension::ObservationBytes,
    )
}

fn observe_root_manifest_facts(
    discovery: &mut BoundedRecoveryFilesystemDiscovery,
    limits: PhysicalRecoveryLimits,
    roots: &mut RootObservations,
    counters: &mut PhysicalRecoveryDiscoveryCounters,
) -> Result<(ManifestFactsDiscovery, ManifestFactsDiscovery), DiscoveryFailure> {
    let declaration = limits.declaration();
    let mut remaining_manifest_entries = declaration.manifest_entries;
    let admitted_manifest_blocks = declaration
        .manifest_entries
        .checked_mul(2)
        .and_then(|value| value.checked_add(2))
        .ok_or(PhysicalRecoveryBlock::DiscoveryLimit)?;
    let mut remaining_manifest_blocks = admitted_manifest_blocks;
    let current_manifest_facts_result = observe_manifest_facts(
        discovery,
        &roots.current,
        ManifestObservationBudget {
            remaining_bytes: &mut roots.remaining_manifest_bytes,
            admitted_bytes: declaration.manifest_bytes,
            remaining_entries: &mut remaining_manifest_entries,
            admitted_entries: declaration.manifest_entries,
            remaining_blocks: &mut remaining_manifest_blocks,
            admitted_blocks: admitted_manifest_blocks,
        },
    );
    counters.manifest_bytes = declaration.manifest_bytes - roots.remaining_manifest_bytes;
    counters.manifest_entries = declaration.manifest_entries - remaining_manifest_entries;
    let current_manifest_facts = current_manifest_facts_result?;
    let previous_manifest_facts_result = observe_manifest_facts(
        discovery,
        &roots.previous,
        ManifestObservationBudget {
            remaining_bytes: &mut roots.remaining_manifest_bytes,
            admitted_bytes: declaration.manifest_bytes,
            remaining_entries: &mut remaining_manifest_entries,
            admitted_entries: declaration.manifest_entries,
            remaining_blocks: &mut remaining_manifest_blocks,
            admitted_blocks: admitted_manifest_blocks,
        },
    );
    counters.manifest_bytes = declaration.manifest_bytes - roots.remaining_manifest_bytes;
    counters.manifest_entries = declaration.manifest_entries - remaining_manifest_entries;
    let previous_manifest_facts = match previous_manifest_facts_result {
        Ok(facts) => facts,
        Err(failure) => {
            let (_, trace) = current_manifest_facts.into_parts();
            return Err(failure.with_integrity_trace(trace));
        }
    };
    counters.manifest_blocks =
        current_manifest_facts.block_count() + previous_manifest_facts.block_count();
    Ok((current_manifest_facts, previous_manifest_facts))
}

fn observe_checkpoint(
    discovery: &mut BoundedRecoveryFilesystemDiscovery,
    limits: PhysicalRecoveryLimits,
) -> Result<CheckpointDiscovery, DiscoveryFailure> {
    let declaration = limits.declaration();
    let artifact = discovery
        .read_current_checkpoint(declaration.observation_bytes)
        .map_err(|failure| {
            map_discovery_failure(
                failure,
                PhysicalRecoveryLimitDimension::ObservationBytes,
                PhysicalRecoveryLimitDimension::ObservationBytes,
            )
        })?;
    let Some(bytes) = artifact.bytes() else {
        return Ok(CheckpointDiscovery::Absent);
    };
    Ok(
        match inspect_checkpoint_stream(
            bytes,
            declaration.dirty_frames,
            declaration.operation_bindings,
        ) {
            Ok(checkpoint) => CheckpointDiscovery::Admitted(checkpoint),
            Err(denial) => CheckpointDiscovery::Rejected(denial),
        },
    )
}

fn observe_wal(
    discovery: &mut BoundedRecoveryFilesystemDiscovery,
    limits: PhysicalRecoveryLimits,
) -> Result<(WalDiscovery, Vec<PhysicalRecoveryResidue>, u64), DiscoveryFailure> {
    let declaration = limits.declaration();
    let observed = discovery
        .read_wal_artifacts(declaration.wal_segments, declaration.wal_bytes)
        .map_err(|failure| {
            map_discovery_failure(
                failure,
                PhysicalRecoveryLimitDimension::WalSegments,
                PhysicalRecoveryLimitDimension::WalBytes,
            )
        })?;
    let wal_entries = observed.len() as u64;
    let (regular, mut residue) = partition_wal_entries(observed);
    let inspected =
        inspect_physical_wal_artifacts(regular, declaration.wal_frames).map_err(|denial| {
            match denial {
            worth_store_recovery_physics::PhysicalWalArtifactInspectionDenial::CounterOverflow => {
                DiscoveryFailure::from(PhysicalRecoveryBlock::DiscoveryLimit)
            }
            worth_store_recovery_physics::PhysicalWalArtifactInspectionDenial::FrameLimitExceeded {
                observed,
                admitted,
            } => discovery_limit(
                PhysicalRecoveryLimitDimension::WalFrames,
                observed,
                admitted,
            ),
        }
        })?;
    validate_wal_limits(&inspected, limits)?;
    residue.extend_from_slice(inspected.residue());
    Ok((wal_discovery(inspected), residue, wal_entries))
}

fn partition_wal_entries(
    observed: Vec<ObservedWalArtifact>,
) -> (Vec<(String, Vec<u8>)>, Vec<PhysicalRecoveryResidue>) {
    let mut regular = Vec::new();
    let mut residue = Vec::new();
    for artifact in observed {
        let name = artifact.name().to_string_lossy().into_owned();
        if artifact.entry_type()
            != worth_store_physical_format::store_namespace::NamespaceEntryType::RegularFile
        {
            residue.push(PhysicalRecoveryResidue::new(
                name,
                PhysicalRecoveryResidueKind::NonRegularWalEntry,
            ));
            continue;
        }
        regular.push((name, artifact.bytes().unwrap_or_default().to_vec()));
    }
    (regular, residue)
}

fn validate_wal_limits(
    inspected: &InspectedPhysicalWalArtifacts,
    limits: PhysicalRecoveryLimits,
) -> Result<(), DiscoveryFailure> {
    let declaration = limits.declaration();
    let wal_frames = inspected.frames_scanned();
    let wal_bytes = inspected.byte_count();
    if wal_bytes > declaration.wal_bytes {
        return Err(discovery_limit(
            PhysicalRecoveryLimitDimension::WalBytes,
            wal_bytes,
            declaration.wal_bytes,
        ));
    }
    debug_assert!(wal_frames <= declaration.wal_frames);
    Ok(())
}

fn wal_discovery(inspected: InspectedPhysicalWalArtifacts) -> WalDiscovery {
    WalDiscovery {
        candidates: inspected.candidates().to_vec(),
        rejected: inspected.rejected(),
        scanned_frames: inspected.frames_scanned(),
        valid_frames: inspected.frame_count(),
        valid_bytes: inspected.valid_byte_count(),
        observed_bytes: inspected.byte_count(),
        torn_suffix_frames: inspected.torn_suffix_frames(),
        torn_suffix_bytes: inspected.torn_suffix_bytes(),
        corruption_denials: inspected.corruption_denials(),
        scanned_segments: inspected.canonical_segment_count(),
        valid_segments: inspected.valid_segment_count(),
        corruptions: inspected.corruptions().to_vec(),
    }
}
