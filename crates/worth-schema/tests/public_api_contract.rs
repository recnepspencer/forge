use worth_schema::facade::topology_authoring::{
    milestone_one_default_primitive_corpus, WorthMilestoneOnePrimitiveScenario,
};
use worth_schema::facade::{
    admit_worth_query_mutation_batch, worth_query_mutation_support_contract,
    RawWorthTopologyIntent, WorthQueryAspectPath, WorthQueryCollection,
    WorthQueryComputedDeclarationBuilder, WorthQueryLiveDeclarationBuilder,
    WorthQueryMutationAdmission, WorthQueryMutationAdmissionBlocker,
    WorthQueryMutationSupportContract, WorthQuerySchemaBasis,
};

fn _live_declaration_contract() {
    let _ = WorthQueryLiveDeclarationBuilder::new(
        "worth.public.topology",
        WorthQueryCollection::TopologyEntity,
        WorthQuerySchemaBasis::TopologyEntityLiveView,
    )
    .select([WorthQueryAspectPath::TOPOLOGY_STRUCTURE])
    .build()
    .unwrap();
}

fn _computed_declaration_contract() {
    let _ = WorthQueryComputedDeclarationBuilder::new("worth.public.validation")
        .reads([WorthQueryAspectPath::TOPOLOGY_STRUCTURE])
        .produces([WorthQueryAspectPath::DIAGNOSTICS_DECISIONS])
        .build()
        .unwrap();
}

fn _query_aspect_roundtrip_contract() {
    let path = WorthQueryAspectPath::from_str("topology.structure").unwrap();
    let _aspect = path.into_worth_aspect();
}

fn _query_mutation_admission_contract(
    intent: &RawWorthTopologyIntent,
) -> WorthQueryMutationAdmission {
    admit_worth_query_mutation_batch(intent)
}

fn _query_mutation_support_contract(
) -> Result<WorthQueryMutationSupportContract, forge_query::facade::ForgeQueryRuntimeError> {
    worth_query_mutation_support_contract()
}

fn _topology_authoring_contract() -> Vec<WorthMilestoneOnePrimitiveScenario> {
    milestone_one_default_primitive_corpus()
}

#[test]
fn worth_schema_public_query_declaration_boundaries_compile() {
    let _ = _live_declaration_contract;
    let _ = _computed_declaration_contract;
    let _ = _query_aspect_roundtrip_contract;
    let _ = _query_mutation_admission_contract;
    let _ = _query_mutation_support_contract;
    let _ = _topology_authoring_contract;
    let _ = WorthQueryMutationAdmissionBlocker::ExistingIdentityBindingRequired;
}
