use forge_store_physical_format::{
    OfflinePhysicalVerifier, PersistedPhysicalLayout, PhysicalBinaryEncodingWitness,
    PhysicalHeaderAuthority,
};

use super::{
    authority::semantic_fixture_digest, FixtureAuthorityReceipt, FixtureCapabilityDeclaration,
    FixtureMutationBoundarySet, FixtureScaleDeclaration, LargeStoreFixtureProfile,
    PersistedStoreFixtureManifest, PhysicalArtifactFixtureCatalog,
    ProductionBackedFixtureMaterialization, ProductionBackedFixtureSource,
    ProductionBackedPhysicalFixture, SyntheticFixtureAuthorityDenied,
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
        let (profile, scale, source, layout) = materialization.into_parts();
        FixtureNeedsBoundary {
            name: self.name,
            profile,
            scale,
            source,
            layout,
        }
    }
}

pub struct FixtureNeedsBoundary {
    name: String,
    profile: LargeStoreFixtureProfile,
    scale: FixtureScaleDeclaration,
    source: ProductionBackedFixtureSource,
    layout: PersistedPhysicalLayout,
}

impl FixtureNeedsBoundary {
    pub fn capability(self, declaration: FixtureCapabilityDeclaration) -> FixtureReadyBuilder {
        FixtureReadyBuilder {
            name: self.name,
            profile: self.profile,
            scale: self.scale,
            source: self.source,
            layout: self.layout,
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
    capability_declarations: Vec<FixtureCapabilityDeclaration>,
}

impl FixtureReadyBuilder {
    pub fn capability(mut self, declaration: FixtureCapabilityDeclaration) -> Self {
        self.capability_declarations.push(declaration);
        self
    }

    pub fn and_reopen_through_physical_authority(
        self,
    ) -> Result<ProductionBackedPhysicalFixture, SyntheticFixtureAuthorityDenied> {
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
        let manifest = PersistedStoreFixtureManifest::from_reopened_fixture(
            self.name,
            self.profile,
            self.scale,
            self.source,
            semantic_fixture_digest(&self.layout, &report),
            catalog,
            self.capability_declarations,
            boundaries,
        );
        Ok(ProductionBackedPhysicalFixture::from_manifest_and_receipt(
            manifest, receipt,
        ))
    }
}

fn canonical_offline_verifier() -> OfflinePhysicalVerifier {
    let encoding = PhysicalBinaryEncodingWitness::s1_canonical()
        .expect("physical certification requires canonical S.1 binary encoding");
    OfflinePhysicalVerifier::s1(PhysicalHeaderAuthority::s1(encoding))
}
