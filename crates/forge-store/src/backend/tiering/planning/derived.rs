use crate::{
    backend::{
        engine::{StateBackedStoreBackend, StatePersistence},
        tiering::{classification::summarize_window, observation},
    },
    failure::StoreError,
    tiering::{
        ColdDerivedFamilyPolicy, DerivedPlacementPlanningReport, DerivedTierMovePlan,
        FamilyLocalPlacementPlan, PlacementArtifactFamily, PlacementExecutionOrigin,
        PlacementObservationScopeClass, PlacementPolicyClass, TierLocalityFootprint,
        TierMoveBreadthSummary, TierMoveRejection, TierResidenceClass, WorkingSetDebtSummary,
    },
};

use super::shared::{ensure_family_artifact_present, move_budget_for_origin};

pub(crate) fn plan_derived_tier_move<P: StatePersistence>(
    backend: &StateBackedStoreBackend<P>,
    policy_class: PlacementPolicyClass,
    family: ColdDerivedFamilyPolicy,
    artifact_id: &str,
    execution_origin: PlacementExecutionOrigin,
) -> Result<DerivedPlacementPlanningReport, StoreError> {
    let family_key = family.label();
    let window = observation::observe_working_set(
        backend,
        PlacementObservationScopeClass::ArtifactFamily,
        family_key,
    )?;
    let demand_summary = summarize_window(&window);
    backend.counters().record_working_set_reclassifications(1);
    let locality_footprint = TierLocalityFootprint::new(
        PlacementObservationScopeClass::ArtifactFamily,
        family_key.to_string(),
        vec![artifact_key(family, artifact_id)],
    );
    let breadth_summary = TierMoveBreadthSummary::new(1, 1, 1);

    ensure_family_artifact_present(backend.state(), family, artifact_id)?;

    let report = match policy_class {
        PlacementPolicyClass::AdaptiveDebt(marker) => {
            backend.counters().record_placement_debt(1);
            backend.counters().record_working_set_debt(1);
            DerivedPlacementPlanningReport::new(
                demand_summary,
                None,
                None,
                locality_footprint,
                breadth_summary,
                Some(TierMoveRejection::UnsupportedPolicy { marker }),
                Some(WorkingSetDebtSummary::new(
                    marker,
                    "adaptive derived-family placement remains explicit milestone 13 debt",
                )),
            )
        }
        PlacementPolicyClass::Conservative(policy) => {
            if !policy.cold_derived_families().contains(&family) {
                backend.counters().record_tier_move_rejections(1);
                return Ok(DerivedPlacementPlanningReport::new(
                    demand_summary,
                    None,
                    None,
                    locality_footprint,
                    breadth_summary,
                    Some(TierMoveRejection::WitnessConstructionRequired {
                        witness_type: "derived family is not admitted by the conservative placement policy",
                    }),
                    None,
                ));
            }
            if execution_origin == PlacementExecutionOrigin::Foreground {
                backend.counters().record_tier_move_rejections(1);
                return Ok(DerivedPlacementPlanningReport::new(
                    demand_summary,
                    None,
                    None,
                    locality_footprint,
                    breadth_summary,
                    Some(TierMoveRejection::IllegalExecutionOrigin {
                        origin: execution_origin,
                    }),
                    None,
                ));
            }

            let family_local_plan =
                FamilyLocalPlacementPlan::new(locality_footprint.clone(), TierResidenceClass::Cold);
            let tier_move_plan = DerivedTierMovePlan::new(
                placement_artifact_family(family),
                artifact_id.to_string(),
                TierResidenceClass::Cold,
                move_budget_for_origin(execution_origin),
                execution_origin,
            );
            backend.counters().record_tier_move_plans(1);
            DerivedPlacementPlanningReport::new(
                demand_summary,
                Some(family_local_plan),
                Some(tier_move_plan),
                locality_footprint,
                breadth_summary,
                None,
                None,
            )
        }
    };

    Ok(report)
}

fn placement_artifact_family(family: ColdDerivedFamilyPolicy) -> PlacementArtifactFamily {
    match family {
        ColdDerivedFamilyPolicy::SnapshotFamily => PlacementArtifactFamily::SnapshotFamily,
        ColdDerivedFamilyPolicy::BranchDeltaFamily => PlacementArtifactFamily::BranchDeltaFamily,
        ColdDerivedFamilyPolicy::Milestone6LayoutFamily => {
            PlacementArtifactFamily::Milestone6LayoutFamily
        }
    }
}

fn artifact_key(family: ColdDerivedFamilyPolicy, artifact_id: &str) -> String {
    match family {
        ColdDerivedFamilyPolicy::SnapshotFamily => format!("snapshot:{artifact_id}"),
        ColdDerivedFamilyPolicy::BranchDeltaFamily => format!("branch_delta:{artifact_id}"),
        ColdDerivedFamilyPolicy::Milestone6LayoutFamily => {
            format!("milestone6_layout:{artifact_id}")
        }
    }
}
