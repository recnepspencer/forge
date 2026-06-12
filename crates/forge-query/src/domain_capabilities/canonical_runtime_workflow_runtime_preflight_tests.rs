use forge_relational::facade::identity::{EntityId, PartitionId};
use serde_json::json;

use super::test_support::{declaration_target, ready, success};
use super::{
    materialize_lowered_mutation_intent_declaration, materialize_query_workflow_declaration,
    ForgeQueryWorkflowContributionAuthoring,
};
use crate::harness::fixtures::execution_preflights;
use crate::workflow::{bind_workflow_context, WorkflowBindingSource};

#[test]
fn workflow_runtime_preflight_materializer_preserves_real_preflight_query_and_basis_identity() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let expected = bind_workflow_context(WorkflowBindingSource::RuntimePreflight(&preflight))
        .expect("runtime preflight should bind");

    let declaration = success(materialize_query_workflow_declaration(ready(
        ForgeQueryWorkflowContributionAuthoring::confirmation_required_query_inspection_from_preflight(
            "spatial.runtime.preflight.review",
            "domain workflow should reuse the real runtime preflight binding",
            preflight.clone(),
        )
        .bind_to_declaration_target(declaration_target("intent-workflow-runtime-preflight")),
    )));

    assert_ne!(declaration.binding().digest(), expected.digest());
    assert_eq!(
        declaration.binding().source_digest(),
        expected.source_digest()
    );
    assert_eq!(
        declaration
            .binding()
            .runtime_snapshot_identity()
            .map(|identity| identity.evidence_identity()),
        Some(
            crate::memory_workspace::ForgeQuerySnapshotIdentity::preview(
                preflight.basis().identity().snapshot_identity().clone(),
            )
            .evidence_identity()
        )
    );
    assert_eq!(
        declaration.binding().query_identity_digest(),
        preflight.plan().query().canonical_query_digest().as_str()
    );
    assert_eq!(
        declaration.binding().basis_digest(),
        preflight.basis().proof().digest().as_str()
    );
}

#[test]
fn workflow_runtime_preflight_materializer_is_stronger_than_snapshot_only_surrogate() {
    let snapshot_identity = crate::harness::fixtures::resolved_bases::primary_snapshot_identity();
    let snapshot_evidence = snapshot_identity.evidence_identity();
    let preflight =
        execution_preflights::runtime_preflight_with_snapshot_identity(snapshot_identity);

    let real = success(materialize_query_workflow_declaration(ready(
        ForgeQueryWorkflowContributionAuthoring::confirmation_required_query_inspection_from_preflight(
            "spatial.runtime.preflight.real",
            "real runtime preflight should preserve full query and basis identity",
            preflight.clone(),
        )
        .bind_to_declaration_target(declaration_target("intent-workflow-runtime-real")),
    )));
    let surrogate = success(materialize_query_workflow_declaration(ready(
        ForgeQueryWorkflowContributionAuthoring::confirmation_required_query_inspection(
            "spatial.runtime.preflight.synthetic",
            "snapshot-token-only workflow semantics remain a weaker surrogate path",
            crate::memory_workspace::ForgeQuerySnapshotIdentity::preview(snapshot_evidence),
        )
        .bind_to_declaration_target(declaration_target("intent-workflow-runtime-synthetic")),
    )));

    assert_ne!(real.binding().digest(), surrogate.binding().digest());
    assert_ne!(
        real.binding().query_identity_digest(),
        surrogate.binding().query_identity_digest()
    );
    assert_eq!(
        real.binding()
            .runtime_snapshot_identity()
            .map(|identity| identity.evidence_identity()),
        surrogate
            .binding()
            .runtime_snapshot_identity()
            .map(|identity| identity.evidence_identity())
    );
}

#[test]
fn workflow_runtime_preflight_lowering_from_bundle_preserves_real_runtime_authority() {
    let preflight = execution_preflights::direct_runtime_preflight();
    let authority_binding_identity = crate::ForgeQueryEvidenceIdentity::compose(
        crate::ForgeQueryEvidenceScope::WorkflowMutationLowering,
    )
    .field_shape(
        crate::ForgeQueryEvidenceTag::new("test_authority_binding"),
        "runtime-preflight",
    )
    .field_identity(
        crate::ForgeQueryEvidenceTag::new("binding"),
        "authority-binding:preflight",
    )
    .seal();

    let lowered = success(materialize_lowered_mutation_intent_declaration(ready(
        ForgeQueryWorkflowContributionAuthoring::confirmation_required_mutation_reconciliation_from_preflight(
            "spatial.runtime.preflight.lowering",
            "mutation lowering should preserve real runtime preflight authority",
            preflight.clone(),
            authority_binding_identity.clone(),
            EntityId::new(PartitionId(7), 11, 0),
            json!({"name":"after"}),
        )
        .bind_to_declaration_target(declaration_target("intent-workflow-runtime-lowering")),
    )));

    assert_eq!(
        lowered
            .authority_binding()
            .runtime_snapshot_identity()
            .map(|identity| identity.evidence_identity()),
        Some(
            crate::memory_workspace::ForgeQuerySnapshotIdentity::preview(
                preflight.basis().identity().snapshot_identity().clone(),
            )
            .evidence_identity()
        )
    );
    assert_eq!(
        lowered.authority_binding().binding_digest(),
        authority_binding_identity.as_str()
    );
    assert_eq!(lowered.counters().workflow_mutation_lowering_count(), 1);
}

#[test]
fn workflow_runtime_preflight_materializer_keeps_scope_distinct_across_targets() {
    let preflight = execution_preflights::direct_runtime_preflight();

    let left = success(materialize_query_workflow_declaration(ready(
        ForgeQueryWorkflowContributionAuthoring::confirmation_required_query_inspection_from_preflight(
            "spatial.runtime.preflight.scope",
            "real runtime preflight bindings must stay scoped by contribution target",
            preflight.clone(),
        )
        .bind_to_declaration_target(declaration_target("intent-workflow-runtime-left")),
    )));
    let right = success(materialize_query_workflow_declaration(ready(
        ForgeQueryWorkflowContributionAuthoring::confirmation_required_query_inspection_from_preflight(
            "spatial.runtime.preflight.scope",
            "real runtime preflight bindings must stay scoped by contribution target",
            preflight.clone(),
        )
        .bind_to_declaration_target(declaration_target("intent-workflow-runtime-right")),
    )));

    assert_eq!(
        left.binding().query_identity_digest(),
        right.binding().query_identity_digest()
    );
    assert_eq!(
        left.binding().basis_digest(),
        right.binding().basis_digest()
    );
    assert_ne!(left.binding().digest(), right.binding().digest());
    assert_ne!(
        left.report().declaration_digest(),
        right.report().declaration_digest()
    );
}
