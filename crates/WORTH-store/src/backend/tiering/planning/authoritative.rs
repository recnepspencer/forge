use worth_relational::facade::history::BranchId;

use crate::{
    backend::{
        engine::{StateBackedStoreBackend, StatePersistence},
        tiering::{classification::summarize_window, observation},
    },
    failure::StoreError,
    tiering::{
        AuthoritativePlacementPlanningReport, AuthoritativeTierMovePlan, PlacementExecutionOrigin,
        PlacementObservationScopeClass, PlacementPolicyClass, RetainedRangePlacementPlan,
        TierLocalityFootprint, TierMoveBreadthSummary, TierMoveRejection, TierResidenceClass,
        WorkingSetDebtSummary,
    },
};

use super::shared::{branch_id_for_basis, move_budget_for_origin};

pub(crate) fn plan_authoritative_tier_move<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
    policy_class: PlacementPolicyClass,
    scope_class: PlacementObservationScopeClass,
    scope_key: &str,
    execution_origin: PlacementExecutionOrigin,
) -> Result<AuthoritativePlacementPlanningReport, StoreError> {
    let window = observation::observe_working_set(backend, scope_class, scope_key)?;
    let demand_summary = summarize_window(&window);
    backend.counters().record_working_set_reclassifications(1);
    let locality_footprint = TierLocalityFootprint::new(
        scope_class,
        scope_key.to_string(),
        window.observed_artifact_keys().to_vec(),
    );
    let breadth_summary =
        TierMoveBreadthSummary::new(window.observed_artifact_keys().len() as u64, 1, 1);

    let report = match policy_class {
        PlacementPolicyClass::AdaptiveDebt(marker) => {
            backend.counters().record_placement_debt(1);
            backend.counters().record_working_set_debt(1);
            AuthoritativePlacementPlanningReport::new(
                demand_summary,
                None,
                None,
                locality_footprint,
                breadth_summary,
                Some(TierMoveRejection::UnsupportedPolicy { marker }),
                Some(WorkingSetDebtSummary::new(
                    marker,
                    "adaptive placement policy remains explicit milestone 13 debt",
                )),
            )
        }
        PlacementPolicyClass::Conservative(_policy) => {
            let (retained_range_plan, tier_move_plan, rejection) = match scope_class {
                PlacementObservationScopeClass::Branch => branch_plan(scope_key, execution_origin),
                PlacementObservationScopeClass::RetainedBasis => {
                    retained_basis_plan(backend, scope_key, execution_origin)?
                }
                PlacementObservationScopeClass::ArtifactFamily => {
                    backend.counters().record_tier_move_rejections(1);
                    (
                        None,
                        None,
                        Some(TierMoveRejection::WitnessConstructionRequired {
                            witness_type: "authoritative placement planning requires branch or retained-basis scope",
                        }),
                    )
                }
            };

            if tier_move_plan.is_some() {
                backend.counters().record_tier_move_plans(1);
            }

            AuthoritativePlacementPlanningReport::new(
                demand_summary,
                retained_range_plan,
                tier_move_plan,
                locality_footprint,
                breadth_summary,
                rejection,
                None,
            )
        }
    };

    Ok(report)
}

fn branch_plan(
    scope_key: &str,
    execution_origin: PlacementExecutionOrigin,
) -> (
    Option<RetainedRangePlacementPlan>,
    Option<AuthoritativeTierMovePlan>,
    Option<TierMoveRejection>,
) {
    let branch_id = BranchId(scope_key.to_string());
    let plan = AuthoritativeTierMovePlan::new(
        format!("authoritative_branch_head:{scope_key}"),
        TierResidenceClass::Hot,
        move_budget_for_origin(execution_origin),
        execution_origin,
    );
    (
        Some(RetainedRangePlacementPlan::new(
            branch_id,
            format!("branch:{scope_key}"),
            TierResidenceClass::Hot,
        )),
        Some(plan),
        None,
    )
}

fn retained_basis_plan<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
    scope_key: &str,
    execution_origin: PlacementExecutionOrigin,
) -> Result<
    (
        Option<RetainedRangePlacementPlan>,
        Option<AuthoritativeTierMovePlan>,
        Option<TierMoveRejection>,
    ),
    StoreError,
> {
    let branch_id = branch_id_for_basis(backend.state(), scope_key)?;
    let plan = AuthoritativeTierMovePlan::new(
        format!("retained_authority:{scope_key}"),
        TierResidenceClass::Warm,
        move_budget_for_origin(execution_origin),
        execution_origin,
    );
    Ok((
        Some(RetainedRangePlacementPlan::new(
            branch_id,
            scope_key.to_string(),
            TierResidenceClass::Warm,
        )),
        Some(plan),
        None,
    ))
}
