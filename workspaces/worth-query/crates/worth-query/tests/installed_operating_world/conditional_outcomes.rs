use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use worth_proof::TransitionOutcome;
use worth_query::facade::{domain, foundation};

use super::conditional_node_contract::{conditional_node_result, dependency, node, ManualRefresh};
use super::installed_operation_fixture::conditional_workspace::shared_signal_node_workspace;
use super::installed_operation_fixture::{
    conditional_installation, conditional_installation_with_change, conditional_workspace_with,
    GeometryDomain, ReadExecutionInput, ReadFamily, ReadVertex,
};

#[test]
fn unchanged_correspondence_versions_stop_before_condition_and_compute() {
    let node = node(
        "dependency-versioned",
        domain::WorthQueryComparatorRequirement::ExactCanonicalValue,
        domain::WorthQuerySemanticLocality::SourceRecord,
    );
    let installation = conditional_installation(&node);
    let contacts = Arc::new(AtomicUsize::new(0));
    let mut workspace = conditional_workspace_with(
        "dependency-versioned",
        node,
        installation,
        CountedCompute(Arc::clone(&contacts)),
    )
    .unwrap();
    let installed = workspace.domain(GeometryDomain).unwrap();
    let world = workspace.operating_world(observation_basis());
    let bound = world
        .family(ReadFamily)
        .bind(&installed, ReadVertex)
        .unwrap();
    let first = bound
        .execute(ReadExecutionInput::default(), &mut workspace)
        .unwrap();
    assert_eq!(first.counters().conditional_compute_contacts, 1);
    drop(first);

    let world = workspace.operating_world(observation_basis());
    let bound = world
        .family(ReadFamily)
        .bind(&installed, ReadVertex)
        .unwrap();
    let TransitionOutcome::Deferred(second) =
        bound.execute(ReadExecutionInput::default(), &mut workspace)
    else {
        panic!("unchanged semantic dependency versions must defer Query work")
    };

    assert_eq!(contacts.load(Ordering::SeqCst), 1);
    assert_eq!(
        second.conditional_provenance()[0].class(),
        domain::WorthQueryConditionalOutcomeClass::DependencyUnchanged
    );
    assert!(second.conditional_provenance()[0].artifact_reuse_admitted());
    assert_eq!(second.counters().conditional_dependency_checks, 1);
    assert_eq!(second.counters().conditional_comparator_checks, 1);
    assert_eq!(second.counters().conditional_condition_checks, 0);
    assert_eq!(second.counters().conditional_reuse_checks, 1);
    assert_eq!(second.counters().graph_provider_contacts, 0);
    assert_eq!(second.counters().executor_contacts, 0);
}

#[test]
fn unrequested_on_demand_node_defers_without_compute_or_query_work() {
    let dependency = dependency(domain::WorthQuerySemanticLocality::SourceRecord);
    let node = conditional_node_result(
        "on-demand-deferred",
        dependency,
        domain::WorthQueryConditionalEvaluationCondition::on_demand(),
        domain::WorthQueryConditionalTrigger::on_demand::<ManualRefresh>(),
        domain::WorthQueryMaintenancePosture::OnDemandOnly,
    )
    .unwrap();
    let mut installation = conditional_installation(&node);
    installation.providers = worth_runtime_bridge::facade::BridgeConditionalProviderSet::new()
        .trigger(UnrequestedTrigger);
    let contacts = Arc::new(AtomicUsize::new(0));
    let mut workspace = conditional_workspace_with(
        "on-demand-deferred",
        node,
        installation,
        CountedCompute(Arc::clone(&contacts)),
    )
    .unwrap();
    let installed = workspace.domain(GeometryDomain).unwrap();
    let bound = workspace
        .operating_world(observation_basis())
        .family(ReadFamily)
        .bind(&installed, ReadVertex)
        .unwrap();

    let TransitionOutcome::Deferred(deferred) =
        bound.execute(ReadExecutionInput::default(), &mut workspace)
    else {
        panic!("an unrequested on-demand node must remain deferred")
    };

    assert_eq!(contacts.load(Ordering::SeqCst), 0);
    assert_eq!(
        deferred.conditional_provenance()[0].class(),
        domain::WorthQueryConditionalOutcomeClass::DeferredOnDemand
    );
    assert_eq!(deferred.counters().conditional_compute_contacts, 0);
    assert_eq!(deferred.counters().graph_provider_contacts, 0);
    assert_eq!(deferred.counters().executor_contacts, 0);
}

#[test]
fn temporal_wake_defers_without_compute_or_query_work() {
    let dependency = dependency(domain::WorthQuerySemanticLocality::SourceRecord);
    let node = conditional_node_result(
        "temporal-deferred",
        dependency,
        domain::WorthQueryConditionalEvaluationCondition::temporal(
            domain::WorthQueryTemporalCondition::IntervalNanoseconds(1_000),
        ),
        domain::WorthQueryConditionalTrigger::Temporal(
            domain::WorthQueryTemporalWake::MonotonicClock,
        ),
        domain::WorthQueryMaintenancePosture::Temporal,
    )
    .unwrap();
    let mut installation = conditional_installation(&node);
    installation.providers =
        worth_runtime_bridge::facade::BridgeConditionalProviderSet::new().wake(DeferredWake);
    let contacts = Arc::new(AtomicUsize::new(0));
    let mut workspace = conditional_workspace_with(
        "temporal-deferred",
        node,
        installation,
        CountedCompute(Arc::clone(&contacts)),
    )
    .unwrap();
    let installed = workspace.domain(GeometryDomain).unwrap();
    let bound = workspace
        .operating_world(observation_basis())
        .family(ReadFamily)
        .bind(&installed, ReadVertex)
        .unwrap();

    let TransitionOutcome::Deferred(deferred) =
        bound.execute(ReadExecutionInput::default(), &mut workspace)
    else {
        panic!("an unready temporal wake must remain deferred")
    };

    assert_eq!(contacts.load(Ordering::SeqCst), 0);
    assert_eq!(
        deferred.conditional_provenance()[0].class(),
        domain::WorthQueryConditionalOutcomeClass::DeferredTemporal
    );
    assert_eq!(deferred.counters().conditional_compute_contacts, 0);
    assert_eq!(deferred.counters().graph_provider_contacts, 0);
    assert_eq!(deferred.counters().executor_contacts, 0);
}

#[test]
fn authoritative_patch_reenters_the_exact_query_owned_signal_graph() {
    let node = node(
        "authoritative-change",
        domain::WorthQueryComparatorRequirement::ExactCanonicalValue,
        domain::WorthQuerySemanticLocality::SourceRecord,
    );
    let location = domain::WorthQueryConditionalNodeLocation::operation(node.identity()).unwrap();
    let (installation, request, _) = conditional_installation_with_change(&node);
    let contacts = Arc::new(AtomicUsize::new(0));
    let mut workspace = conditional_workspace_with(
        "authoritative-change",
        node,
        installation,
        CountedCompute(Arc::clone(&contacts)),
    )
    .unwrap();
    let installed = workspace.domain(GeometryDomain).unwrap();

    execute_changed(&mut workspace, &installed);
    execute_unchanged(&mut workspace, &installed);
    assert_eq!(contacts.load(Ordering::SeqCst), 1);

    let TransitionOutcome::Success(delivery) = workspace
        .deliver_conditional_authoritative_change(
            GeometryDomain,
            ReadVertex,
            ReadFamily,
            &location,
            0,
            request,
        )
        .unwrap()
    else {
        panic!("the authoritative patch must reach the retained conditional graph")
    };
    assert_eq!(delivery.signal_seeds_emitted(), 1);
    assert_eq!(delivery.slots_touched(), 1);

    execute_changed(&mut workspace, &installed);
    assert_eq!(contacts.load(Ordering::SeqCst), 2);
}

fn execute_changed(
    workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
    installed: &domain::WorthQueryInstalledDomainHandle<GeometryDomain>,
) {
    let bound = workspace
        .operating_world(observation_basis())
        .family(ReadFamily)
        .bind(installed, ReadVertex)
        .unwrap();
    let TransitionOutcome::Success(executed) =
        bound.execute(ReadExecutionInput::default(), workspace)
    else {
        panic!("changed conditional dependency must compute")
    };
    drop(executed);
}

fn execute_unchanged(
    workspace: &mut worth_query::facade::runtime::WorthQueryWorkspace,
    installed: &domain::WorthQueryInstalledDomainHandle<GeometryDomain>,
) {
    let bound = workspace
        .operating_world(observation_basis())
        .family(ReadFamily)
        .bind(installed, ReadVertex)
        .unwrap();
    let TransitionOutcome::Deferred(deferred) =
        bound.execute(ReadExecutionInput::default(), workspace)
    else {
        panic!("unchanged conditional dependency must stop before compute")
    };
    assert_eq!(
        deferred.conditional_provenance()[0].class(),
        domain::WorthQueryConditionalOutcomeClass::DependencyUnchanged
    );
}

#[test]
fn detached_bridge_compute_provider_is_rejected_instead_of_replaced() {
    let node = node(
        "detached-compute",
        domain::WorthQueryComparatorRequirement::ExactCanonicalValue,
        domain::WorthQuerySemanticLocality::SourceRecord,
    );
    let mut installation = conditional_installation(&node);
    let detached_contacts = Arc::new(AtomicUsize::new(0));
    installation.providers = worth_runtime_bridge::facade::BridgeConditionalProviderSet::new()
        .compute(DetachedCompute(Arc::clone(&detached_contacts)));
    let query_contacts = Arc::new(AtomicUsize::new(0));

    let result = conditional_workspace_with(
        "detached-compute",
        node,
        installation,
        CountedCompute(Arc::clone(&query_contacts)),
    );

    let Err(error) = result else {
        panic!("a detached Bridge compute provider must reject runtime construction")
    };
    assert!(error.message().contains("ExtraComputeProvider"));
    assert_eq!(detached_contacts.load(Ordering::SeqCst), 0);
    assert_eq!(query_contacts.load(Ordering::SeqCst), 0);
}

#[test]
fn two_declarations_cannot_implicitly_share_one_signal_node() {
    let first = node(
        "first-owner",
        domain::WorthQueryComparatorRequirement::ExactCanonicalValue,
        domain::WorthQuerySemanticLocality::SourceRecord,
    );
    let second = node(
        "second-owner",
        domain::WorthQueryComparatorRequirement::ExactCanonicalValue,
        domain::WorthQuerySemanticLocality::SourceRecord,
    );

    let result = shared_signal_node_workspace("shared-signal-node", first, second);

    let Err(error) = result else {
        panic!("implicit Signal-node sharing must reject runtime construction")
    };
    assert!(error.message().contains("SignalNodeAlreadyBound"));
}

struct UnrequestedTrigger;

impl worth_runtime_bridge::facade::BridgeConditionalTriggerProvider for UnrequestedTrigger {
    fn requested(&self) -> bool {
        false
    }
}

struct DeferredWake;

impl worth_runtime_bridge::facade::BridgeConditionalWakeProvider for DeferredWake {
    fn resolve(
        &self,
        _declaration: &domain::WorthQueryPortableConditionalNodeDeclaration,
        _context: worth_runtime_bridge::facade::BridgeConditionalResolverContext,
    ) -> Result<worth_signal::facade::InstalledSignalConditionDecision, String> {
        Ok(worth_signal::facade::InstalledSignalConditionDecision::Deferred)
    }
}

struct DetachedCompute(Arc<AtomicUsize>);

impl worth_runtime_bridge::facade::BridgeConditionalComputeProvider for DetachedCompute {
    fn compute(
        &self,
        _context: &mut dyn std::any::Any,
    ) -> Result<worth_signal::facade::NodeEvaluationResult, String> {
        self.0.fetch_add(1, Ordering::SeqCst);
        Err("detached compute must never be invoked".into())
    }
}

struct CountedCompute(Arc<AtomicUsize>);

impl domain::WorthQueryConditionalNodeComputeProvider<GeometryDomain, ReadVertex, ReadFamily>
    for CountedCompute
{
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

fn observation_basis() -> foundation::AdmittedBasisCapability<foundation::ObservationLaneWitness> {
    foundation::basis_lifecycle()
        .current_head()
        .for_observation()
        .unwrap()
        .admit()
        .unwrap()
        .capability()
        .clone()
}
