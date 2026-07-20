use super::{
    FixtureCapabilityDeclaration, FixtureMutationBoundarySet, FixtureScaleDeclaration,
    LargeStoreFixtureProfile, MaterializedFixtureScaleEvidence, PhysicalArtifactFixtureCatalog,
    ProductionBackedFixtureSource,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedStoreFixtureManifest {
    name: String,
    profile: LargeStoreFixtureProfile,
    scale: FixtureScaleDeclaration,
    source: ProductionBackedFixtureSource,
    semantic_digest: String,
    artifact_catalog: PhysicalArtifactFixtureCatalog,
    capability_declarations: Vec<FixtureCapabilityDeclaration>,
    mutation_boundaries: FixtureMutationBoundarySet,
    materialized_scale: Option<MaterializedFixtureScaleEvidence>,
}

pub(crate) struct ReopenedFixtureManifestParts {
    pub(crate) name: String,
    pub(crate) profile: LargeStoreFixtureProfile,
    pub(crate) scale: FixtureScaleDeclaration,
    pub(crate) source: ProductionBackedFixtureSource,
    pub(crate) semantic_digest: String,
    pub(crate) artifact_catalog: PhysicalArtifactFixtureCatalog,
    pub(crate) capability_declarations: Vec<FixtureCapabilityDeclaration>,
    pub(crate) mutation_boundaries: FixtureMutationBoundarySet,
    pub(crate) materialized_scale: Option<MaterializedFixtureScaleEvidence>,
}

impl PersistedStoreFixtureManifest {
    pub(crate) fn from_reopened_fixture(parts: ReopenedFixtureManifestParts) -> Self {
        Self {
            name: parts.name,
            profile: parts.profile,
            scale: parts.scale,
            source: parts.source,
            semantic_digest: parts.semantic_digest,
            artifact_catalog: parts.artifact_catalog,
            capability_declarations: parts.capability_declarations,
            mutation_boundaries: parts.mutation_boundaries,
            materialized_scale: parts.materialized_scale,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn profile(&self) -> LargeStoreFixtureProfile {
        self.profile
    }

    pub const fn scale(&self) -> FixtureScaleDeclaration {
        self.scale
    }

    pub const fn source(&self) -> ProductionBackedFixtureSource {
        self.source
    }

    pub fn semantic_digest(&self) -> &str {
        &self.semantic_digest
    }

    pub const fn artifact_catalog(&self) -> &PhysicalArtifactFixtureCatalog {
        &self.artifact_catalog
    }

    pub fn capability_declarations(&self) -> &[FixtureCapabilityDeclaration] {
        &self.capability_declarations
    }

    pub const fn mutation_boundaries(&self) -> &FixtureMutationBoundarySet {
        &self.mutation_boundaries
    }

    pub const fn materialized_scale(&self) -> Option<MaterializedFixtureScaleEvidence> {
        self.materialized_scale
    }

    pub fn evidence_identity(&self) -> [u8; 32] {
        use sha2::{Digest, Sha256};
        let mut digest = Sha256::new();
        digest.update(b"worth-store-physical-fixture-manifest-v1");
        update_text(&mut digest, &self.name);
        digest.update([profile_tag(self.profile)]);
        let scale = self.scale;
        for value in [
            scale.declared_store_bytes(),
            scale.resident_memory_budget_bytes(),
            scale.foreground_io_bytes(),
            scale.background_io_bytes(),
            scale.blob_bytes(),
            scale.wal_tail_bytes(),
            scale.damaged_region_bytes(),
        ] {
            digest.update(value.to_be_bytes());
        }
        digest.update(scale.checkpoint_count().to_be_bytes());
        digest.update(scale.compaction_run_count().to_be_bytes());
        digest.update([profile_tag(scale.profile())]);
        digest.update([non_claim_tag(scale.non_claim())]);
        digest.update(self.source.root_reference().to_be_bytes());
        update_text(&mut digest, &self.semantic_digest);
        let catalog = &self.artifact_catalog;
        for value in [
            catalog.root_manifest_candidates(),
            catalog.persisted_pages(),
            catalog.persisted_extents(),
            catalog.discovered_references(),
            catalog.page_slots(),
            catalog.extents(),
            catalog.free_space_entries(),
        ] {
            digest.update(value.to_be_bytes());
        }
        for value in [
            catalog.segment_manifest_bytes(),
            catalog.extent_manifest_bytes(),
            catalog.free_space_map_bytes(),
        ] {
            digest.update(value.to_be_bytes());
        }
        digest.update((self.capability_declarations.len() as u64).to_be_bytes());
        for capability in &self.capability_declarations {
            digest.update([mutation_boundary_tag(capability.mutation_boundary())]);
        }
        digest.update((self.mutation_boundaries.len() as u64).to_be_bytes());
        for boundary in self.mutation_boundaries.iter() {
            digest.update([mutation_boundary_tag(boundary)]);
        }
        match self.materialized_scale {
            Some(evidence) => {
                digest.update([1]);
                digest.update(evidence.evidence_identity());
            }
            None => digest.update([0]),
        }
        digest.finalize().into()
    }
}

const fn non_claim_tag(non_claim: Option<super::FixtureProfileNonClaim>) -> u8 {
    match non_claim {
        None => 0,
        Some(super::FixtureProfileNonClaim::BlobCorrectnessNotCertified) => 1,
    }
}

const fn mutation_boundary_tag(boundary: super::FixtureMutationBoundary) -> u8 {
    use super::FixtureMutationBoundary as Boundary;
    match boundary {
        Boundary::PageImage => 1,
        Boundary::FrameBody => 2,
        Boundary::WalFrame => 3,
        Boundary::Manifest => 4,
        Boundary::Index => 5,
        Boundary::Chunk => 6,
        Boundary::AuditRecord => 7,
        Boundary::KeyEnvelope => 8,
        Boundary::TenantMetadata => 9,
        Boundary::RepairArtifact => 10,
    }
}

fn update_text(digest: &mut sha2::Sha256, value: &str) {
    use sha2::Digest;
    digest.update((value.len() as u64).to_be_bytes());
    digest.update(value.as_bytes());
}

const fn profile_tag(profile: LargeStoreFixtureProfile) -> u8 {
    match profile {
        LargeStoreFixtureProfile::StoreLargerThanMemory => 1,
        LargeStoreFixtureProfile::CheckpointHeavy => 2,
        LargeStoreFixtureProfile::CompactionHeavy => 3,
        LargeStoreFixtureProfile::ForegroundUnderBackgroundIo => 4,
        LargeStoreFixtureProfile::BlobLargerThanMemoryReadiness => 5,
        LargeStoreFixtureProfile::OperationalRecoveryRelease => 6,
    }
}
