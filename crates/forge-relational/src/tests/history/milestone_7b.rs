use serde::Serialize;

use crate::facade::history::BranchId;
use crate::facade::merge::{
    AspectMergePolicyDeclaration, AspectMergePolicyKind, IdentityBasisDeclaration,
    IdentityBasisKind, IdentityBasisScope, MergeIntent,
};
use crate::facade::runtime::RelationalRuntimeApi;
use crate::facade::schema::{
    AspectBinding, AspectComparator, AspectKey, AspectPrecision, DeclaredAspect,
    EntityKindRegistration, KindAspectDeclarations, RelationKindRegistration,
    RelationalSchemaRegistry, SchemaId, SchemaVersionId,
};
use crate::schema::data::RelationPayloadClass;
use crate::symbols::data::InternedString;
use crate::tests::support::{
    certification_digest, checkpoint_and_recover_with, create_branch_from_main, create_entity,
    persisted_runtime_with_test_schema, update_entity, update_entity_on_branch,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct MergePlanningCertificationBundle {
    schema_snapshot_digest: String,
    artifact_digest: String,
    decision_log_digest: String,
    lowered_plan_digest: String,
    roundtrip_digest: String,
    recovered_digest: String,
}

fn run_merge_planning_certification() -> MergePlanningCertificationBundle {
    let mut runtime = persisted_runtime_with_test_schema();
    let shared = create_entity(&mut runtime, "shared");
    create_branch_from_main(&mut runtime, "feature");
    update_entity(&mut runtime, shared, "shared-main");
    update_entity_on_branch(
        &mut runtime,
        shared,
        "shared-feature",
        BranchId("feature".to_string()),
    );

    let artifact = runtime
        .merge_access()
        .inspect_planning_scope(crate::merge::data::MergePlanningRequest::new(
            BranchId("main".to_string()),
            BranchId("feature".to_string()),
            MergeIntent::ReconcileIntoTarget,
        ))
        .expect("live merge planning artifact");
    let artifact_json =
        serde_json::to_string(&artifact).expect("serialize live merge planning artifact");
    let roundtripped: crate::facade::merge::MergePlanningArtifactCore =
        serde_json::from_str(&artifact_json).expect("deserialize live merge planning artifact");

    let (_recovery, recovered) =
        checkpoint_and_recover_with(&mut runtime, persisted_runtime_with_test_schema);
    let recovered_artifact = recovered
        .merge_access()
        .inspect_planning_scope(crate::merge::data::MergePlanningRequest::new(
            BranchId("main".to_string()),
            BranchId("feature".to_string()),
            MergeIntent::ReconcileIntoTarget,
        ))
        .expect("recovered merge planning artifact");

    assert_eq!(roundtripped, artifact);
    assert_eq!(recovered_artifact, artifact);

    MergePlanningCertificationBundle {
        schema_snapshot_digest: certification_digest(&artifact.schema_snapshot),
        artifact_digest: certification_digest(&artifact.digest_basis),
        decision_log_digest: certification_digest(&artifact.decision_log),
        lowered_plan_digest: certification_digest(&artifact.lowered_plan),
        roundtrip_digest: certification_digest(&roundtripped.digest_basis),
        recovered_digest: certification_digest(&recovered_artifact.digest_basis),
    }
}

#[test]
fn merge_planning_artifact_certification_is_stable_across_roundtrip_and_recovery() {
    let certification = run_merge_planning_certification();
    assert_eq!(certification.artifact_digest, certification.roundtrip_digest);
    assert_eq!(certification.artifact_digest, certification.recovered_digest);
    assert!(certification.schema_snapshot_digest.len() > 8);
    assert!(certification.artifact_digest.len() > 8);
    assert!(certification.decision_log_digest.len() > 8);
    assert!(certification.lowered_plan_digest.len() > 8);
}

#[test]
fn merge_planning_schema_snapshot_changes_when_schema_semantics_change() {
    fn runtime_with_registry(
        merge_policy: AspectMergePolicyKind,
    ) -> crate::facade::runtime::RelationalRuntime {
        let name_key = AspectKey(InternedString::Raw("name".to_string()));
        let registry = RelationalSchemaRegistry::new()
            .register_entity_kind(EntityKindRegistration {
                kind_id: crate::facade::identity::KindId(1),
                kind_name: "test.entity".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                aspect_declarations: KindAspectDeclarations::new(vec![DeclaredAspect {
                    key: name_key.clone(),
                    binding: AspectBinding::EntityPayloadField {
                        field: InternedString::Raw("name".to_string()),
                    },
                    comparator: AspectComparator::JsonScalarEquality,
                    precision: AspectPrecision::Structured,
                }])
                .with_identity_declarations(vec![IdentityBasisDeclaration {
                    scope: IdentityBasisScope::AspectKey(name_key.clone()),
                    basis: IdentityBasisKind::DeclaredKeySet(vec![name_key.clone()].into()),
                }])
                .with_merge_policy_declarations(vec![AspectMergePolicyDeclaration {
                    aspect_key: name_key,
                    policy: merge_policy,
                }]),
            })
            .and_then(|registry| {
                registry.register_relation_kind(RelationKindRegistration {
                    kind_id: crate::facade::identity::KindId(2),
                    kind_name: "test.relation".to_string(),
                    schema_id: SchemaId("test".to_string()),
                    schema_version_id: SchemaVersionId(1),
                    payload_class: RelationPayloadClass::PayloadBearingRelation,
                    cross_context_policy: crate::config::data::CrossContextPolicy::AllowExplicit,
                    cascade_delete_policy: crate::config::data::CascadeDeletePolicy::CascadeDeleteRelations,
                    aspect_declarations: KindAspectDeclarations::default(),
                    relation_integrity: crate::schema::data::RelationIntegrityDeclarations::default(),
                })
            })
            .unwrap();
        RelationalRuntimeApi::builder().schema_registry(registry).build()
    }

    fn snapshot_digest(runtime: &mut crate::facade::runtime::RelationalRuntime) -> String {
        let shared = create_entity(runtime, "shared");
        create_branch_from_main(runtime, "feature");
        update_entity(runtime, shared, "shared-main");
        update_entity_on_branch(
            runtime,
            shared,
            "shared-feature",
            BranchId("feature".to_string()),
        );
        let artifact = runtime
            .merge_access()
            .inspect_planning_scope(crate::merge::data::MergePlanningRequest::new(
                BranchId("main".to_string()),
                BranchId("feature".to_string()),
                MergeIntent::ReconcileIntoTarget,
            ))
            .expect("merge planning artifact");
        certification_digest(&artifact.schema_snapshot)
    }

    let mut prefer_richer_runtime = runtime_with_registry(AspectMergePolicyKind::PreferRicher);
    let mut fail_on_conflict_runtime = runtime_with_registry(AspectMergePolicyKind::FailOnConflict);
    let prefer_richer_digest = snapshot_digest(&mut prefer_richer_runtime);
    let fail_on_conflict_digest = snapshot_digest(&mut fail_on_conflict_runtime);

    assert_ne!(prefer_richer_digest, fail_on_conflict_digest);
}
