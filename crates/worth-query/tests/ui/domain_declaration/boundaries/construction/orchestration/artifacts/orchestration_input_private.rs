use worth_query::facade::foundation::{WorthQueryAdmittedWorldBasis, WorthQueryDeclarationEntryOrchestrationArtifactPolicy, WorthQueryDeclarationEntryOrchestrationExposureLevel, WorthQueryDeclarationEntryOrchestrationInput, WorthQueryDomainEntryMarker};

#[derive(Clone, Copy, Eq, PartialEq)]
struct GeometryDomain;

impl WorthQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str { "example.geometry.orchestration" }
    fn display_name(&self) -> &'static str { "GeometryOrchestrationDomain" }
    fn required_capability_families(&self) -> &'static [worth_query::facade::foundation::WorthQueryCapabilityFamily] {
        &[]
    }
}

struct FakeInput;

impl worth_query::facade::foundation::WorthQueryDeclarationInput<GeometryDomain> for FakeInput {
    type Family = FakeFamily;
    fn canonical_declaration_entries(
        &self,
    ) -> Vec<worth_query::facade::foundation::WorthQueryDeclarationCanonicalEntry> {
        Vec::new()
    }
}

struct FakeFamily;

impl worth_query::facade::foundation::WorthQueryDeclarationFamilyMarker<GeometryDomain> for FakeFamily {
    type PrimaryAuthority = worth_query::facade::foundation::WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = worth_query::facade::foundation::WorthQuerySignalNotCompatiblePosture;
    type GroupedPosture = worth_query::facade::foundation::WorthQuerySingleOnlyGrouping;
    fn semantic_family_key() -> &'static str { "fake" }
    fn legality_contract() -> worth_query::facade::foundation::WorthQueryDeclarationLegalityContract {
        worth_query::facade::foundation::WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

fn main() {
    let _ = WorthQueryDeclarationEntryOrchestrationInput::<GeometryDomain, FakeInput> {
        declaration_family_key: "fake",
        world_basis: unsafe { std::mem::zeroed::<WorthQueryAdmittedWorldBasis>() },
        exposure_level: WorthQueryDeclarationEntryOrchestrationExposureLevel::Checked,
        artifact_policy: WorthQueryDeclarationEntryOrchestrationArtifactPolicy::CheckedOutcomeOnly,
        _marker: std::marker::PhantomData,
    };
}
