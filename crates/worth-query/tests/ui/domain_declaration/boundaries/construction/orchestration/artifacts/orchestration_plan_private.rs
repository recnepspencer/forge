use worth_query::facade::foundation::{WorthQueryDeclarationEntryOrchestrationInput, WorthQueryDeclarationEntryOrchestrationPlan, WorthQueryDomainEntryMarker};

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

fn fake_input() -> WorthQueryDeclarationEntryOrchestrationInput<GeometryDomain, FakeInput> {
    panic!("not constructible")
}

fn main() {
    let _ = WorthQueryDeclarationEntryOrchestrationPlan::<GeometryDomain, FakeInput> {
        input: fake_input(),
        ceiling_stage: worth_query::facade::foundation::WorthQueryDeclarationEntryOrchestrationStage::EnvelopeConstructed,
        step_plan: Vec::new(),
        orchestration_identity_digest: String::new(),
    };
}
