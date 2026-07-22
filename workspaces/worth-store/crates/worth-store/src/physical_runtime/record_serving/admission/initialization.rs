use worth_store_physical_backend::{ArtifactTreeFailureKind, QualifiedFilesystemMedia};
use worth_store_physical_format::{
    durable_artifact_checksum, BootstrapCatalog, CurrentRootCatalogEntry,
    CurrentRootCatalogGeneration, DurableFreeSpaceManifestHeader, DurablePhysicalRootManifest,
    PhysicalFreeSpaceMembershipBlock, RecordAllocationClass, RecordArtifactFile,
    RecordFreeSpaceManifestEntry,
};

use super::super::residency::initialization_artifacts::InitializationRecordArtifacts;
use super::super::{
    admission::bootstrap::{
        backend_after_effect, backend_before_effect, BootstrapTransitionFailure,
        PhysicalRecordBootstrapOwner,
    },
    residency::artifact_tree::{RecordFamilyCreationFailure, RecordFamilyInventory},
    PhysicalRecordInitialization, RecordBootstrapDenial, RecordBootstrapFailure,
};

pub(in crate::physical_runtime::record_serving) fn initialize(
    media: &QualifiedFilesystemMedia,
    request: PhysicalRecordInitialization,
) -> Result<PhysicalRecordBootstrapOwner, BootstrapTransitionFailure> {
    if !request.placement.admits(request.format) || !request.access.admits(request.format) {
        return Err(BootstrapTransitionFailure::Denied(
            RecordBootstrapDenial::ConfigurationMismatch,
        ));
    }
    let artifacts = InitializationRecordArtifacts::new(media);
    match artifacts.inventory().map_err(backend_before_effect)? {
        RecordFamilyInventory::ProvenAbsent => {}
        RecordFamilyInventory::Published => {
            return Err(BootstrapTransitionFailure::Denied(
                RecordBootstrapDenial::RecordFamilyAlreadyExists,
            ));
        }
        RecordFamilyInventory::Residue => {
            return Err(BootstrapTransitionFailure::Denied(
                RecordBootstrapDenial::AmbiguousRecordFamilyResidue,
            ));
        }
    }
    let format = request.format.declaration();
    let free_entry =
        RecordFreeSpaceManifestEntry::new(RecordAllocationClass::Extent, 1, 1, u64::MAX - 1, 1)
            .expect("initial allocatable extent range is nonempty");
    let tree_identity = new_tree_identity()?;
    let free_block = PhysicalFreeSpaceMembershipBlock::leaf(
        tree_identity,
        1,
        1,
        vec![free_entry],
        request.placement.manifest_capacity().get(),
    )
    .ok_or(BootstrapTransitionFailure::Failed(
        RecordBootstrapFailure::FormatEncoding,
    ))?;
    let free_block_bytes = free_block.encode(format);
    let free_space = DurableFreeSpaceManifestHeader::new(
        1,
        tree_identity,
        request.placement.manifest_capacity().get(),
        1,
        1,
        1,
        1,
        2,
        Some(free_block.reference(durable_artifact_checksum(&free_block_bytes))),
    )
    .ok_or(BootstrapTransitionFailure::Failed(
        RecordBootstrapFailure::FormatEncoding,
    ))?;
    let free_space_bytes = free_space.encode(format);
    let current_root = DurablePhysicalRootManifest::builder(
        1,
        tree_identity,
        request.placement.manifest_capacity().get(),
        durable_artifact_checksum(&free_space_bytes),
    )
    .free_space_root(free_space.root())
    .admit()
    .ok_or(BootstrapTransitionFailure::Failed(
        RecordBootstrapFailure::FormatEncoding,
    ))?;
    let root_generation = CurrentRootCatalogGeneration::new(1).ok_or(
        BootstrapTransitionFailure::Failed(RecordBootstrapFailure::FormatEncoding),
    )?;
    let catalog = BootstrapCatalog::new(
        media.store_identity(),
        format,
        CurrentRootCatalogEntry::new(root_generation),
    )
    .encode();

    artifacts
        .create_record_family()
        .map_err(|failure| match failure {
            RecordFamilyCreationFailure::BeforeEffect(failure)
                if failure.kind() == ArtifactTreeFailureKind::DeniedBeforeEffect =>
            {
                backend_before_effect(failure)
            }
            RecordFamilyCreationFailure::BeforeEffect(failure)
                if failure.kind() == ArtifactTreeFailureKind::AlreadyExists =>
            {
                BootstrapTransitionFailure::Denied(
                    RecordBootstrapDenial::AmbiguousRecordFamilyResidue,
                )
            }
            RecordFamilyCreationFailure::BeforeEffect(failure)
            | RecordFamilyCreationFailure::AfterEffect(failure) => backend_after_effect(failure),
        })?;
    let free_block_artifact = RecordArtifactFile::FreeSpaceMembershipBlock {
        generation: 1,
        block: 1,
    };
    artifacts
        .write_new(free_block_artifact, &free_block_bytes)
        .map_err(backend_after_effect)?;
    artifacts
        .synchronize_artifact(free_block_artifact)
        .map_err(backend_after_effect)?;
    artifacts
        .synchronize_artifact_parent(free_block_artifact)
        .map_err(backend_after_effect)?;
    let root_artifact = RecordArtifactFile::RootManifest { generation: 1 };
    artifacts
        .write_new(root_artifact, &current_root.encode(format))
        .map_err(backend_after_effect)?;
    artifacts
        .synchronize_artifact(root_artifact)
        .map_err(backend_after_effect)?;
    artifacts
        .synchronize_artifact_parent(root_artifact)
        .map_err(backend_after_effect)?;
    let free_space_artifact = RecordArtifactFile::FreeSpaceManifest { generation: 1 };
    artifacts
        .write_new(free_space_artifact, &free_space_bytes)
        .map_err(backend_after_effect)?;
    artifacts
        .synchronize_artifact(free_space_artifact)
        .map_err(backend_after_effect)?;
    artifacts
        .synchronize_artifact_parent(free_space_artifact)
        .map_err(backend_after_effect)?;

    let candidate = RecordArtifactFile::CatalogCandidate { publication: 1 };
    artifacts
        .write_new(candidate, &catalog)
        .map_err(backend_after_effect)?;
    artifacts
        .synchronize_artifact(candidate)
        .map_err(backend_after_effect)?;
    artifacts
        .replace_catalog(candidate)
        .map_err(backend_after_effect)?;
    artifacts
        .synchronize_record_family()
        .map_err(backend_after_effect)?;
    Ok(PhysicalRecordBootstrapOwner {
        format: request.format,
        access: request.access,
        current_root: CurrentRootCatalogEntry::new(root_generation),
        observed_staging_residue: false,
    })
}

fn new_tree_identity() -> Result<u64, BootstrapTransitionFailure> {
    let mut bytes = [0_u8; 8];
    getrandom::fill(&mut bytes).map_err(|_| {
        BootstrapTransitionFailure::Denied(RecordBootstrapDenial::IdentityEntropyUnavailable)
    })?;
    let identity = u64::from_le_bytes(bytes);
    (identity != 0)
        .then_some(identity)
        .ok_or(BootstrapTransitionFailure::Denied(
            RecordBootstrapDenial::IdentityEntropyUnavailable,
        ))
}
