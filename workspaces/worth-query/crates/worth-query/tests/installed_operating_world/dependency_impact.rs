use worth_proof::TransitionOutcome;
use worth_query::facade::{certification, domain, foundation, read, runtime};

use super::conditional_node_contract::node;
use super::installed_operation_fixture::{
    conditional_installation_with_change, conditional_public_workspace_with, configured_runtime,
    mutation_workflow_workspace, workflow_workspace, DirectConditionalCompute, GeometryDomain,
    MutationFamily, ReadExecutionInput, ReadFamily, ReadVertex, WorkflowMutation, WorkflowRead,
};
use super::operation_reexecution::intent;
use crate::support::public_bridge_runtime::PublicBridgeRuntimeHarness;

pub(super) type SettledDirect = domain::WorthQuerySettledDomainProjection<
    GeometryDomain,
    ReadVertex,
    ReadFamily,
    foundation::ObservationLaneWitness,
>;

#[test]
fn completed_workflow_closure_retains_declared_and_realized_roles_at_exact_d_cost() {
    let mut workspace = mutation_workflow_workspace("dependency-impact-workflow").unwrap();
    let installed = workspace.domain(GeometryDomain).unwrap();
    let trace = workspace
        .operating_world(mutation_basis())
        .family(MutationFamily)
        .bind(&installed, WorkflowMutation)
        .unwrap()
        .start_workflow(&mut workspace)
        .unwrap()
        .advance(
            "mutate",
            domain::WorthQueryWorkflowValue::Text("commit".into()),
            &mut workspace,
        )
        .unwrap()
        .complete()
        .unwrap();
    let closure = trace.semantic_aspect_dependency_closure().unwrap();
    let d = closure.dependencies().len();
    let counters = closure.counters();

    assert_eq!(d, 12);
    assert!(has_source(closure, |source| matches!(
        source,
        domain::WorthQuerySemanticAspectDependencyView::EffectFamily(_)
    )));
    assert!(has_source(closure, |source| matches!(
        source,
        domain::WorthQuerySemanticAspectDependencyView::InstalledInvariant("workflow-invariant:1")
    )));
    assert!(has_source(closure, |source| matches!(
        source,
        domain::WorthQuerySemanticAspectDependencyView::RealizedWorkflowEffect(_)
    )));
    assert!(has_source(closure, |source| matches!(
        source,
        domain::WorthQuerySemanticAspectDependencyView::RealizedWorkflowInvariant(_)
    )));
    assert!(has_source(closure, |source| matches!(
        source,
        domain::WorthQuerySemanticAspectDependencyView::RealizedWorkflowOutput { .. }
    )));
    assert_eq!(counters.effect_contract_edges, 1);
    assert_eq!(counters.invariant_contract_edges, 1);
    assert_eq!(counters.realized_effect_edges, 1);
    assert_eq!(counters.realized_invariant_edges, 1);
    assert_eq!(counters.realized_workflow_output_edges, 1);
    assert_exact_d_work(closure, d);
}

#[test]
fn certification_replay_preserves_the_compiled_workflow_closure() {
    let mut workspace = workflow_workspace("dependency-impact-replay").unwrap();
    let basis = observation_basis();
    let original = bind_workflow(&workspace, basis.clone())
        .reexecute(intent(), &mut workspace)
        .unwrap();
    let original_closure = original.semantic_aspect_dependency_closure().unwrap();

    let replay = certification::replay_installed_workflow(
        certification::issue_query_certification_replay_capability(),
        &original,
        bind_workflow(&workspace, basis),
        intent(),
        &mut workspace,
    )
    .unwrap();
    assert_eq!(
        replay.comparison(),
        &domain::WorthQueryReplayComparison::Equivalent
    );
    assert_exact_d_work(original_closure, original_closure.dependencies().len());
}

#[test]
fn owner_delivery_impact_is_exact_and_foreign_evidence_is_rejected() {
    let (_owner_workspace, owner_settled, owner_delivery) =
        changed_projection("dependency-impact-owner");
    let owner_conditional = &owner_settled.conditional_provenance()[0];
    let decision = owner_settled
        .classify_authoritative_impact(&owner_delivery, owner_conditional)
        .unwrap();
    assert_eq!(decision.class(), domain::WorthQueryImpactClass::ValuePatch);
    assert_eq!(
        decision.affected_roles(),
        [
            domain::WorthQuerySemanticDependencyRole::ProjectedValue,
            domain::WorthQuerySemanticDependencyRole::ConditionalEligibilityOrSemanticCleanliness,
        ]
    );
    assert_eq!(decision.owner_change_count(), 1);
    assert_eq!(decision.affected_dependency_count(), 5);
    assert_eq!(decision.counters().owner_changes_inspected, 1);
    assert_eq!(decision.counters().index_lookups, 5);
    assert_eq!(decision.counters().affected_edges, 5);
    assert_eq!(decision.counters().conditional_outcomes_inspected, 1);
    assert_eq!(decision.counters().unrelated_dependency_scans, 0);
    assert_eq!(decision.counters().consumer_registry_scans, 0);

    let (_foreign_workspace, foreign_settled, foreign_delivery) =
        changed_projection("dependency-impact-foreign");
    let foreign_runtime = owner_settled
        .classify_authoritative_impact(&foreign_delivery, owner_conditional)
        .unwrap_err();
    assert_eq!(
        foreign_runtime.kind(),
        domain::WorthQueryImpactAdmissionDenialKind::ForeignRuntime
    );
    assert_eq!(foreign_runtime.counters().runtime_authority_checks, 1);
    assert_eq!(foreign_runtime.counters().installation_generation_checks, 0);
    assert_eq!(foreign_runtime.counters().owner_changes_inspected, 0);
    let foreign_conditional = owner_settled
        .classify_authoritative_impact(
            &owner_delivery,
            &foreign_settled.conditional_provenance()[0],
        )
        .unwrap_err();
    assert_eq!(
        foreign_conditional.kind(),
        domain::WorthQueryImpactAdmissionDenialKind::ConditionalAuthorityMismatch
    );
    assert_eq!(foreign_conditional.counters().runtime_authority_checks, 1);
    assert_eq!(
        foreign_conditional.counters().conditional_location_checks,
        1
    );
    assert_eq!(
        foreign_conditional.counters().conditional_authority_checks,
        1
    );
    assert_eq!(foreign_conditional.counters().owner_changes_inspected, 0);
}

#[test]
fn refresh_rejects_foreign_owner_evidence_before_drain_or_projection_work() {
    let (_foreign_workspace, _foreign_settled, foreign_delivery) =
        changed_projection("dependency-impact-refresh-foreign");
    let mut owner = configured_runtime()
        .workspace("dependency-impact-refresh-owner")
        .unwrap();
    let live = match settle_plain_direct(&mut owner)
        .into_lifecycle()
        .promote(&mut owner)
    {
        domain::WorthQueryProjectionPromotionOutcome::Promoted(live) => live,
        _ => panic!("ordinary owner projection did not promote"),
    };
    let error = match live.refresh_owner_delivery(&foreign_delivery, &mut owner) {
        Ok(_) => panic!("foreign owner evidence reached live refresh"),
        Err(error) => error,
    };
    let domain::WorthQueryLiveProjectionRefreshError::Impact {
        denial,
        work,
        owner_delivery_retained,
    } = error
    else {
        panic!("foreign owner evidence denied outside impact admission")
    };
    assert_eq!(
        denial.kind(),
        domain::WorthQueryImpactAdmissionDenialKind::ForeignRuntime
    );
    assert_eq!(denial.counters().runtime_authority_checks, 1);
    assert_eq!(denial.counters().installation_generation_checks, 0);
    assert_eq!(denial.counters().owner_changes_inspected, 0);
    assert!(!owner_delivery_retained);
    assert_eq!(work.authority_checks(), 1);
    assert_eq!(work.drain_calls(), 0);
    assert_eq!(work.delivery_batches(), 0);
    assert_eq!(work.impact_classifications(), 0);
    assert_eq!(work.conditional_dependency_checks(), 0);
    assert_eq!(work.conditional_semantic_reads(), 0);
    assert_eq!(work.read_calls(), 0);
    assert_eq!(work.projection_calls(), 0);
    assert_eq!(work.native_rebind_calls(), 0);
}

pub(super) fn changed_projection(
    name: &str,
) -> (
    runtime::WorthQueryWorkspace,
    SettledDirect,
    worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
) {
    let node = node(
        "dependency-impact-change",
        domain::WorthQueryComparatorRequirement::ExactCanonicalValue,
        domain::WorthQuerySemanticLocality::SourceRecord,
    );
    let location = domain::WorthQueryConditionalNodeLocation::operation(node.identity()).unwrap();
    let (installation, request, snapshots) = conditional_installation_with_change(&node);
    let harness = PublicBridgeRuntimeHarness::new();
    harness.set_relational_snapshot(snapshots[0].snapshot_id(), snapshots[0].version_id());
    let mut workspace = conditional_public_workspace_with(
        name,
        node,
        installation,
        DirectConditionalCompute,
        &harness,
    )
    .unwrap();
    let installed = workspace.domain(GeometryDomain).unwrap();
    let first = bind_direct(&workspace, &installed)
        .execute(ReadExecutionInput::default(), &mut workspace)
        .unwrap();
    assert_eq!(
        first.conditional_provenance()[0].class(),
        domain::WorthQueryConditionalOutcomeClass::ComputedChanged
    );
    drop(first);
    let TransitionOutcome::Deferred(unchanged) =
        bind_direct(&workspace, &installed).execute(ReadExecutionInput::default(), &mut workspace)
    else {
        panic!("unchanged dependency did not defer before the owner delivery")
    };
    assert_eq!(
        unchanged.conditional_provenance()[0].class(),
        domain::WorthQueryConditionalOutcomeClass::DependencyUnchanged
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
        panic!("owner change did not reach its installed conditional graph")
    };
    harness.set_relational_snapshot(snapshots[1].snapshot_id(), snapshots[1].version_id());
    let bound = bind_direct(&workspace, &installed);
    let consumer = bound.consumer_projection_contract().unwrap();
    let executed = bound
        .execute(ReadExecutionInput::default(), &mut workspace)
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
    (workspace, settled, delivery)
}

fn settle_plain_direct(workspace: &mut runtime::WorthQueryWorkspace) -> SettledDirect {
    let installed = workspace.domain(GeometryDomain).unwrap();
    let bound = bind_direct(workspace, &installed);
    let consumer = bound.consumer_projection_contract().unwrap();
    bound
        .execute(ReadExecutionInput::default(), workspace)
        .unwrap()
        .publish()
        .unwrap()
        .consume(consumer, read::project_facts().entity_identities())
        .unwrap()
        .settle()
        .unwrap()
}

pub(super) fn bind_direct(
    workspace: &runtime::WorthQueryWorkspace,
    installed: &domain::WorthQueryInstalledDomainHandle<GeometryDomain>,
) -> domain::WorthQueryBoundDomainOperation<
    GeometryDomain,
    ReadVertex,
    ReadFamily,
    foundation::ObservationLaneWitness,
> {
    workspace
        .operating_world(observation_basis())
        .family(ReadFamily)
        .bind(installed, ReadVertex)
        .unwrap()
}

fn has_source(
    closure: &domain::WorthQueryCompiledSemanticAspectDependencyClosure,
    predicate: impl Fn(domain::WorthQuerySemanticAspectDependencyView<'_>) -> bool,
) -> bool {
    closure
        .dependencies()
        .iter()
        .any(|dependency| predicate(dependency.source()))
}

fn assert_exact_d_work(
    closure: &domain::WorthQueryCompiledSemanticAspectDependencyClosure,
    d: usize,
) {
    let counters = closure.counters();
    assert_eq!(counters.compiled_dependency_count, d);
    assert_eq!(counters.canonical_traversal_edges, d);
    assert_eq!(counters.uniqueness_hash_checks, d);
    assert_eq!(counters.closure_edges_traversed, d - 1);
    assert_eq!(
        closure.measured_compilation_width(),
        counters.compiled_dependency_count
            + counters.impact_index_dependency_visits
            + counters.impact_index_entries
            + counters.impact_mask_propagation_edges
            + counters.workflow_graph_edges_traversed
    );
    assert_eq!(closure.closure_evidence().dependency_count(), d);
    assert_eq!(closure.closure_evidence().closure_edge_count(), d - 1);
    assert_eq!(counters.unrelated_definition_scans, 0);
    assert_eq!(counters.unrelated_runtime_scans, 0);
    assert_eq!(counters.consumer_registry_scans, 0);
}

pub(super) fn closure_summary(
    closure: &domain::WorthQueryCompiledSemanticAspectDependencyClosure,
) -> (
    Vec<domain::WorthQuerySemanticDependencyRole>,
    domain::WorthQuerySemanticAspectDependencyCompilationCounters,
    domain::WorthQuerySemanticDependencyClosureEvidence,
) {
    (
        closure
            .dependencies()
            .iter()
            .map(|dependency| dependency.role())
            .collect(),
        closure.counters(),
        closure.closure_evidence(),
    )
}

fn bind_workflow(
    workspace: &runtime::WorthQueryWorkspace,
    basis: foundation::AdmittedBasisCapability<foundation::ObservationLaneWitness>,
) -> domain::WorthQueryBoundDomainOperation<
    GeometryDomain,
    WorkflowRead,
    ReadFamily,
    foundation::ObservationLaneWitness,
> {
    let installed = workspace.domain(GeometryDomain).unwrap();
    workspace
        .operating_world(basis)
        .family(ReadFamily)
        .bind(&installed, WorkflowRead)
        .unwrap()
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

pub(super) fn mutation_basis(
) -> foundation::AdmittedBasisCapability<foundation::MutationPreparationLaneWitness> {
    foundation::basis_lifecycle()
        .current_head()
        .for_mutation_preparation()
        .unwrap()
        .admit()
        .unwrap()
}
