use hadwiger_research::facade::{
    declare_research_request_checked, hadwiger_research_domain_package,
    HadwigerDeclaredFamilyCheckedExt, HadwigerResearchDomainEntry, HadwigerResearchHandle,
    HadwigerResearchOperatingContext, HadwigerResearchQueryExt, MotifArtifact,
    MotifSeedDeclaration, MotifTerminal,
};
use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};

fn main() {
    let handle = installed_declarations().unwrap();
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

fn installed_declarations() -> Result<HadwigerResearchHandle, String> {
    let schema = WorthQueryTestBackendSchema::single_collection("HadwigerCandidate")
        .aspect("identity.id", "identity.id").map_err(|error| error.to_string())?;
    let workspace = in_memory_test_runtime().with_schema(schema)
        .domain_package(hadwiger_research_domain_package())
        .workspace("hadwiger-motif-mutation-boundary").map_err(|error| error.to_string())?;
    let installed = workspace.domain(HadwigerResearchDomainEntry).map_err(|error| error.to_string())?;
    installed.research_declarations(&workspace, HadwigerResearchOperatingContext::default())
        .map_err(|error| error.to_string())
}
