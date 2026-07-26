use super::tests::{contract, envelope, request, support_with_capacity};
use super::*;

#[test]
fn consuming_capacity_reservation_saturates_and_drop_releases_the_exact_pool() {
    let capacity = std::sync::Arc::new(
        WorthQueryFixedExecutionCapacity::mint("one-shot-capacity", 1).unwrap(),
    );
    let support = support_with_capacity(envelope(8), capacity);
    let first = admitted_with_support("binding", support.clone());
    let second = admitted_with_support("binding", support.clone());
    let retry = admitted_with_support("binding", support);

    let reserved = reserve_execution_resource_plan(first).expect("first arrival reserves");
    assert!(reserve_execution_resource_plan(second).is_none());
    drop(reserved);

    assert!(reserve_execution_resource_plan(retry).is_some());
}

#[test]
fn equal_capacity_labels_cannot_merge_distinct_provider_authorities() {
    let first: std::sync::Arc<dyn WorthQueryExecutionCapacityPort> =
        std::sync::Arc::new(WorthQueryFixedExecutionCapacity::new("shared-label", 1).unwrap());
    let second: std::sync::Arc<dyn WorthQueryExecutionCapacityPort> =
        std::sync::Arc::new(WorthQueryFixedExecutionCapacity::new("shared-label", 1).unwrap());
    let operation = admitted_with_support("operation", support_with_capacity(envelope(8), first));
    let stage = admitted_with_support(
        "operation:stage",
        support_with_capacity(envelope(8), second),
    );
    let workflow = WorthQueryAdmittedWorkflowResourcePlan::assemble(
        operation,
        [("stage".to_owned(), stage)].into_iter().collect(),
    );

    assert!(reserve_workflow_resource_plan(workflow).is_none());
}

fn admitted_with_support(
    binding: &str,
    support: WorthQueryExecutionResourceSupportSnapshot,
) -> WorthQueryAdmittedExecutionResourcePlan {
    admit_execution_resource_plan(
        binding,
        &contract(8),
        &request(8),
        support,
        WorthQueryExecutionResourceAdmissionCounters::default(),
    )
    .unwrap()
}
