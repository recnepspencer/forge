use std::collections::BTreeMap;
use std::fs;

use forge_query::facade::consumer_kit::{
    ForgeQueryGraphObligationResidueManifest, ForgeQueryGraphObligationResidueRow,
};
use worth_kernel::query_obligation_selection::boundary_inventory::query_selection_boundary_inventory;
use worth_kernel::query_obligation_selection::selection_substrate::{
    deny_in_memory_query_obligation_selection_authority,
    deny_local_query_obligation_selector_authority,
    deny_local_support_row_query_obligation_authority,
    deny_source_grep_query_obligation_audit_authority,
    query_primitive_construction_family_cardinality_closeout,
    query_primitive_construction_residue_baseline_v1,
    query_primitive_construction_residue_contract, QueryObligationSelectionErrorKind,
    QueryObligationSelectionInput, QueryObligationSelectionSubstrate,
};
use worth_primitives::PrimitiveWitnessDescriptor;

use super::public_api_query_obligation_selection_support::primitive_construction_birth_declared_touched_basis;

#[test]
fn phase6_primitive_selection_uses_query_substrate_ceremony_not_legacy_source_grep() {
    let touched_basis = primitive_construction_birth_declared_touched_basis(
        &PrimitiveWitnessDescriptor::SimplexSolid,
        "phase6-local-ceremony",
    );
    let input = QueryObligationSelectionInput::from_topology_touched_basis(touched_basis.proof())
        .expect("real primitive touched basis should enter Query selection");
    let selected = QueryObligationSelectionSubstrate::select_execution_backed_obligations(input)
        .expect("primitive construction selection should be execution-backed");
    let closeout = selected.closeout();
    let ceremony = closeout.local_ceremony_closeout();

    assert!(selected.execution_proof().has_real_executor_rows());
    assert_eq!(selected.selected_obligation_count(), 1);
    assert_eq!(ceremony.evaluated_source_count(), 8);
    assert!(ceremony.is_clean());
    assert!(ceremony.is_query_owned_selection_substrate());
    assert_eq!(ceremony.rejected_forbidden_authority_count(), 0);
    assert!(!ceremony.audit_digest().is_empty());
}

#[test]
fn selector_residue_count_cannot_grow_after_introduction() {
    let previous = residue_manifest(1, 2, "phase-6-owner", "blocked by query gap", "delete it")
        .expect("previous residue manifest");
    let drift_cases = [
        (
            residue_manifest(2, 2, "phase-6-owner", "blocked by query gap", "delete it"),
            "current count growth",
        ),
        (
            residue_manifest(1, 3, "phase-6-owner", "blocked by query gap", "delete it"),
            "cap drift",
        ),
        (
            residue_manifest(1, 2, "new-owner", "blocked by query gap", "delete it"),
            "owner drift",
        ),
        (
            residue_manifest(1, 2, "phase-6-owner", "different blocker", "delete it"),
            "blocker drift",
        ),
        (
            residue_manifest(
                1,
                2,
                "phase-6-owner",
                "blocked by query gap",
                "different removal trigger",
            ),
            "removal-trigger drift",
        ),
    ];

    for (candidate, reason) in drift_cases {
        let candidate = candidate.expect("candidate residue manifest");
        assert!(
            ForgeQueryGraphObligationResidueManifest::certify_candidate_against_previous(
                &previous, &candidate
            )
            .is_err(),
            "Phase 6 must fail closed on {reason}"
        );
    }
}

#[test]
fn phase6_forbidden_local_authority_paths_have_typed_denials() {
    assert_denial_kind(
        deny_local_query_obligation_selector_authority("local selector table"),
        QueryObligationSelectionErrorKind::LocalSelectorAuthorityDenied,
    );
    assert_denial_kind(
        deny_local_query_obligation_selector_authority("local graph walk selector"),
        QueryObligationSelectionErrorKind::LocalSelectorAuthorityDenied,
    );
    assert_denial_kind(
        deny_local_support_row_query_obligation_authority("private support matrix"),
        QueryObligationSelectionErrorKind::LocalSupportRowAuthorityDenied,
    );
    assert_denial_kind(
        deny_source_grep_query_obligation_audit_authority("source-grep audit"),
        QueryObligationSelectionErrorKind::SourceGrepAuditAuthorityDenied,
    );
    assert_denial_kind(
        deny_in_memory_query_obligation_selection_authority("in-memory adoption proof"),
        QueryObligationSelectionErrorKind::InMemorySelectionAuthorityDenied,
    );
}

#[test]
fn live_primitive_residue_manifest_matches_declared_phase6_contract() {
    let selected = select_simplex_phase6_obligations("phase6-residue-contract");
    let contract = query_primitive_construction_residue_contract();
    let baseline = query_primitive_construction_residue_baseline_v1();
    let expected_rows = contract
        .rows()
        .iter()
        .map(|row| (row.class(), row))
        .collect::<BTreeMap<_, _>>();
    let baseline_rows = baseline
        .rows()
        .iter()
        .map(|row| (row.class(), row))
        .collect::<BTreeMap<_, _>>();
    let residue_rows = selected.residue_manifest().rows();

    assert_eq!(
        baseline.version(),
        "touched-graph-milestone-5-primitive-residue-v1"
    );
    assert_eq!(residue_rows.len(), expected_rows.len());
    assert_eq!(residue_rows.len(), baseline_rows.len());
    for row in residue_rows {
        let expected = expected_rows
            .get(row.class())
            .unwrap_or_else(|| panic!("unexpected primitive residue class `{}`", row.class()));
        let baseline = baseline_rows
            .get(row.class())
            .unwrap_or_else(|| panic!("residue class `{}` is missing from baseline", row.class()));
        assert_eq!(row.owner(), expected.owner());
        assert_eq!(row.owner(), baseline.owner());
        assert_eq!(row.introduced_in(), expected.introduced_in());
        assert_eq!(row.introduced_in(), baseline.introduced_in());
        assert_eq!(row.current_count(), expected.current_count());
        assert_eq!(row.current_count(), baseline.current_count());
        assert_eq!(
            row.must_not_exceed_count(),
            expected.must_not_exceed_count()
        );
        assert_eq!(
            row.must_not_exceed_count(),
            baseline.must_not_exceed_count()
        );
        assert_eq!(row.blocker(), expected.blocker());
        assert_eq!(row.blocker(), baseline.blocker());
        assert_eq!(row.removal_trigger(), expected.removal_trigger());
        assert_eq!(row.removal_trigger(), baseline.removal_trigger());
        assert_eq!(row.decision(), expected.decision());
    }
}

#[test]
fn deleted_in_memory_selector_projection_is_not_in_live_inventory() {
    let inventory = query_selection_boundary_inventory();

    assert!(
        inventory
            .row_named("primitive_construction_graph_obligation_adoption_proof")
            .is_none(),
        "the old primitive in-memory adoption projection must stay deleted"
    );
}

#[test]
fn public_facade_does_not_export_local_ceremony_or_residue_helpers() {
    let public_sources = [
        include_str!("../../../lib.rs"),
        include_str!("../../../query_obligation_selection/mod.rs"),
        include_str!("../../../query_obligation_selection/selection_substrate/mod.rs"),
    ]
    .join("\n");
    for forbidden in [
        "primitive_construction_graph_obligation_local_ceremony_audit",
        "primitive_construction_graph_obligation_audit_sources",
        "primitive_construction_graph_obligation_residue_manifest",
        "selection_substrate_local_ceremony_audit",
    ] {
        assert!(
            !public_sources.contains(forbidden),
            "public query-obligation facade must not export `{forbidden}`"
        );
    }
    assert!(!public_sources.contains("pub fn from_query_proof"));

    let public_api = fs::read_to_string(
        repo_root()
            .join("crates/worth-kernel/src/query_obligation_selection/selection_substrate/mod.rs"),
    )
    .expect("read public query obligation substrate module");
    assert!(!public_api.contains("pub use query_consumer_kit_lane"));
}

#[test]
fn primitive_family_cardinality_gap_is_explicit_residue_not_test_literal() {
    let closeout = query_primitive_construction_family_cardinality_closeout();
    let contract = query_primitive_construction_residue_contract();
    let family_gap_row = contract
        .rows()
        .iter()
        .find(|row| row.class() == closeout.capped_residue_class())
        .expect("family cardinality gap residue row");

    assert_eq!(closeout.spec_expected_family_count(), 7);
    assert_eq!(
        closeout.missing_family_count(),
        closeout.spec_expected_family_count() - closeout.runtime_family_count()
    );
    assert_eq!(
        family_gap_row.current_count(),
        closeout.missing_family_count()
    );
    assert_eq!(
        family_gap_row.must_not_exceed_count(),
        closeout.missing_family_count()
    );
}

fn residue_manifest(
    current_count: usize,
    capped_count: usize,
    owner: &'static str,
    blocker: &'static str,
    removal_trigger: &'static str,
) -> Result<
    ForgeQueryGraphObligationResidueManifest,
    forge_query::facade::consumer_kit::ForgeQueryGraphObligationConsumerKitError,
> {
    ForgeQueryGraphObligationResidueManifest::capped([
        ForgeQueryGraphObligationResidueRow::explicit(
            "phase6-selector-residue-growth-probe",
            "worth-kernel query obligation selection",
            owner,
            current_count,
            capped_count,
            blocker,
            removal_trigger,
            "growth after introduction must fail closed",
        )?,
    ])
}

fn select_simplex_phase6_obligations(
    label: &str,
) -> worth_kernel::query_obligation_selection::selection_substrate::QuerySelectedGraphObligations {
    let touched_basis = primitive_construction_birth_declared_touched_basis(
        &PrimitiveWitnessDescriptor::SimplexSolid,
        label,
    );
    let input = QueryObligationSelectionInput::from_topology_touched_basis(touched_basis.proof())
        .expect("real primitive touched basis should enter Query selection");
    QueryObligationSelectionSubstrate::select_execution_backed_obligations(input)
        .expect("primitive construction selection should be execution-backed")
}

fn assert_denial_kind(
    error: worth_kernel::query_obligation_selection::selection_substrate::QueryObligationSelectionError,
    expected: QueryObligationSelectionErrorKind,
) {
    assert_eq!(error.kind(), expected);
    assert!(!error.detail().is_empty());
}

fn repo_root() -> &'static std::path::Path {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("worth-kernel manifest should live under crates/worth-kernel")
}
