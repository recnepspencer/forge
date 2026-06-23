use std::collections::BTreeSet;

use forge_query::facade::consumer_kit::ForgeQueryGraphObligationResidueManifest;
use forge_query::facade::{
    ForgeQueryGraphObligationExecutionStatus, ForgeQueryGraphObligationKind,
    ForgeQueryGraphObligationSupportLane, ForgeQueryGraphObligationSupportStatus,
};
use worth_kernel::query_obligation_selection::selection_substrate::{
    query_primitive_construction_family_cardinality_closeout,
    query_primitive_construction_residue_baseline_v1, QueryObligationSelectionAuthorityKind,
    QueryObligationSelectionInput, QueryObligationSelectionSubstrate,
};

use super::public_api_query_obligation_selection_support::{
    primitive_construction_birth_cases, PrimitiveConstructionBirthCase,
};

#[test]
fn primitive_construction_migrated_lane_closes_after_old_selector_deletion() {
    let selected_by_family = primitive_construction_birth_cases()
        .into_iter()
        .map(|case| {
            let selected = select_primitive_case(&case, "phase3-parity");
            let closeout = selected.closeout();
            let counters = closeout.selection_counters();

            assert_eq!(
                closeout.authority_kind(),
                QueryObligationSelectionAuthorityKind::TopologyTouchedBasis
            );
            assert_eq!(closeout.selected_obligation_count(), 1);
            assert_eq!(closeout.execution_row_count(), 1);
            assert_eq!(closeout.selected_registration_digests().len(), 1);
            let selected_obligations = selected
                .execution_proof()
                .selection_proof()
                .selected_obligations();
            assert_eq!(selected_obligations.len(), 1);
            assert_eq!(
                selected_obligations[0].obligation_kind(),
                ForgeQueryGraphObligationKind::AdvisoryObligation
            );
            assert_eq!(
                selected_obligations[0].support_lane(),
                ForgeQueryGraphObligationSupportLane::GraphComposition
            );
            assert_eq!(
                selected_obligations[0].support_status(),
                ForgeQueryGraphObligationSupportStatus::Supported
            );
            assert!(!selected_obligations[0].rule_identity_digest().is_empty());
            assert!(!selected_obligations[0].execution_budget_digest().is_empty());
            assert_eq!(counters.matched_obligation_count(), 1);
            assert_eq!(counters.registration_full_scan_count(), 0);
            assert!(counters.attempted_bucket_lookup_count() > 0);
            assert!(counters.candidate_registration_count() > 0);
            assert!(selected.execution_proof().has_real_executor_rows());
            assert!(!selected.execution_proof().envelope_digest().is_empty());
            assert_eq!(
                selected.execution_proof().execution_statuses(),
                vec![ForgeQueryGraphObligationExecutionStatus::Executed]
            );
            assert!(!closeout.authority_digest().is_empty());
            assert!(!closeout.touch_descriptor_digest().is_empty());
            assert!(!closeout.operating_world_digest().is_empty());
            assert!(!closeout.execution_proof_digest().is_empty());
            assert!(!closeout.adoption_manifest_digest().is_empty());
            assert!(!closeout.residue_manifest_digest().is_empty());

            (
                case.family().as_str(),
                closeout.selected_registration_digests().to_vec(),
            )
        })
        .collect::<Vec<_>>();

    let family_cardinality = query_primitive_construction_family_cardinality_closeout();
    assert_eq!(
        selected_by_family.len(),
        family_cardinality.runtime_family_count()
    );
    assert_eq!(
        family_cardinality.spec_expected_family_count(),
        family_cardinality.runtime_family_count() + family_cardinality.missing_family_count()
    );
    assert_eq!(
        selected_by_family
            .iter()
            .map(|(family, _)| *family)
            .collect::<BTreeSet<_>>()
            .len(),
        selected_by_family.len()
    );
}

#[test]
fn primitive_construction_replay_preserves_selected_obligation_identity() {
    for case in primitive_construction_birth_cases() {
        let first = select_primitive_case(&case, "phase3-replay").closeout();
        let second = select_primitive_case(&case, "phase3-replay").closeout();

        assert_eq!(first.authority_kind(), second.authority_kind());
        assert_eq!(first.authority_digest(), second.authority_digest());
        assert_eq!(
            first.touch_descriptor_digest(),
            second.touch_descriptor_digest()
        );
        assert_eq!(
            first.operating_world_digest(),
            second.operating_world_digest()
        );
        assert_eq!(
            first.selected_registration_digests(),
            second.selected_registration_digests()
        );
        assert_eq!(
            first.selected_obligation_count(),
            second.selected_obligation_count()
        );
        assert_eq!(first.execution_row_count(), second.execution_row_count());
        assert_eq!(
            first.execution_proof_digest(),
            second.execution_proof_digest()
        );
        assert_eq!(
            first.adoption_manifest_digest(),
            second.adoption_manifest_digest()
        );
        assert_eq!(
            first.residue_manifest_digest(),
            second.residue_manifest_digest()
        );
        assert_eq!(
            first.selection_counters().matched_obligation_count(),
            second.selection_counters().matched_obligation_count()
        );
        assert_eq!(
            first.selection_counters().registration_full_scan_count(),
            second.selection_counters().registration_full_scan_count()
        );
    }
}

#[test]
fn primitive_construction_local_selector_residue_is_deleted_or_capped() {
    let selected = select_primitive_case(
        &primitive_construction_birth_cases()
            .into_iter()
            .next()
            .expect("primitive construction case"),
        "phase3-residue",
    );
    let residue = selected.residue_manifest();

    assert_eq!(residue.rows().len(), 3);
    assert_residue_rows_are_capped(residue);
    assert_residue_class(residue, "kernel-handoff-only-result-helper");
    assert_residue_class(residue, "kernel-motion-preflight-sequencing");
    assert_residue_class(
        residue,
        query_primitive_construction_family_cardinality_closeout().capped_residue_class(),
    );
    assert_residue_matches_baseline(residue);
}

fn select_primitive_case(
    case: &PrimitiveConstructionBirthCase,
    label: &str,
) -> worth_kernel::query_obligation_selection::selection_substrate::QuerySelectedGraphObligations {
    let touched_basis = case.declared_touched_basis(label);
    let input = QueryObligationSelectionInput::from_topology_touched_basis(touched_basis.proof())
        .expect("real primitive construction touched basis must become selection input");
    let selected = QueryObligationSelectionSubstrate::select_execution_backed_obligations(input)
        .expect("primitive construction must select through Query-owned substrate");

    assert_eq!(
        selected.authority_digest(),
        touched_basis.proof().basis_digest()
    );
    assert_eq!(
        selected.touch_descriptor_digest(),
        touched_basis.proof().touch_descriptor().descriptor_digest()
    );
    selected
}

fn assert_residue_rows_are_capped(residue: &ForgeQueryGraphObligationResidueManifest) {
    let classes = residue
        .rows()
        .iter()
        .map(|row| row.class())
        .collect::<BTreeSet<_>>();
    assert_eq!(classes.len(), residue.rows().len());
    for row in residue.rows() {
        assert!(row.current_count() <= row.must_not_exceed_count());
        assert!(!row.owner().is_empty());
        assert!(!row.introduced_in().is_empty());
        assert!(!row.blocker().is_empty());
        assert!(!row.removal_trigger().is_empty());
        assert!(!row.decision().is_empty());
        assert!(
            !row.decision()
                .contains("execution-backed selected-obligation authority"),
            "residue row must not present old primitive residue as selected authority: {row:?}"
        );
    }
}

fn assert_residue_class(residue: &ForgeQueryGraphObligationResidueManifest, class: &str) {
    assert!(
        residue.rows().iter().any(|row| row.class() == class),
        "missing primitive construction residue class `{class}`"
    );
}

fn assert_residue_matches_baseline(residue: &ForgeQueryGraphObligationResidueManifest) {
    let baseline = query_primitive_construction_residue_baseline_v1();

    assert_eq!(
        baseline.version(),
        "touched-graph-milestone-5-primitive-residue-v1"
    );
    assert_eq!(residue.rows().len(), baseline.rows().len());
    for baseline_row in baseline.rows() {
        let live_row = residue
            .rows()
            .iter()
            .find(|row| row.class() == baseline_row.class())
            .unwrap_or_else(|| panic!("missing baseline residue class `{}`", baseline_row.class()));
        assert_eq!(live_row.owner(), baseline_row.owner());
        assert_eq!(live_row.introduced_in(), baseline_row.introduced_in());
        assert_eq!(live_row.current_count(), baseline_row.current_count());
        assert_eq!(
            live_row.must_not_exceed_count(),
            baseline_row.must_not_exceed_count()
        );
        assert_eq!(live_row.blocker(), baseline_row.blocker());
        assert_eq!(live_row.removal_trigger(), baseline_row.removal_trigger());
    }
}
