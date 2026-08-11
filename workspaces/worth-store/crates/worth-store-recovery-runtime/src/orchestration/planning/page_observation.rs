use std::collections::{BTreeMap, BTreeSet};

use worth_store::physical_runtime::AdmittedRecoveryFilesystemMedia;
use worth_store_physical_format::{
    CurrentPhysicalRecordPlacement, DurablePhysicalRootManifest, PhysicalRecordFormatDeclaration,
    RecordArtifactFile,
};
use worth_store_recovery_physics::{
    PhysicalRedoTarget, PhysicalRedoTargetIdentity, RecoveryPageObservation,
};

mod allocation_truth;
mod failure;
mod materialized;
mod publication_inventory;
mod selected_basis;

pub(in crate::orchestration::planning) use allocation_truth::InlineAllocationTruth;
pub(super) use failure::PageObservationFailure;
use materialized::{observe_extent, observe_inline};

pub(super) struct PageObservationAttempt {
    pub(super) result: Result<ObservedPageBasis, PageObservationFailure>,
    pub(super) artifact_reads: u64,
    pub(super) bytes_read: u64,
}

pub(super) struct ObservedPageBasis {
    pub(super) observations: Vec<RecoveryPageObservation>,
    pub(super) inline_truth: Option<allocation_truth::InlineAllocationTruth>,
    pub(super) publication_inventory: crate::progression::RecoveryPublicationSourceInventory,
}

pub(super) use selected_basis::{artifact_read_ceiling, ArtifactReadCeilingDenial};

pub(super) fn observe_selected_pages(
    media: AdmittedRecoveryFilesystemMedia,
    root_manifest: &DurablePhysicalRootManifest,
    placements: &[CurrentPhysicalRecordPlacement],
    targets: &[PhysicalRedoTarget],
    format: PhysicalRecordFormatDeclaration,
    maximum_entries: u64,
    maximum_manifest_entries: u64,
    maximum_bytes: u64,
) -> (AdmittedRecoveryFilesystemMedia, PageObservationAttempt) {
    let mut discovery = media
        .bounded_discovery(maximum_entries, maximum_bytes)
        .expect("admitted nonzero recovery limits create a bounded planning reader");
    let result = observe(
        &mut discovery,
        root_manifest,
        placements,
        targets,
        format,
        maximum_manifest_entries,
        maximum_bytes,
    );
    let counters = discovery.counters();
    (
        discovery.finish(),
        PageObservationAttempt {
            result,
            artifact_reads: counters.addressed_artifacts_read,
            bytes_read: counters.bytes_read,
        },
    )
}

fn observe(
    discovery: &mut worth_store::physical_runtime::BoundedRecoveryFilesystemDiscovery,
    root_manifest: &DurablePhysicalRootManifest,
    placements: &[CurrentPhysicalRecordPlacement],
    targets: &[PhysicalRedoTarget],
    format: PhysicalRecordFormatDeclaration,
    maximum_manifest_entries: u64,
    byte_limit: u64,
) -> Result<ObservedPageBasis, PageObservationFailure> {
    let mut inline_targets = BTreeMap::new();
    let mut extent_targets = BTreeMap::<u64, BTreeMap<u32, &PhysicalRedoTarget>>::new();
    let selected_inline_locations = placements
        .iter()
        .filter_map(|placement| match placement {
            CurrentPhysicalRecordPlacement::Inline(inline) => {
                Some((inline.segment().get(), inline.page().get()))
            }
            CurrentPhysicalRecordPlacement::Extent(_) => None,
        })
        .collect::<BTreeSet<_>>();
    for target in targets {
        match target.identity() {
            PhysicalRedoTargetIdentity::InlinePage { segment, page, .. } => {
                inline_targets.entry((segment, page)).or_insert(target);
            }
            PhysicalRedoTargetIdentity::ExtentChunk { extent, chunk, .. } => {
                extent_targets
                    .entry(extent)
                    .or_default()
                    .entry(chunk)
                    .or_insert(target);
            }
        }
    }
    let existing_inline_targets = inline_targets.values().copied().filter(|target| {
        matches!(
            target.identity(),
            PhysicalRedoTargetIdentity::InlinePage { segment, page, .. }
                if selected_inline_locations.contains(&(segment, page))
        )
    });
    let mut manifest_entries =
        super::segment_observation::ManifestEntryBudget::new(maximum_manifest_entries);
    let segment_entries = super::segment_observation::resolve_inline_targets(
        discovery,
        root_manifest,
        existing_inline_targets,
        format,
        &mut manifest_entries,
        byte_limit,
    )?;
    let mut observations = Vec::new();
    let absence_identity =
        selected_basis::selected_absence_identity(root_manifest, placements, format);
    let mut extent_manifests = BTreeMap::new();
    for placement in placements {
        match *placement {
            CurrentPhysicalRecordPlacement::Inline(inline) => {
                let Some(target) =
                    inline_targets.remove(&(inline.segment().get(), inline.page().get()))
                else {
                    continue;
                };
                observations.push(observe_inline(
                    discovery,
                    inline,
                    target,
                    format,
                    byte_limit,
                    &segment_entries,
                )?);
            }
            CurrentPhysicalRecordPlacement::Extent(extent) => {
                let Some(matching) = extent_targets.remove(&extent.extent().get()) else {
                    continue;
                };
                for target in matching.into_values() {
                    observations.push(observe_extent(
                        discovery,
                        extent,
                        target,
                        format,
                        byte_limit,
                        &mut extent_manifests,
                    )?);
                }
            }
        }
    }
    let absent_targets = inline_targets
        .into_values()
        .chain(extent_targets.into_values().flat_map(BTreeMap::into_values))
        .collect();
    let absent = allocation_truth::admit_absent_targets(
        discovery,
        root_manifest,
        placements,
        absent_targets,
        format,
        &mut manifest_entries,
        byte_limit,
        absence_identity,
    )?;
    observations.extend(absent.observations);
    let publication_inventory = publication_inventory::observe(
        discovery,
        root_manifest,
        format,
        &mut manifest_entries,
        byte_limit,
    )?;
    Ok(ObservedPageBasis {
        observations,
        inline_truth: absent.inline_truth,
        publication_inventory,
    })
}

pub(super) fn required(
    result: Result<
        worth_store::physical_runtime::ObservedRecoveryArtifact,
        worth_store::physical_runtime::RecoveryDiscoveryFailure,
    >,
    target: Option<PhysicalRedoTargetIdentity>,
    artifact: RecordArtifactFile,
) -> Result<Vec<u8>, PageObservationFailure> {
    match result {
        Ok(observed) => observed
            .into_bytes()
            .ok_or(PageObservationFailure::MissingArtifact { target, artifact }),
        Err(worth_store::physical_runtime::RecoveryDiscoveryFailure::ByteLimitExceeded {
            ..
        }) => Err(PageObservationFailure::ByteLimit),
        Err(failure) => Err(PageObservationFailure::Media { target, failure }),
    }
}
