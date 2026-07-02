use forge_proof::{
    AssumptionBasis, AuthorityMarker, AuthorityProves, AuthorityWitness, CurrentValidity,
    FreshnessScopedBasis, Proof, ProofMarker, Recipe, Resolved,
};
use forge_store_physical_format::{MinimalManifestVerifierReport, PersistedPhysicalLayout};

use super::{FixtureScaleDeclaration, LargeStoreFixtureProfile, ProductionBackedFixtureSource};

#[derive(Debug, PartialEq, Eq)]
pub struct FixtureConstructionAuthority {
    _private: (),
}

impl AuthorityMarker for FixtureConstructionAuthority {}

#[derive(Debug, PartialEq, Eq)]
pub struct StoreFixtureAuthority {
    _private: (),
}

impl AuthorityMarker for StoreFixtureAuthority {}
impl AuthorityProves<FixtureProvenance> for StoreFixtureAuthority {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FixtureProvenance;

impl ProofMarker for FixtureProvenance {}

pub type FixtureConstructionProofBasis =
    FreshnessScopedBasis<CurrentValidity, AssumptionBasis<FixtureConstructionBasis>>;
pub type ResolvedFixtureConstructionRecipe =
    Recipe<Resolved, FixtureConstructionBasis, FixtureConstructionProofBasis>;

#[derive(Debug, PartialEq, Eq)]
pub struct FixtureAuthorityReceipt {
    profile: LargeStoreFixtureProfile,
    scale: FixtureScaleDeclaration,
    source: ProductionBackedFixtureSource,
    semantic_digest: String,
    reopened_root_count: u32,
    reopened_reference_count: u32,
    construction_proof: ResolvedFixtureConstructionRecipe,
    store_fixture_proof: Proof<FixtureProvenance, StoreFixtureAuthority>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixtureConstructionBasis {
    profile: LargeStoreFixtureProfile,
    source: ProductionBackedFixtureSource,
    semantic_digest: String,
}

impl FixtureAuthorityReceipt {
    pub(crate) fn from_reopened_layout(
        profile: LargeStoreFixtureProfile,
        source: ProductionBackedFixtureSource,
        layout: &PersistedPhysicalLayout,
        report: &MinimalManifestVerifierReport,
    ) -> Self {
        let semantic_digest = semantic_fixture_digest(layout, report);
        let construction_basis = FixtureConstructionBasis {
            profile,
            source,
            semantic_digest: semantic_digest.clone(),
        };
        Self {
            profile,
            scale: profile.scale_declaration(),
            source,
            semantic_digest,
            reopened_root_count: report.traversal().root_count(),
            reopened_reference_count: report.layout().discovered_references().len() as u32,
            construction_proof: Recipe::new(construction_basis.clone())
                .resolve_with_authority(construction_basis, fixture_construction_authority()),
            store_fixture_proof: Proof::from_authority_witness(&store_fixture_authority()),
        }
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

    pub const fn reopened_root_count(&self) -> u32 {
        self.reopened_root_count
    }

    pub const fn reopened_reference_count(&self) -> u32 {
        self.reopened_reference_count
    }

    pub const fn reopened_through_physical_authority(&self) -> bool {
        self.reopened_root_count == 1 && self.reopened_reference_count > 0
    }

    pub const fn construction_proof(&self) -> &ResolvedFixtureConstructionRecipe {
        &self.construction_proof
    }

    pub const fn store_fixture_proof(&self) -> &Proof<FixtureProvenance, StoreFixtureAuthority> {
        &self.store_fixture_proof
    }
}

impl FixtureConstructionBasis {
    pub const fn profile(&self) -> LargeStoreFixtureProfile {
        self.profile
    }

    pub const fn source(&self) -> ProductionBackedFixtureSource {
        self.source
    }

    pub fn semantic_digest(&self) -> &str {
        &self.semantic_digest
    }
}

fn fixture_construction_authority() -> AuthorityWitness<FixtureConstructionAuthority> {
    AuthorityWitness::from_authority_marker(FixtureConstructionAuthority { _private: () })
}

fn store_fixture_authority() -> AuthorityWitness<StoreFixtureAuthority> {
    AuthorityWitness::from_authority_marker(StoreFixtureAuthority { _private: () })
}

pub(crate) fn semantic_fixture_digest(
    layout: &PersistedPhysicalLayout,
    report: &MinimalManifestVerifierReport,
) -> String {
    let mut state = Fnv64::new();
    state.write_usize(layout.root_manifest_candidates().len());
    for root in layout.root_manifest_candidates() {
        state.write_bytes(root);
    }
    state.write_bytes(layout.segment_manifest());
    state.write_bytes(layout.extent_manifest());
    state.write_bytes(layout.free_space_map());
    state.write_usize(layout.pages().len());
    for page in layout.pages() {
        state.write_u64(page.cell().generation().get());
        state.write_bytes(page.bytes());
    }
    state.write_usize(layout.extents().len());
    for extent in layout.extents() {
        state.write_u64(extent.cell().generation().get());
        state.write_bytes(extent.bytes());
    }
    state.write_u64(report.traversal().root_count() as u64);
    state.write_u64(report.traversal().segment_count() as u64);
    state.write_u64(report.traversal().page_slot_count() as u64);
    state.write_u64(report.traversal().extent_count() as u64);
    state.write_u64(report.traversal().free_space_count() as u64);
    format!("{:016x}", state.finish())
}

struct Fnv64 {
    value: u64,
}

impl Fnv64 {
    const fn new() -> Self {
        Self {
            value: 0xcbf29ce484222325,
        }
    }

    fn write_bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.value ^= u64::from(*byte);
            self.value = self.value.wrapping_mul(0x100000001b3);
        }
    }

    fn write_u64(&mut self, value: u64) {
        self.write_bytes(&value.to_le_bytes());
    }

    fn write_usize(&mut self, value: usize) {
        self.write_u64(value as u64);
    }

    const fn finish(&self) -> u64 {
        self.value
    }
}
