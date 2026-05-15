use super::*;

#[test]
fn intent_admission_inventory_lists_current_runtime_floor() {
    let inventory = forge_query_intent_admission_coverage_inventory();
    let implemented = inventory
        .rows()
        .iter()
        .filter(|row| row.status() == ForgeQueryIntentAdmissionCoverageStatus::Implemented)
        .map(|row| row.entrypoint())
        .collect::<Vec<_>>();

    assert!(implemented.contains(&ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent));
    assert!(implemented
        .contains(&ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteNextEffectWriteIntent));
}

#[test]
fn intent_admission_family_inventory_freezes_the_phase_one_family_map() {
    let family_inventory = forge_query_intent_admission_family_inventory();
    let families = family_inventory
        .rows()
        .iter()
        .map(|row| row.family())
        .collect::<Vec<_>>();

    assert_eq!(
        families,
        vec![
            ForgeQueryIntentAdmissionFamily::AuthoritativeUserIntent,
            ForgeQueryIntentAdmissionFamily::EffectTriggeredWriteIntent,
            ForgeQueryIntentAdmissionFamily::BasisUseIntent,
            ForgeQueryIntentAdmissionFamily::ProjectionConsumptionIntent,
            ForgeQueryIntentAdmissionFamily::ReadExecutionIntent,
            ForgeQueryIntentAdmissionFamily::InspectionMaterializationIntent,
        ]
    );
    assert_eq!(
        family_inventory.rows()[0].common_path_front_door().label(),
        "runtime.intent(declaration).execute()"
    );
    assert_eq!(
        family_inventory.rows()[1]
            .advanced_path_front_door()
            .label(),
        "runtime.next_effect_write_intent(&effect, version, contract).review()?.admit()?.execute()"
    );
    assert_eq!(
        family_inventory.rows()[2].common_path_front_door().label(),
        "forge_query_basis_observation_intent(raw).admit()"
    );
    assert_eq!(
        family_inventory.rows()[4]
            .common_path_front_door()
            .deferred_reason(),
        Some("read-execution-neighbor-deferred-until-covered")
    );
}

#[test]
fn planned_neighbors_do_not_claim_a_real_execution_seam() {
    let inventory = forge_query_intent_admission_coverage_inventory();

    for row in inventory
        .rows()
        .iter()
        .filter(|row| row.status() == ForgeQueryIntentAdmissionCoverageStatus::PlannedNeighbor)
    {
        assert_eq!(row.execution_seam(), None);
        assert_eq!(
            row.execution_boundary(),
            ForgeQueryIntentAdmissionExecutionBoundary::DeferredNeighbor(
                "neighbor-deferred-until-covered"
            )
        );
    }
}

#[test]
fn support_matrix_matches_phase_one_inventory_and_freezes_deferred_neighbors() {
    let inventory = forge_query_intent_admission_coverage_inventory();
    let support_matrix = forge_query_intent_admission_support_matrix();

    assert_eq!(inventory.rows().len(), support_matrix.rows().len());

    for (coverage, support) in inventory.rows().iter().zip(support_matrix.rows().iter()) {
        assert_eq!(coverage.family(), support.family());
        assert_eq!(coverage.entrypoint(), support.entrypoint());
        assert_eq!(coverage.execution_boundary(), support.execution_boundary());

        match coverage.status() {
            ForgeQueryIntentAdmissionCoverageStatus::Implemented => {
                assert_eq!(
                    support.posture(),
                    ForgeQueryIntentAdmissionSupportPosture::Admitted
                );
                assert_ne!(
                    coverage.raw_authoring_constructor().deferred_reason(),
                    Some("deferred-until-covered")
                );
                assert_ne!(
                    coverage.common_path_front_door().deferred_reason(),
                    Some("deferred-until-covered")
                );
                assert_ne!(
                    coverage.advanced_path_front_door().deferred_reason(),
                    Some("deferred-until-covered")
                );
                assert!(matches!(
                    support.detail(),
                    ForgeQueryIntentAdmissionSupportDetail::ImplementedRuntimeIntentFloor
                        | ForgeQueryIntentAdmissionSupportDetail::ImplementedBasisObservationScope
                        | ForgeQueryIntentAdmissionSupportDetail::ImplementedProjectionConsumptionContract
                ));
            }
            ForgeQueryIntentAdmissionCoverageStatus::PlannedNeighbor => {
                assert_eq!(
                    support.posture(),
                    ForgeQueryIntentAdmissionSupportPosture::Deferred
                );
                assert_eq!(
                    coverage.raw_authoring_constructor().deferred_reason(),
                    Some(coverage.raw_authoring_constructor().label())
                );
                assert_eq!(
                    coverage.common_path_front_door().deferred_reason(),
                    Some(coverage.common_path_front_door().label())
                );
                assert_eq!(
                    coverage.advanced_path_front_door().deferred_reason(),
                    Some(coverage.advanced_path_front_door().label())
                );
            }
        }
    }
}

#[test]
fn family_inventory_stays_in_sync_with_coverage_inventory_by_family() {
    let family_inventory = forge_query_intent_admission_family_inventory();
    let coverage_inventory = forge_query_intent_admission_coverage_inventory();

    for family_row in family_inventory.rows() {
        let matching_coverage = coverage_inventory
            .rows()
            .iter()
            .find(|coverage| coverage.family() == family_row.family())
            .expect("every family row should have a matching coverage row");

        assert_eq!(
            family_row.raw_authoring_constructor(),
            matching_coverage.raw_authoring_constructor()
        );
        assert_eq!(
            family_row.common_path_front_door(),
            matching_coverage.common_path_front_door()
        );
        assert_eq!(
            family_row.advanced_path_front_door(),
            matching_coverage.advanced_path_front_door()
        );
    }
}

#[test]
fn coverage_inventory_carries_phase_one_required_metadata_as_typed_fields() {
    let inventory = forge_query_intent_admission_coverage_inventory();
    let execute_intent = inventory
        .rows()
        .iter()
        .find(|row| row.entrypoint() == ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent)
        .expect("execute_intent row should exist");
    let deferred_read = inventory
        .rows()
        .iter()
        .find(|row| {
            row.entrypoint()
                == ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteReadNeighborDeferred
        })
        .expect("deferred read row should exist");

    assert_eq!(
        execute_intent.eligibility_authority(),
        ForgeQueryIntentAdmissionEligibilityAuthority::RuntimeIntentAuthorityAdapter
    );
    assert_eq!(
        execute_intent.admitted_plan_kind(),
        ForgeQueryIntentAdmissionPlanKind::AuthoritativeIntentExecutionPlan
    );
    assert_eq!(
        execute_intent.admitted_execution_handoff(),
        ForgeQueryIntentAdmissionExecutionHandoffInventory::Available(
            "ForgeQueryAdmittedIntentExecutionHandoff"
        )
    );

    let basis_observation = inventory
        .rows()
        .iter()
        .find(|row| {
            row.entrypoint() == ForgeQueryIntentAdmissionCoveredEntrypoint::BasisObservation
        })
        .expect("basis observation row should exist");
    let projection = inventory
        .rows()
        .iter()
        .find(|row| {
            row.entrypoint() == ForgeQueryIntentAdmissionCoveredEntrypoint::ProjectionConsumption
        })
        .expect("projection row should exist");
    assert_eq!(
        execute_intent.result_artifact(),
        ForgeQueryIntentAdmissionResultArtifact::ForgeQueryIntentReceipt
    );
    assert_eq!(
        execute_intent.advisory_decision_class(),
        ForgeQueryIntentAdmissionDecisionClass::AdvisoryNotYetExercisedOnCoveredEntrypoint
    );
    assert_eq!(
        execute_intent.violation_decision_class(),
        ForgeQueryIntentAdmissionDecisionClass::AdmissionOrExecutionViolation
    );
    assert_eq!(
        execute_intent.common_path_front_door(),
        ForgeQueryIntentAdmissionSurfaceDescriptor::Available(
            "runtime.intent(declaration).execute()"
        )
    );

    assert_eq!(
        deferred_read.eligibility_authority(),
        ForgeQueryIntentAdmissionEligibilityAuthority::DeferredReadExecutionAuthority
    );
    assert_eq!(
        deferred_read.admitted_plan_kind(),
        ForgeQueryIntentAdmissionPlanKind::DeferredReadExecutionPlan
    );
    assert_eq!(
        deferred_read
            .admitted_execution_handoff()
            .no_execution_handoff_reason(),
        Some("read-execution-neighbor-deferred-until-covered")
    );
    assert_eq!(
        deferred_read.result_artifact(),
        ForgeQueryIntentAdmissionResultArtifact::DeferredReadExecutionArtifact
    );
    assert_eq!(
        deferred_read.advisory_decision_class(),
        ForgeQueryIntentAdmissionDecisionClass::DeferredNeighborSupport
    );
    assert_eq!(
        deferred_read.violation_decision_class(),
        ForgeQueryIntentAdmissionDecisionClass::NeighborUnsupportedUntilCoverage
    );
    assert_eq!(
        deferred_read.raw_authoring_constructor().deferred_reason(),
        Some("read-execution-neighbor-deferred-until-covered")
    );

    assert_eq!(
        basis_observation.eligibility_authority(),
        ForgeQueryIntentAdmissionEligibilityAuthority::BasisLifecycleObservationAuthority
    );
    assert_eq!(
        basis_observation.admitted_plan_kind(),
        ForgeQueryIntentAdmissionPlanKind::BasisObservationPlan
    );
    assert_eq!(
        basis_observation.result_artifact(),
        ForgeQueryIntentAdmissionResultArtifact::ScopedObservationBasis
    );
    assert_eq!(
        basis_observation
            .admitted_execution_handoff()
            .no_execution_handoff_reason(),
        Some(
            "basis-observation-admitted-plan-scopes-to-lower-runtime-evidence-without-query-execution-handoff"
        )
    );

    assert_eq!(
        projection.eligibility_authority(),
        ForgeQueryIntentAdmissionEligibilityAuthority::ProjectionConsumptionEligibilityAuthority
    );
    assert_eq!(
        projection.admitted_plan_kind(),
        ForgeQueryIntentAdmissionPlanKind::ProjectionConsumptionPlan
    );
    assert_eq!(
        projection.result_artifact(),
        ForgeQueryIntentAdmissionResultArtifact::MaterializedProjectionContract
    );
    assert_eq!(
        projection
            .admitted_execution_handoff()
            .no_execution_handoff_reason(),
        Some("projection-consumption-admitted-plan-binds-contract-without-query-execution-handoff")
    );
}
