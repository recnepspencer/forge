use std::collections::{BTreeSet, VecDeque};

use worth_store::physical_runtime::BoundedRecoveryFilesystemDiscovery;
use worth_store_physical_format::{
    durable_artifact_checksum, DurablePhysicalRootManifest, PhysicalRecordFormatDeclaration,
    PhysicalSegmentMembershipBlock, RecordArtifactFile, RecordSegmentPageManifestEntry,
    SegmentManifestBlockReference, SegmentMembershipBlockDecodeLimits,
};

use super::artifact_read::{required, retain_successor};
use super::denial::{admit_successor_read, consume_successor, invalid, segment_denial};
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
) -> Result<Vec<RecordSegmentPageManifestEntry>, PhysicalRecoverySuccessorCandidateDenial> {
    let mut pending = root.segment_root().into_iter().collect::<VecDeque<_>>();
    let mut visited = BTreeSet::new();
    let mut entries = Vec::new();
    while let Some(reference) = pending.pop_front() {
        let artifact = RecordArtifactFile::SegmentMembershipBlock {
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
            discovery.read_segment_membership_block(
                reference.generation(),
                reference.block(),
                byte_limit,
            ),
            artifact,
        )?;
        let (block, found_format) = PhysicalSegmentMembershipBlock::decode_bounded(
            &bytes,
            root.node_capacity(),
            SegmentMembershipBlockDecodeLimits {
                leaf_entries: budget.remaining(),
                branch_children: budget.remaining(),
            },
        )
        .map_err(|denial| segment_denial(denial, artifact, budget))?;
        if found_format != format {
            return Err(invalid(artifact));
        }
        validate(root, format, reference, &block, &bytes, artifact)?;
        if let Some(found) = block.entries() {
            consume_successor(budget, found.len(), artifact)?;
            materialization.retain_segment_entries(found.len());
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
    entries
        .sort_unstable_by_key(|entry| (entry.page_cell().segment_id().get(), entry.page().get()));
    Ok(entries)
}

fn validate(
    root: &DurablePhysicalRootManifest,
    format: PhysicalRecordFormatDeclaration,
    reference: SegmentManifestBlockReference,
    block: &PhysicalSegmentMembershipBlock,
    bytes: &[u8],
    artifact: RecordArtifactFile,
) -> Result<(), PhysicalRecoverySuccessorCandidateDenial> {
    (block.tree_identity() == root.tree_identity()
        && block.reference(durable_artifact_checksum(bytes)) == reference
        && block.encode(format) == bytes)
        .then_some(())
        .ok_or_else(|| invalid(artifact))
}
