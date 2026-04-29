use forge_relational::facade::history::BranchId;
use worth_schema::facade::{
    admit_worth_query_mutation_batch, worth_query_mutation_support_contract,
    RawWorthTopologyIntent, WorthBoundaryFailure, WorthQueryAspectPath, WorthQueryCollection,
    WorthQueryComputedDeclarationBuilder, WorthQueryLiveDeclarationBuilder,
    WorthQueryMutationAdmission, WorthQueryMutationAdmissionBlocker,
    WorthQueryMutationSupportContract, WorthQuerySchemaBasis, WorthTopologyAuthority,
    WorthTopologyAuthorityError, WorthTracedTopologyCommit,
};

fn _apply_main_contract(
    authority: &mut WorthTopologyAuthority<'_>,
    intent: RawWorthTopologyIntent,
) -> Result<WorthTracedTopologyCommit, WorthBoundaryFailure<WorthTopologyAuthorityError>> {
    authority.apply_topology_intent_traced(intent)
}

fn _apply_branch_contract(
    authority: &mut WorthTopologyAuthority<'_>,
    intent: RawWorthTopologyIntent,
    branch_id: BranchId,
) -> Result<WorthTracedTopologyCommit, WorthBoundaryFailure<WorthTopologyAuthorityError>> {
    authority.apply_topology_intent_on_branch_traced(intent, branch_id)
}

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

#[test]
fn worth_schema_public_authority_and_query_declaration_boundaries_compile() {
    let _ = _apply_main_contract;
    let _ = _apply_branch_contract;
    let _ = _live_declaration_contract;
    let _ = _computed_declaration_contract;
    let _ = _query_aspect_roundtrip_contract;
    let _ = _query_mutation_admission_contract;
    let _ = _query_mutation_support_contract;
    let _ = WorthQueryMutationAdmissionBlocker::ExistingIdentityBindingRequired;
}
