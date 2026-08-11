use worth_store::physical_runtime::{BoundedRecoveryFilesystemDiscovery, ObservedWalArtifact};
use worth_store_physical_format::{
    inspect_checkpoint_stream, BootstrapCatalog, DurableRootSelector, RootSelectorRole,
    BOOTSTRAP_CATALOG_BYTES, ROOT_SELECTOR_BYTES,
};
use worth_store_recovery_physics::{
    admit_physical_root_slot, inspect_physical_wal_artifacts, InspectedPhysicalWalArtifacts,
    PhysicalRecoveryResidue, PhysicalRecoveryResidueKind, PhysicalRootSlotObservation,
};

use crate::entry::{
    PhysicalRecoveryBlockKind as PhysicalRecoveryBlock, PhysicalRecoveryLimitDimension,
    PhysicalRecoveryLimits,
};
use crate::progression::PhysicalRecoveryDiscoveryCounters;

use super::super::manifest_facts::{observe_manifest_facts, ManifestObservationBudget};
use super::super::ManifestFactsDiscovery;
use super::{
    discovery_limit, map_cumulative_discovery_failure, map_discovery_failure, BootstrapDiscovery,
    CheckpointDiscovery, DiscoveryFailure, WalDiscovery,
};

mod counters;

use counters::{record_checkpoint_counters, record_root_counters, record_wal_counters};

pub(super) struct ObservedSources {
    pub(super) current: PhysicalRootSlotObservation,
    pub(super) previous: PhysicalRootSlotObservation,
    pub(super) bootstrap: BootstrapDiscovery,
    pub(super) current_manifest_facts: ManifestFactsDiscovery,
    pub(super) previous_manifest_facts: ManifestFactsDiscovery,
    pub(super) checkpoint: CheckpointDiscovery,
    pub(super) wal: WalDiscovery,
    pub(super) residue: Vec<PhysicalRecoveryResidue>,
}

struct RootObservations {
    current: PhysicalRootSlotObservation,
    previous: PhysicalRootSlotObservation,
    remaining_manifest_bytes: u64,
}

pub(super) fn observe_all(
    discovery: &mut BoundedRecoveryFilesystemDiscovery,
    limits: PhysicalRecoveryLimits,
    counters: &mut PhysicalRecoveryDiscoveryCounters,
) -> Result<ObservedSources, DiscoveryFailure> {
    let declaration = limits.declaration();
    let mut roots = observe_root_slots(discovery, limits, counters)?;
    let bootstrap = observe_fallback_anchor(discovery, &roots)?;
    let (current_manifest_facts, previous_manifest_facts) =
        observe_root_manifest_facts(discovery, limits, &mut roots, counters)?;
    let checkpoint = observe_checkpoint(discovery, limits)?;
    record_checkpoint_counters(counters, &checkpoint);
    let (wal, residue, wal_entries) = observe_wal(discovery, limits)?;
    record_wal_counters(counters, &wal, &residue, wal_entries);
    if discovery.counters().bytes_read > declaration.observation_bytes {
        return Err(discovery_limit(
            PhysicalRecoveryLimitDimension::ObservationBytes,
            discovery.counters().bytes_read,
            declaration.observation_bytes,
        ));
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
    })
}

fn observe_fallback_anchor(
    discovery: &mut BoundedRecoveryFilesystemDiscovery,
    roots: &RootObservations,
) -> Result<BootstrapDiscovery, DiscoveryFailure> {
    if !matches!(
        roots.current,
        PhysicalRootSlotObservation::Rejected { selector: None, .. }
    ) || !matches!(roots.previous, PhysicalRootSlotObservation::Admitted(_))
    {
        return Ok(BootstrapDiscovery::NotRequired);
    }
    let artifact = discovery
        .read_bootstrap_catalog(BOOTSTRAP_CATALOG_BYTES as u64)
        .map_err(map_selector_discovery_failure)?;
    let Some(bytes) = artifact.bytes() else {
        return Ok(BootstrapDiscovery::Absent);
    };
    Ok(match BootstrapCatalog::decode(bytes) {
        Ok(catalog) => BootstrapDiscovery::Admitted(catalog),
        Err(denial) => BootstrapDiscovery::Rejected(denial),
    })
}

fn observe_root_slots(
    discovery: &mut BoundedRecoveryFilesystemDiscovery,
    limits: PhysicalRecoveryLimits,
    counters: &mut PhysicalRecoveryDiscoveryCounters,
) -> Result<RootObservations, DiscoveryFailure> {
    let declaration = limits.declaration();
    let current_bytes = read_current_selector(discovery)?;
    let previous_bytes = read_previous_selector(discovery)?;
    counters.selector_slots = discovery.counters().fixed_slots_read;
    let store = discovery.store_identity();
    let mut remaining_manifest_bytes = declaration.manifest_bytes;
    let current_manifest = read_addressed_manifest(
        discovery,
        current_bytes.as_deref(),
        &mut remaining_manifest_bytes,
        declaration.manifest_bytes,
    )?;
    let previous_manifest = read_addressed_manifest(
        discovery,
        previous_bytes.as_deref(),
        &mut remaining_manifest_bytes,
        declaration.manifest_bytes,
    )?;
    let maximum_manifest_entries = u16::try_from(declaration.manifest_entries).unwrap_or(u16::MAX);
    let current = admit_physical_root_slot(
        store,
        RootSelectorRole::Current,
        current_bytes.as_deref(),
        current_manifest.as_deref(),
        maximum_manifest_entries,
    );
    let previous = admit_physical_root_slot(
        store,
        RootSelectorRole::Previous,
        previous_bytes.as_deref(),
        previous_manifest.as_deref(),
        maximum_manifest_entries,
    );
    record_root_counters(counters, &current, &previous);
    Ok(RootObservations {
        current,
        previous,
        remaining_manifest_bytes,
    })
}

fn read_current_selector(
    discovery: &mut BoundedRecoveryFilesystemDiscovery,
) -> Result<Option<Vec<u8>>, DiscoveryFailure> {
    discovery
        .read_current_selector(ROOT_SELECTOR_BYTES as u64)
        .map_err(map_selector_discovery_failure)
        .map(|artifact| artifact.into_bytes())
}

fn read_previous_selector(
    discovery: &mut BoundedRecoveryFilesystemDiscovery,
) -> Result<Option<Vec<u8>>, DiscoveryFailure> {
    discovery
        .read_previous_selector(ROOT_SELECTOR_BYTES as u64)
        .map_err(map_selector_discovery_failure)
        .map(|artifact| artifact.into_bytes())
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
    let previous_manifest_facts = previous_manifest_facts_result?;
    counters.manifest_blocks =
        current_manifest_facts.block_count() + previous_manifest_facts.block_count();
    Ok((current_manifest_facts, previous_manifest_facts))
}

fn read_addressed_manifest(
    discovery: &mut BoundedRecoveryFilesystemDiscovery,
    selector_bytes: Option<&[u8]>,
    remaining_bytes: &mut u64,
    admitted_bytes: u64,
) -> Result<Option<Vec<u8>>, DiscoveryFailure> {
    let Some(selector) = selector_bytes.and_then(|bytes| DurableRootSelector::decode(bytes).ok())
    else {
        return Ok(None);
    };
    let bytes = discovery
        .read_root_manifest(selector.root_generation(), *remaining_bytes)
        .map_err(|failure| {
            map_cumulative_discovery_failure(
                failure,
                PhysicalRecoveryLimitDimension::ManifestEntries,
                PhysicalRecoveryLimitDimension::ManifestBytes,
                admitted_bytes,
                *remaining_bytes,
            )
        })
        .map(|artifact| artifact.into_bytes())?;
    *remaining_bytes = remaining_bytes
        .checked_sub(bytes.as_ref().map_or(0, |bytes| bytes.len() as u64))
        .ok_or(PhysicalRecoveryBlock::DiscoveryLimit)?;
    Ok(bytes)
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
