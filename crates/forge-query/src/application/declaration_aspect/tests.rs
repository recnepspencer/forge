use crate::application::{
    route_scoped_declaration_aspect_contract, ForgeQueryDeclarationAspectContract,
    ForgeQueryDeclarationAspectCoverage, ForgeQueryDeclarationAspectFit,
};

#[test]
fn aspect_fit_reports_exact_and_superset_matches() {
    let contract = ForgeQueryDeclarationAspectContract::from_slices(
        &["selection.active_edge"],
        &["selection.active_edge"],
        &["selection.active_edge"],
        &[],
        &[],
    );
    assert_eq!(
        ForgeQueryDeclarationAspectCoverage::from_present(["selection.active_edge"])
            .fit_against(&contract),
        ForgeQueryDeclarationAspectFit::Exact
    );
    assert_eq!(
        ForgeQueryDeclarationAspectCoverage::from_present([
            "selection.active_edge",
            "selection.local_topology"
        ])
        .fit_against(&contract),
        ForgeQueryDeclarationAspectFit::CompatibleSuperset
    );
}

#[test]
fn aspect_fit_reports_partial_missing_and_conflict() {
    let contract = ForgeQueryDeclarationAspectContract::from_slices(
        &["selection.active_edge", "selection.local_topology"],
        &[],
        &[],
        &[],
        &["selection.material_edit"],
    );
    assert_eq!(
        ForgeQueryDeclarationAspectCoverage::from_present(["selection.active_edge"])
            .fit_against(&contract),
        ForgeQueryDeclarationAspectFit::Partial
    );
    assert_eq!(
        ForgeQueryDeclarationAspectCoverage::from_present(["selection.active_face"])
            .fit_against(&contract),
        ForgeQueryDeclarationAspectFit::MissingRequired
    );
    assert_eq!(
        ForgeQueryDeclarationAspectCoverage::from_slices(
            &["selection.active_edge", "selection.material_edit"],
            &[],
            &[]
        )
        .fit_against(&contract),
        ForgeQueryDeclarationAspectFit::Conflict
    );
}

#[test]
fn masked_required_aspects_do_not_count_as_present() {
    let contract = ForgeQueryDeclarationAspectContract::from_slices(
        &["selection.active_edge"],
        &["selection.active_edge"],
        &["selection.active_edge"],
        &[],
        &[],
    );
    assert_eq!(
        ForgeQueryDeclarationAspectCoverage::from_slices(
            &["selection.active_edge"],
            &["selection.active_edge"],
            &[]
        )
        .fit_against(&contract),
        ForgeQueryDeclarationAspectFit::MissingRequired
    );
}

#[test]
fn scoped_coverage_keeps_only_contract_relevant_slices() {
    let contract = ForgeQueryDeclarationAspectContract::from_slices(
        &["selection.active_edge"],
        &["selection.local_topology"],
        &[],
        &[],
        &[],
    );
    let scoped = ForgeQueryDeclarationAspectCoverage::from_slices(
        &[
            "selection.active_edge",
            "selection.local_topology",
            "selection.active_face",
        ],
        &["selection.private_authority"],
        &["selection.conflicting"],
    )
    .scoped_to_contract(&contract);

    assert_eq!(
        scoped.present(),
        &[
            "selection.active_edge".to_string(),
            "selection.local_topology".to_string()
        ]
    );
    assert!(scoped.masked().is_empty());
    assert!(scoped.conflicting().is_empty());
}

#[test]
fn route_scoped_contract_drops_declaration_only_published_slices() {
    let contract = ForgeQueryDeclarationAspectContract::from_slices(
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
