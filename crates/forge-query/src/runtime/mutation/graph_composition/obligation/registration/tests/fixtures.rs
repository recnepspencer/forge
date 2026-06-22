use crate::runtime::{
    ForgeQueryAspectTouch, ForgeQueryGraphCompositionBreadth, ForgeQueryGraphCompositionProgram,
    ForgeQueryGraphCompositionProgramStep, ForgeQueryGraphCompositionProgramStepKind,
    ForgeQueryGraphObligationOperatingWorldSelector, ForgeQueryGraphObligationRegistration,
    ForgeQueryGraphObligationRuleIdentity, ForgeQueryGraphTouchDescriptor,
    ForgeQueryGraphTouchSelector, ForgeQueryMutationMetadata, ForgeQuerySymbolicTargetReference,
    ForgeQueryWriteCommand,
};
use forge_relational::facade::identity::KindId;

pub(super) fn symbolic_relation_touch_descriptor(
    collection: &str,
    touched_aspect_path: &str,
) -> ForgeQueryGraphTouchDescriptor {
    symbolic_relation_touch_descriptor_with_relation_kind_id(collection, touched_aspect_path, None)
}

pub(super) fn symbolic_relation_touch_descriptor_with_relation_kind_id(
    collection: &str,
    touched_aspect_path: &str,
    relation_kind_id: Option<KindId>,
) -> ForgeQueryGraphTouchDescriptor {
    let command = ForgeQueryWriteCommand::DeleteSymbolicAspects {
        reference: ForgeQuerySymbolicTargetReference::new("edge")
            .unwrap()
            .in_target_collection(collection)
            .unwrap(),
        touched_aspects: vec![touch(touched_aspect_path)],
        metadata: ForgeQueryMutationMetadata::new(),
        naming_intent: None,
    };
    let breadth = ForgeQueryGraphCompositionBreadth::new(1, 0, 0);
    let mut step = ForgeQueryGraphCompositionProgramStep::new(
        0,
        ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement,
        collection,
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

pub(super) fn touch(aspect_path: &str) -> ForgeQueryAspectTouch {
    ForgeQueryAspectTouch::from_authoring_path(aspect_path.to_string())
        .expect("test aspect path should parse")
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
