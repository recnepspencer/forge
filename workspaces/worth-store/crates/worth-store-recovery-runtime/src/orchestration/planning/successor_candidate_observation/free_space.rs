use std::collections::{BTreeSet, VecDeque};

use worth_store::physical_runtime::BoundedRecoveryFilesystemDiscovery;
use worth_store_physical_format::{
    durable_artifact_checksum, DurableFreeSpaceManifestHeader, DurablePhysicalRootManifest,
    FreeSpaceBlockReference, FreeSpaceMembershipBlockDecodeLimits,
    PhysicalFreeSpaceMembershipBlock, PhysicalRecordFormatDeclaration, RecordArtifactFile,
    RecordFreeSpaceManifestEntry,
};

use super::artifact_read::{observed, required, retain_successor};
use super::denial::{admit_successor_read, consume_successor, free_denial, invalid};
use super::materialization::CandidateMaterialization;
use crate::entry::PhysicalRecoverySuccessorCandidateDenial;
use crate::orchestration::planning::manifest_entry_budget::ManifestEntryBudget;
use crate::progression::RecoveryObservedCandidateArtifact;

pub(super) fn read(
    discovery: &mut BoundedRecoveryFilesystemDiscovery,
    root: &DurablePhysicalRootManifest,
    format: PhysicalRecordFormatDeclaration,
    budget: &mut ManifestEntryBudget,
    byte_limit: u64,
    artifacts: &mut Vec<RecoveryObservedCandidateArtifact>,
    referenced_artifacts: &mut Vec<RecordArtifactFile>,
    materialization: &mut CandidateMaterialization,
) -> Result<
    (
        DurableFreeSpaceManifestHeader,
        Vec<RecordFreeSpaceManifestEntry>,
    ),
    PhysicalRecoverySuccessorCandidateDenial,
> {
    let header_artifact = RecordArtifactFile::FreeSpaceManifest {
        generation: root.generation(),
    };
    let header_bytes = required(
        discovery.read_free_space_manifest(root.generation(), byte_limit),
        header_artifact,
    )?;
    let (header, found_format) =
        DurableFreeSpaceManifestHeader::decode(&header_bytes, root.node_capacity())
            .map_err(|_| invalid(header_artifact))?;
    if found_format != format
        || header.generation() != root.generation()
        || header.root() != root.free_space_root()
        || durable_artifact_checksum(&header_bytes) != root.free_space_checksum()
        || header.encode(format) != header_bytes
    {
        return Err(invalid(header_artifact));
    }
    materialization.retain_free_space_header(header_bytes.len());
    artifacts.push(observed(header_artifact, header_bytes));
    referenced_artifacts.push(header_artifact);
    materialization.retain_reference();
    let mut pending = header.root().into_iter().collect::<VecDeque<_>>();
    let mut visited = BTreeSet::new();
    let mut entries = Vec::new();
    while let Some(reference) = pending.pop_front() {
        let artifact = RecordArtifactFile::FreeSpaceMembershipBlock {
            generation: reference.generation(),
            block: reference.block(),
        };
        referenced_artifacts.push(artifact);
        materialization.retain_reference();
        admit_successor_read(budget, artifact)?;
        if !visited.insert((reference.generation(), reference.block())) {
            return Err(invalid(artifact));
        }
        let bytes = required(
            discovery.read_free_space_membership_block(
                reference.generation(),
                reference.block(),
                byte_limit,
            ),
            artifact,
        )?;
        let (block, found_format) = PhysicalFreeSpaceMembershipBlock::decode_bounded(
            &bytes,
            header.node_capacity(),
            FreeSpaceMembershipBlockDecodeLimits {
                leaf_entries: budget.remaining(),
                branch_children: budget.remaining(),
            },
        )
        .map_err(|denial| free_denial(denial, artifact, budget))?;
        if found_format != format {
            return Err(invalid(artifact));
        }
        validate(&header, format, reference, &block, &bytes, artifact)?;
        if let Some(found) = block.entries() {
            consume_successor(budget, found.len(), artifact)?;
            materialization.retain_free_entries(found.len());
            entries.extend_from_slice(found);
        } else if let Some(children) = block.children() {
            consume_successor(budget, children.len(), artifact)?;
            pending.extend(children.iter().copied());
        }
        let retained_bytes = bytes.len();
        if retain_successor(root.generation(), artifact, bytes, artifacts) {
            materialization.retain_artifact(retained_bytes);
        }
    }
    entries.sort_unstable_by_key(|entry| (entry.class() as u8, entry.owner()));
    Ok((header, entries))
}

fn validate(
    header: &DurableFreeSpaceManifestHeader,
    format: PhysicalRecordFormatDeclaration,
    reference: FreeSpaceBlockReference,
    block: &PhysicalFreeSpaceMembershipBlock,
    bytes: &[u8],
    artifact: RecordArtifactFile,
) -> Result<(), PhysicalRecoverySuccessorCandidateDenial> {
    (block.tree_identity() == header.tree_identity()
        && block.reference(durable_artifact_checksum(bytes)) == reference
        && block.encode(format) == bytes)
        .then_some(())
        .ok_or_else(|| invalid(artifact))
}
