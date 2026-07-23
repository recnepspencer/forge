use worth_query::facade::domain;

use super::conditional_node_contract::node;
use super::installed_operation_fixture::{
    conditional_workspace, GeometryDomain, ReadExecutionInput, ReadFamily, ReadVertex,
};

#[test]
fn rebuilt_conditional_lookup_retains_the_exact_installed_authority() {
    let declaration = node(
        "rebuilt-conditional-index",
        domain::WorthQueryComparatorRequirement::ExactCanonicalValue,
        domain::WorthQuerySemanticLocality::SourceRecord,
    );
    let mut workspace = conditional_workspace("rebuilt-conditional-index", declaration).unwrap();

    let installed = workspace.domain(GeometryDomain).unwrap();
    let before_rebuild = workspace
        .observe_operating_world()
        .unwrap()
        .family(ReadFamily)
        .bind(&installed, ReadVertex)
        .unwrap();

    let report = workspace.rebuild_conditional_execution_index();
    assert_eq!(report.authoritative_installations(), 1);
    assert_eq!(report.rebuilt_lookup_entries(), 1);
    assert!(report.exact_index_parity());

    let bound = workspace
        .observe_operating_world()
        .unwrap()
        .family(ReadFamily)
        .bind(&installed, ReadVertex)
        .unwrap();
    before_rebuild.same_installation_with(&bound).unwrap();
    let executed = bound
        .execute(ReadExecutionInput::default(), &mut workspace)
        .unwrap();

    assert_eq!(executed.conditional_provenance().len(), 1);
    assert_eq!(
        executed.conditional_provenance()[0].class(),
        domain::WorthQueryConditionalOutcomeClass::ComputedChanged
    );
    assert_eq!(
        executed.conditional_provenance()[0]
            .declaration()
            .identity(),
        "rebuilt-conditional-index"
    );
    assert_eq!(executed.counters().conditional_compute_contacts, 1);
    assert_eq!(executed.counters().executor_contacts, 1);
}
