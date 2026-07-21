use super::active_application_session_test_support::{
    admit_candidate_complete_catalog, component_candidate_submission,
    source_backed_component_session,
};

pub(super) struct RegionalActivationInputs {
    pub(super) runtime: crate::runtime::WorthUiRuntime,
    pub(super) pending: crate::runtime::WorthUiPendingActivation,
    pub(super) admitted_catalog: crate::graph::UiAdmittedAllocationCatalogBasisSet,
}

pub(super) fn regional_activation_inputs() -> RegionalActivationInputs {
    let session = source_backed_component_session();
    let mut prepared = session
        .prepare_replacement(component_candidate_submission(
            &session,
            "regional-transaction-candidate",
            "workspace.component.active_session_candidate",
        ))
        .expect("real source-backed candidate prepares");
    let admitted_catalog = admit_candidate_complete_catalog(&mut prepared);
    let lowered = session
        .lower_prepared_replacement(*prepared)
        .expect("real source-backed candidate lowers");
    let pending_cutover = session
        .stage_prepared_replacement(lowered)
        .expect("real source-backed candidate stages");
    let (runtime, pending) =
        session.into_runtime_and_pending_after_staging_for_test(pending_cutover);
    RegionalActivationInputs {
        runtime,
        pending,
        admitted_catalog,
    }
}

pub(super) fn candidate_reclamation_probe(
    candidate_plan: &crate::runtime::WorthUiExecutionPlan,
) -> crate::runtime::planning::plan_topology::WorthUiPlanRegionStorageReclamationProbe {
    assert!(
        !candidate_plan.regional_evidence().transitions().is_empty(),
        "fixture must own candidate-only regional storage"
    );
    candidate_plan.region_storage_reclamation_probe_for_test()
}

pub(super) fn assert_commit_resource_denial(
    denial: crate::runtime::WorthUiAllocationCatalogActivationDenial,
    expected_active_successor_builds: u16,
) {
    let crate::runtime::WorthUiAllocationCatalogActivationDenial::Attempt(denial) = denial else {
        panic!("resource denial retains the canonical attempt")
    };
    assert!(matches!(
        denial.reason(),
        crate::runtime::UiCommittedAllocationActivationDenialReason::CommitResourceUnavailable
    ));
    assert_eq!(
        denial.evidence().counters().active_successor_builds(),
        expected_active_successor_builds
    );
    assert!(denial.evidence().live_state_unchanged());
}

pub(super) fn assert_candidate_reclaimed(
    probe: Option<
        crate::runtime::planning::plan_topology::WorthUiPlanRegionStorageReclamationProbe,
    >,
) {
    assert!(
        probe.expect("candidate probe captured").is_reclaimed(),
        "candidate-only regional root must be reclaimed exactly once on denial"
    );
}

pub(super) fn assert_active_unchanged(
    runtime: &crate::runtime::WorthUiRuntime,
    active_before: &crate::runtime::active::WorthUiActiveExecutionPlan,
    observation_before: crate::runtime::WorthUiActiveRuntimeObservation,
) {
    assert_eq!(runtime.inspect_active(), observation_before);
    assert_eq!(&runtime.active.active_plan(), active_before);
}
