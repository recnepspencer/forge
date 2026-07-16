use worth_query::facade::domain::WorthQueryDeclaredFamilyChecked;
use hadwiger_research::facade::{
    declare_research_request_checked, hadwiger_research_domain_package,
    research_declaration_entry_readiness, CandidateGraphDeclaration,
    HadwigerResearchDomainEntry, HadwigerResearchHandle, HadwigerResearchOperatingContext,
    HadwigerResearchQueryExt,
};
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};

fn admitted_handle_dx() -> Result<(), String> {
    let handle = installed_declarations()?;

    let request = CandidateGraphDeclaration::new("moser-spindle-seed")
        .with_graph_version("v1")
        .with_source_note("phase-1 declaration-entry smoke");

    let checked = declare_research_request_checked(&handle, request);
    let readiness = research_declaration_entry_readiness::<CandidateGraphDeclaration>(&handle);

    match checked {
        WorthQueryDeclaredFamilyChecked::Admitted(declaration) => {
            assert_eq!(
                declaration.declaration_family_key(),
                "hadwiger.candidate_graph"
            );
        }
        _ => panic!("candidate graph declaration should admit"),
    }
    assert!(!readiness.rows().is_empty());

    Ok(())
}

fn main() {
    let _ = admitted_handle_dx as fn() -> Result<(), String>;
}

fn installed_declarations() -> Result<HadwigerResearchHandle, String> {
    let schema = WorthQueryTestBackendSchema::single_collection("HadwigerCandidate")
        .aspect("identity.id", "identity.id").map_err(|error| error.to_string())?;
    let workspace = in_memory_test_runtime().with_schema(schema)
        .domain_package(hadwiger_research_domain_package())
        .workspace("hadwiger-declaration-dx").map_err(|error| error.to_string())?;
    let installed = workspace.domain(HadwigerResearchDomainEntry).map_err(|error| error.to_string())?;
    installed.research_declarations(&workspace, HadwigerResearchOperatingContext::default())
        .map_err(|error| error.to_string())
}
