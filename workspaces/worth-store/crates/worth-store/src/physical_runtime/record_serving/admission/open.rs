use worth_store_physical_backend::{ArtifactTreeFailureKind, QualifiedFilesystemMedia};
use worth_store_physical_format::{
    durable_artifact_checksum, BootstrapCatalog, DurableFreeSpaceManifestHeader,
    DurablePhysicalRootManifest, RecordArtifactFile,
};

use super::super::residency::serving_artifacts::ServingRecordArtifacts;
use super::super::{
    admission::bootstrap::{
        backend_before_effect, BootstrapCatalogReadLimits, BootstrapTransitionFailure,
        PhysicalRecordBootstrapOwner, RecordServingRebindReason, RecordServingStaleReason,
        RecordServingState,
    },
    publication::publication_residue::observe_publication_residue,
    residency::artifact_tree::RecordFamilyInventory,
    PhysicalRecordFormatMismatch, PhysicalRecordOpen, RecordBootstrapDenial,
    UnsupportedPhysicalRecordFormat,
};

pub(in crate::physical_runtime::record_serving) fn open(
    media: &QualifiedFilesystemMedia,
    loader: &(dyn super::super::residency::frame_ports::FrameLoadPort + Send + Sync),
    request: PhysicalRecordOpen,
) -> Result<PhysicalRecordBootstrapOwner, BootstrapTransitionFailure> {
    if !request.access.admits(request.format) {
        return Err(BootstrapTransitionFailure::Denied(
            RecordBootstrapDenial::ConfigurationMismatch,
        ));
    }
    let artifacts = ServingRecordArtifacts::new(media, loader);
    let limits = BootstrapCatalogReadLimits::for_format(request.format, request.access);
    match artifacts.inventory().map_err(backend_before_effect)? {
        RecordFamilyInventory::ProvenAbsent => {
            return Err(BootstrapTransitionFailure::Denied(
                RecordBootstrapDenial::RecordFamilyAbsent,
            ));
        }
        RecordFamilyInventory::Residue => {
            return Err(BootstrapTransitionFailure::Denied(
                RecordBootstrapDenial::AmbiguousRecordFamilyResidue,
            ));
        }
        RecordFamilyInventory::Published => {}
    }
    let catalog_bytes = artifacts
        .load_bounded(RecordArtifactFile::BootstrapCatalog, limits.catalog_bytes())
        .map_err(|failure| match failure {
            super::super::residency::frame_loading::FrameLoadFailure::Backend(failure)
                if failure.kind() == ArtifactTreeFailureKind::Absent =>
            {
                BootstrapTransitionFailure::Denied(RecordBootstrapDenial::CatalogMissing)
            }
            super::super::residency::frame_loading::FrameLoadFailure::Residency(reason) => {
                BootstrapTransitionFailure::Denied(RecordBootstrapDenial::ResidencyUnavailable(
                    reason,
                ))
            }
            super::super::residency::frame_loading::FrameLoadFailure::Backend(failure) => {
                match failure.kind() {
                    ArtifactTreeFailureKind::AccessLimitExceeded
                    | ArtifactTreeFailureKind::Damaged => {
                        BootstrapTransitionFailure::Denied(RecordBootstrapDenial::CatalogDamaged)
                    }
                    _ => backend_before_effect(failure),
                }
            }
            _ => BootstrapTransitionFailure::Denied(RecordBootstrapDenial::CatalogDamaged),
        })?;
    let catalog = BootstrapCatalog::decode(&catalog_bytes).map_err(classify_catalog_denial)?;
    if catalog.store_identity() != media.store_identity() {
        return Err(BootstrapTransitionFailure::RebindRequired(
            RecordServingRebindReason::StoreIdentityMismatch,
        ));
    }
    if catalog.format() != request.format.declaration() {
        return Err(BootstrapTransitionFailure::Denied(
            RecordBootstrapDenial::PhysicalRecordFormatMismatch(PhysicalRecordFormatMismatch::new(
                request.format.declaration(),
                catalog.format(),
            )),
        ));
    }
    let observed_staging_residue = artifacts
        .has_staging_residue()
        .map_err(backend_before_effect)?;
    Ok(PhysicalRecordBootstrapOwner {
        format: request.format,
        access: request.access,
        current_root: catalog.current_root(),
        observed_staging_residue,
    })
}

pub(in crate::physical_runtime::record_serving) fn load_current_root(
    media: &QualifiedFilesystemMedia,
    loader: &(dyn super::super::residency::frame_ports::FrameLoadPort + Send + Sync),
    bootstrap: PhysicalRecordBootstrapOwner,
) -> Result<RecordServingState, BootstrapTransitionFailure> {
    let limits = BootstrapCatalogReadLimits::for_format(bootstrap.format, bootstrap.access);
    let generation = bootstrap.current_root.generation().get();
    let root_bytes = ServingRecordArtifacts::new(media, loader)
        .load_bounded(
            RecordArtifactFile::RootManifest { generation },
            limits.current_root_bytes().get(),
        )
        .map_err(|failure| {
            BootstrapTransitionFailure::Denied(match failure {
                super::super::residency::frame_loading::FrameLoadFailure::Residency(reason) => {
                    RecordBootstrapDenial::ResidencyUnavailable(reason)
                }
                super::super::residency::frame_loading::FrameLoadFailure::Backend(failure) => {
                    match failure.kind() {
                        ArtifactTreeFailureKind::Absent
                        | ArtifactTreeFailureKind::AccessLimitExceeded
                        | ArtifactTreeFailureKind::Damaged => {
                            RecordBootstrapDenial::CurrentRootDamaged
                        }
                        _ => RecordBootstrapDenial::BackendUnavailable(failure),
                    }
                }
                _ => RecordBootstrapDenial::CurrentRootDamaged,
            })
        })?;
    let (current_root, root_format) =
        DurablePhysicalRootManifest::decode(&root_bytes, limits.current_root_entries())
            .map_err(classify_root_denial)?;
    if !super::super::planning::policy_units::manifest_capacity_can_branch(
        current_root.node_capacity(),
    ) {
        return Err(BootstrapTransitionFailure::Denied(
            RecordBootstrapDenial::CurrentRootDamaged,
        ));
    }
    if current_root.generation() != generation {
        return Err(BootstrapTransitionFailure::Stale(
            RecordServingStaleReason::CatalogSelectedRootGenerationMismatch,
        ));
    }
    if root_format != bootstrap.format.declaration() {
        return Err(BootstrapTransitionFailure::Denied(
            RecordBootstrapDenial::PhysicalRecordFormatMismatch(PhysicalRecordFormatMismatch::new(
                bootstrap.format.declaration(),
                root_format,
            )),
        ));
    }
    let free_space_bytes = ServingRecordArtifacts::new(media, loader)
        .load_bounded(
            RecordArtifactFile::FreeSpaceManifest { generation },
            limits.current_root_bytes().get(),
        )
        .map_err(|failure| {
            BootstrapTransitionFailure::Denied(match failure {
                super::super::residency::frame_loading::FrameLoadFailure::Residency(reason) => {
                    RecordBootstrapDenial::ResidencyUnavailable(reason)
                }
                _ => RecordBootstrapDenial::FreeSpaceManifestDamaged,
            })
        })?;
    let (free_space, free_format) =
        DurableFreeSpaceManifestHeader::decode(&free_space_bytes, limits.current_root_entries())
            .map_err(classify_free_space_denial)?;
    if !super::super::planning::policy_units::manifest_capacity_can_branch(
        free_space.node_capacity(),
    ) {
        return Err(BootstrapTransitionFailure::Denied(
            RecordBootstrapDenial::FreeSpaceManifestDamaged,
        ));
    }
    if durable_artifact_checksum(&free_space_bytes) != current_root.free_space_checksum() {
        return Err(BootstrapTransitionFailure::Denied(
            RecordBootstrapDenial::FreeSpaceManifestDamaged,
        ));
    }
    if free_space.generation() != generation {
        return Err(BootstrapTransitionFailure::Stale(
            RecordServingStaleReason::FreeSpaceGenerationMismatch,
        ));
    }
    if free_space.root() != current_root.free_space_root() {
        return Err(BootstrapTransitionFailure::Denied(
            RecordBootstrapDenial::FreeSpaceManifestDamaged,
        ));
    }
    if free_format != bootstrap.format.declaration() {
        return Err(BootstrapTransitionFailure::Denied(
            RecordBootstrapDenial::PhysicalRecordFormatMismatch(PhysicalRecordFormatMismatch::new(
                bootstrap.format.declaration(),
                free_format,
            )),
        ));
    }
    if free_space.next_segment() == 0
        || free_space.next_page() == 0
        || free_space.next_extent() == 0
    {
        return Err(BootstrapTransitionFailure::Denied(
            RecordBootstrapDenial::FreeSpaceManifestDamaged,
        ));
    }
    let publication_residue = observe_publication_residue(
        &ServingRecordArtifacts::new(media, loader),
        &current_root,
        &free_space,
        bootstrap.observed_staging_residue,
    )
    .map_err(backend_before_effect)?;
    Ok(RecordServingState {
        format: bootstrap.format,
        access: bootstrap.access,
        current_root,
        publication_residue,
        free_space,
    })
}

fn classify_catalog_denial(
    denial: worth_store_physical_format::BootstrapCatalogDenial,
) -> BootstrapTransitionFailure {
    match denial {
        worth_store_physical_format::BootstrapCatalogDenial::Frame(
            worth_store_physical_format::DurableFrameDenial::UnsupportedFormat(reason),
        ) => BootstrapTransitionFailure::Denied(
            RecordBootstrapDenial::UnsupportedPhysicalRecordFormat(
                UnsupportedPhysicalRecordFormat::new(reason),
            ),
        ),
        _ => BootstrapTransitionFailure::Denied(RecordBootstrapDenial::CatalogDamaged),
    }
}

fn classify_root_denial(
    denial: worth_store_physical_format::RootManifestDenial,
) -> BootstrapTransitionFailure {
    match denial {
        worth_store_physical_format::RootManifestDenial::Frame(
            worth_store_physical_format::DurableFrameDenial::UnsupportedFormat(reason),
        ) => BootstrapTransitionFailure::Denied(
            RecordBootstrapDenial::UnsupportedPhysicalRecordFormat(
                UnsupportedPhysicalRecordFormat::new(reason),
            ),
        ),
        worth_store_physical_format::RootManifestDenial::IdentityMismatch => {
            BootstrapTransitionFailure::Stale(
                RecordServingStaleReason::CatalogSelectedRootGenerationMismatch,
            )
        }
        _ => BootstrapTransitionFailure::Denied(RecordBootstrapDenial::CurrentRootDamaged),
    }
}

fn classify_free_space_denial(
    denial: worth_store_physical_format::FreeSpaceRoutingDenial,
) -> BootstrapTransitionFailure {
    match denial {
        worth_store_physical_format::FreeSpaceRoutingDenial::Frame(
            worth_store_physical_format::DurableFrameDenial::UnsupportedFormat(reason),
        ) => BootstrapTransitionFailure::Denied(
            RecordBootstrapDenial::UnsupportedPhysicalRecordFormat(
                UnsupportedPhysicalRecordFormat::new(reason),
            ),
        ),
        _ => BootstrapTransitionFailure::Denied(RecordBootstrapDenial::FreeSpaceManifestDamaged),
    }
}
