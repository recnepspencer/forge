use worth_store_physical_format::{
    OfflinePhysicalVerifier, PersistedPhysicalLayout, PhysicalBinaryEncodingWitness,
    PhysicalHeaderAuthority,
};

use super::{
    authority::semantic_fixture_digest, FixtureAuthorityReceipt, FixtureCapabilityDeclaration,
    FixtureMutationBoundarySet, FixtureScaleDeclaration, LargeStoreFixtureProfile,
    MaterializedFixtureScaleEvidence, PersistedStoreFixtureManifest,
    PhysicalArtifactFixtureCatalog, ProductionBackedFixtureMaterialization,
    ProductionBackedFixtureSource, ProductionBackedPhysicalFixture, ReopenedFixtureManifestParts,
    SyntheticFixtureAuthorityDenied,
};

pub struct PhysicalFixtureBuilder;

impl PhysicalFixtureBuilder {
    pub fn production_backed(name: impl Into<String>) -> FixtureNeedsMaterialization {
        FixtureNeedsMaterialization { name: name.into() }
    }
}

pub struct FixtureNeedsMaterialization {
    name: String,
}

impl FixtureNeedsMaterialization {
    pub fn materialize_with(
        self,
        materialization: ProductionBackedFixtureMaterialization,
    ) -> FixtureNeedsBoundary {
        let (profile, scale, source, layout, replay_artifact, materialized_scale) =
            materialization.into_parts();
        FixtureNeedsBoundary {
            name: self.name,
            profile,
            scale,
            source,
            layout,
            replay_artifact,
            materialized_scale,
        }
    }
}

pub struct FixtureNeedsBoundary {
    name: String,
    profile: LargeStoreFixtureProfile,
    scale: FixtureScaleDeclaration,
    source: ProductionBackedFixtureSource,
    layout: PersistedPhysicalLayout,
    replay_artifact: Option<worth_store_physical_format::InMemoryPhysicalFormatReplayArtifact>,
    materialized_scale: Option<MaterializedFixtureScaleEvidence>,
}

impl FixtureNeedsBoundary {
    pub fn capability(self, declaration: FixtureCapabilityDeclaration) -> FixtureReadyBuilder {
        FixtureReadyBuilder {
            name: self.name,
            profile: self.profile,
            scale: self.scale,
            source: self.source,
            layout: self.layout,
            replay_artifact: self.replay_artifact,
            materialized_scale: self.materialized_scale,
            capability_declarations: vec![declaration],
        }
    }
}

pub struct FixtureReadyBuilder {
    name: String,
    profile: LargeStoreFixtureProfile,
    scale: FixtureScaleDeclaration,
    source: ProductionBackedFixtureSource,
    layout: PersistedPhysicalLayout,
    replay_artifact: Option<worth_store_physical_format::InMemoryPhysicalFormatReplayArtifact>,
    materialized_scale: Option<MaterializedFixtureScaleEvidence>,
    capability_declarations: Vec<FixtureCapabilityDeclaration>,
}

impl FixtureReadyBuilder {
    pub fn capability(mut self, declaration: FixtureCapabilityDeclaration) -> Self {
        self.capability_declarations.push(declaration);
        self
    }

    pub fn and_reopen_through_physical_authority(
        mut self,
    ) -> Result<ProductionBackedPhysicalFixture, SyntheticFixtureAuthorityDenied> {
        self.capability_declarations
            .sort_unstable_by_key(FixtureCapabilityDeclaration::mutation_boundary);
        self.capability_declarations.dedup();
        let report = canonical_offline_verifier().verify(&self.layout)?;
        let receipt = FixtureAuthorityReceipt::from_reopened_layout(
            self.profile,
            self.source,
            &self.layout,
            &report,
        );
        let boundaries =
            FixtureMutationBoundarySet::from_capabilities(&self.capability_declarations)
                .expect("FixtureReadyBuilder always contains at least one boundary");
        let catalog = PhysicalArtifactFixtureCatalog::from_reopened_layout(&self.layout, &report);
        let manifest =
            PersistedStoreFixtureManifest::from_reopened_fixture(ReopenedFixtureManifestParts {
                name: self.name,
                profile: self.profile,
                scale: self.scale,
                source: self.source,
                semantic_digest: semantic_fixture_digest(&self.layout, &report),
                artifact_catalog: catalog,
                capability_declarations: self.capability_declarations,
                mutation_boundaries: boundaries,
                materialized_scale: self.materialized_scale,
            });
        Ok(ProductionBackedPhysicalFixture::from_manifest_and_receipt(
            manifest,
            receipt,
            self.layout,
            self.replay_artifact,
        ))
    }
}

fn canonical_offline_verifier() -> OfflinePhysicalVerifier {
    let encoding = PhysicalBinaryEncodingWitness::physical_format_canonical()
        .expect("physical certification requires canonical S.1 binary encoding");
    OfflinePhysicalVerifier::for_canonical_physical_format(
        PhysicalHeaderAuthority::for_canonical_physical_format(encoding),
    )
}
