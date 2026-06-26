use forge_query::facade::{
    ForgeQueryGraphObligationBudgetExceededPolicy, ForgeQueryGraphObligationExecutionBudget,
    ForgeQueryGraphObligationExecutionScope, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationSupportPosture, ForgeQueryGraphObligationSupportStatus,
};

use crate::topology_operators::TopologyTouchedOperatingWorld;
use crate::validator_invariant_catalog::test_support::{
    catalog_closeout_from_test_family_rows, WorthTopologyLegalityTestFamilyRow,
};
use crate::validator_invariant_catalog::{
    WorthTopologyLegalitySelectionCloseout, WorthTopologyLegalitySelectionDenialKind,
};

use super::fixtures::{loop_touch_applicability, routing_closure_for_loop_touch};

#[test]
fn budget_exceeded_denies_selected_obligation_with_budget_evidence() {
    let closeout =
        catalog_closeout_from_test_family_rows([WorthTopologyLegalityTestFamilyRow::invariant(
            "budgeted_loop_obligation",
            loop_touch_applicability(),
        )
        .with_support_posture(
            ForgeQueryGraphObligationSupportPosture::supported(
                ForgeQueryGraphObligationSupportLane::WorthTopoOperatorCatalog,
            )
            .with_execution_budget(
                ForgeQueryGraphObligationExecutionBudget::bounded_sparse(
                    ForgeQueryGraphObligationExecutionScope::TouchedAspect,
                    ForgeQueryGraphObligationBudgetExceededPolicy::FailClosed,
                )
                .with_max_state_scope(1),
            ),
        )])
        .expect("budget test catalog should build through real Query projection");
    let routing_closure = routing_closure_for_loop_touch(TopologyTouchedOperatingWorld::mainline());

    let selection =
        WorthTopologyLegalitySelectionCloseout::from_phase_two_closeout_and_routing_closure(
            &closeout,
            &routing_closure,
        )
        .expect("budgeted selection should deny through selected-plan proof");

    assert!(selection
        .selected_plan()
        .selected_obligation_rows()
        .is_empty());
    assert_eq!(selection.selected_plan().denial_rows().len(), 1);
    let denial = &selection.selected_plan().denial_rows()[0];
    assert_eq!(
        denial.kind(),
        WorthTopologyLegalitySelectionDenialKind::BudgetExceeded
    );
    assert!(denial.registration_digest().is_some());
    assert!(denial.execution_budget_digest().is_some());
    assert_eq!(
        selection.selected_plan().counters().budget_denial_count(),
        1
    );
    assert_eq!(
        selection
            .selected_plan()
            .counters()
            .support_posture_denial_count(),
        0
    );
}

#[test]
fn query_support_postures_deny_without_collapsing_status_identity() {
    let lane = ForgeQueryGraphObligationSupportLane::WorthTopoOperatorCatalog;
    let closeout = catalog_closeout_from_test_family_rows([
        WorthTopologyLegalityTestFamilyRow::invariant(
            "unsupported_loop",
            loop_touch_applicability(),
        )
        .with_support_posture(ForgeQueryGraphObligationSupportPosture::unsupported(lane)),
        WorthTopologyLegalityTestFamilyRow::invariant(
            "diagnostic_only_loop",
            loop_touch_applicability(),
        )
        .with_support_posture(ForgeQueryGraphObligationSupportPosture::diagnostic_only(
            lane,
        )),
        WorthTopologyLegalityTestFamilyRow::invariant("deferred_loop", loop_touch_applicability())
            .with_support_posture(
                ForgeQueryGraphObligationSupportPosture::deferred_to_backstop(lane),
            ),
    ])
    .expect("support posture test catalog should build through real Query projection");
    let routing_closure = routing_closure_for_loop_touch(TopologyTouchedOperatingWorld::mainline());

    let selection =
        WorthTopologyLegalitySelectionCloseout::from_phase_two_closeout_and_routing_closure(
            &closeout,
            &routing_closure,
        )
        .expect("support posture selection should produce denial rows");

    let statuses = selection
        .selected_plan()
        .denial_rows()
        .iter()
        .map(|denial| {
            denial
                .support_status()
                .expect("denial should carry Query status")
        })
        .collect::<std::collections::BTreeSet<_>>();
    assert!(selection
        .selected_plan()
        .selected_obligation_rows()
        .is_empty());
    assert_eq!(statuses.len(), 3);
    assert!(statuses.contains(&ForgeQueryGraphObligationSupportStatus::Unsupported));
    assert!(statuses.contains(&ForgeQueryGraphObligationSupportStatus::DiagnosticOnly));
    assert!(statuses.contains(&ForgeQueryGraphObligationSupportStatus::DeferredToBackstop));
    assert_eq!(
        selection
            .selected_plan()
            .counters()
            .support_posture_denial_count(),
        3
    );
}
