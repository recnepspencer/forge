use worth_foundational::facade::{
    aspects, AbsenceLaw, AspectBinding, AspectEquivalenceBasis, AspectEvolutionPolicy,
    AspectIdentity, AspectMaskContract, AspectValue, AuthoritativeAspectChangeKind,
    ContractValidationInput, PortableAspectContractBasis, PortableAspectPatchOperation,
    PortableRecordAspectPatch, ScalarAspectType, StructAspectValue,
};
use worth_proof::TransitionOutcome;
use worth_runtime_bridge::facade::TruthDeltaSurfaceKind;

use crate::facade::identity::{KindId, PartitionId};
use crate::facade::schema::DeclaredAspectContractBinding;
use crate::facade::transactions::{
    ApplyEntityAspectPatchIntent, CreateIntent, EntityAspectCreateIntent, EntityMutationIntent,
    MutationIntent, WorkerIntentBatch,
};
use crate::tests::support::{
    aspect_key, changed_entities, entity_summary_struct_aspect, field_key, AspectSchemaFixture,
};

#[test]
fn real_whole_and_field_set_clear_operations_keep_their_exact_publication_meaning() {
    let note = optional_note_contract();
    let summary = entity_summary_struct_aspect(aspect_key("summary"), field_key("summary"));
    let summary_contract = summary.contract.clone();
    let mut runtime = AspectSchemaFixture {
        entity_aspects: vec![
            DeclaredAspectContractBinding {
                binding: AspectBinding::EntityField {
                    field: field_key("note"),
                },
                contract: note.clone(),
            },
            summary,
        ],
        ..AspectSchemaFixture::default()
    }
    .build_runtime();

    let initial_summary = StructAspectValue::new([
        (field_key("title"), AspectValue::String("solid".into())),
        (field_key("status"), AspectValue::String("draft".into())),
    ])
    .unwrap();
    let initial = PortableRecordAspectPatch::new([
        PortableAspectPatchOperation::SetWhole {
            basis: PortableAspectContractBasis::from_contract(&note),
            value: ContractValidationInput::Scalar(AspectValue::String("remember".into())),
        },
        PortableAspectPatchOperation::SetWhole {
            basis: PortableAspectContractBasis::from_contract(&summary_contract),
            value: ContractValidationInput::Struct(initial_summary),
        },
    ]);
    let mut create = crate::tests::support::test_owner_begin_transaction_for_main(&mut runtime);
    create.push_batch(WorkerIntentBatch::new("publication-set-whole").push(
        MutationIntent::Create(CreateIntent::EntityAspects(EntityAspectCreateIntent {
            partition_id: PartitionId::main(),
            kind_id: KindId(1),
            client_key: crate::facade::symbols::ClientKey::raw("publication-parity"),
            aspect_patch: initial,
        })),
    ));
    let created = create.commit().unwrap();
    let entity = changed_entities(&created)[0];
    let TransitionOutcome::Success(created_publication) =
        runtime.publish_commit_for_bridge(created.commit.commit_id, "model")
    else {
        panic!("real whole-set commit publishes")
    };
    assert_change(
        created_publication.bridge_envelope(),
        note.key(),
        AuthoritativeAspectChangeKind::WholeAspectSet,
        TruthDeltaSurfaceKind::AuthoritativeAspect,
    );

    let update = PortableRecordAspectPatch::new([
        PortableAspectPatchOperation::ClearWhole {
            basis: PortableAspectContractBasis::from_contract(&note),
        },
        PortableAspectPatchOperation::PatchFields {
            basis: PortableAspectContractBasis::from_contract(&summary_contract),
            selected_fields: vec![field_key("status")],
            field_sets: Vec::new(),
            field_clears: vec![field_key("status")],
        },
    ]);
    let mut transaction =
        crate::tests::support::test_owner_begin_transaction_for_main(&mut runtime);
    transaction.push_batch(WorkerIntentBatch::new("publication-clear-parity").push(
        MutationIntent::Entity(EntityMutationIntent::ApplyAspectPatch(
            ApplyEntityAspectPatchIntent {
                entity_id: entity,
                aspect_patch: update,
            },
        )),
    ));
    let updated = transaction.commit().unwrap();
    let TransitionOutcome::Success(publication) =
        runtime.publish_commit_for_bridge(updated.commit.commit_id, "model")
    else {
        panic!("real clear commit publishes")
    };
    assert_change(
        publication.bridge_envelope(),
        note.key(),
        AuthoritativeAspectChangeKind::WholeAspectClear,
        TruthDeltaSurfaceKind::AuthoritativeAspect,
    );
    assert_change(
        publication.bridge_envelope(),
        summary_contract.key(),
        AuthoritativeAspectChangeKind::FieldClear,
        TruthDeltaSurfaceKind::EntityField,
    );
}

fn optional_note_contract() -> worth_foundational::facade::AspectContract {
    aspects()
        .contract()
        .for_key(aspect_key("note"))
        .identified_by(AspectIdentity(0x9140_0003))
        .at_revision(aspects().vocabulary().revision(1))
        .scalar_with(
            ScalarAspectType::String,
            AspectMaskContract::scalar(),
            AbsenceLaw::Optional,
            AspectEquivalenceBasis::ExactCanonicalValue,
            AspectEvolutionPolicy::ExplicitBreakRequired,
        )
}

fn assert_change(
    envelope: &worth_runtime_bridge::facade::BridgeCommittedPatchEnvelope,
    aspect_key: &worth_foundational::facade::AspectKey,
    kind: AuthoritativeAspectChangeKind,
    surface: TruthDeltaSurfaceKind,
) {
    assert!(envelope.patch_body().canonical_items().iter().any(|item| {
        item.surface_kind() == surface
            && item
                .semantic_change()
                .is_some_and(|change| change.aspect_key() == aspect_key && change.kind() == kind)
    }));
}
