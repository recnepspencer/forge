use forge_query::facade::ForgeQueryDeclaredFamilyChecked;
use hadwiger_research::facade::{
    admit_hadwiger_research_handle, declare_research_request_checked,
    orchestrate_research_request_entry, research_declaration_entry_readiness,
    CandidateGraphDeclaration, HadwigerResearchAdmissionError, HadwigerResearchOperatingContext,
};

fn admitted_handle_dx() -> Result<(), HadwigerResearchAdmissionError> {
    let context = HadwigerResearchOperatingContext::finite_lower_bound_real();
    let handle = admit_hadwiger_research_handle(context)?;

    let request = CandidateGraphDeclaration::new("moser-spindle-seed")
        .with_graph_version("v1")
        .with_source_note("phase-1 declaration-entry smoke");

    let checked = declare_research_request_checked(&handle, request.clone());
    let readiness = research_declaration_entry_readiness::<CandidateGraphDeclaration>(&handle);
    let _outcome = orchestrate_research_request_entry(&handle, request);

    match checked {
        ForgeQueryDeclaredFamilyChecked::Admitted(declaration) => {
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
    let _ = admitted_handle_dx as fn() -> Result<(), HadwigerResearchAdmissionError>;
}
