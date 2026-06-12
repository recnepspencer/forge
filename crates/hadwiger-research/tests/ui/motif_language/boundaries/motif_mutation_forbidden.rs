use hadwiger_research::facade::{
    admit_hadwiger_research_handle, declare_research_request_checked,
    HadwigerDeclaredFamilyCheckedExt, HadwigerResearchOperatingContext, MotifArtifact,
    MotifSeedDeclaration, MotifTerminal,
};

fn main() {
    let handle =
        admit_hadwiger_research_handle(HadwigerResearchOperatingContext::finite_lower_bound_real())
            .unwrap();
    let declaration = declare_research_request_checked(
        &handle,
        MotifSeedDeclaration::new("mutable-motif"),
    )
    .admitted()
    .unwrap();
    let mut motif = MotifArtifact::builder("mutable-motif", declaration.into())
        .with_terminal(MotifTerminal::new("north").unwrap())
        .unwrap()
        .finish()
        .unwrap();

    motif.terminals_mut().clear();
}
