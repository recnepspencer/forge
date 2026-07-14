use worth_query::facade::WORTHQueryDeclaredFamilyChecked;
use hadwiger_research::facade::{
    declare_research_request_checked, hadwiger_research_domain_package, CandidateGraphDeclaration,
    HadwigerResearchDomainEntry, HadwigerResearchHandle, HadwigerResearchOperatingContext,
    HadwigerResearchQueryExt,
};
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};

fn main() -> Result<(), String> {
    let handle = installed_declarations()?;
    let request = CandidateGraphDeclaration::new("candidate-a").with_graph_version("v1");

    let helper = declare_research_request_checked(&handle, request.clone());
    let direct = match handle.declare_checked(request) {
        WORTHQueryDeclaredFamilyChecked::Admitted(declaration) => declaration,
        _ => panic!("direct declaration should admit"),
    };

    let helper = match helper {
        WORTHQueryDeclaredFamilyChecked::Admitted(declaration) => declaration,
        _ => panic!("helper declaration should admit"),
    };

    assert_eq!(helper.declaration_digest(), direct.declaration_digest());

    Ok(())
}

fn installed_declarations() -> Result<HadwigerResearchHandle, String> {
    let schema = WorthQueryTestBackendSchema::single_collection("HadwigerCandidate")
        .aspect("identity.id", "identity.id").map_err(|error| error.to_string())?;
    let workspace = in_memory_test_runtime().with_schema(schema)
        .domain_package(hadwiger_research_domain_package())
        .workspace("hadwiger-helper-parity-dx").map_err(|error| error.to_string())?;
    let installed = workspace.domain(HadwigerResearchDomainEntry).map_err(|error| error.to_string())?;
    installed.research_declarations(&workspace, HadwigerResearchOperatingContext::default())
        .map_err(|error| error.to_string())
}
