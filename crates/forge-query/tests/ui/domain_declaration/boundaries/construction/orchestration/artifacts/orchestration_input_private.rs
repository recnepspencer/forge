use forge_query::facade::{
    ForgeQueryAdmittedWorldBasis,
    ForgeQueryDeclarationEntryOrchestrationArtifactPolicy,
    ForgeQueryDeclarationEntryOrchestrationExposureLevel,
    ForgeQueryDeclarationEntryOrchestrationInput, ForgeQueryDomainEntryMarker,
};

#[derive(Clone, Copy, Eq, PartialEq)]
struct GeometryDomain;

impl ForgeQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str { "example.geometry.orchestration" }
    fn display_name(&self) -> &'static str { "GeometryOrchestrationDomain" }
    fn required_capability_families(&self) -> &'static [forge_query::facade::ForgeQueryCapabilityFamily] {
        &[]
    }
}

struct FakeInput;

impl forge_query::facade::ForgeQueryDeclarationInput<GeometryDomain> for FakeInput {
    type Family = FakeFamily;
    fn canonical_declaration_entries(
        &self,
    ) -> Vec<forge_query::facade::ForgeQueryDeclarationCanonicalEntry> {
        Vec::new()
    }
}

struct FakeFamily;

impl forge_query::facade::ForgeQueryDeclarationFamilyMarker<GeometryDomain> for FakeFamily {
    type PrimaryAuthority = forge_query::facade::ForgeQueryRelationalTruthAuthority;
    type SignalCompatibility = forge_query::facade::ForgeQuerySignalNotCompatiblePosture;
    type GroupedPosture = forge_query::facade::ForgeQuerySingleOnlyGrouping;
    fn semantic_family_key() -> &'static str { "fake" }
    fn legality_contract() -> forge_query::facade::ForgeQueryDeclarationLegalityContract {
        forge_query::facade::ForgeQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

fn main() {
    let _ = ForgeQueryDeclarationEntryOrchestrationInput::<GeometryDomain, FakeInput> {
        declaration_family_key: "fake",
        world_basis: unsafe { std::mem::zeroed::<ForgeQueryAdmittedWorldBasis>() },
        exposure_level: ForgeQueryDeclarationEntryOrchestrationExposureLevel::Checked,
        artifact_policy: ForgeQueryDeclarationEntryOrchestrationArtifactPolicy::CheckedOutcomeOnly,
        _marker: std::marker::PhantomData,
    };
}
