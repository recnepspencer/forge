use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use worth_proof::TransitionOutcome;
use worth_query::facade::{domain, read};

use super::conditional_node_contract::node;
use super::dependency_impact::bind_direct;
use super::installed_operation_fixture::{
    conditional_causal_mismatch_installation, conditional_public_workspace_with, GeometryDomain,
    ReadExecutionInput, ReadFamily, ReadVertex,
};
use crate::support::public_bridge_runtime::PublicBridgeRuntimeHarness;

#[test]
fn wrong_owner_receipt_mutates_nothing_and_the_exact_receipt_can_retry() {
    let node = node(
        "dependency-impact-causal-mismatch",
        domain::WorthQueryComparatorRequirement::ExactCanonicalValue,
        domain::WorthQuerySemanticLocality::WholeLogicalGraph,
    );
    let location = domain::WorthQueryConditionalNodeLocation::operation(node.identity()).unwrap();
    let (installation, request, snapshots, switch) =
        conditional_causal_mismatch_installation(&node);
    let harness = PublicBridgeRuntimeHarness::new();
    harness.set_relational_snapshot(snapshots[0].snapshot_id(), snapshots[0].version_id());
    let compute_contacts = Arc::new(AtomicUsize::new(0));
    let mut workspace = conditional_public_workspace_with(
        "dependency-impact-causal-mismatch",
        node,
        installation,
        CountedConditionalCompute(compute_contacts.clone()),
        &harness,
    )
    .unwrap();

    let installed = workspace.domain(GeometryDomain).unwrap();
    let bound = bind_direct(&workspace, &installed);
    let consumer = bound.consumer_projection_contract().unwrap();
    let settled = bound
        .execute(ReadExecutionInput::default(), &mut workspace)
        .unwrap()
        .publish()
        .unwrap()
        .consume(consumer, read::project_facts().entity_identities())
        .unwrap()
        .settle()
        .unwrap();
    let owner_outcome = workspace
        .deliver_conditional_authoritative_change(
            GeometryDomain,
            ReadVertex,
            ReadFamily,
            domain::WorthQueryConditionalAuthoritativeChangeDeliveryRequest::new(
                location.clone(),
                0,
                request.clone(),
            ),
        )
        .unwrap();
    let TransitionOutcome::Success(owner_delivery) = owner_outcome else {
        panic!("the owner delivery should admit before live registration: {owner_outcome:?}")
    };
    assert_eq!(owner_delivery.change_set().changes().len(), 1);
    let live = match settled.into_lifecycle().promote(&mut workspace) {
        domain::WorthQueryProjectionPromotionOutcome::Promoted(live) => live,
        _ => panic!("the settled conditional projection should promote"),
    };

    switch.include_conflicting_change();
    let TransitionOutcome::Success(queued_delivery) = workspace
        .deliver_conditional_authoritative_change(
            GeometryDomain,
            ReadVertex,
            ReadFamily,
            domain::WorthQueryConditionalAuthoritativeChangeDeliveryRequest::new(
                location.clone(),
                0,
                request,
            ),
        )
        .unwrap()
    else {
        panic!("the changed delivery should reach the same installed owner")
    };
    assert_eq!(queued_delivery.change_set().changes().len(), 1);
    assert_eq!(
        queued_delivery.change_set().commit_identity(),
        owner_delivery.change_set().commit_identity()
    );
    assert_ne!(
        queued_delivery.change_set().changes(),
        owner_delivery.change_set().changes()
    );
    harness.set_relational_snapshot(snapshots[1].snapshot_id(), snapshots[1].version_id());
    let compute_contacts_before_wrong = compute_contacts.load(Ordering::SeqCst);

    let error = match live.refresh_owner_delivery(&owner_delivery, &mut workspace) {
        Ok(_) => panic!("same commit identity must not conceal a different change multiset"),
        Err(error) => error,
    };
    let domain::WorthQueryLiveProjectionRefreshError::Impact {
        denial,
        work,
        owner_delivery_retained,
    } = error
    else {
        panic!("causal mismatch should stop in impact admission")
    };
    assert_eq!(
        denial.kind(),
        domain::WorthQueryImpactAdmissionDenialKind::CausalDeliveryMismatch
    );
    assert_eq!(denial.counters().staged_changes_inspected, 1);
    assert_eq!(denial.counters().owner_changes_inspected, 1);
    assert_eq!(denial.counters().causal_keys_materialized, 2);
    assert_eq!(denial.counters().causal_key_lookups, 1);
    assert_eq!(denial.counters().owner_order_checks, 0);
    assert!(owner_delivery_retained);
    assert_eq!(
        compute_contacts.load(Ordering::SeqCst),
        compute_contacts_before_wrong
    );
    assert_eq!(work.authority_checks(), 1);
    assert_eq!(work.drain_calls(), 0);
    assert_eq!(work.delivery_batches(), 0);
    assert_eq!(work.causal_staged_changes_inspected(), 1);
    assert_eq!(work.causal_owner_changes_inspected(), 1);
    assert_eq!(work.causal_keys_materialized(), 2);
    assert_eq!(work.causal_key_lookups(), 1);
    assert_eq!(work.conditional_dependency_checks(), 0);
    assert_eq!(work.conditional_semantic_reads(), 0);
    assert_eq!(work.conditional_condition_checks(), 0);
    assert_eq!(work.conditional_comparator_checks(), 0);
    assert_eq!(work.conditional_compute_contacts(), 0);
    assert_eq!(work.conditional_semantic_changes(), 0);
    assert_eq!(work.conditional_reuse_checks(), 0);
    assert_eq!(work.read_calls(), 0);
    assert_eq!(work.projection_calls(), 0);
    assert_eq!(work.impact_classifications(), 0);

    let refreshed = live
        .refresh_owner_delivery(&queued_delivery, &mut workspace)
        .expect("the exact retained owner receipt should remain retryable");
    assert_eq!(
        compute_contacts.load(Ordering::SeqCst),
        compute_contacts_before_wrong + 1
    );
    assert_eq!(refreshed.work().causal_staged_changes_inspected(), 1);
    assert_eq!(refreshed.work().causal_owner_changes_inspected(), 1);
    assert_eq!(refreshed.work().causal_keys_materialized(), 2);
    assert_eq!(refreshed.work().causal_key_lookups(), 1);
    assert_eq!(refreshed.work().conditional_compute_contacts(), 1);
    assert_eq!(refreshed.work().impact_classifications(), 1);
    assert_eq!(refreshed.work().drain_calls(), 1);
    assert_eq!(refreshed.work().delivery_batches(), 1);
    assert_eq!(refreshed.work().read_calls(), 1);
    assert_eq!(refreshed.work().projection_calls(), 1);
}

struct CountedConditionalCompute(Arc<AtomicUsize>);

impl domain::WorthQueryConditionalNodeComputeProvider<GeometryDomain, ReadVertex, ReadFamily>
    for CountedConditionalCompute
{
    type SemanticContract = ();

    fn semantic_contract(&self) -> Self::SemanticContract {}

    fn compute(
        &self,
        _context: &domain::WorthQueryConditionalComputeContext,
    ) -> Result<worth_signal::facade::NodeEvaluationResult, String> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Ok(worth_signal::facade::NodeEvaluationResult::from_version(
            worth_signal::facade::AspectVersion::from_updates([(
                worth_signal::facade::Aspect::new(0),
                1,
            )]),
        ))
    }
}
