use forge_query::facade::ForgeQueryDeclaredFamilyChecked;
use hadwiger_research::facade::{
    admit_hadwiger_research_handle, declare_research_request_checked,
    CandidateGraphDeclaration, HadwigerResearchAdmissionError, HadwigerResearchOperatingContext,
};

fn main() -> Result<(), HadwigerResearchAdmissionError> {
    let handle = admit_hadwiger_research_handle(
        HadwigerResearchOperatingContext::finite_lower_bound_real(),
    )?;
    let request = CandidateGraphDeclaration::new("candidate-a").with_graph_version("v1");

    let helper = declare_research_request_checked(&handle, request.clone());
    let direct = match handle.declare_checked(request) {
        ForgeQueryDeclaredFamilyChecked::Admitted(declaration) => declaration,
        _ => panic!("direct declaration should admit"),
    };

    let helper = match helper {
        ForgeQueryDeclaredFamilyChecked::Admitted(declaration) => declaration,
        _ => panic!("helper declaration should admit"),
    };

    assert_eq!(helper.declaration_digest(), direct.declaration_digest());

    Ok(())
}
