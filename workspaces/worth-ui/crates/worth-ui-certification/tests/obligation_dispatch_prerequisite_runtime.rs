#[path = "fixtures/obligation_dispatch_prerequisite_support/mod.rs"]
pub mod obligation_dispatch_prerequisite_support;

use worth_ui::facade::admission::{
    UiAdmissionHostCapability, UiAdmissionQueryBasis, UiAdmissionSelectionBudget,
    UiAdmissionStaleEvidence,
};
use worth_ui::facade::obligations::UiObligationCheckKind;
use worth_ui::facade::obligations::{
    UiObligationDispatchStopPosture, UiObligationFamily, UiObligationVerdictClass,
};

use self::obligation_dispatch_prerequisite_support::{
    apps::{query_touch_app, service_touch_app, structural_touch_app},
    targets::{
        ambiguous_host_capability_target, ambiguous_query_basis_target,
        available_host_capability_target, budget_exceeded_target, execute_for_target,
        missing_host_capability_target, stale_query_basis_target, wrong_query_basis_target,
    },
    touches::{query_touch, service_touch, structural_touch},
};

#[test]
fn query_basis_stop_postures_lower_through_selected_dispatch() {
    let app = query_touch_app();
    let touch = query_touch(&app);

    let wrong_basis = execute_for_target(&app, &touch, wrong_query_basis_target(&touch));
    let stale = execute_for_target(&app, &touch, stale_query_basis_target(&touch));
    let ambiguous = execute_for_target(&app, &touch, ambiguous_query_basis_target(&touch));

    assert_eq!(
        wrong_basis.dispatch.plan_stop_posture(),
        UiObligationDispatchStopPosture::WrongQueryBasis {
            required: UiAdmissionQueryBasis::GraphAligned,
            observed: UiAdmissionQueryBasis::WrongWorldProjection,
        }
    );
    assert_eq!(
        stale.dispatch.plan_stop_posture(),
        UiObligationDispatchStopPosture::Stale {
            required: UiAdmissionQueryBasis::GraphAligned,
            observed: UiAdmissionQueryBasis::StaleReceipt,
            evidence: UiAdmissionStaleEvidence::QueryReceiptExpired,
        }
    );
    assert_eq!(
        ambiguous.dispatch.plan_stop_posture(),
        UiObligationDispatchStopPosture::Ambiguous {
            required_query_basis: Some(UiAdmissionQueryBasis::GraphAligned),
            observed_query_basis: Some(UiAdmissionQueryBasis::AmbiguousSources),
            required_host_capability: None,
            observed_host_capability: None,
        }
    );
    assert!(wrong_basis.verdicts.iter().all(|verdict| {
        verdict.class() == UiObligationVerdictClass::Violation
            && verdict.stop_posture() == wrong_basis.dispatch.plan_stop_posture()
    }));
    assert!(stale.verdicts.iter().all(|verdict| {
        verdict.class() == UiObligationVerdictClass::Violation
            && verdict.stop_posture() == stale.dispatch.plan_stop_posture()
    }));
    assert!(ambiguous.verdicts.iter().all(|verdict| {
        verdict.class() == UiObligationVerdictClass::Violation
            && verdict.stop_posture() == ambiguous.dispatch.plan_stop_posture()
    }));
}

#[test]
fn host_capability_stop_postures_and_service_deferral_lower_through_selected_dispatch() {
    let app = service_touch_app();
    let touch = service_touch(&app);

    let available = execute_for_target(&app, &touch, available_host_capability_target(&touch));
    let missing = execute_for_target(&app, &touch, missing_host_capability_target(&touch));
    let ambiguous = execute_for_target(&app, &touch, ambiguous_host_capability_target(&touch));

    assert_eq!(
        available
            .dispatch
            .entries()
            .iter()
            .map(|entry| entry.selected().family())
            .collect::<Vec<_>>(),
        vec![
            UiObligationFamily::StructuralLegality,
            UiObligationFamily::PortalHostRequirement,
        ]
    );
    assert_eq!(
        available
            .verdicts
            .iter()
            .map(|verdict| (verdict.family(), verdict.class(), verdict.stop_posture()))
            .collect::<Vec<_>>(),
        vec![
            (
                Some(UiObligationFamily::StructuralLegality),
                UiObligationVerdictClass::Success,
                UiObligationDispatchStopPosture::None,
            ),
            (
                Some(UiObligationFamily::PortalHostRequirement),
                UiObligationVerdictClass::Advisory,
                UiObligationDispatchStopPosture::Deferred,
            ),
        ]
    );
    assert_eq!(
        missing.dispatch.plan_stop_posture(),
        UiObligationDispatchStopPosture::WrongHostCapability {
            required: UiAdmissionHostCapability::Available,
            observed: UiAdmissionHostCapability::Missing,
        }
    );
    assert_eq!(
        ambiguous.dispatch.plan_stop_posture(),
        UiObligationDispatchStopPosture::Ambiguous {
            required_query_basis: None,
            observed_query_basis: None,
            required_host_capability: Some(UiAdmissionHostCapability::Available),
            observed_host_capability: Some(UiAdmissionHostCapability::Ambiguous),
        }
    );
    assert!(missing.verdicts.iter().all(|verdict| {
        verdict.class() == UiObligationVerdictClass::Violation
            && verdict.stop_posture() == missing.dispatch.plan_stop_posture()
    }));
    assert!(ambiguous.verdicts.iter().all(|verdict| {
        verdict.class() == UiObligationVerdictClass::Violation
            && verdict.stop_posture() == ambiguous.dispatch.plan_stop_posture()
    }));
}

#[test]
fn budget_exceeded_stop_posture_overrides_selected_dispatch_execution() {
    let app = structural_touch_app();
    let touch = structural_touch(&app);
    let dispatch = execute_for_target(&app, &touch, budget_exceeded_target(&touch));

    assert_eq!(
        dispatch.dispatch.plan_stop_posture(),
        UiObligationDispatchStopPosture::BudgetExceeded {
            budget: UiAdmissionSelectionBudget::ordinary_lane_budget(0),
            attempted_lane_cost: 1,
        }
    );
    assert!(dispatch.verdicts.iter().all(|verdict| {
        verdict.class() == UiObligationVerdictClass::Violation
            && verdict.stop_posture() == dispatch.dispatch.plan_stop_posture()
    }));
    assert_eq!(
        dispatch
            .verdicts
            .iter()
            .map(|verdict| verdict.check_kind())
            .collect::<Vec<_>>(),
        vec![Some(UiObligationCheckKind::BlockingInvariant)]
    );
}
