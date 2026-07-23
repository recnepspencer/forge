use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use worth_proof::TransitionOutcome;
use worth_query::facade::{domain, foundation, read, runtime};

use super::conditional_node_contract::{dependency, node};
use super::dependency_impact::bind_direct;
use super::installed_operation_fixture::conditional_workspace::{
    conditional_public_controlled_workspace_with, conditional_public_workspace_with,
};
use super::installed_operation_fixture::{
    conditional_installation_with_change, conditional_installation_with_repeated_value_changes,
    GeometryDomain, ReadExecutionInput, ReadFamily, ReadVertex,
};
use crate::support::public_bridge_runtime::PublicBridgeRuntimeHarness;

type LiveDirect = domain::WorthQueryLiveBoundDomainProjection<
    GeometryDomain,
    ReadVertex,
    ReadFamily,
    foundation::ObservationLaneWitness,
>;

#[test]
fn failed_emission_retries_the_retained_classification_without_signal_reentry() {
    let node = node(
        "dependency-impact-emission-retry",
        domain::WorthQueryComparatorRequirement::ExactCanonicalValue,
        domain::WorthQuerySemanticLocality::SourceRecord,
    );
    let location = domain::WorthQueryConditionalNodeLocation::operation(node.identity()).unwrap();
    let (installation, request, snapshots) = conditional_installation_with_change(&node);
    let harness = PublicBridgeRuntimeHarness::new();
    harness.set_relational_snapshot(snapshots[0].snapshot_id(), snapshots[0].version_id());
    let contacts = Arc::new(AtomicUsize::new(0));
    let mut workspace = conditional_public_controlled_workspace_with(
        "dependency-impact-emission-retry",
        node,
        installation,
        CountedCompute(Arc::clone(&contacts)),
        &harness,
    )
    .unwrap();
    let live = promote_live(&mut workspace);
    let TransitionOutcome::Success(delivery) = workspace
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
        panic!("the owner delivery should stage for the live target")
    };
    harness.set_relational_snapshot(snapshots[1].snapshot_id(), snapshots[1].version_id());
    let before = contacts.load(Ordering::SeqCst);
    workspace.fail_next_classified_live_emissions(1);

    let error = match live.refresh_owner_delivery(&delivery, &mut workspace) {
        Ok(_) => panic!("the injected emission failure must escape refresh"),
        Err(error) => error,
    };
    assert!(error.owner_delivery_retained());
    assert_eq!(error.work().conditional_compute_contacts(), 1);
    assert_eq!(error.work().impact_classifications(), 1);
    assert_eq!(error.work().drain_calls(), 0);
    assert_eq!(contacts.load(Ordering::SeqCst), before + 1);

    let retried = live
        .refresh_owner_delivery(&delivery, &mut workspace)
        .expect("the exact classified delivery should retry");
    assert_eq!(contacts.load(Ordering::SeqCst), before + 1);
    assert_eq!(retried.work().conditional_dependency_checks(), 0);
    assert_eq!(retried.work().conditional_compute_contacts(), 0);
    assert_eq!(retried.work().impact_classifications(), 0);
    assert_eq!(retried.work().impact_classification_reuses(), 1);
    assert_eq!(retried.delivery().batches().len(), 1);
    assert!(live.refresh(&mut workspace).unwrap().delivery().is_empty());
}

#[test]
fn one_owner_decision_is_shared_across_two_live_targets() {
    let node = advancing_output_node("dependency-impact-shared-decision");
    let location = domain::WorthQueryConditionalNodeLocation::operation(node.identity()).unwrap();
    let (installation, shared_request, snapshots) = conditional_installation_with_change(&node);
    let harness = PublicBridgeRuntimeHarness::new();
    harness.set_relational_snapshot(snapshots[0].snapshot_id(), snapshots[0].version_id());
    let contacts = Arc::new(AtomicUsize::new(0));
    let mut workspace = conditional_public_workspace_with(
        "dependency-impact-shared-decision",
        node,
        installation,
        CountedCompute(Arc::clone(&contacts)),
        &harness,
    )
    .unwrap();
    let first = promote_live(&mut workspace);
    let second = promote_live(&mut workspace);
    let before = contacts.load(Ordering::SeqCst);
    let TransitionOutcome::Success(delivery) = workspace
        .deliver_conditional_authoritative_change(
            GeometryDomain,
            ReadVertex,
            ReadFamily,
            domain::WorthQueryConditionalAuthoritativeChangeDeliveryRequest::new(
                location.clone(),
                0,
                shared_request,
            ),
        )
        .unwrap()
    else {
        panic!("the owner delivery should stage for both live targets")
    };
    harness.set_relational_snapshot(snapshots[1].snapshot_id(), snapshots[1].version_id());

    let first_refresh = first
        .refresh_owner_delivery(&delivery, &mut workspace)
        .unwrap();
    let second_refresh = second
        .refresh_owner_delivery(&delivery, &mut workspace)
        .unwrap();
    assert_eq!(contacts.load(Ordering::SeqCst), before + 1);
    assert_eq!(first_refresh.work().impact_classifications(), 1);
    assert_eq!(second_refresh.work().impact_classifications(), 1);
    assert_eq!(second_refresh.work().impact_classification_reuses(), 0);
    assert_eq!(second_refresh.work().conditional_decision_reuses(), 1);
    assert_eq!(
        second_refresh
            .work()
            .conditional_reentry_runtime_key_checks(),
        1
    );
    assert_eq!(
        second_refresh
            .work()
            .conditional_reentry_lowering_identity_checks(),
        1
    );
    assert_eq!(
        second_refresh
            .work()
            .conditional_reentry_installed_lowering_lookups(),
        1
    );
    assert_eq!(
        second_refresh
            .work()
            .conditional_reentry_signal_graph_checks(),
        1
    );
    assert_eq!(
        second_refresh
            .work()
            .conditional_reentry_signal_contract_checks(),
        1
    );
    assert_eq!(
        second_refresh
            .work()
            .conditional_reentry_snapshot_identity_checks(),
        2
    );
    assert_eq!(
        second_refresh.work().conditional_reentry_query_rebindings(),
        1
    );
    assert_eq!(
        second_refresh
            .work()
            .conditional_reentry_unrelated_lowering_scans(),
        0
    );
    assert_eq!(second_refresh.work().conditional_compute_contacts(), 0);
    assert_eq!(
        first_refresh.impact().class(),
        second_refresh.impact().class()
    );
    assert_eq!(
        first_refresh.impact().affected_roles(),
        second_refresh.impact().affected_roles()
    );
    assert_eq!(first_refresh.delivery().batches().len(), 1);
    assert_eq!(second_refresh.delivery().batches().len(), 1);
}

#[test]
fn later_owner_receipt_cannot_overtake_the_target_queue() {
    let node = node(
        "dependency-impact-owner-order",
        domain::WorthQueryComparatorRequirement::ExactCanonicalValue,
        domain::WorthQuerySemanticLocality::SourceRecord,
    );
    let location = domain::WorthQueryConditionalNodeLocation::operation(node.identity()).unwrap();
    let (installation, [first_request, later_request], snapshots) =
        conditional_installation_with_repeated_value_changes(&node);
    let harness = PublicBridgeRuntimeHarness::new();
    harness.set_relational_snapshot(snapshots[0].snapshot_id(), snapshots[0].version_id());
    let contacts = Arc::new(AtomicUsize::new(0));
    let mut workspace = conditional_public_workspace_with(
        "dependency-impact-owner-order",
        node,
        installation,
        CountedCompute(Arc::clone(&contacts)),
        &harness,
    )
    .unwrap();
    let live = promote_live(&mut workspace);
    let TransitionOutcome::Success(first) = workspace
        .deliver_conditional_authoritative_change(
            GeometryDomain,
            ReadVertex,
            ReadFamily,
            domain::WorthQueryConditionalAuthoritativeChangeDeliveryRequest::new(
                location.clone(),
                0,
                first_request,
            ),
        )
        .unwrap()
    else {
        panic!("the first owner receipt should stage")
    };
    let TransitionOutcome::Success(later) = workspace
        .deliver_conditional_authoritative_change(
            GeometryDomain,
            ReadVertex,
            ReadFamily,
            domain::WorthQueryConditionalAuthoritativeChangeDeliveryRequest::new(
                location.clone(),
                0,
                later_request,
            ),
        )
        .unwrap()
    else {
        panic!("the later owner receipt should stage")
    };
    let before = contacts.load(Ordering::SeqCst);

    let error = match live.refresh_owner_delivery(&later, &mut workspace) {
        Ok(_) => panic!("a later receipt must not overtake the target queue"),
        Err(error) => error,
    };
    let domain::WorthQueryLiveProjectionRefreshError::Impact { denial, work, .. } = error else {
        panic!("out-of-order owner delivery should stop at causal admission")
    };
    assert_eq!(
        denial.kind(),
        domain::WorthQueryImpactAdmissionDenialKind::OwnerDeliveryOutOfOrder
    );
    assert_eq!(denial.counters().owner_order_checks, 1);
    assert_eq!(denial.counters().owner_changes_inspected, 1);
    assert_eq!(contacts.load(Ordering::SeqCst), before);
    assert_eq!(work.conditional_dependency_checks(), 0);
    assert_eq!(work.drain_calls(), 0);
    assert_eq!(work.read_calls(), 0);
    assert_eq!(work.projection_calls(), 0);

    harness.set_relational_snapshot(snapshots[1].snapshot_id(), snapshots[1].version_id());
    assert_eq!(
        live.refresh_owner_delivery(&first, &mut workspace)
            .unwrap()
            .delivery()
            .batches()
            .len(),
        1
    );
    harness.set_relational_snapshot(snapshots[2].snapshot_id(), snapshots[2].version_id());
    assert_eq!(
        live.refresh_owner_delivery(&later, &mut workspace)
            .unwrap()
            .delivery()
            .batches()
            .len(),
        1
    );
}

fn promote_live(workspace: &mut runtime::WorthQueryWorkspace) -> LiveDirect {
    let installed = workspace.domain(GeometryDomain).unwrap();
    let bound = bind_direct(workspace, &installed);
    let consumer = bound.consumer_projection_contract().unwrap();
    let settled = bound
        .execute(ReadExecutionInput::default(), workspace)
        .unwrap()
        .publish()
        .unwrap()
        .consume(consumer, read::project_facts().entity_identities())
        .unwrap()
        .settle()
        .unwrap();
    match settled.into_lifecycle().promote(workspace) {
        domain::WorthQueryProjectionPromotionOutcome::Promoted(live) => live,
        _ => panic!("the settled projection should promote"),
    }
}

struct CountedCompute(Arc<AtomicUsize>);

struct VersionComparatorFamily;

impl domain::WorthQueryComparatorFamily for VersionComparatorFamily {
    const PORTABLE_IDENTITY: &'static str = "worth.tests.comparators.version-change";
}

struct FanoutRefresh;

impl domain::WorthQueryOnDemandTriggerFamily for FanoutRefresh {
    const PORTABLE_IDENTITY: &'static str = "worth.tests.triggers.fanout-refresh";
}

fn advancing_output_node(identity: &str) -> domain::WorthQueryPortableConditionalNodeDeclaration {
    let dependency = dependency(domain::WorthQuerySemanticLocality::SourceRecord);
    domain::WorthQueryPortableConditionalNodeDeclaration::declare(
        identity,
        domain::WorthQueryConditionalNodeRole::Computed,
    )
    .dependencies([dependency.clone()])
    .outputs([domain::WorthQueryConditionalNodeOutput::OperationOutput {
        projection_role: domain::WorthQueryOperationProjectionRole::new("vertex").unwrap(),
    }])
    .required_context([domain::WorthQueryConditionalNodeContext::Basis])
    .evaluation(
        domain::WorthQueryConditionalEvaluationCondition::on_demand(),
        domain::WorthQueryConditionalTrigger::on_demand::<FanoutRefresh>(),
    )
    .comparison(
        domain::WorthQueryComparatorRequirement::registered::<VersionComparatorFamily>(),
        domain::WorthQueryOutputEquivalenceRequirement::registered::<VersionComparatorFamily>(),
    )
    .artifact_policy(
        domain::WorthQueryArtifactReuseEquivalence::NotReusable,
        domain::WorthQueryMaintenancePosture::OnDemandOnly,
        domain::WorthQueryArtifactPosture::Ephemeral,
    )
    .output_relationship(domain::WorthQueryOutputRelationship::ContributesToOperationOutput)
    .finish()
    .unwrap()
}

impl domain::WorthQueryConditionalNodeComputeProvider<GeometryDomain, ReadVertex, ReadFamily>
    for CountedCompute
{
    type SemanticContract = ();

    fn semantic_contract(&self) -> Self::SemanticContract {}

    fn compute(
        &self,
        _context: &domain::WorthQueryConditionalComputeContext,
    ) -> Result<worth_signal::facade::NodeEvaluationResult, String> {
        let version = self.0.fetch_add(1, Ordering::SeqCst) as u64 + 1;
        Ok(worth_signal::facade::NodeEvaluationResult::from_version(
            worth_signal::facade::AspectVersion::from_updates([(
                worth_signal::facade::Aspect::new(0),
                version,
            )]),
        ))
    }
}
