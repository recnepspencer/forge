use worth_store_physical_backend::{ArtifactTreeFailure, QualifiedFilesystemMedia};
use worth_store_physical_format::{
    durable_artifact_checksum, CurrentPhysicalRecordPlacement, DurablePhysicalRootManifest,
    ManifestBlockReference, PersistedRecordIdentity, PhysicalRootRoutingBlock, RecordArtifactFile,
};

use crate::physical_runtime::record_serving::{
    residency::serving_artifacts::ServingRecordArtifacts, AdmittedPhysicalRecordFormat,
    AdmittedRecordAccessPolicy,
};

use super::ManifestDiscoveryCounterSnapshot;

pub(in crate::physical_runtime::record_serving) struct ManifestReader<'media> {
    artifacts: ServingRecordArtifacts<'media>,
    format: AdmittedPhysicalRecordFormat,
    access: AdmittedRecordAccessPolicy,
    root: &'media DurablePhysicalRootManifest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime::record_serving) enum ManifestLookupFailure {
    Backend(ArtifactTreeFailure),
    Residency(worth_store_buffer_pool::PhysicalResidencyDenial),
    Damaged,
}

impl<'media> ManifestReader<'media> {
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
            artifacts: ServingRecordArtifacts::new(media, loader),
            format,
            access,
            root,
        }
    }

    pub(in crate::physical_runtime::record_serving) fn locate(
        &self,
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
            let block = self.read_block(reference, counters)?;
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
                RecordArtifactFile::RootRoutingBlock {
                    generation: reference.generation(),
                    block: reference.block(),
                },
                limit,
            )
            .map_err(|failure| match failure {
                crate::physical_runtime::record_serving::residency::frame_loading::FrameLoadFailure::Backend(reason) => ManifestLookupFailure::Backend(reason),
                crate::physical_runtime::record_serving::residency::frame_loading::FrameLoadFailure::Residency(reason) => ManifestLookupFailure::Residency(reason),
                _ => ManifestLookupFailure::Damaged,
            })?;
        counters.observe_block(bytes.len());
        let checksum = durable_artifact_checksum(&bytes);
        let (block, found_format) =
            PhysicalRootRoutingBlock::decode(&bytes, self.root.node_capacity())
                .map_err(|_| ManifestLookupFailure::Damaged)?;
        if found_format != self.format.declaration()
            || block.tree_identity() != self.root.tree_identity()
            || block.level() != reference.level()
            || block.reference(checksum) != reference
        {
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
