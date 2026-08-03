use worth_store_physical_backend::ArtifactTreeFailure;
#[cfg(feature = "certification-test-authority")]
use worth_store_physical_backend::QualifiedFilesystemMedia;
use worth_store_physical_format::{
    durable_artifact_checksum, CurrentPhysicalRecordPlacement, DurablePhysicalRootManifest,
    ManifestBlockReference, PersistedRecordIdentity, PhysicalRootRoutingBlock, RecordArtifactFile,
};

use crate::physical_runtime::record_serving::{
    residency::{record_frame_reader::RecordFrameReader, PhysicalResidencyWorkPort},
    AdmittedPhysicalRecordFormat, AdmittedRecordAccessPolicy,
};

use super::ManifestDiscoveryCounterSnapshot;

pub(in crate::physical_runtime::record_serving) struct ManifestReader<'media> {
    artifacts: RecordFrameReader<'media>,
    format: AdmittedPhysicalRecordFormat,
    access: AdmittedRecordAccessPolicy,
    root: DurablePhysicalRootManifest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime::record_serving) enum ManifestLookupFailure {
    Backend(ArtifactTreeFailure),
    Residency(worth_store_buffer_pool::PhysicalResidencyDenial),
    Frame(crate::physical_runtime::record_serving::residency::frame_loading::FrameLoadFailureKind),
    Damaged,
}

impl<'media> ManifestReader<'media> {
    #[cfg(feature = "certification-test-authority")]
    pub(in crate::physical_runtime::record_serving) fn with_loader(
        media: &'media QualifiedFilesystemMedia,
        loader: &'media (dyn crate::physical_runtime::record_serving::residency::frame_ports::FrameLoadPort
                     + Send
                     + Sync),
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
    ) -> ManifestReader<'static> {
        ManifestReader {
            artifacts: RecordFrameReader::serving(residency),
            format,
            access,
            root,
        }
    }

    pub(in crate::physical_runtime::record_serving) fn locate(
        &self,
        allocation: &worth_store_buffer_pool::OperationAllocationGrant,
        record: PersistedRecordIdentity,
        counters: &mut ManifestDiscoveryCounterSnapshot,
    ) -> Result<Option<CurrentPhysicalRecordPlacement>, ManifestLookupFailure> {
        let Some(mut reference) = self.root.routing_root() else {
            return Ok(None);
        };
        if !reference.contains(record) {
            return Ok(None);
        }
        loop {
            let block = self.read_block(allocation, reference, counters)?;
            match block {
                PhysicalRootRoutingBlock::Leaf { entries, .. } => {
                    let (result, comparisons) =
                        super::super::counted_search::binary_search_by(&entries, |entry| {
                            entry.record().cmp(&record)
                        });
                    counters.observe_comparisons(comparisons);
                    return Ok(result.ok().map(|index| entries[index]));
                }
                PhysicalRootRoutingBlock::Branch { children, .. } => {
                    let (index, comparisons) =
                        super::super::counted_search::partition_point(&children, |child| {
                            child.last() < record
                        });
                    counters.observe_comparisons(comparisons);
                    let child = children
                        .get(index)
                        .copied()
                        .filter(|child| child.contains(record));
                    let Some(child) = child else {
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
        reference: ManifestBlockReference,
        counters: &mut ManifestDiscoveryCounterSnapshot,
    ) -> Result<PhysicalRootRoutingBlock, ManifestLookupFailure> {
        let limit = self
            .access
            .transfer_limit()
            .get()
            .min(self.format.declaration().page_size().bytes());
        let bytes = self
            .artifacts
            .load_bounded(
                allocation,
                RecordArtifactFile::RootRoutingBlock {
                    generation: reference.generation(),
                    block: reference.block(),
                },
                limit,
            )
            .map_err(|failure| {
                counters.observe_failed_work(failure.work_trace());
                match failure.kind() {
                    crate::physical_runtime::record_serving::residency::frame_loading::FrameLoadFailureKind::Backend(reason) => ManifestLookupFailure::Backend(reason),
                    crate::physical_runtime::record_serving::residency::frame_loading::FrameLoadFailureKind::Residency(reason) => ManifestLookupFailure::Residency(reason),
                    kind => ManifestLookupFailure::Frame(kind),
                }
            })?;
        counters.observe_block(bytes.len(), bytes.work_trace());
        let checksum = durable_artifact_checksum(&bytes);
        let (block, found_format) =
            match PhysicalRootRoutingBlock::decode(&bytes, self.root.node_capacity()) {
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

    pub(in crate::physical_runtime::record_serving) const fn format_declaration(
        &self,
    ) -> worth_store_physical_format::PhysicalRecordFormatDeclaration {
        self.format.declaration()
    }
}
