use crate::application::{
    assert_declaration_aspect_projections, route_scoped_declaration_aspect_contract,
    test_declaration_aspect_keys, WorthQueryDeclarationAspectContract,
    WorthQueryDeclarationAspectCoverage, WorthQueryDeclarationAspectFit,
};

#[test]
fn aspect_fit_reports_exact_and_superset_matches() {
    let contract = WorthQueryDeclarationAspectContract::from_slices(
        &["selection.active_edge"],
        &["selection.active_edge"],
        &["selection.active_edge"],
        &[],
        &[],
    );
    assert_eq!(
        WorthQueryDeclarationAspectCoverage::from_present(test_declaration_aspect_keys(&[
            "selection.active_edge"
        ]))
        .fit_against(&contract),
        WorthQueryDeclarationAspectFit::Exact
    );
    assert_eq!(
        WorthQueryDeclarationAspectCoverage::from_present(test_declaration_aspect_keys(&[
            "selection.active_edge",
            "selection.local_topology"
        ]))
        .fit_against(&contract),
        WorthQueryDeclarationAspectFit::CompatibleSuperset
    );
}

#[test]
fn aspect_fit_reports_partial_missing_and_conflict() {
    let contract = WorthQueryDeclarationAspectContract::from_slices(
        &["selection.active_edge", "selection.local_topology"],
        &[],
        &[],
        &[],
        &["selection.material_edit"],
    );
    assert_eq!(
        WorthQueryDeclarationAspectCoverage::from_present(test_declaration_aspect_keys(&[
            "selection.active_edge"
        ]))
        .fit_against(&contract),
        WorthQueryDeclarationAspectFit::Partial
    );
    assert_eq!(
        WorthQueryDeclarationAspectCoverage::from_present(test_declaration_aspect_keys(&[
            "selection.active_face"
        ]))
        .fit_against(&contract),
        WorthQueryDeclarationAspectFit::MissingRequired
    );
    assert_eq!(
        WorthQueryDeclarationAspectCoverage::from_slices(
            &["selection.active_edge", "selection.material_edit"],
            &[],
            &[]
        )
        .fit_against(&contract),
        WorthQueryDeclarationAspectFit::Conflict
    );
}

#[test]
fn masked_required_aspects_do_not_count_as_present() {
    let contract = WorthQueryDeclarationAspectContract::from_slices(
        &["selection.active_edge"],
        &["selection.active_edge"],
        &["selection.active_edge"],
        &[],
        &[],
    );
    assert_eq!(
        WorthQueryDeclarationAspectCoverage::from_slices(
            &["selection.active_edge"],
            &["selection.active_edge"],
            &[]
        )
        .fit_against(&contract),
        WorthQueryDeclarationAspectFit::MissingRequired
    );
}

#[test]
fn scoped_coverage_keeps_only_contract_relevant_slices() {
    let contract = WorthQueryDeclarationAspectContract::from_slices(
        &["selection.active_edge"],
        &["selection.local_topology"],
        &[],
        &[],
        &[],
    );
    let scoped = WorthQueryDeclarationAspectCoverage::from_slices(
        &[
            "selection.active_edge",
            "selection.local_topology",
            "selection.active_face",
        ],
        &["selection.private_authority"],
        &["selection.conflicting"],
    )
    .scoped_to_contract(&contract);

    assert_declaration_aspect_projections(
        scoped.present(),
        &["selection.active_edge", "selection.local_topology"],
    );
    assert!(scoped.masked().is_empty());
    assert!(scoped.conflicting().is_empty());
}

#[test]
fn route_scoped_contract_drops_declaration_only_published_slices() {
    let contract = WorthQueryDeclarationAspectContract::from_slices(
        &["selection.active_edge"],
        &["selection.local_topology"],
        &["selection.material_edit"],
        &["selection.private_authority"],
        &["selection.disallowed"],
    );

    let route_scoped = route_scoped_declaration_aspect_contract(&contract);

    assert_eq!(route_scoped.required(), contract.required());
    assert_eq!(route_scoped.preserved(), contract.preserved());
    assert!(route_scoped.published().is_empty());
    assert_eq!(route_scoped.masked(), contract.masked());
    assert_eq!(route_scoped.incompatible(), contract.incompatible());
}
