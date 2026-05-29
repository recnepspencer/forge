use forge_query::facade::{
    ForgeQueryDeclarationEntryOrchestrationInput,
    ForgeQueryDeclarationEntryOrchestrationPlan, ForgeQueryDomainEntryMarker,
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

fn fake_input() -> ForgeQueryDeclarationEntryOrchestrationInput<GeometryDomain, FakeInput> {
    panic!("not constructible")
}

fn main() {
    let _ = ForgeQueryDeclarationEntryOrchestrationPlan::<GeometryDomain, FakeInput> {
        input: fake_input(),
        ceiling_stage: forge_query::facade::ForgeQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
        step_plan: Vec::new(),
        orchestration_identity_digest: String::new(),
    };
}
