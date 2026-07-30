#[cfg(feature = "certification-test-authority")]
use worth_store_physical_backend::QualifiedFilesystemMedia;
use worth_store_physical_format::{
    durable_artifact_checksum, DurablePhysicalRootManifest, PhysicalPageId, PhysicalSegmentId,
    PhysicalSegmentMembershipBlock, RecordArtifactFile, RecordSegmentPageManifestEntry,
    SegmentManifestBlockReference, SegmentPageKey,
};

use super::super::access::manifest_routing::{
    ManifestDiscoveryCounterSnapshot, ManifestLookupFailure,
};
use super::super::residency::{record_frame_reader::RecordFrameReader, PhysicalResidencyWorkPort};
use super::super::{AdmittedPhysicalRecordFormat, AdmittedRecordAccessPolicy};

mod update_planning;
pub(in crate::physical_runtime::record_serving) use update_planning::{
    plan_segment_membership_updates, SegmentMembershipPublicationPlan,
    SegmentMembershipUpdateContext,
};

pub(in crate::physical_runtime::record_serving) struct SegmentMembershipReader<'media> {
    artifacts: RecordFrameReader<'media>,
    format: AdmittedPhysicalRecordFormat,
    access: AdmittedRecordAccessPolicy,
    root: DurablePhysicalRootManifest,
}

impl<'media> SegmentMembershipReader<'media> {
    #[cfg(feature = "certification-test-authority")]
    pub(in crate::physical_runtime::record_serving) fn with_loader(
        media: &'media QualifiedFilesystemMedia,
        loader: &'media (dyn super::super::residency::frame_ports::FrameLoadPort + Send + Sync),
        format: AdmittedPhysicalRecordFormat,
        access: AdmittedRecordAccessPolicy,
        root: &'media DurablePhysicalRootManifest,
    ) -> Self {
        Self {
            artifacts: RecordFrameReader::bootstrap(media, loader),
            format,
            access,
            root: root.clone(),
        }
    }

    pub(in crate::physical_runtime::record_serving) fn serving(
        residency: PhysicalResidencyWorkPort,
        format: AdmittedPhysicalRecordFormat,
        access: AdmittedRecordAccessPolicy,
        root: DurablePhysicalRootManifest,
    ) -> SegmentMembershipReader<'static> {
        SegmentMembershipReader {
            artifacts: RecordFrameReader::serving(residency),
            format,
            access,
            root,
        }
    }

    pub(in crate::physical_runtime::record_serving) fn locate(
        &self,
        allocation: &worth_store_buffer_pool::OperationAllocationGrant,
        segment: PhysicalSegmentId,
        page: PhysicalPageId,
        counters: &mut ManifestDiscoveryCounterSnapshot,
    ) -> Result<Option<RecordSegmentPageManifestEntry>, ManifestLookupFailure> {
        let key = SegmentPageKey::new(segment, page);
        let Some(mut reference) = self.root.segment_root() else {
            return Ok(None);
        };
        if !reference.contains(key) {
            return Ok(None);
        }
        loop {
            match self.read_block(allocation, reference, counters)? {
                PhysicalSegmentMembershipBlock::Leaf { entries, .. } => {
                    let (result, comparisons) =
                        super::super::access::counted_search::binary_search_by(&entries, |entry| {
                            SegmentPageKey::from(*entry).cmp(&key)
                        });
                    counters.observe_comparisons(comparisons);
                    return Ok(result.ok().map(|index| entries[index]));
                }
                PhysicalSegmentMembershipBlock::Branch { children, .. } => {
                    let (index, comparisons) =
                        super::super::access::counted_search::partition_point(&children, |child| {
                            child.last() < key
                        });
                    counters.observe_comparisons(comparisons);
                    let Some(child) = children
                        .get(index)
                        .copied()
                        .filter(|child| child.contains(key))
                    else {
                        return Ok(None);
                    };
                    reference = child;
                }
            }
        }
    }

    pub(in crate::physical_runtime::record_serving) fn read_block(
        &self,
        allocation: &worth_store_buffer_pool::OperationAllocationGrant,
        reference: SegmentManifestBlockReference,
        counters: &mut ManifestDiscoveryCounterSnapshot,
    ) -> Result<PhysicalSegmentMembershipBlock, ManifestLookupFailure> {
        let limit = self
            .access
            .transfer_limit()
            .get()
            .min(self.format.declaration().page_size().bytes());
        let bytes = self
            .artifacts
            .load_bounded(
                allocation,
                RecordArtifactFile::SegmentMembershipBlock {
                    generation: reference.generation(),
                    block: reference.block(),
                },
                limit,
            )
            .map_err(|failure| {
                counters.observe_failed_work(failure.work_trace());
                frame_load_failure(failure)
            })?;
        counters.observe_block(bytes.len(), bytes.work_trace());
        let checksum = durable_artifact_checksum(&bytes);
        let (block, found_format) =
            match PhysicalSegmentMembershipBlock::decode(&bytes, self.root.node_capacity()) {
                Ok(decoded) => decoded,
                Err(_) => {
                    bytes.reject_projection_failure();
                    return Err(ManifestLookupFailure::Damaged);
                }
            };
        if found_format != self.format.declaration()
            || block.tree_identity() != self.root.tree_identity()
            || block.level() != reference.level()
            || block.reference(checksum) != reference
        {
            bytes.reject_projection_failure();
            return Err(ManifestLookupFailure::Damaged);
        }
        Ok(block)
    }

    fn format_declaration(&self) -> worth_store_physical_format::PhysicalRecordFormatDeclaration {
        self.format.declaration()
    }
}

fn frame_load_failure(
    failure: crate::physical_runtime::record_serving::residency::frame_loading::FrameLoadFailure,
) -> ManifestLookupFailure {
    use crate::physical_runtime::record_serving::residency::frame_loading::FrameLoadFailureKind;
    match failure.kind() {
        FrameLoadFailureKind::Backend(reason) => ManifestLookupFailure::Backend(reason),
        FrameLoadFailureKind::Residency(reason) => ManifestLookupFailure::Residency(reason),
        kind => ManifestLookupFailure::Frame(kind),
    }
}
