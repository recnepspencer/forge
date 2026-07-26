use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use worth_proof::TransitionOutcome;
use worth_query::facade::{domain, read};

use super::conditional_node_contract::node;
use super::dependency_impact::{bind_direct, closure_summary};
use super::installed_operation_fixture::conditional_workspace::conditional_public_sibling_workspace_with_change;
use super::installed_operation_fixture::{
    conditional_installation_with_change, conditional_installation_with_repeated_value_changes,
    conditional_public_workspace_with, DirectConditionalCompute, GeometryDomain,
    ReadExecutionInput, ReadFamily, ReadVertex,
};
use crate::support::public_bridge_runtime::PublicBridgeRuntimeHarness;

mod conditional_scope;

#[test]
fn settled_and_live_refresh_preserve_one_capability_closure_and_impact() {
    let node = node(
        "dependency-impact-live",
        domain::WorthQueryComparatorRequirement::ExactCanonicalValue,
        domain::WorthQuerySemanticLocality::SourceRecord,
    );
    let location = domain::WorthQueryConditionalNodeLocation::operation(node.identity()).unwrap();
    let (installation, request, snapshots) = conditional_installation_with_change(&node);
    let harness = PublicBridgeRuntimeHarness::new();
    harness.set_relational_snapshot(snapshots[0].snapshot_id(), snapshots[0].version_id());
    let mut workspace = conditional_public_workspace_with(
        "dependency-impact-live",
        node,
        installation,
        DirectConditionalCompute,
        &harness,
    )
    .unwrap();
    let installed = workspace.domain(GeometryDomain).unwrap();
    let bound = bind_direct(&workspace, &installed);
    let consumer = bound.consumer_projection_contract().unwrap();
    let executed = bound
        .admit_execution_resources(
            ReadExecutionInput::default(),
            crate::suite::installed_operation_fixture::execution_resource_request(),
            &workspace,
        )
        .unwrap()
        .execute(&mut workspace)
        .unwrap();
    assert_eq!(
        executed.conditional_provenance()[0].class(),
        domain::WorthQueryConditionalOutcomeClass::ComputedChanged
    );
    let settled = executed
        .publish()
        .unwrap()
        .consume(consumer, read::project_facts().entity_identities())
        .unwrap()
        .settle()
        .unwrap();
    let closure = closure_summary(settled.semantic_aspect_dependency_closure());
    let live = match settled.into_lifecycle().promote(&mut workspace) {
        domain::WorthQueryProjectionPromotionOutcome::Promoted(live) => live,
        _ => panic!("settled output did not retain its unchanged conditional through promotion"),
    };
    assert_eq!(
        live.conditional_provenance()[0].class(),
        domain::WorthQueryConditionalOutcomeClass::DependencyUnchanged
    );
    assert_eq!(
        closure_summary(live.snapshot().semantic_aspect_dependency_closure()),
        closure
    );

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
        panic!("the owner change did not reach its installed conditional graph")
    };
    let before_classification = live.refresh(&mut workspace).unwrap();
    assert!(before_classification.delivery().is_empty());
    assert_eq!(before_classification.work().delivery_batches(), 0);
    assert_eq!(
        before_classification.impact().class(),
        domain::WorthQueryImpactClass::UnaffectedOrSuppressed
    );
    harness.set_relational_snapshot(snapshots[1].snapshot_id(), snapshots[1].version_id());
    let refreshed = live
        .refresh_owner_delivery(&delivery, &mut workspace)
        .unwrap();
    assert_eq!(
        refreshed.impact().class(),
        domain::WorthQueryImpactClass::ValuePatch
    );
    assert_eq!(
        refreshed.impact().affected_roles(),
        [
            domain::WorthQuerySemanticDependencyRole::ProjectedValue,
            domain::WorthQuerySemanticDependencyRole::ConditionalEligibilityOrSemanticCleanliness,
        ]
    );
    assert_eq!(refreshed.impact().owner_change_count(), 1);
    assert_eq!(refreshed.impact().affected_dependency_count(), 5);
    assert_eq!(refreshed.impact().counters().index_lookups, 5);
    assert_eq!(refreshed.work().authority_checks(), 1);
    assert_eq!(refreshed.work().drain_calls(), 1);
    assert_eq!(refreshed.work().delivery_batches(), 1);
    assert_eq!(refreshed.work().impact_classifications(), 1);
    assert_eq!(refreshed.work().read_calls(), 1);
    assert_eq!(refreshed.work().projection_calls(), 1);
}

#[test]
fn unchanged_conditional_omits_only_its_consequence_when_the_aspect_is_also_projected() {
    let (changed, unchanged) = authority_correct_overlap_impact_counters();
    assert_eq!(changed.affected_edges, 5);
    assert_eq!(unchanged.affected_edges, 3);
}

pub(super) fn authority_correct_overlap_impact_counters() -> (
    domain::WorthQueryImpactCounters,
    domain::WorthQueryImpactCounters,
) {
    let node = node(
        "dependency-impact-overlap-node",
        domain::WorthQueryComparatorRequirement::registered::<OverlapComparatorFamily>(),
        domain::WorthQuerySemanticLocality::SourceRecord,
    );
    let location = domain::WorthQueryConditionalNodeLocation::operation(node.identity()).unwrap();
    let (mut installation, requests, snapshots) =
        conditional_installation_with_repeated_value_changes(&node);
    let semantic_changes = Arc::new(AtomicUsize::new(0));
    installation.providers = worth_runtime_bridge::facade::BridgeConditionalProviderSet::new()
        .dependency_comparator(FirstOwnerChangeComparator(Arc::clone(&semantic_changes)));
    let [first_request, repeated_request] = requests;
    let harness = PublicBridgeRuntimeHarness::new();
    harness.set_relational_snapshot(snapshots[0].snapshot_id(), snapshots[0].version_id());
    let mut workspace = conditional_public_workspace_with(
        "dependency-impact-overlap",
        node,
        installation,
        DirectConditionalCompute,
        &harness,
    )
    .unwrap();
    let installed = workspace.domain(GeometryDomain).unwrap();
    let bound = bind_direct(&workspace, &installed);
    let consumer = bound.consumer_projection_contract().unwrap();
    let settled = bound
        .admit_execution_resources(
            ReadExecutionInput::default(),
            crate::suite::installed_operation_fixture::execution_resource_request(),
            &workspace,
        )
        .unwrap()
        .execute(&mut workspace)
        .unwrap()
        .publish()
        .unwrap()
        .consume(consumer, read::project_facts().entity_identities())
        .unwrap()
        .settle()
        .unwrap();
    let closure = settled.semantic_aspect_dependency_closure();
    let conditional_contract = closure
        .dependencies()
        .iter()
        .find_map(|dependency| match dependency.source() {
            domain::WorthQuerySemanticAspectDependencyView::ConditionalNodeContract(node) => {
                Some(node.dependencies()[0].contract())
            }
            _ => None,
        })
        .expect("the installed operation carries its conditional trigger");
    assert!(closure.dependencies().iter().any(|dependency| {
        matches!(
            dependency.source(),
            domain::WorthQuerySemanticAspectDependencyView::NativeProjection(projection)
                if projection.contract() == conditional_contract
        )
    }));
    let live = match settled.into_lifecycle().promote(&mut workspace) {
        domain::WorthQueryProjectionPromotionOutcome::Promoted(live) => live,
        _ => panic!("the unchanged overlap projection should promote"),
    };
    assert_eq!(
        live.conditional_provenance()[0].class(),
        domain::WorthQueryConditionalOutcomeClass::DependencyUnchanged
    );
    let TransitionOutcome::Success(first_delivery) = workspace
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
        panic!("the owner change should reach the promoted capability")
    };
    harness.set_relational_snapshot(snapshots[1].snapshot_id(), snapshots[1].version_id());
    let changed = live
        .refresh_owner_delivery(&first_delivery, &mut workspace)
        .unwrap();
    assert_eq!(
        changed.impact().affected_roles(),
        [
            domain::WorthQuerySemanticDependencyRole::ProjectedValue,
            domain::WorthQuerySemanticDependencyRole::ConditionalEligibilityOrSemanticCleanliness,
        ]
    );
    assert_eq!(changed.work().conditional_comparator_checks(), 1);
    assert_eq!(changed.work().conditional_compute_contacts(), 1);
    assert_eq!(changed.work().conditional_semantic_changes(), 1);

    let TransitionOutcome::Success(repeated_delivery) = workspace
        .deliver_conditional_authoritative_change(
            GeometryDomain,
            ReadVertex,
            ReadFamily,
            domain::WorthQueryConditionalAuthoritativeChangeDeliveryRequest::new(
                location.clone(),
                0,
                repeated_request,
            ),
        )
        .unwrap()
    else {
        panic!("the repeated authoritative value should retain its own owner receipt")
    };
    assert_ne!(
        first_delivery.change_set().commit_identity(),
        repeated_delivery.change_set().commit_identity()
    );
    harness.set_relational_snapshot(snapshots[2].snapshot_id(), snapshots[2].version_id());
    let unchanged = live
        .refresh_owner_delivery(&repeated_delivery, &mut workspace)
        .unwrap();
    let impact = unchanged.impact();

    assert_eq!(impact.class(), domain::WorthQueryImpactClass::ValuePatch);
    assert_eq!(
        impact.affected_roles(),
        [domain::WorthQuerySemanticDependencyRole::ProjectedValue]
    );
    assert_eq!(impact.affected_dependency_count(), 3);
    assert_eq!(impact.counters().affected_edges, 3);
    assert_eq!(impact.counters().conditional_outcomes_inspected, 1);
    assert_eq!(unchanged.work().conditional_dependency_checks(), 1);
    assert_eq!(unchanged.work().conditional_comparator_checks(), 1);
    assert_eq!(unchanged.work().conditional_compute_contacts(), 0);
    assert_eq!(unchanged.work().conditional_semantic_changes(), 0);
    assert_eq!(semantic_changes.load(Ordering::SeqCst), 2);
    (changed.impact().counters(), unchanged.impact().counters())
}

struct OverlapComparatorFamily;

impl domain::WorthQueryComparatorFamily for OverlapComparatorFamily {
    const PORTABLE_IDENTITY: &'static str = "worth.tests.comparators.first-owner-change";
}

struct FirstOwnerChangeComparator(Arc<AtomicUsize>);

impl worth_runtime_bridge::facade::BridgeConditionalProviderSemantics
    for FirstOwnerChangeComparator
{
    type SemanticContract = ();

    fn semantic_contract(&self) -> Self::SemanticContract {}
}

impl worth_runtime_bridge::facade::BridgeConditionalComparatorProvider
    for FirstOwnerChangeComparator
{
    fn has_meaningful_change(
        &self,
        _aspect: worth_signal::facade::Aspect,
        cached: u64,
        current: u64,
    ) -> Result<bool, String> {
        if cached == current {
            return Ok(false);
        }
        Ok(self.0.fetch_add(1, Ordering::SeqCst) == 0)
    }
}
