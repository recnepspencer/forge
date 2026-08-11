use super::super::lane::{
    UnifiedFacadeLane, UnifiedFacadePerturbationClass, UnifiedFacadeRejection,
};
use super::super::row_catalog::UnifiedFacadeRejectionRowSpec;
use super::capability_lanes::{live_lane, query_read_lane};
use crate::application::WorthQueryApplicationFacade;
use crate::basis_lifecycle::basis_lifecycle;
use crate::facade::foundation::{WorthQueryConfig, WorthQueryQueryConfig, WorthQuerySignalConfig};
use crate::facade::policy::QueryContextBindingSource;
use crate::harness::certification::RejectionCertificationRow;
use crate::harness::fixtures::execution_preflights;

pub(super) fn rejection_row(
    spec: &UnifiedFacadeRejectionRowSpec,
) -> RejectionCertificationRow<
    UnifiedFacadePerturbationClass,
    UnifiedFacadeLane,
    UnifiedFacadeRejection,
> {
    let control_lane = query_read_lane();
    let parity_lane = live_lane();
    let hostile_lane = match spec.row_name {
        "missing-owning-live-section" => {
            let facade = WorthQueryApplicationFacade::new(
                WorthQueryConfig::runtime_backed_default()
                    .with_signal(WorthQuerySignalConfig::disabled()),
            )
            .expect("disabling live should retain a valid facade config");
            let error = facade
                .live_query_capability()
                .expect_err("disabled live capability should deny");
            UnifiedFacadeRejection::from_error(&error)
        }
        "invalid-workflow-support-posture" => {
            let facade = WorthQueryApplicationFacade::new(
                WorthQueryConfig::runtime_backed_default().with_relational(
                    crate::facade::foundation::WorthQueryRelationalConfig::enabled()
                        .with_workflow_orchestration(false)
                        .with_historical_evaluation(true),
                ),
            )
            .expect("disabling workflow inside an enabled relational section should preserve a valid facade config");
            let error = facade
                .workflow_query_capability()
                .expect_err("disabled workflow capability should deny");
            UnifiedFacadeRejection::from_error(&error)
        }
        "deferred-durable-artifacts" => {
            let facade = WorthQueryApplicationFacade::runtime_backed_default();
            let error = facade
                .durable_artifact_capability()
                .expect_err("durable artifacts should remain deferred debt");
            UnifiedFacadeRejection::from_error(&error)
        }
        "invalid-unified-configuration" => {
            let error = WorthQueryApplicationFacade::new(
                WorthQueryConfig::runtime_backed_default()
                    .with_query(WorthQueryQueryConfig::disabled())
                    .with_signal(WorthQuerySignalConfig::enabled()),
            )
            .expect_err("invalid unified config should deny before facade construction");
            UnifiedFacadeRejection::from_config_error(&error)
        }
        "broad-collection-diff-denied" => {
            let facade = WorthQueryApplicationFacade::runtime_backed_default();
            let contexts = facade
                .query_context_capability()
                .expect("query context capability should admit");
            let left_preflight =
                execution_preflights::ordered_collection_without_traversal_preflight();
            let right_preflight =
                execution_preflights::alternate_basis_ordered_collection_preflight();
            let left = contexts
                .capability()
                .admit_basis_context(
                    basis_lifecycle().current_head(),
                    QueryContextBindingSource::RuntimeCurrent(&left_preflight),
                )
                .expect("left context should admit");
            let right = contexts
                .capability()
                .admit_basis_context(
                    basis_lifecycle().branch_head("branch:ordered-collection", true),
                    QueryContextBindingSource::RuntimeBranch(&right_preflight),
                )
                .expect("right context should admit");
            let diff = contexts
                .capability()
                .bind_diff_context(&left, &right)
                .expect("diff context should bind");
            let left_execution = contexts
                .capability()
                .execute_basis_context(&left)
                .expect("left context should execute");
            let right_execution = contexts
                .capability()
                .execute_basis_context(&right)
                .expect("right context should execute");
            let error = contexts
                .capability()
                .shape_diff_result_bundle(&diff, &left_execution, &right_execution)
                .expect_err("broad collection diff should deny through the unified facade");
            UnifiedFacadeRejection::from_query_context_error(contexts.counters(), &error)
        }
        other => panic!("unexpected unified facade rejection row {other}"),
    };

    RejectionCertificationRow {
        row_name: spec.row_name,
        perturbation_class: spec.perturbation_class,
        control_lane,
        hostile_lane,
        parity_lane,
    }
}
