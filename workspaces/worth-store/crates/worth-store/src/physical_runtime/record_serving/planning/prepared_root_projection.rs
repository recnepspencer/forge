use std::collections::BTreeMap;
use std::num::NonZeroU64;

use worth_store_physical_format::{
    CurrentPhysicalRecordPlacement, DurablePhysicalRootManifest, PersistedRecordIdentity,
    RecordArtifactFile, RecordSegmentPageManifestEntry, SegmentGenerationCell, SegmentPageKey,
};

use super::inline_segment_plan::InlineSegmentAllocation;
use crate::physical_runtime::record_serving::publication::append_observation::PublicationObservation;

/// Record-serving meaning required to project one successor physical root.
///
/// Durability progression carries this value opaquely. Only record-serving
/// planning may interpret it when the settled group reaches root cutover.
pub(in crate::physical_runtime) struct PreparedPhysicalRootProjection {
    pub(in crate::physical_runtime::record_serving) root_publication_allocation_bytes: NonZeroU64,
    pub(in crate::physical_runtime::record_serving) source_root: DurablePhysicalRootManifest,
    pub(in crate::physical_runtime::record_serving) manifest_capacity_transition:
        crate::physical_runtime::PhysicalManifestCapacityTransition,
    pub(in crate::physical_runtime::record_serving) placement:
        crate::physical_runtime::record_serving::AdmittedRecordPlacementPolicy,
    pub(in crate::physical_runtime::record_serving) records: Vec<PersistedRecordIdentity>,
    pub(in crate::physical_runtime::record_serving) payload_manifests:
        Vec<(RecordArtifactFile, Vec<u8>)>,
    pub(in crate::physical_runtime::record_serving) placements:
        BTreeMap<PersistedRecordIdentity, CurrentPhysicalRecordPlacement>,
    pub(in crate::physical_runtime::record_serving) segment_updates:
        BTreeMap<SegmentPageKey, RecordSegmentPageManifestEntry>,
    pub(in crate::physical_runtime::record_serving) inline_allocations:
        Vec<InlineSegmentAllocation>,
    pub(in crate::physical_runtime::record_serving) last_inline_record:
        Option<PersistedRecordIdentity>,
    pub(in crate::physical_runtime::record_serving) last_inline_segment:
        Option<SegmentGenerationCell>,
    pub(in crate::physical_runtime::record_serving) observation: PublicationObservation,
}

impl PreparedPhysicalRootProjection {
    pub(in crate::physical_runtime) const fn source_root_generation(&self) -> u64 {
        self.source_root.generation()
    }

    pub(in crate::physical_runtime) const fn manifest_capacity_transition(
        &self,
    ) -> crate::physical_runtime::PhysicalManifestCapacityTransition {
        self.manifest_capacity_transition
    }

    pub(in crate::physical_runtime) const fn recovery_manifest_capacity(&self) -> u16 {
        self.placement.manifest_capacity().get()
    }

    pub(in crate::physical_runtime) fn recovery_placements(
        &self,
    ) -> impl ExactSizeIterator<Item = CurrentPhysicalRecordPlacement> + '_ {
        self.placements.values().copied()
    }

    pub(in crate::physical_runtime) fn recovery_record_identities(
        &self,
    ) -> impl ExactSizeIterator<Item = PersistedRecordIdentity> + '_ {
        self.records.iter().copied()
    }

    pub(in crate::physical_runtime) fn recovery_segment_updates(
        &self,
    ) -> impl ExactSizeIterator<Item = RecordSegmentPageManifestEntry> + '_ {
        self.segment_updates.values().copied()
    }

    pub(in crate::physical_runtime) fn recovery_payload_manifests(
        &self,
    ) -> impl ExactSizeIterator<Item = &(RecordArtifactFile, Vec<u8>)> {
        self.payload_manifests.iter()
    }

    pub(in crate::physical_runtime) const fn root_publication_allocation_bytes(
        &self,
    ) -> NonZeroU64 {
        self.root_publication_allocation_bytes
    }

    pub(in crate::physical_runtime) fn recovery_inline_allocations(
        &self,
    ) -> impl ExactSizeIterator<Item = super::inline_segment_plan::InlineSegmentAllocation> + '_
    {
        self.inline_allocations.iter().copied()
    }

    pub(in crate::physical_runtime) const fn recovery_last_inline_record(
        &self,
    ) -> Option<PersistedRecordIdentity> {
        self.last_inline_record
    }

    pub(in crate::physical_runtime) const fn recovery_last_inline_segment(
        &self,
    ) -> Option<SegmentGenerationCell> {
        self.last_inline_segment
    }

    pub(in crate::physical_runtime::record_serving) fn completion_projection(
        &self,
    ) -> crate::physical_runtime::record_serving::PreparedRecordCompletionProjection {
        crate::physical_runtime::record_serving::PreparedRecordCompletionProjection::new(
            &self.records,
            self.observation,
        )
    }

    pub(in crate::physical_runtime::record_serving) fn settle_data_observation(
        &mut self,
        effect_count: usize,
    ) {
        self.observation.settle_data_effects(effect_count);
    }

    pub(in crate::physical_runtime::record_serving) fn into_payload_plan(
        self,
    ) -> super::prepared_payload::PreparedRecordPayloadPlan {
        super::prepared_payload::PreparedRecordPayloadPlan {
            source_root: self.source_root,
            manifest_capacity_transition: self.manifest_capacity_transition,
            placement: self.placement,
            records: self.records,
            data: Vec::new(),
            payload_manifests: self.payload_manifests,
            placements: self.placements,
            segment_updates: self.segment_updates,
            inline_allocations: self.inline_allocations,
            last_inline_record: self.last_inline_record,
            last_inline_segment: self.last_inline_segment,
            observation: self.observation,
        }
    }
}
