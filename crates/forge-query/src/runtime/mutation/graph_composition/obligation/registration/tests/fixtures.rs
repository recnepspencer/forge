use crate::runtime::{
    ForgeQueryAspectTouch, ForgeQueryGraphCompositionBreadth, ForgeQueryGraphCompositionProgram,
    ForgeQueryGraphCompositionProgramStep, ForgeQueryGraphCompositionProgramStepKind,
    ForgeQueryGraphObligationOperatingWorldSelector, ForgeQueryGraphObligationRegistration,
    ForgeQueryGraphObligationRuleIdentity, ForgeQueryGraphTouchDescriptor,
    ForgeQueryGraphTouchSelector, ForgeQueryMutationMetadata,
    ForgeQueryMutationTargetCollectionIdentity, ForgeQuerySymbolicTargetReference,
    ForgeQueryWriteCommand,
};
use forge_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_relational::facade::identity::KindId;

pub(super) fn symbolic_relation_touch_descriptor(
    collection: &str,
    touched_fixture: &str,
) -> ForgeQueryGraphTouchDescriptor {
    symbolic_relation_touch_descriptor_with_relation_kind_id(collection, touched_fixture, None)
}

pub(super) fn symbolic_relation_touch_descriptor_with_relation_kind_id(
    collection: &str,
    touched_fixture: &str,
    relation_kind_id: Option<KindId>,
) -> ForgeQueryGraphTouchDescriptor {
    let command = ForgeQueryWriteCommand::DeleteSymbolicAspects {
        reference: ForgeQuerySymbolicTargetReference::new("edge")
            .unwrap()
            .in_target_collection(collection)
            .unwrap(),
        touched_aspects: vec![touch(touched_fixture)],
        metadata: ForgeQueryMutationMetadata::new(),
        naming_intent: None,
    };
    let breadth = ForgeQueryGraphCompositionBreadth::new(1, 0, 0);
    let mut step = ForgeQueryGraphCompositionProgramStep::new(
        0,
        ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement,
        Some(target_collection(collection)),
        Some("edge".to_string()),
    );
    if let Some(relation_kind_id) = relation_kind_id {
        step = step.with_relation_kind_id(relation_kind_id);
    }
    let program = ForgeQueryGraphCompositionProgram::new(vec![step], &breadth);
    ForgeQueryGraphTouchDescriptor::from_authoritative_mutation_batch(
        &program,
        &breadth,
        &[command],
    )
    .unwrap()
}

pub(super) fn touch(touch_fixture: &str) -> ForgeQueryAspectTouch {
    native_touch(touch_fixture)
}

fn native_touch(aspect_fixture: &str) -> ForgeQueryAspectTouch {
    let mut segments = aspect_fixture.split('.');
    let aspect_key = AspectKey::new(
        segments
            .next()
            .expect("test touch fixture should name an aspect"),
    )
    .expect("test aspect key should admit");
    let field_segments = segments
        .map(|segment| FieldKey::new(segment).expect("test field key should admit"))
        .collect::<Vec<_>>();
    if field_segments.is_empty() {
        ForgeQueryAspectTouch::whole_aspect(aspect_key)
    } else {
        ForgeQueryAspectTouch::aspect_field_path(
            aspect_key,
            CanonicalFieldPath::new(field_segments).expect("test field path should admit"),
        )
    }
}

pub(super) fn registration(
    rule_name: &str,
    selector: ForgeQueryGraphTouchSelector,
) -> ForgeQueryGraphObligationRegistration {
    ForgeQueryGraphObligationRegistration::schema_contract_validator(
        ForgeQueryGraphObligationRuleIdentity::new("test.graph-obligation", rule_name, "v1")
            .unwrap(),
        selector,
        ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    )
}

fn target_collection(collection: &str) -> ForgeQueryMutationTargetCollectionIdentity {
    ForgeQueryMutationTargetCollectionIdentity::new("graph-composition-test", collection)
}
