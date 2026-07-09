mod serde_support;

use crate::facade::history::BranchId;
use crate::facade::identity::KindId;
use crate::facade::merge::{
    MergeExecutionOutcome, MergeExecutionRequest, MergeIntent,
    RelationalMergeCorrespondenceWitness, RelationalMergeCorrespondenceWitnessPosture,
};
use crate::facade::runtime::{RelationalRuntime, RelationalRuntimeApi};
use crate::facade::schema::{
    EntityKindRegistration, KindAspectContractDeclarations, RelationIntegrityDeclarations,
    RelationKindRegistration, RelationalSchemaRegistry, SchemaId, SchemaVersionId,
};
use crate::tests::support::{
    aspect_key, checkpoint_and_recover_with, create_branch_from_main, create_entity,
    create_entity_outcome_on_branch, entity_field_aspect, field_key,
    persisted_runtime_with_test_schema, update_entity, update_entity_on_branch,
    CascadeDeletePolicy, CrossContextPolicy,
};
use crate::transactions::data::PublishedMergeExecutionAuthority;
use std::sync::Arc;

use serde_support::{WORTHd_row_with_basis, recomputed_witness, row_payloads, witness_payload};

#[test]
fn retained_merge_correspondence_witness_preserves_exact_admitted_truth() {
    let runtime = merge_ready_runtime();
    let prepared = runtime
        .merge()
        .prepare_merge_execution(merge_request())
        .expect("prepared merge");
    let witness = runtime
        .merge()
        .retain_merge_correspondence_witness_from_prepared_execution(&prepared);

    assert_eq!(
        witness.request_digest(),
        prepared.request().request_digest()
    );
    assert_eq!(
        witness.branch_basis_digest(),
        prepared.execution_ready_plan().basis.basis_digest()
    );
    assert_eq!(
        witness.rows().len(),
        prepared.artifact().identity_discovery.candidates.len()
    );
    assert_eq!(witness.admitted_rows().count(), 1);
    assert!(witness.rows().iter().any(|row| row.posture()
        == RelationalMergeCorrespondenceWitnessPosture::Admitted
        && row.target_record().is_some()
        && matches!(
            row.reason(),
            crate::facade::merge::IdentityResolutionReason::ExactStorageIdentity
        )));
    assert!(witness.rows().iter().any(|row| {
        row.posture() == RelationalMergeCorrespondenceWitnessPosture::UnavailableMissingTarget
            && row.target_record().is_none()
    }));
    assert!(!witness.witness_digest().is_empty());
}

#[test]
fn retained_merge_correspondence_witness_rejects_WORTHd_or_incomplete_truth() {
    let runtime = merge_ready_runtime();
    let prepared = runtime
        .merge()
        .prepare_merge_execution(merge_request())
        .expect("prepared merge");
    let witness = runtime
        .merge()
        .retain_merge_correspondence_witness_from_prepared_execution(&prepared);

    let encoded = rmp_serde::to_vec_named(&witness).expect("encode witness");
    let decoded: RelationalMergeCorrespondenceWitness =
        rmp_serde::from_slice(&encoded).expect("decode witness");
    assert_eq!(decoded, witness);

    let WORTHd_digest = rmp_serde::to_vec_named(&witness_payload(
        &witness,
        row_payloads(
            witness.rows(),
            Some("WORTHd-row-candidate-digest"),
            None,
            None,
        ),
        Some(witness.witness_digest()),
    ))
    .expect("encode WORTHd row digest payload");
    let WORTHd_digest_result: Result<RelationalMergeCorrespondenceWitness, _> =
        rmp_serde::from_slice(&WORTHd_digest);
    assert!(WORTHd_digest_result.is_err());

    let WORTHd_witness_digest = rmp_serde::to_vec_named(&witness_payload(
        &witness,
        row_payloads(witness.rows(), None, None, None),
        Some("WORTHd-witness-digest"),
    ))
    .expect("encode WORTHd witness payload");
    let WORTHd_witness_result: Result<RelationalMergeCorrespondenceWitness, _> =
        rmp_serde::from_slice(&WORTHd_witness_digest);
    assert!(WORTHd_witness_result.is_err());

    let WORTHd_rows = row_payloads(
        witness.rows(),
        None,
        Some(RelationalMergeCorrespondenceWitnessPosture::DeniedAmbiguous),
        None,
    );
    let WORTHd_row_result: Result<RelationalMergeCorrespondenceWitness, _> = rmp_serde::from_slice(
        &rmp_serde::to_vec_named(&witness_payload(
            &witness,
            WORTHd_rows.clone(),
            Some(witness.witness_digest()),
        ))
        .expect("encode WORTHd row payload"),
    );
    assert!(WORTHd_row_result.is_err());

    let WORTHd_row_with_recomputed_witness = witness_payload(&witness, WORTHd_rows, None);
    let WORTHd_row_with_recomputed_witness_result: Result<RelationalMergeCorrespondenceWitness, _> =
        rmp_serde::from_slice(
            &rmp_serde::to_vec_named(&WORTHd_row_with_recomputed_witness)
                .expect("encode recomputed WORTHd row payload"),
        );
    assert!(WORTHd_row_with_recomputed_witness_result.is_err());
}

#[test]
fn retained_merge_correspondence_witness_preserves_authority_basis_distinctions() {
    let runtime = merge_ready_runtime();
    let prepared = runtime
        .merge()
        .prepare_merge_execution(merge_request())
        .expect("prepared merge");
    let witness = runtime
        .merge()
        .retain_merge_correspondence_witness_from_prepared_execution(&prepared);
    let admitted_row = witness
        .admitted_rows()
        .next()
        .expect("admitted correspondence row")
        .clone();

    let identical_rows = vec![admitted_row.clone()];
    let identical_witness = recomputed_witness(&witness, identical_rows.clone());
    assert_eq!(
        identical_witness,
        recomputed_witness(&witness, identical_rows)
    );

    let lineage_shifted_row = WORTHd_row_with_basis(
        &admitted_row,
        crate::facade::merge::IdentityBasisKind::LineageIdentity,
    );
    let lineage_shifted_witness = recomputed_witness(&witness, vec![lineage_shifted_row]);
    assert_ne!(lineage_shifted_witness, identical_witness);
    assert_ne!(
        lineage_shifted_witness.witness_digest(),
        identical_witness.witness_digest()
    );
}

#[test]
fn retained_merge_correspondence_witness_preserves_runtime_declared_key_ambiguity_as_denial() {
    let mut runtime = runtime_with_declared_key_identity_registry();
    create_entity(&mut runtime, "shared-name");
    create_entity(&mut runtime, "shared-name");
    create_branch_from_main(&mut runtime, "feature");
    create_entity_outcome_on_branch(&mut runtime, "shared-name", BranchId("feature".to_string()));

    let artifact = runtime
        .merge()
        .inspect_planning_scope(crate::facade::merge::MergePlanningRequest::new(
            BranchId("main".to_string()),
            BranchId("feature".to_string()),
            MergeIntent::ReconcileIntoTarget,
        ))
        .expect("planning artifact");
    let witness = runtime
        .merge()
        .retain_merge_correspondence_witness_from_planning_artifact(&artifact);
    let denied_row = witness
        .rows()
        .iter()
        .find(|row| row.posture() == RelationalMergeCorrespondenceWitnessPosture::DeniedAmbiguous)
        .expect("declared-key ambiguous correspondence row");

    assert!(matches!(
        denied_row.reason(),
        crate::facade::merge::IdentityResolutionReason::DeclaredBasisAmbiguousVisibleTargetMatch
    ));
    assert!(matches!(
        denied_row.authority_basis(),
        crate::facade::merge::IdentityBasisKind::DeclaredKeySet(_)
    ));
    assert!(denied_row.target_record().is_none());
}

#[test]
fn retained_merge_correspondence_witness_survives_publication_and_recovery() {
    let mut runtime = merge_ready_runtime();
    let outcome = execute_merge(&mut runtime);
    let live_witness = outcome.execution_summary.correspondence_witness.clone();
    let live_authority = published_merge_authority(&runtime, outcome.commit.commit.commit_id);
    let (_recovery, recovered) =
        checkpoint_and_recover_with(&mut runtime, persisted_runtime_with_test_schema);
    let recovered_authority =
        published_merge_authority(&recovered, outcome.commit.commit.commit_id);

    assert_eq!(
        live_witness,
        live_authority.execution_summary.correspondence_witness
    );
    assert_eq!(
        live_witness,
        recovered_authority.execution_summary.correspondence_witness
    );
    assert_eq!(
        live_witness.witness_digest(),
        outcome
            .execution_summary
            .proof_packet
            .correspondence_witness_digest()
    );
    assert!(outcome
        .execution_summary
        .retains_consistent_proof_packet_authority());
}

fn merge_ready_runtime() -> RelationalRuntime {
    let mut runtime = persisted_runtime_with_test_schema();
    let shared = create_entity(&mut runtime, "shared");
    create_branch_from_main(&mut runtime, "feature");
    update_entity(&mut runtime, shared, "shared-value");
    update_entity_on_branch(
        &mut runtime,
        shared,
        "shared-value",
        BranchId("feature".to_string()),
    );
    create_entity_outcome_on_branch(
        &mut runtime,
        "feature-only",
        BranchId("feature".to_string()),
    );
    runtime
}

fn runtime_with_declared_key_identity_registry() -> RelationalRuntime {
    let name_key = aspect_key("name");
    let registry = RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_contract_declarations: KindAspectContractDeclarations::new(vec![
                entity_field_aspect(name_key.clone(), field_key("name")),
            ])
            .with_identity_declarations(vec![
                crate::facade::merge::IdentityBasisDeclaration {
                    scope: crate::facade::merge::IdentityBasisScope::AspectKey(name_key.clone()),
                    basis: crate::facade::merge::IdentityBasisKind::DeclaredKeySet(Arc::from([
                        name_key,
                    ])),
                },
            ]),
        })
        .and_then(|registry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "test.relation".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                aspect_contract_declarations: KindAspectContractDeclarations::default(),
                relation_integrity: RelationIntegrityDeclarations::default(),
            })
        })
        .expect("declared-key identity registry");
    RelationalRuntimeApi::builder()
        .schema_registry(registry)
        .build()
}

fn merge_request() -> MergeExecutionRequest {
    MergeExecutionRequest::new(
        BranchId("main".to_string()),
        BranchId("feature".to_string()),
        MergeIntent::ReconcileIntoTarget,
    )
}

fn execute_merge(runtime: &mut RelationalRuntime) -> MergeExecutionOutcome {
    let prepared = runtime
        .merge()
        .prepare_merge_execution(merge_request())
        .expect("prepared merge");
    runtime
        .execute_prepared_merge(prepared)
        .expect("executed merge")
}

fn published_merge_authority(
    runtime: &RelationalRuntime,
    commit_id: crate::facade::history::CommitId,
) -> PublishedMergeExecutionAuthority {
    runtime
        .replay()
        .canonical_commit_envelope(commit_id)
        .and_then(|envelope| envelope.merge_execution_authority.clone())
        .expect("published merge authority")
}
