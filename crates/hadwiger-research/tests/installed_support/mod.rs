use hadwiger_research::facade::{
    hadwiger_research_domain_package, HadwigerResearchDomainEntry, HadwigerResearchHandle,
    HadwigerResearchOperatingContext, HadwigerResearchQueryExt,
};
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};

pub fn installed_hadwiger_research_handle() -> Result<HadwigerResearchHandle, String> {
    let schema = WorthQueryTestBackendSchema::single_collection("HadwigerCandidate")
        .aspect("identity.id", "identity.id")
        .map_err(|error| error.to_string())?;
    let workspace = in_memory_test_runtime()
        .with_schema(schema)
        .domain_package(hadwiger_research_domain_package())
        .workspace("hadwiger-installed-declaration-world")
        .map_err(|error| error.to_string())?;
    let installed = workspace
        .domain(HadwigerResearchDomainEntry)
        .map_err(|error| error.to_string())?;
    installed
        .research_declarations(
            &workspace,
            HadwigerResearchOperatingContext::finite_lower_bound_real(),
        )
        .map_err(|error| error.to_string())
}
