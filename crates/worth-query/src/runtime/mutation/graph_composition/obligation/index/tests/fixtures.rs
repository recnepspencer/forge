use crate::runtime::{
    WorthQueryAspectMutationOperation, WorthQueryAuthoredAspectMutation,
    WorthQueryGraphCompositionBreadth, WorthQueryGraphCompositionProgram,
    WorthQueryGraphCompositionProgramStep, WorthQueryGraphCompositionProgramStepKind,
    WorthQueryGraphObligationOperatingWorldSelector, WorthQueryGraphObligationRegistration,
    WorthQueryGraphObligationRegistrationCatalog, WorthQueryGraphObligationRuleIdentity,
    WorthQueryGraphTouchDescriptor, WorthQueryGraphTouchLifecycleFamily,
    WorthQueryGraphTouchSelector, WorthQueryMutationMetadata,
    WorthQueryMutationTargetCollectionIdentity, WorthQuerySymbolicTargetReference,
    WorthQueryWriteCommand,
};
use worth_foundational::facade::{AspectKey, AspectValue, CanonicalFieldPath, FieldKey};
use worth_relational::facade::identity::KindId;

pub(super) fn symbolic_relation_retirement_descriptor() -> WorthQueryGraphTouchDescriptor {
    let command = WorthQueryWriteCommand::DeleteSymbolicAspects {
        reference: WorthQuerySymbolicTargetReference::new("edge")
            .unwrap()
            .in_target_collection("topology.edge")
            .unwrap(),
        touched_aspects: vec![touch("weight")],
        metadata: WorthQueryMutationMetadata::new(),
        naming_intent: None,
    };
    let breadth = WorthQueryGraphCompositionBreadth::new(1, 0, 0);
    let step = WorthQueryGraphCompositionProgramStep::new(
        0,
        WorthQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement,
        Some(target_collection("topology.edge")),
        Some("edge".to_string()),
    )
    .with_relation_kind_id(KindId(77));
    let program = WorthQueryGraphCompositionProgram::new(vec![step], &breadth);
    WorthQueryGraphTouchDescriptor::from_authoritative_mutation_batch(
        &program,
        &breadth,
        &[command],
    )
    .unwrap()
}

pub(super) fn multi_component_descriptor() -> WorthQueryGraphTouchDescriptor {
    let commands = vec![
        WorthQueryWriteCommand::UpdateSymbolicAspects {
            reference: WorthQuerySymbolicTargetReference::new("edge")
                .unwrap()
                .in_target_collection("topology.edge")
                .unwrap(),
            aspects: vec![
                WorthQueryAuthoredAspectMutation::new(touch("capacity"), int_value(1)).unwrap(),
            ],
            metadata: WorthQueryMutationMetadata::new(),
            naming_intent: None,
            continuity_intent: None,
        },
        WorthQueryWriteCommand::DeleteSymbolicAspects {
            reference: WorthQuerySymbolicTargetReference::new("face")
                .unwrap()
                .in_target_collection("topology.face")
                .unwrap(),
            touched_aspects: vec![touch("boundary")],
            metadata: WorthQueryMutationMetadata::new(),
            naming_intent: None,
        },
    ];
    let breadth = WorthQueryGraphCompositionBreadth::new(2, 0, 0);
    let steps = vec![
        WorthQueryGraphCompositionProgramStep::new(
            0,
            WorthQueryGraphCompositionProgramStepKind::SymbolicRelationFollowupMutation,
            Some(target_collection("topology.edge")),
            Some("edge".to_string()),
        )
        .with_relation_kind_id(KindId(77)),
        WorthQueryGraphCompositionProgramStep::new(
            1,
            WorthQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement,
            Some(target_collection("topology.face")),
            Some("face".to_string()),
        )
        .with_relation_kind_id(KindId(88)),
    ];
    let program = WorthQueryGraphCompositionProgram::new(steps, &breadth);
    WorthQueryGraphTouchDescriptor::from_authoritative_mutation_batch(&program, &breadth, &commands)
        .unwrap()
}

pub(super) fn touch(touch_fixture: &str) -> crate::runtime::WorthQueryAspectTouch {
    native_touch(touch_fixture)
}

pub(super) fn set_operation(touch_fixture: &str) -> WorthQueryAspectMutationOperation {
    WorthQueryAspectMutationOperation::set(touch(touch_fixture))
}

fn int_value(value: i64) -> AspectValue {
    AspectValue::Int64(value)
}

fn target_collection(collection: &str) -> WorthQueryMutationTargetCollectionIdentity {
    WorthQueryMutationTargetCollectionIdentity::new("graph-composition-test", collection)
}

fn native_touch(aspect_fixture: &str) -> crate::runtime::WorthQueryAspectTouch {
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
        crate::runtime::WorthQueryAspectTouch::whole_aspect(aspect_key)
    } else {
        crate::runtime::WorthQueryAspectTouch::aspect_field_path(
            aspect_key,
            CanonicalFieldPath::new(field_segments).expect("test field path should admit"),
        )
    }
}

pub(super) fn catalog(
    registrations: Vec<WorthQueryGraphObligationRegistration>,
) -> WorthQueryGraphObligationRegistrationCatalog {
    WorthQueryGraphObligationRegistrationCatalog::from_registrations(registrations).unwrap()
}

pub(super) fn schema_registration(
    name: &str,
    selector: WorthQueryGraphTouchSelector,
    world: WorthQueryGraphObligationOperatingWorldSelector,
) -> WorthQueryGraphObligationRegistration {
    WorthQueryGraphObligationRegistration::schema_contract_validator(rule(name), selector, world)
}

pub(super) fn blocking_registration(
    name: &str,
    selector: WorthQueryGraphTouchSelector,
    world: WorthQueryGraphObligationOperatingWorldSelector,
) -> WorthQueryGraphObligationRegistration {
    WorthQueryGraphObligationRegistration::blocking_invariant(rule(name), selector, world)
}

pub(super) fn relation_kind_id_selector() -> WorthQueryGraphTouchSelector {
    WorthQueryGraphTouchSelector::relation_kind_id(77)
}

pub(super) fn collection_selector() -> WorthQueryGraphTouchSelector {
    WorthQueryGraphTouchSelector::relation_kind("topology.edge").unwrap()
}

pub(super) fn unrelated_collection_selector() -> WorthQueryGraphTouchSelector {
    WorthQueryGraphTouchSelector::relation_kind("topology.face").unwrap()
}

pub(super) fn impossible_collection_selector() -> WorthQueryGraphTouchSelector {
    WorthQueryGraphTouchSelector::relation_kind("topology.impossible").unwrap()
}

pub(super) fn lifecycle_selector() -> WorthQueryGraphTouchSelector {
    WorthQueryGraphTouchSelector::lifecycle_family(
        WorthQueryGraphTouchLifecycleFamily::SameBatchSymbolicRelationRetirement,
    )
}

fn rule(name: &str) -> WorthQueryGraphObligationRuleIdentity {
    WorthQueryGraphObligationRuleIdentity::new("test.graph-obligation-index", name, "v1").unwrap()
}
