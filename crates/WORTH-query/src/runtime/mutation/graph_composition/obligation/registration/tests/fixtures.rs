use crate::runtime::{
    WorthQueryAspectTouch, WorthQueryGraphCompositionBreadth, WorthQueryGraphCompositionProgram,
    WorthQueryGraphCompositionProgramStep, WorthQueryGraphCompositionProgramStepKind,
    WorthQueryGraphObligationOperatingWorldSelector, WorthQueryGraphObligationRegistration,
    WorthQueryGraphObligationRuleIdentity, WorthQueryGraphTouchDescriptor,
    WorthQueryGraphTouchSelector, WorthQueryMutationMetadata,
    WorthQueryMutationTargetCollectionIdentity, WorthQuerySymbolicTargetReference,
    WorthQueryWriteCommand,
};
use worth_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use worth_relational::facade::identity::KindId;

pub(super) fn symbolic_relation_touch_descriptor(
    collection: &str,
    touched_fixture: &str,
) -> WorthQueryGraphTouchDescriptor {
    symbolic_relation_touch_descriptor_with_relation_kind_id(collection, touched_fixture, None)
}

pub(super) fn symbolic_relation_touch_descriptor_with_relation_kind_id(
    collection: &str,
    touched_fixture: &str,
    relation_kind_id: Option<KindId>,
) -> WorthQueryGraphTouchDescriptor {
    let command = WorthQueryWriteCommand::DeleteSymbolicAspects {
        reference: WorthQuerySymbolicTargetReference::new("edge")
            .unwrap()
            .in_target_collection(collection)
            .unwrap(),
        touched_aspects: vec![touch(touched_fixture)],
        metadata: WorthQueryMutationMetadata::new(),
        naming_intent: None,
    };
    let breadth = WorthQueryGraphCompositionBreadth::new(1, 0, 0);
    let mut step = WorthQueryGraphCompositionProgramStep::new(
        0,
        WorthQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement,
        Some(target_collection(collection)),
        Some("edge".to_string()),
    );
    if let Some(relation_kind_id) = relation_kind_id {
        step = step.with_relation_kind_id(relation_kind_id);
    }
    let program = WorthQueryGraphCompositionProgram::new(vec![step], &breadth);
    WorthQueryGraphTouchDescriptor::from_authoritative_mutation_batch(
        &program,
        &breadth,
        &[command],
    )
    .unwrap()
}

pub(super) fn touch(touch_fixture: &str) -> WorthQueryAspectTouch {
    native_touch(touch_fixture)
}

fn native_touch(aspect_fixture: &str) -> WorthQueryAspectTouch {
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
        WorthQueryAspectTouch::whole_aspect(aspect_key)
    } else {
        WorthQueryAspectTouch::aspect_field_path(
            aspect_key,
            CanonicalFieldPath::new(field_segments).expect("test field path should admit"),
        )
    }
}

pub(super) fn registration(
    rule_name: &str,
    selector: WorthQueryGraphTouchSelector,
) -> WorthQueryGraphObligationRegistration {
    WorthQueryGraphObligationRegistration::schema_contract_validator(
        WorthQueryGraphObligationRuleIdentity::new("test.graph-obligation", rule_name, "v1")
            .unwrap(),
        selector,
        WorthQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
    )
}

fn target_collection(collection: &str) -> WorthQueryMutationTargetCollectionIdentity {
    WorthQueryMutationTargetCollectionIdentity::new("graph-composition-test", collection)
}
