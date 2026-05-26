use forge_query::facade::{
    ForgeQueryDeclarationEntryOrchestrationProof, ForgeQueryDeclarationEntryOrchestrationStageRecord,
    ForgeQueryDomainEntryMarker,
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

fn fake_outcome(
) -> forge_query::facade::ForgeQueryDeclarationEntryOrchestrationChecked<GeometryDomain, FakeInput>
{
    panic!("not constructible")
}

fn main() {
    let _ = ForgeQueryDeclarationEntryOrchestrationProof::<GeometryDomain, FakeInput> {
        outcome: fake_outcome(),
        stage_records: Vec::<ForgeQueryDeclarationEntryOrchestrationStageRecord>::new(),
        orchestration_digest: String::new(),
    };
}
