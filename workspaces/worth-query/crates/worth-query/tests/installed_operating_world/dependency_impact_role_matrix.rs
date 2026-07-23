use std::collections::BTreeSet;

use worth_proof::TransitionOutcome;
use worth_query::facade::{domain, foundation, read, runtime};

use super::installed_operation_fixture::collection_impact::{
    conditional_collection_workspace_with_change, impact_collection_workspace, ImpactCollectionRead,
};
use super::installed_operation_fixture::{
    grouped_lineage_workflow_workspace, mutation_workflow_workspace, GeometryDomain,
    LineageEvidenceScenario, MutationFamily, ReadFamily, WorkflowMutation,
};
use crate::support::public_bridge_runtime::PublicBridgeRuntimeHarness;

#[test]
fn every_role_is_compiled_from_real_operation_or_execution_evidence() {
    let mut lineage = grouped_lineage_workflow_workspace(
        "dependency-impact-role-lineage",
        vec![LineageEvidenceScenario::SingularSuccessor],
    )
    .unwrap();
    let lineage_trace =
        super::operation_lineage::execute(&mut lineage, super::operation_lineage::mutation_basis());
    let mut roles = compiled_roles(lineage_trace.semantic_aspect_dependency_closure().unwrap());

    let mut mutation = mutation_workflow_workspace("dependency-impact-role-invariant").unwrap();
    let installed = mutation.domain(GeometryDomain).unwrap();
    let mutation_trace = mutation
        .operating_world(super::dependency_impact::mutation_basis())
        .family(MutationFamily)
        .bind(&installed, WorkflowMutation)
        .unwrap()
        .start_workflow(&mut mutation)
        .unwrap()
        .advance(
            "mutate",
            domain::WorthQueryWorkflowValue::Text("commit".into()),
            &mut mutation,
        )
        .unwrap()
        .complete()
        .unwrap();
    roles.extend(compiled_roles(
        mutation_trace.semantic_aspect_dependency_closure().unwrap(),
    ));

    let (_workspace, settled, delivery) =
        super::dependency_impact::changed_projection("dependency-impact-role-conditional");
    roles.extend(compiled_roles(settled.semantic_aspect_dependency_closure()));
    let decision = settled
        .classify_authoritative_impact(&delivery, &settled.conditional_provenance()[0])
        .unwrap();
    assert_eq!(decision.class(), domain::WorthQueryImpactClass::ValuePatch);
    assert_eq!(
        decision.affected_roles(),
        [
            domain::WorthQuerySemanticDependencyRole::ProjectedValue,
            domain::WorthQuerySemanticDependencyRole::ConditionalEligibilityOrSemanticCleanliness,
        ]
    );

    let expected = BTreeSet::from([
        domain::WorthQuerySemanticDependencyRole::OperationalIdentity,
        domain::WorthQuerySemanticDependencyRole::SelectionOrMembership,
        domain::WorthQuerySemanticDependencyRole::Ordering,
        domain::WorthQuerySemanticDependencyRole::ProjectedValue,
        domain::WorthQuerySemanticDependencyRole::Grouping,
        domain::WorthQuerySemanticDependencyRole::WindowBoundary,
        domain::WorthQuerySemanticDependencyRole::SupportAndLifecycle,
        domain::WorthQuerySemanticDependencyRole::ConditionalEligibilityOrSemanticCleanliness,
        domain::WorthQuerySemanticDependencyRole::InstalledDomainInvariant,
        domain::WorthQuerySemanticDependencyRole::AdvisoryOnlyContext,
    ]);
    assert_eq!(roles, expected);

    let roles_without_public_event_seam = BTreeSet::from([
        domain::WorthQuerySemanticDependencyRole::OperationalIdentity,
        domain::WorthQuerySemanticDependencyRole::SupportAndLifecycle,
        domain::WorthQuerySemanticDependencyRole::InstalledDomainInvariant,
        domain::WorthQuerySemanticDependencyRole::AdvisoryOnlyContext,
    ]);
    assert!(roles_without_public_event_seam.is_subset(&roles));
}

#[test]
fn collection_roles_are_classified_from_real_bridge_and_structural_changes() {
    let node = super::conditional_node_contract::node(
        "dependency-impact-role-collection",
        domain::WorthQueryComparatorRequirement::ExactCanonicalValue,
        domain::WorthQuerySemanticLocality::SourceRecord,
    );
    let location = domain::WorthQueryConditionalNodeLocation::operation(node.identity()).unwrap();
    let harness = PublicBridgeRuntimeHarness::new();
    let (mut owner, request, snapshots) = conditional_collection_workspace_with_change(
        "dependency-impact-role-owner-collection",
        node,
        &harness,
    )
    .unwrap();
    let installed = owner.domain(GeometryDomain).unwrap();
    let first = bind_collection(&owner, &installed)
        .execute((), &mut owner)
        .unwrap();
    drop(first);
    let TransitionOutcome::Deferred(_) =
        bind_collection(&owner, &installed).execute((), &mut owner)
    else {
        panic!("unchanged collection dependency should defer")
    };
    let TransitionOutcome::Success(delivery) = owner
        .deliver_conditional_authoritative_change(
            GeometryDomain,
            ImpactCollectionRead,
            ReadFamily,
            domain::WorthQueryConditionalAuthoritativeChangeDeliveryRequest::new(
                location.clone(),
                0,
                request,
            ),
        )
        .unwrap()
    else {
        panic!("authoritative collection field change should deliver")
    };
    harness.set_relational_snapshot(snapshots[1].snapshot_id(), snapshots[1].version_id());
    let settled = settle_collection(&mut owner);
    let impact = settled
        .classify_authoritative_impact(&delivery, &settled.conditional_provenance()[0])
        .unwrap();
    assert_eq!(impact.class(), domain::WorthQueryImpactClass::WindowShift);
    assert_eq!(
        impact.affected_roles(),
        [
            domain::WorthQuerySemanticDependencyRole::SelectionOrMembership,
            domain::WorthQuerySemanticDependencyRole::Ordering,
            domain::WorthQuerySemanticDependencyRole::ProjectedValue,
            domain::WorthQuerySemanticDependencyRole::Grouping,
            domain::WorthQuerySemanticDependencyRole::WindowBoundary,
            domain::WorthQuerySemanticDependencyRole::ConditionalEligibilityOrSemanticCleanliness,
        ]
    );

    let mut ordinary = impact_collection_workspace("dependency-impact-role-structural").unwrap();
    let live = match settle_collection(&mut ordinary)
        .into_lifecycle()
        .promote(&mut ordinary)
    {
        domain::WorthQueryProjectionPromotionOutcome::Promoted(live) => live,
        _ => panic!("ordinary collection projection should promote"),
    };
    ordinary
        .insert("Vertex", |row| {
            row.aspect("identity.id", "structural-member")
        })
        .unwrap();
    let structural = live.refresh(&mut ordinary).unwrap();
    assert_eq!(
        structural.impact().class(),
        domain::WorthQueryImpactClass::WindowShift
    );
    assert_eq!(
        structural.impact().affected_roles(),
        [
            domain::WorthQuerySemanticDependencyRole::SelectionOrMembership,
            domain::WorthQuerySemanticDependencyRole::Ordering,
            domain::WorthQuerySemanticDependencyRole::ProjectedValue,
            domain::WorthQuerySemanticDependencyRole::Grouping,
            domain::WorthQuerySemanticDependencyRole::WindowBoundary,
        ]
    );
    assert_eq!(structural.impact().counters().unrelated_dependency_scans, 0);
    assert_eq!(structural.impact().counters().consumer_registry_scans, 0);
}

fn compiled_roles(
    closure: &domain::WorthQueryCompiledSemanticAspectDependencyClosure,
) -> BTreeSet<domain::WorthQuerySemanticDependencyRole> {
    closure
        .dependencies()
        .iter()
        .map(|dependency| dependency.role())
        .collect()
}

fn settle_collection(
    workspace: &mut runtime::WorthQueryWorkspace,
) -> domain::WorthQuerySettledDomainProjection<
    GeometryDomain,
    ImpactCollectionRead,
    ReadFamily,
    foundation::ObservationLaneWitness,
> {
    let installed = workspace.domain(GeometryDomain).unwrap();
    let bound = bind_collection(workspace, &installed);
    let consumer = bound.consumer_projection_contract().unwrap();
    bound
        .execute((), workspace)
        .unwrap()
        .publish()
        .unwrap()
        .consume(consumer, read::project_facts().entity_identities())
        .unwrap()
        .settle()
        .unwrap()
}

fn bind_collection(
    workspace: &runtime::WorthQueryWorkspace,
    installed: &domain::WorthQueryInstalledDomainHandle<GeometryDomain>,
) -> domain::WorthQueryBoundDomainOperation<
    GeometryDomain,
    ImpactCollectionRead,
    ReadFamily,
    foundation::ObservationLaneWitness,
> {
    workspace
        .operating_world(observation_basis())
        .family(ReadFamily)
        .bind(installed, ImpactCollectionRead)
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
