use super::*;

const UNRELATED_RUN_COUNT: usize = 12;

#[test]
fn managed_run_work_is_invariant_to_unrelated_live_authority_width() {
    let disposed = Arc::new(AtomicUsize::new(0));
    let unrelated = (0..UNRELATED_RUN_COUNT)
        .map(|index| unrelated_artifact_run(index, Arc::clone(&disposed)))
        .collect::<Vec<_>>();

    let runtime = query_runtime();
    let plan = admitted_plan("cost-bound-target", 8);
    let operation = direct_authority(&runtime, &plan);
    let attempt = runtime
        .start_direct_resource_attempt(&operation, plan)
        .expect("target operation should start");
    let lower = causal_fixture::managed_admission_context();
    let admitted = runtime
        .managed_run_admission(&lower.bridge, &lower.relational)
        .admit_direct(&operation, attempt, lower.read_request())
        .expect("target run should admit independently of unrelated authority width");

    assert_exact_admission_work(admitted.counters());
    let terminal = admitted
        .start()
        .completed()
        .expect("target run with no provider work should complete");
    assert_exact_admission_work(terminal.counters());
    assert_eq!(terminal.provider_work().issued_call_count(), 0);
    let cleanup = terminal.cleanup().expect("target cleanup should complete");
    assert_exact_admission_work(cleanup.counters());
    assert_eq!(cleanup.attempt().capacity().released_reservation_count(), 1);
    assert!(cleanup.relational().released());
    assert!(cleanup.bridge().reservation_released());
    assert_eq!(disposed.load(Ordering::Acquire), 0);

    assert_rejection_work_is_constant_with_unrelated_authority();
    assert_eq!(disposed.load(Ordering::Acquire), 0);

    drop(unrelated);
    assert_eq!(disposed.load(Ordering::Acquire), UNRELATED_RUN_COUNT);
}

pub(super) fn unrelated_artifact_run(
    index: usize,
    disposed: Arc<AtomicUsize>,
) -> (
    crate::domain_computation::WorthQueryRunningWorkflowRun,
    crate::domain_computation::artifact_owner::WorthQueryMoveOnlyArtifactHandle,
) {
    let runtime = query_runtime();
    let operation_label = format!("unrelated-workflow-{index}");
    let stage_label = format!("unrelated-workflow-{index}:producer");
    let operation_resources = admitted_plan(&operation_label, 8);
    let stage_resources = admitted_plan(&stage_label, 4);
    let resources = WorthQueryAdmittedWorkflowResourcePlan::assemble(
        operation_resources,
        BTreeMap::from([("producer".to_owned(), stage_resources)]),
    );
    let output =
        crate::domain_computation::artifact_owner::installed_artifact_contract_for_managed_run();
    let operation =
        workflow_authority_with_output_artifact(&runtime, &resources, "producer", output);
    let attempt = runtime
        .start_workflow_resource_attempt(&operation, resources)
        .expect("unrelated workflow should reserve");
    let lower = causal_fixture::managed_admission_context();
    let running = runtime
        .managed_run_admission(&lower.bridge, &lower.relational)
        .admit_workflow(&operation, attempt, lower.read_request())
        .expect("unrelated workflow should admit")
        .start()
        .expect("unrelated workflow should start");
    let production = running
        .artifacts()
        .production_authority("producer")
        .expect("producer stage should validate")
        .expect("producer should own artifact authority");
    let admission =
        crate::domain_computation::artifact_owner::WorthQueryArtifactProductionAuthority::admit(
            &production,
            WorthQueryArtifactProductionEvidence::new(
                format!("unrelated-provenance-{index}"),
                format!("unrelated-dependency-{index}"),
            ),
        );
    let handle =
        crate::domain_computation::artifact_owner::WorthQueryArtifactProductionAuthority::register_exact(
            &production,
            admission,
            CostBoundArtifactResource(disposed),
        )
        .expect("unrelated artifact should register");
    (running, handle)
}

pub(super) fn assert_exact_admission_work(counters: &super::super::WorthQueryManagedRunCounters) {
    assert_eq!(counters.query_runtime_check_count(), 1);
    assert_eq!(counters.resource_attempt_check_count(), 1);
    assert_eq!(counters.bridge_intent_check_count(), 1);
    assert_eq!(counters.bridge_source_check_count(), 1);
    assert_eq!(counters.relational_basis_check_count(), 1);
    assert_eq!(counters.semantic_basis_check_count(), 1);
}

fn assert_rejection_work_is_constant_with_unrelated_authority() {
    let owner = query_runtime();
    let foreign = query_runtime();
    let plan = admitted_plan("cost-bound-rejection", 8);
    let operation = direct_authority(&owner, &plan);
    let attempt = owner
        .start_direct_resource_attempt(&operation, plan)
        .expect("rejection target should start");
    let lower = causal_fixture::causal_lower_execution_basis(
        operation.binding_identity(),
        attempt.attempt_identity().as_str(),
    );
    let rejection =
        match foreign.admit_direct_run(&operation, attempt, lower.bridge, lower.relational) {
            Ok(_) => panic!("foreign runtime admitted another runtime's run"),
            Err(rejection) => rejection,
        };
    assert_eq!(rejection.denial().counters().query_runtime_check_count(), 1);
    assert_eq!(
        rejection.denial().counters().resource_attempt_check_count(),
        0
    );
    assert_eq!(rejection.denial().counters().bridge_intent_check_count(), 0);
    assert_eq!(rejection.denial().counters().bridge_source_check_count(), 0);
    assert_eq!(
        rejection.denial().counters().relational_basis_check_count(),
        0
    );
    assert_eq!(
        rejection.denial().counters().semantic_basis_check_count(),
        0
    );
}

struct CostBoundArtifactResource(Arc<AtomicUsize>);

impl WorthQueryArtifactProviderResource for CostBoundArtifactResource {
    const PROVIDER_FAMILY: &'static str = "WORTH.tests.affinity.provider";

    fn canonical_semantic_projection(&self) -> Vec<u8> {
        b"unrelated-managed-artifact".to_vec()
    }

    fn retained_bytes(&self) -> usize {
        32
    }

    fn dispose(&mut self) {
        self.0.fetch_add(1, Ordering::AcqRel);
    }
}
