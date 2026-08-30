use worth_store_buffer_pool::PhysicalFrameLease;
use worth_store_physical_format::{
    store_namespace::StableStoreIdentity, DurablePhysicalRootManifest,
    PhysicalRecordFormatDeclaration, RecordArtifactFile,
};
use worth_store_physical_integrity::{
    validate_root_manifest, IntegrityValidatedRootManifest, PhysicalArtifactScope,
    PhysicalByteRange, RootManifestIntegrityValidation, UntrustedPhysicalArtifact,
};

use super::ResidentIntegrityRecordBinding;
use crate::physical_runtime::{LifecycleGeneration, RootProtocolAdmissionDenial};

pub(in crate::physical_runtime) struct IntegrityAdmittedResidentRootManifest<'frame> {
    source: ResidentIntegrityRecordBinding<'frame>,
    projection: AdmittedRootManifestProjection,
}

struct BoundResidentRootManifestSource<'frame> {
    lease: &'frame PhysicalFrameLease,
    lifecycle: LifecycleGeneration,
    scope: PhysicalArtifactScope,
}

#[derive(Clone, Copy)]
struct AdmittedRootManifestProjection {
    generation: u64,
    tree_identity: u64,
    node_capacity: u16,
    free_space_checksum: u32,
    record_count: u64,
    next_block: u64,
    next_segment_block: u64,
    routing_root: Option<worth_store_physical_format::ManifestBlockReference>,
    segment_root: Option<worth_store_physical_format::SegmentManifestBlockReference>,
    free_space_root: Option<worth_store_physical_format::FreeSpaceBlockReference>,
    last_inline_record: Option<worth_store_physical_format::PersistedRecordIdentity>,
    last_inline_segment: Option<worth_store_physical_format::SegmentGenerationCell>,
}

pub(in crate::physical_runtime) fn admit_loaded_root_manifest(
    lease: &PhysicalFrameLease,
    lifecycle: LifecycleGeneration,
    store: StableStoreIdentity,
    format: PhysicalRecordFormatDeclaration,
    generation: u64,
) -> Result<IntegrityAdmittedResidentRootManifest<'_>, RootProtocolAdmissionDenial> {
    let source =
        BoundResidentRootManifestSource::bind(lease, lifecycle, store, format, generation)?;
    let input = UntrustedPhysicalArtifact::from_bounded_bytes(lease);
    let validated = match validate_root_manifest(input, source.scope).0 {
        RootManifestIntegrityValidation::Intact(validated) => validated,
        RootManifestIntegrityValidation::Rejected(rejection) => {
            return Err(RootProtocolAdmissionDenial::from_validation(rejection));
        }
    };
    source.admit(input, validated)
}

impl<'frame> BoundResidentRootManifestSource<'frame> {
    fn bind(
        lease: &'frame PhysicalFrameLease,
        lifecycle: LifecycleGeneration,
        store: StableStoreIdentity,
        format: PhysicalRecordFormatDeclaration,
        generation: u64,
    ) -> Result<Self, RootProtocolAdmissionDenial> {
        let coordinate = lease.key().coordinate();
        let expected = RecordArtifactFile::RootManifest { generation };
        if coordinate.artifact() != expected || lease.key().store() != store {
            return Err(RootProtocolAdmissionDenial::SourceArtifactMismatch);
        }
        let range = PhysicalByteRange::new(coordinate.offset(), u64::from(coordinate.length()))
            .map_err(|_| RootProtocolAdmissionDenial::SourceRangeMismatch)?;
        let scope = PhysicalArtifactScope::root_manifest(store, format, generation, range)
            .map_err(|_| RootProtocolAdmissionDenial::SourceArtifactMismatch)?;
        Ok(Self {
            lease,
            lifecycle,
            scope,
        })
    }

    fn admit(
        self,
        input: UntrustedPhysicalArtifact<'frame>,
        validated: IntegrityValidatedRootManifest<'frame>,
    ) -> Result<IntegrityAdmittedResidentRootManifest<'frame>, RootProtocolAdmissionDenial> {
        if !validated.matches_input(input) {
            return Err(RootProtocolAdmissionDenial::SourceIncarnationMismatch);
        }
        let projection = AdmittedRootManifestProjection::from_validated(&validated);
        let source = ResidentIntegrityRecordBinding::bind_root_manifest(
            self.lease,
            self.lifecycle,
            validated,
        )
        .map_err(|_| RootProtocolAdmissionDenial::ResidentFrame)?;
        Ok(IntegrityAdmittedResidentRootManifest { source, projection })
    }
}

impl IntegrityAdmittedResidentRootManifest<'_> {
    pub(in crate::physical_runtime) fn project(
        self,
    ) -> Result<DurablePhysicalRootManifest, RootProtocolAdmissionDenial> {
        let _exact_resident_incarnation = (
            self.source.lifecycle_generation(),
            self.source.frame_generation(),
            self.source.scope(),
        );
        self.projection.project()
    }
}

impl AdmittedRootManifestProjection {
    fn from_validated(validated: &IntegrityValidatedRootManifest<'_>) -> Self {
        Self {
            generation: validated.root_generation(),
            tree_identity: validated.tree_identity(),
            node_capacity: validated.node_capacity(),
            free_space_checksum: validated.free_space_checksum(),
            record_count: validated.record_count(),
            next_block: validated.next_block(),
            next_segment_block: validated.next_segment_block(),
            routing_root: validated.routing_root(),
            segment_root: validated.segment_root(),
            free_space_root: validated.free_space_root(),
            last_inline_record: validated.last_inline_record(),
            last_inline_segment: validated.last_inline_segment(),
        }
    }

    fn project(self) -> Result<DurablePhysicalRootManifest, RootProtocolAdmissionDenial> {
        DurablePhysicalRootManifest::builder(
            self.generation,
            self.tree_identity,
            self.node_capacity,
            self.free_space_checksum,
        )
        .record_count(self.record_count)
        .next_block(self.next_block)
        .next_segment_block(self.next_segment_block)
        .routing_root(self.routing_root)
        .segment_root(self.segment_root)
        .free_space_root(self.free_space_root)
        .last_inline_record(self.last_inline_record)
        .last_inline_segment(self.last_inline_segment)
        .admit()
        .ok_or(RootProtocolAdmissionDenial::OwnerProjectionRejected)
    }
}
