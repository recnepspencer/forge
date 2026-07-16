use worth_query::facade::consumer_kit::{in_memory_test_runtime, WorthQueryTestBackendSchema};
use worth_query::facade::domain;
use worth_query::facade::foundation::basis_lifecycle;

use super::{
    hadwiger_research_domain_package, HadwigerCandidateContribution, HadwigerResearchDomainEntry,
    HadwigerResearchQueryExt,
};

#[test]
fn hadwiger_candidate_search_uses_the_installed_domain_read_journey() {
    let mut workspace = candidate_workspace("hadwiger-reference-read");
    let handle = workspace.domain(HadwigerResearchDomainEntry).unwrap();
    let completion = handle
        .candidate_search()
        .expect("Hadwiger candidate search should declare")
        .using(domain::current())
        .run(&mut workspace)
        .unwrap()
        .into_result()
        .expect("candidate search should complete");

    assert_eq!(
        completion
            .completion()
            .journey_counters()
            .planning_attempt_count(),
        1
    );
    assert_eq!(
        completion
            .completion()
            .journey_counters()
            .lower_runtime_execution_completed_count(),
        1
    );
    assert_eq!(
        completion
            .receipt()
            .installed_authority()
            .package_identity(),
        handle.package_identity()
    );

    let projection = completion.project(domain::project_facts().entity_identities());
    assert_eq!(
        projection
            .receipt()
            .installed_authority()
            .package_identity(),
        handle.package_identity()
    );

    let inspection_basis = basis_lifecycle()
        .historical_snapshot("hadwiger-reference-read", true)
        .inspect()
        .expect("Hadwiger reference inspection basis should admit");
    let inspection = completion
        .inspect()
        .using(domain::inspection_basis(inspection_basis))
        .run(&workspace)
        .expect("Hadwiger installed read should inspect");
    assert_eq!(
        inspection
            .receipt()
            .installed_authority()
            .package_identity(),
        handle.package_identity()
    );
    assert!(inspection.outcome().settled().is_some());
}

#[test]
fn hadwiger_contribution_lowers_through_the_installed_handle() {
    let mut workspace = candidate_workspace("hadwiger-reference-workflow");
    let handle = workspace.domain(HadwigerResearchDomainEntry).unwrap();
    let label = domain::WorthQuerySessionLabel::scoped_strs("hadwiger", ["candidate-17"])
        .expect("candidate label should admit");
    let declaration = handle
        .candidate_promotion(
            label.clone(),
            HadwigerCandidateContribution::new("candidate-17"),
        )
        .expect("Hadwiger contribution should declare");
    let context = domain::preview(&workspace, label).expect("preview context should admit");
    let outcome = declaration.using(context).run(&mut workspace).unwrap();
    let completion = outcome
        .completed()
        .expect("Hadwiger workflow should complete");

    assert_eq!(
        completion.completion().aftermath().closeout_kind(),
        domain::WorthQueryPreviewCloseoutKind::Promoted
    );
    assert_eq!(
        completion
            .completion()
            .counters()
            .lower_runtime_execution_completed_count(),
        1
    );
    assert_eq!(
        completion
            .receipt()
            .installed_authority()
            .package_identity(),
        handle.package_identity()
    );
}

fn candidate_workspace(name: &str) -> worth_query::facade::runtime::WorthQueryWorkspace {
    let schema = WorthQueryTestBackendSchema::single_collection("HadwigerCandidate")
        .aspect_contracts(crate::query_entry::hadwiger_native_aspect_contracts())
        .expect("Hadwiger native aspect contracts should build")
        .aspect("identity.id", "identity.id")
        .expect("identity aspect should build")
        .aspect("colorability.lower_bound", "colorability.lower_bound")
        .expect("colorability aspect should build");
    in_memory_test_runtime()
        .with_schema(schema)
        .domain_package(hadwiger_research_domain_package())
        .workspace(name)
        .expect("Hadwiger reference workspace should build")
}
