use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryContributionComposedOrchestrationInput,
};
use schema::facade::platform::entities::TopologyEntityKind;
use topology::facade::TopologyCreateTopologyEntityDeclaration;
use topology::query_domain::{
    topology_current_head_authoritative_context, topology_query_domain_entry,
};

fn main() {
    let query = ForgeQueryApplicationFacade::runtime_backed_default();
    let handle = topology_query_domain_entry(&query)
        .with_operating_context(topology_current_head_authoritative_context())
        .validate()
        .unwrap()
        .admit()
        .unwrap();
    let raw_declaration = TopologyCreateTopologyEntityDeclaration::new(
        "copied.mutation.record",
        TopologyEntityKind::Vertex,
    );
    let input = ForgeQueryContributionComposedOrchestrationInput::new(raw_declaration);

    let _proof = handle.orchestrate_declaration_with_contributions_proof(input);
}
