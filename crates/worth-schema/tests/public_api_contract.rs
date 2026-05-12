use schema::facade::topology_authoring::{
    milestone_one_default_primitive_corpus, MilestoneOnePrimitiveScenario,
};
use schema::facade::{
    admit_query_mutation_batch, query_mutation_support_contract, QueryAspectPath, QueryCollection,
    QueryComputedDeclarationBuilder, QueryLiveDeclarationBuilder, QueryMutationAdmission,
    QueryMutationAdmissionBlocker, QueryMutationSupportContract, QuerySchemaBasis,
    RawTopologyIntent,
};

fn _live_declaration_contract() {
    let _ = QueryLiveDeclarationBuilder::new(
        ".public.topology",
        QueryCollection::TopologyEntity,
        QuerySchemaBasis::TopologyEntityLiveView,
    )
    .select([QueryAspectPath::TOPOLOGY_STRUCTURE])
    .build()
    .unwrap();
}

fn _computed_declaration_contract() {
    let _ = QueryComputedDeclarationBuilder::new(".public.validation")
        .reads([QueryAspectPath::TOPOLOGY_STRUCTURE])
        .produces([QueryAspectPath::DIAGNOSTICS_DECISIONS])
        .build()
        .unwrap();
}

fn _query_aspect_roundtrip_contract() {
    let path = QueryAspectPath::from_str("topology.structure").unwrap();
    let _aspect = path.into_aspect();
}

fn _query_mutation_admission_contract(intent: &RawTopologyIntent) -> QueryMutationAdmission {
    admit_query_mutation_batch(intent)
}

fn _query_mutation_support_contract(
) -> Result<QueryMutationSupportContract, forge_query::facade::ForgeQueryRuntimeError> {
    query_mutation_support_contract()
}

fn _topology_authoring_contract() -> Vec<MilestoneOnePrimitiveScenario> {
    milestone_one_default_primitive_corpus()
}

#[test]
fn schema_public_query_declaration_boundaries_compile() {
    let _ = _live_declaration_contract;
    let _ = _computed_declaration_contract;
    let _ = _query_aspect_roundtrip_contract;
    let _ = _query_mutation_admission_contract;
    let _ = _query_mutation_support_contract;
    let _ = _topology_authoring_contract;
    let _ = QueryMutationAdmissionBlocker::ExistingIdentityBindingRequired;
}
