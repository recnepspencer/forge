#[cfg(feature = "certification-test-authority")]
use worth_store_physical_backend::QualifiedFilesystemMedia;
use worth_store_physical_format::{
    durable_artifact_checksum, DurableFreeSpaceManifestHeader, FreeSpaceBlockReference,
    FreeSpaceKey, PhysicalFreeSpaceMembershipBlock, RecordArtifactFile,
    RecordFreeSpaceManifestEntry,
};

use super::super::super::access::manifest_routing::{
    ManifestDiscoveryCounterSnapshot, ManifestLookupFailure,
};
use super::super::super::residency::{
    record_frame_reader::RecordFrameReader, ServingFrameResidency,
};
use super::super::super::{AdmittedPhysicalRecordFormat, AdmittedRecordAccessPolicy};

pub(in crate::physical_runtime::record_serving) struct FreeSpaceReader<'media> {
    artifacts: RecordFrameReader<'media>,
    pub(super) format: AdmittedPhysicalRecordFormat,
    access: AdmittedRecordAccessPolicy,
    pub(super) header: &'media DurableFreeSpaceManifestHeader,
}

impl<'media> FreeSpaceReader<'media> {
    #[cfg(feature = "certification-test-authority")]
    pub(in crate::physical_runtime::record_serving) fn with_loader(
        media: &'media QualifiedFilesystemMedia,
        loader: &'media (dyn super::super::super::residency::frame_ports::FrameLoadPort
                     + Send
                     + Sync),
        format: AdmittedPhysicalRecordFormat,
        access: AdmittedRecordAccessPolicy,
        header: &'media DurableFreeSpaceManifestHeader,
    ) -> Self {
        Self {
            artifacts: RecordFrameReader::bootstrap(media, loader),
            format,
            access,
            header,
        }
    }

    pub(in crate::physical_runtime::record_serving) fn serving(
        residency: ServingFrameResidency,
        format: AdmittedPhysicalRecordFormat,
        access: AdmittedRecordAccessPolicy,
        header: &'media DurableFreeSpaceManifestHeader,
    ) -> Self {
        Self {
            artifacts: RecordFrameReader::serving(residency),
            format,
            access,
            header,
        }
    }

    pub(in crate::physical_runtime::record_serving) fn locate(
        &self,
        allocation: &worth_store_buffer_pool::OperationAllocationGrant,
        key: FreeSpaceKey,
        counters: &mut ManifestDiscoveryCounterSnapshot,
    ) -> Result<Option<RecordFreeSpaceManifestEntry>, ManifestLookupFailure> {
        let Some(mut reference) = self.header.root() else {
            return Ok(None);
        };
        if !reference.contains(key) {
            return Ok(None);
        }
        loop {
            match self.read_block(allocation, reference, counters)? {
                PhysicalFreeSpaceMembershipBlock::Leaf { entries, .. } => {
                    let (result, comparisons) =
                        super::super::super::access::counted_search::binary_search_by(
                            &entries,
                            |entry| FreeSpaceKey::from(*entry).cmp(&key),
                        );
                    counters.observe_comparisons(comparisons);
                    return Ok(result.ok().map(|index| entries[index]));
                }
                PhysicalFreeSpaceMembershipBlock::Branch { children, .. } => {
                    let (index, comparisons) =
                        super::super::super::access::counted_search::partition_point(
                            &children,
                            |child| child.last() < key,
                        );
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
        reference: FreeSpaceBlockReference,
        counters: &mut ManifestDiscoveryCounterSnapshot,
    ) -> Result<PhysicalFreeSpaceMembershipBlock, ManifestLookupFailure> {
        let bytes = self
            .artifacts
            .load_bounded(
                allocation,
                RecordArtifactFile::FreeSpaceMembershipBlock {
                    generation: reference.generation(),
                    block: reference.block(),
                },
                self.access
                    .transfer_limit()
                    .get()
                    .min(self.format.declaration().page_size().bytes()),
            )
            .map_err(|failure| {
                counters.observe_failed_work(failure.work_trace());
                frame_load_failure(failure)
            })?;
        counters.observe_block(bytes.len(), bytes.work_trace());
        let checksum = durable_artifact_checksum(&bytes);
        let (block, found_format) =
            match PhysicalFreeSpaceMembershipBlock::decode(&bytes, self.header.node_capacity()) {
                Ok(decoded) => decoded,
                Err(_) => {
                    bytes.reject_projection_failure();
                    return Err(ManifestLookupFailure::Damaged);
                }
            };
        if found_format != self.format.declaration()
            || block.tree_identity() != self.header.tree_identity()
            || block.level() != reference.level()
            || block.reference(checksum) != reference
        {
            bytes.reject_projection_failure();
            return Err(ManifestLookupFailure::Damaged);
        }
        Ok(block)
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
