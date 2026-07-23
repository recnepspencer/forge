use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use worth_proof::TransitionOutcome;
use worth_query::facade::{domain, foundation, read};

use super::conditional_node_contract::{
    conditional_node_result, distance_dependency, threshold, Millimeters,
};
use super::installed_operation_fixture::{
    conditional_installation_with_change, conditional_public_workspace_with, GeometryDomain,
    ReadExecutionInput, ReadFamily, ReadVertex,
};
use crate::support::public_bridge_runtime::PublicBridgeRuntimeHarness;

#[test]
fn typed_threshold_uses_authoritative_snapshots_and_signal_owned_comparison() {
    let dependency = distance_dependency();
    let node = conditional_node_result(
        "distance-threshold",
        dependency.clone(),
        domain::WorthQueryConditionalEvaluationCondition::delta_threshold(
            dependency,
            threshold::<Millimeters>(),
        ),
        domain::WorthQueryConditionalTrigger::DependencyChange,
        domain::WorthQueryMaintenancePosture::LazyUntilObserved,
    )
    .unwrap();
    let (installation, change, snapshots) = conditional_installation_with_change(&node);
    let contacts = Arc::new(AtomicUsize::new(0));
    let harness = PublicBridgeRuntimeHarness::new();
    harness.set_relational_snapshot(snapshots[0].snapshot_id(), snapshots[0].version_id());
    let mut workspace = conditional_public_workspace_with(
        "typed-threshold-execution",
        node,
        installation,
        ThresholdCompute(Arc::clone(&contacts)),
        &harness,
    )
    .unwrap();
    let installed = workspace.domain(GeometryDomain).unwrap();

    let first = bind(&workspace, &installed);
    let baseline = match first.execute(ReadExecutionInput::default(), &mut workspace) {
        TransitionOutcome::Deferred(baseline) => baseline,
        TransitionOutcome::Failed(denial) | TransitionOutcome::Denied(denial) => {
            panic!(
                "threshold baseline denied as {:?}: {}",
                denial.kind(),
                denial.detail()
            )
        }
        _ => panic!("the first semantic observation must establish a baseline"),
    };
    assert_eq!(
        baseline.conditional_provenance()[0].class(),
        domain::WorthQueryConditionalOutcomeClass::Suppressed
    );
    assert_eq!(baseline.counters().conditional_semantic_reads, 1);
    assert_eq!(contacts.load(Ordering::SeqCst), 0);

    let location =
        domain::WorthQueryConditionalNodeLocation::operation("distance-threshold").unwrap();
    let TransitionOutcome::Success(delivery) = workspace
        .deliver_conditional_authoritative_change(
            GeometryDomain,
            ReadVertex,
            ReadFamily,
            domain::WorthQueryConditionalAuthoritativeChangeDeliveryRequest::new(
                location.clone(),
                0,
                change,
            ),
        )
        .unwrap()
    else {
        panic!("the authoritative distance patch must dirty the retained Signal dependency")
    };
    assert_eq!(delivery.signal_seeds_emitted(), 1);
    harness.set_relational_snapshot(snapshots[1].snapshot_id(), snapshots[1].version_id());

    let second = bind(&workspace, &installed);
    let consumer = second.consumer_projection_contract().unwrap();
    let executed = match second.execute(ReadExecutionInput::default(), &mut workspace) {
        TransitionOutcome::Success(executed) => executed,
        TransitionOutcome::Deferred(deferred) => panic!(
            "threshold remained deferred as {:?} after {} semantic observations",
            deferred.conditional_provenance()[0].class(),
            deferred.conditional_provenance()[0].semantic_observation_count()
        ),
        TransitionOutcome::Failed(denial) | TransitionOutcome::Denied(denial) => {
            panic!(
                "threshold execution denied as {:?}: {}",
                denial.kind(),
                denial.detail()
            )
        }
        _ => panic!("a 0.02 millimeter delta must cross the inclusive 0.01 threshold"),
    };
    assert_eq!(
        executed.conditional_provenance()[0].class(),
        domain::WorthQueryConditionalOutcomeClass::ComputedChanged
    );
    assert_eq!(
        executed.conditional_provenance()[0].semantic_observation_count(),
        1
    );
    assert_eq!(
        executed.conditional_provenance()[0]
            .semantic_observation(0)
            .unwrap()
            .dependency_ordinal(),
        0
    );
    assert_eq!(executed.counters().conditional_semantic_reads, 1);
    assert_eq!(contacts.load(Ordering::SeqCst), 1);
    let settled = executed
        .publish()
        .unwrap()
        .consume(consumer, read::project_facts().entity_identities())
        .unwrap()
        .settle()
        .unwrap();
    assert_eq!(
        settled.conditional_provenance()[0].class(),
        domain::WorthQueryConditionalOutcomeClass::ComputedChanged
    );
}

fn bind(
    workspace: &worth_query::facade::runtime::WorthQueryWorkspace,
    installed: &domain::WorthQueryInstalledDomainHandle<GeometryDomain>,
) -> domain::WorthQueryBoundDomainOperation<
    GeometryDomain,
    ReadVertex,
    ReadFamily,
    foundation::ObservationLaneWitness,
> {
    workspace
        .observe_operating_world()
        .unwrap()
        .family(ReadFamily)
        .bind(installed, ReadVertex)
        .unwrap()
}

pub(super) struct ThresholdCompute(pub(super) Arc<AtomicUsize>);

impl domain::WorthQueryConditionalNodeComputeProvider<GeometryDomain, ReadVertex, ReadFamily>
    for ThresholdCompute
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
                2,
            )]),
        ))
    }
}
