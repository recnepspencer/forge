use sha2::{Digest, Sha256};
use worth_store_physical_format::{
    durable_artifact_checksum, BootstrapCatalog, CurrentRootCatalogEntry,
    CurrentRootCatalogGeneration, DurableFreeSpaceManifestHeader, DurablePhysicalRootManifest,
    DurableRootSelector, PhysicalRecordFormatDeclaration, RecordArtifactFile, RootSelectorIdentity,
    RootSelectorRole,
};

use super::{
    RecoveryBaseImagePlan, RecoveryPublicationCandidateArtifact, RecoveryPublicationSourceInventory,
};

mod inventory;
mod tree;

pub(super) struct RecoveryCandidateBasis {
    pub(super) root: DurablePhysicalRootManifest,
    pub(super) artifacts: Box<[RecoveryPublicationCandidateArtifact]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CandidateBuildDenial {
    Invalid,
}

pub(super) fn build(
    store: worth_store_physical_format::store_namespace::StableStoreIdentity,
    base: &RecoveryBaseImagePlan,
    source: &RecoveryPublicationSourceInventory,
    format: PhysicalRecordFormatDeclaration,
    publication: u64,
) -> Result<RecoveryCandidateBasis, CandidateBuildDenial> {
    let generation = base.destination_generation();
    let selected = base.selected_root();
    let final_inventory = inventory::finalize(
        source,
        base.actions(),
        base.segment_updates(),
        base.root_states(),
        selected.node_capacity(),
    )?;
    let mut build = CandidateBuild {
        format,
        artifacts: Vec::new(),
    };
    let (routing_root, next_block) = tree::root_routing(
        &mut build,
        &final_inventory.placements,
        selected.tree_identity(),
        generation,
        final_inventory.capacity,
        selected.next_block(),
    )?;
    let (segment_root, next_segment_block) = tree::segment_routing(
        &mut build,
        &final_inventory.segments,
        selected.tree_identity(),
        generation,
        final_inventory.capacity,
        selected.next_segment_block(),
    )?;
    let (free_root, next_free_block) = tree::free_space_routing(
        &mut build,
        &final_inventory.free,
        source.free_space.tree_identity(),
        generation,
        final_inventory.capacity,
        source.free_space.next_block(),
    )?;
    let free_space = DurableFreeSpaceManifestHeader::new(
        generation,
        source.free_space.tree_identity(),
        final_inventory.capacity,
        source.free_space.segment_page_capacity(),
        final_inventory.free.len() as u64,
        final_inventory.next_segment,
        final_inventory.next_page,
        final_inventory.next_extent,
        next_free_block,
        free_root,
    )
    .ok_or(CandidateBuildDenial::Invalid)?;
    let free_bytes = free_space.encode(format);
    build.push(
        RecordArtifactFile::FreeSpaceManifest { generation },
        free_bytes.clone(),
    )?;
    let root = DurablePhysicalRootManifest::builder(
        generation,
        selected.tree_identity(),
        final_inventory.capacity,
        durable_artifact_checksum(&free_bytes),
    )
    .record_count(final_inventory.placements.len() as u64)
    .next_block(next_block)
    .next_segment_block(next_segment_block)
    .routing_root(routing_root)
    .segment_root(segment_root)
    .free_space_root(free_root)
    .last_inline_record(
        final_inventory
            .last_inline_record
            .or(selected.last_inline_record()),
    )
    .last_inline_segment(
        final_inventory
            .last_inline_segment
            .or(selected.last_inline_segment()),
    )
    .admit()
    .ok_or(CandidateBuildDenial::Invalid)?;
    build.push(
        RecordArtifactFile::RootManifest { generation },
        root.encode(format),
    )?;
    push_protocol_candidates(
        &mut build,
        store,
        format,
        base.selected_selector(),
        generation,
        publication,
    )?;
    build.artifacts.sort_by_key(|artifact| artifact.artifact);
    Ok(RecoveryCandidateBasis {
        root,
        artifacts: build.artifacts.into_boxed_slice(),
    })
}

fn push_protocol_candidates(
    build: &mut CandidateBuild,
    store: worth_store_physical_format::store_namespace::StableStoreIdentity,
    format: PhysicalRecordFormatDeclaration,
    selected: DurableRootSelector,
    generation: u64,
    publication: u64,
) -> Result<(), CandidateBuildDenial> {
    let previous_identity = selected.identity();
    let current_identity =
        RootSelectorIdentity::new(generation).ok_or(CandidateBuildDenial::Invalid)?;
    let previous = DurableRootSelector::new(
        store,
        format,
        previous_identity,
        RootSelectorRole::Previous,
        selected.root_generation(),
        Some(current_identity),
        Some(generation),
    )
    .ok_or(CandidateBuildDenial::Invalid)?;
    let current = DurableRootSelector::new(
        store,
        format,
        current_identity,
        RootSelectorRole::Current,
        generation,
        Some(previous_identity),
        Some(selected.root_generation()),
    )
    .ok_or(CandidateBuildDenial::Invalid)?;
    let catalog = BootstrapCatalog::new(
        store,
        format,
        CurrentRootCatalogEntry::new(
            CurrentRootCatalogGeneration::new(generation).ok_or(CandidateBuildDenial::Invalid)?,
        ),
    );
    build.push(
        RecordArtifactFile::RootSelectorCandidate {
            role: RootSelectorRole::Previous,
            publication,
        },
        previous.encode().to_vec(),
    )?;
    build.push(
        RecordArtifactFile::RootSelectorCandidate {
            role: RootSelectorRole::Current,
            publication,
        },
        current.encode().to_vec(),
    )?;
    build.push(
        RecordArtifactFile::CatalogCandidate { publication },
        catalog.encode().to_vec(),
    )
}

pub(super) struct CandidateBuild {
    format: PhysicalRecordFormatDeclaration,
    artifacts: Vec<RecoveryPublicationCandidateArtifact>,
}

impl CandidateBuild {
    fn push(
        &mut self,
        artifact: RecordArtifactFile,
        bytes: Vec<u8>,
    ) -> Result<(), CandidateBuildDenial> {
        if bytes.is_empty()
            || self
                .artifacts
                .iter()
                .any(|candidate| candidate.artifact == artifact)
        {
            return Err(CandidateBuildDenial::Invalid);
        }
        let payload_digest = Sha256::digest(&bytes).into();
        self.artifacts.push(RecoveryPublicationCandidateArtifact {
            artifact,
            bytes: bytes.into_boxed_slice(),
            payload_digest,
        });
        Ok(())
    }
}
