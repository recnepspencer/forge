use worth_query::facade::{
    WorthQueryDeclarationEntryOrchestrationProof, WorthQueryDeclarationEntryOrchestrationStageRecord,
    WorthQueryDomainEntryMarker,
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

fn fake_outcome(
) -> worth_query::facade::WorthQueryDeclarationEntryOrchestrationChecked<GeometryDomain, FakeInput>
{
    panic!("not constructible")
}

fn main() {
    let _ = WorthQueryDeclarationEntryOrchestrationProof::<GeometryDomain, FakeInput> {
        outcome: fake_outcome(),
        stage_records: Vec::<WorthQueryDeclarationEntryOrchestrationStageRecord>::new(),
        orchestration_digest: String::new(),
    };
}
