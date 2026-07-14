use hadwiger_research::facade::{
    hadwiger_research_domain_package, materialize_research_graph_invariant_denial,
    HadwigerResearchDomainEntry, ResearchGraphInvariantDenial, ResearchGraphInvariantDenialRequest,
    ResearchGraphInvariantError,
};
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};

pub fn materialize_installed_domain_denial(
    request: ResearchGraphInvariantDenialRequest,
) -> Result<ResearchGraphInvariantDenial, ResearchGraphInvariantError> {
    let schema = WorthQueryTestBackendSchema::single_collection("HadwigerResearchGraph")
        .aspect("identity.id", "identity.id")
        .expect("research graph identity aspect should build");
    let workspace = in_memory_test_runtime()
        .with_schema(schema)
        .domain_package(hadwiger_research_domain_package())
        .workspace("hadwiger-research-graph-invariant-denial")
        .expect("installed Hadwiger workspace should build");
    let handle = workspace
        .domain(HadwigerResearchDomainEntry)
        .expect("installed Hadwiger handle should resolve");

    materialize_research_graph_invariant_denial(&handle, &workspace, request)
}
