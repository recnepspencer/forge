use worth_foundational::facade::{
    AspectBinding, AspectIdentity, AspectKey, AuthoritativeAspectChangeKind, ScalarAspectType,
};
use worth_proof::TransitionOutcome;
use worth_runtime_bridge::facade::TruthDeltaSurfaceKind;

use crate::config::data::CascadeDeletePolicy;
use crate::facade::schema::DeclaredAspectContractBinding;
use crate::facade::transactions::{
    CreateIntent, EntityMutationIntent, EntitySpec, MutationIntent, TransactionOptions,
    UpdateEntityFieldsIntent, WorkerIntentBatch,
};
use crate::tests::support::{
    aspect_key, changed_entities, create_entity_outcome, create_relation_outcome,
    entity_summary_struct_aspect, field_key, string_aspect_field_patch, AspectSchemaFixture,
};

use super::support::runtime_with_test_schema;

#[test]
fn real_entity_and_relation_transactions_preserve_semantic_binding_surfaces() {
    let mut runtime = runtime_with_test_schema();
    let source = changed_entities(&create_entity_outcome(&mut runtime, "source"))[0];
    let target = changed_entities(&create_entity_outcome(&mut runtime, "target"))[0];

    create_relation_outcome(&mut runtime, source, target, "edge");
    let commit = runtime
        .publication()
        .latest_bundle()
        .unwrap()
        .commit
        .commit_id;
    let publication = match runtime.publish_commit_for_bridge(commit, "model") {
        TransitionOutcome::Success(publication) => publication,
        TransitionOutcome::Denied(reason) => panic!("relation publication denied: {reason:?}"),
        TransitionOutcome::Deferred(reason) => panic!("relation publication deferred: {reason:?}"),
        TransitionOutcome::Stale(reason) => panic!("relation publication stale: {reason:?}"),
        TransitionOutcome::RebindRequired(reason) => {
            panic!("relation publication rebind: {reason:?}")
        }
        TransitionOutcome::Failed(reason) => panic!("relation publication failed: {reason:?}"),
    };
    let items = publication.bridge_envelope().patch_body().canonical_items();

    assert_semantic_item(
        items,
        &AspectBinding::RelationField {
            field: worth_foundational::facade::FieldKey::new("label").unwrap(),
        },
        AuthoritativeAspectChangeKind::WholeAspectSet,
        TruthDeltaSurfaceKind::AuthoritativeAspect,
    );
    assert_semantic_item(
        items,
        &AspectBinding::RelationSourceEndpoint,
        AuthoritativeAspectChangeKind::RelationSourceEndpoint,
        TruthDeltaSurfaceKind::EntityRelationEndpoint,
    );
    assert_semantic_item(
        items,
        &AspectBinding::RelationTargetEndpoint,
        AuthoritativeAspectChangeKind::RelationTargetEndpoint,
        TruthDeltaSurfaceKind::EntityRelationEndpoint,
    );
    assert_semantic_item(
        items,
        &AspectBinding::LifecycleTransition,
        AuthoritativeAspectChangeKind::LifecycleCreate,
        TruthDeltaSurfaceKind::LifecycleTransition,
    );
}

#[test]
fn real_entity_transaction_preserves_field_lifecycle_and_structural_surfaces() {
    let mut fixture = AspectSchemaFixture::with_default_declared_aspects(
        CascadeDeletePolicy::CascadeDeleteRelations,
    );
    fixture.entity_aspects.extend([
        entity_summary_struct_aspect(aspect_key("summary"), field_key("summary")),
        structural_binding("region", 91, AspectBinding::StructuralRegion),
        structural_binding("partition", 92, AspectBinding::StructuralPartition),
        structural_binding("facet", 93, AspectBinding::StructuralFacet),
    ]);
    let mut runtime = fixture.build_runtime();
    let mut creation = runtime.begin_transaction(TransactionOptions::default());
    creation.push_batch(
        WorkerIntentBatch::new("structural-create").push(MutationIntent::Create(
            CreateIntent::Entity(EntitySpec {
                partition_id: crate::facade::identity::PartitionId::main(),
                kind_id: crate::facade::identity::KindId(1),
                client_key: crate::symbols::data::ClientKey::raw("solid"),
                fields: string_aspect_field_patch([
                    (aspect_key("name"), field_key("name"), "solid"),
                    (aspect_key("summary"), field_key("title"), "solid"),
                ]),
            }),
        )),
    );
    let created = creation.commit().expect("real structural create");
    let entity = changed_entities(&created)[0];
    let structural_commit = runtime
        .publication()
        .latest_bundle()
        .unwrap()
        .commit
        .commit_id;
    let TransitionOutcome::Success(publication) =
        runtime.publish_commit_for_bridge(structural_commit, "model")
    else {
        panic!("real structural transaction must publish");
    };
    let items = publication.bridge_envelope().patch_body().canonical_items();

    assert_semantic_item(
        items,
        &AspectBinding::LifecycleTransition,
        AuthoritativeAspectChangeKind::LifecycleCreate,
        TruthDeltaSurfaceKind::LifecycleTransition,
    );
    assert_semantic_item(
        items,
        &AspectBinding::StructuralRegion,
        AuthoritativeAspectChangeKind::StructuralCreate,
        TruthDeltaSurfaceKind::EntityRegion,
    );
    assert_semantic_item(
        items,
        &AspectBinding::StructuralPartition,
        AuthoritativeAspectChangeKind::StructuralCreate,
        TruthDeltaSurfaceKind::EntityPartition,
    );
    assert_semantic_item(
        items,
        &AspectBinding::StructuralFacet,
        AuthoritativeAspectChangeKind::StructuralCreate,
        TruthDeltaSurfaceKind::EntityFacet,
    );

    let mut transaction = runtime.begin_transaction(TransactionOptions::default());
    transaction.push_batch(WorkerIntentBatch::new("summary-field-update").push(
        MutationIntent::Entity(EntityMutationIntent::UpdateFields(
            UpdateEntityFieldsIntent {
                entity_id: entity,
                fields: string_aspect_field_patch([
                    (aspect_key("summary"), field_key("title"), "solid"),
                    (aspect_key("summary"), field_key("status"), "ready"),
                ]),
            },
        )),
    ));
    transaction.commit().expect("real struct-field update");
    let field_commit = runtime
        .publication()
        .latest_bundle()
        .unwrap()
        .commit
        .commit_id;
    let TransitionOutcome::Success(field_publication) =
        runtime.publish_commit_for_bridge(field_commit, "model")
    else {
        panic!("real field transaction must publish");
    };
    assert_semantic_item(
        field_publication
            .bridge_envelope()
            .patch_body()
            .canonical_items(),
        &AspectBinding::EntityField {
            field: field_key("summary"),
        },
        AuthoritativeAspectChangeKind::FieldSet,
        TruthDeltaSurfaceKind::EntityField,
    );
}

fn structural_binding(
    key: &str,
    identity: u64,
    binding: AspectBinding,
) -> DeclaredAspectContractBinding {
    let key = AspectKey::new(key).unwrap();
    DeclaredAspectContractBinding {
        binding,
        contract: worth_foundational::aspects()
            .contract()
            .for_key(key)
            .identified_by(AspectIdentity(identity))
            .at_revision(worth_foundational::aspects().vocabulary().revision(1))
            .scalar(ScalarAspectType::String),
    }
}

fn assert_semantic_item(
    items: &[worth_runtime_bridge::facade::BridgeCommittedPatchItem],
    binding: &AspectBinding,
    kind: AuthoritativeAspectChangeKind,
    surface: TruthDeltaSurfaceKind,
) {
    assert!(
        items.iter().any(|item| {
            item.surface_kind() == surface
                && item
                    .semantic_change()
                    .is_some_and(|change| change.binding() == binding && change.kind() == kind)
        }),
        "missing {binding:?} / {kind:?} / {surface:?} from {items:#?}"
    );
}
