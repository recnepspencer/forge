use worth_query::facade::{
    WorthQueryDeclarationEntryOrchestrationInput,
    WorthQueryDeclarationEntryOrchestrationPlan, WorthQueryDomainEntryMarker,
};

#[derive(Clone, Copy, Eq, PartialEq)]
struct GeometryDomain;

impl WorthQueryDomainEntryMarker for GeometryDomain {
    fn domain_key(&self) -> &'static str { "example.geometry.orchestration" }
    fn display_name(&self) -> &'static str { "GeometryOrchestrationDomain" }
    fn required_capability_families(&self) -> &'static [worth_query::facade::WorthQueryCapabilityFamily] {
        &[]
    }
}

struct FakeInput;

impl worth_query::facade::WorthQueryDeclarationInput<GeometryDomain> for FakeInput {
    type Family = FakeFamily;
    fn canonical_declaration_entries(
        &self,
    ) -> Vec<worth_query::facade::WorthQueryDeclarationCanonicalEntry> {
        Vec::new()
    }
}

struct FakeFamily;

impl worth_query::facade::WorthQueryDeclarationFamilyMarker<GeometryDomain> for FakeFamily {
    type PrimaryAuthority = worth_query::facade::WorthQueryRelationalTruthAuthority;
    type SignalCompatibility = worth_query::facade::WorthQuerySignalNotCompatiblePosture;
    type GroupedPosture = worth_query::facade::WorthQuerySingleOnlyGrouping;
    fn semantic_family_key() -> &'static str { "fake" }
    fn legality_contract() -> worth_query::facade::WorthQueryDeclarationLegalityContract {
        worth_query::facade::WorthQueryDeclarationLegalityContract::authoritative_hot_artifact()
    }
}

fn fake_input() -> WorthQueryDeclarationEntryOrchestrationInput<GeometryDomain, FakeInput> {
    panic!("not constructible")
}

fn main() {
    let _ = WorthQueryDeclarationEntryOrchestrationPlan::<GeometryDomain, FakeInput> {
        input: fake_input(),
        ceiling_stage: worth_query::facade::WorthQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
        step_plan: Vec::new(),
        orchestration_identity_digest: String::new(),
    };
}
