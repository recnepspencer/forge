use worth_store_buffer_pool::PhysicalFrameLease;
use worth_store_physical_format::{
    store_namespace::StableStoreIdentity, DurablePhysicalRootManifest,
    PhysicalRecordFormatDeclaration, RecordArtifactFile,
};
use worth_store_physical_integrity::{
    validate_root_manifest, IntegrityValidatedRootManifest, PhysicalArtifactScope,
    PhysicalByteRange, RootManifestIntegrityValidation, UntrustedPhysicalArtifact,
};

use super::{
    denial::ResidentIntegrityAdmissionDenial, load::ResidentAdmissionContext,
    record_binding::ResidentIntegrityRecordBinding,
};
use crate::physical_runtime::{LifecycleGeneration, RootProtocolAdmissionDenial};

pub(in crate::physical_runtime) struct IntegrityAdmittedResidentRootManifest<'frame> {
    source: ResidentIntegrityRecordBinding<'frame>,
    projection: Option<AdmittedRootManifestProjection>,
}

pub(in crate::physical_runtime) struct IntegrityAdmittedResidentRootManifestView<'frame> {
    lease: &'frame PhysicalFrameLease,
    scope: PhysicalArtifactScope,
}

struct BoundResidentRootManifestSource<'frame> {
    lease: &'frame PhysicalFrameLease,
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

pub(in crate::physical_runtime) fn admit_loaded_root_manifest<'frame>(
    lease: &'frame PhysicalFrameLease,
    lifecycle: LifecycleGeneration,
    store: StableStoreIdentity,
    format: PhysicalRecordFormatDeclaration,
    generation: u64,
    counters: &crate::physical_runtime::ResidentAdmissionCounterCells,
) -> Result<IntegrityAdmittedResidentRootManifest<'frame>, RootProtocolAdmissionDenial> {
    let context = ResidentAdmissionContext::new(lifecycle, counters);
    let source = match BoundResidentRootManifestSource::bind(lease, store, format, generation) {
        Ok(source) => source,
        Err(denial) => {
            counters.observe_rejection_before_decoder();
            return Err(denial);
        }
    };
    if let Some(source) = context
        .reuse(lease, source.scope)
        .map_err(map_resident_denial)?
    {
        return Ok(IntegrityAdmittedResidentRootManifest {
            source,
            projection: None,
        });
    }
    let input = context
        .exact_input(lease, source.scope)
        .map_err(map_resident_denial)?;
    context.observe_fresh_validation();
    let validated = match validate_root_manifest(input, source.scope).0 {
        RootManifestIntegrityValidation::Intact(validated) => validated,
        RootManifestIntegrityValidation::Rejected(rejection) => {
            context
                .validation_rejected::<()>(rejection)
                .expect_err("rejected validation records a denial");
            return Err(RootProtocolAdmissionDenial::from_validation(rejection));
        }
    };
    source.admit(input, validated, context)
}

impl<'frame> BoundResidentRootManifestSource<'frame> {
    fn bind(
        lease: &'frame PhysicalFrameLease,
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
        Ok(Self { lease, scope })
    }

    fn admit(
        self,
        input: UntrustedPhysicalArtifact<'frame>,
        validated: IntegrityValidatedRootManifest<'frame>,
        context: ResidentAdmissionContext<'_>,
    ) -> Result<IntegrityAdmittedResidentRootManifest<'frame>, RootProtocolAdmissionDenial> {
        if !validated.matches_input(input) {
            return Err(RootProtocolAdmissionDenial::SourceIncarnationMismatch);
        }
        let projection = AdmittedRootManifestProjection::from_validated(&validated);
        let source = context
            .bind_validated(self.lease, self.scope, validated.into_validation_record())
            .map_err(map_resident_denial)?;
        Ok(IntegrityAdmittedResidentRootManifest {
            source,
            projection: Some(projection),
        })
    }
}

impl<'frame> IntegrityAdmittedResidentRootManifest<'frame> {
    pub(in crate::physical_runtime) fn enter_owner_decoder(
        self,
        current_lifecycle: LifecycleGeneration,
        counters: &crate::physical_runtime::ResidentAdmissionCounterCells,
    ) -> Result<IntegrityAdmittedResidentRootManifestView<'frame>, RootProtocolAdmissionDenial>
    {
        let context = ResidentAdmissionContext::new(current_lifecycle, counters);
        let scope = self.source.scope();
        let lease = context
            .enter_owner_decoder(self.source)
            .map_err(map_resident_denial)?;
        Ok(IntegrityAdmittedResidentRootManifestView { lease, scope })
    }

    pub(in crate::physical_runtime) fn project(
        self,
        current_lifecycle: LifecycleGeneration,
        counters: &crate::physical_runtime::ResidentAdmissionCounterCells,
    ) -> Result<DurablePhysicalRootManifest, RootProtocolAdmissionDenial> {
        let projection = self
            .projection
            .ok_or(RootProtocolAdmissionDenial::OwnerProjectionRejected)?;
        let context = ResidentAdmissionContext::new(current_lifecycle, counters);
        context
            .enter_owner_decoder(self.source)
            .map_err(map_resident_denial)?;
        projection.project()
    }
}

impl IntegrityAdmittedResidentRootManifestView<'_> {
    pub(in crate::physical_runtime) fn bytes(&self) -> &[u8] {
        self.lease
    }

    pub(in crate::physical_runtime) const fn scope(&self) -> PhysicalArtifactScope {
        self.scope
    }
}

fn map_resident_denial(_: ResidentIntegrityAdmissionDenial) -> RootProtocolAdmissionDenial {
    RootProtocolAdmissionDenial::ResidentFrame
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
