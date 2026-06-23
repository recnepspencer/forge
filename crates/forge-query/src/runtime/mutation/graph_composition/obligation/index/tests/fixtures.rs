use crate::runtime::{
    ForgeQueryAspectValue, ForgeQueryGraphCompositionBreadth, ForgeQueryGraphCompositionProgram,
    ForgeQueryGraphCompositionProgramStep, ForgeQueryGraphCompositionProgramStepKind,
    ForgeQueryGraphObligationOperatingWorldSelector, ForgeQueryGraphObligationRegistration,
    ForgeQueryGraphObligationRegistrationCatalog, ForgeQueryGraphObligationRuleIdentity,
    ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchLifecycleFamily,
    ForgeQueryGraphTouchSelector, ForgeQueryMutationMetadata, ForgeQuerySymbolicTargetReference,
    ForgeQueryWriteCommand,
};
use forge_relational::facade::identity::KindId;

pub(super) fn symbolic_relation_retirement_descriptor() -> ForgeQueryGraphTouchDescriptor {
    let command = ForgeQueryWriteCommand::DeleteSymbolicAspects {
        reference: ForgeQuerySymbolicTargetReference::new("edge")
            .unwrap()
            .in_target_collection("topology.edge")
            .unwrap(),
        touched_aspect_paths: vec!["weight".to_string()],
        metadata: ForgeQueryMutationMetadata::new(),
        naming_intent: None,
    };
    let breadth = ForgeQueryGraphCompositionBreadth::new(1, 0, 0);
    let step = ForgeQueryGraphCompositionProgramStep::new(
        0,
        ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement,
        "topology.edge",
        Some("edge".to_string()),
    )
    .with_relation_kind_id(KindId(77));
    let program = ForgeQueryGraphCompositionProgram::new(vec![step], &breadth);
    ForgeQueryGraphTouchDescriptor::from_authoritative_mutation_batch(
        &program,
        &breadth,
        &[command],
    )
    .unwrap()
}

pub(super) fn multi_component_descriptor() -> ForgeQueryGraphTouchDescriptor {
    let commands = vec![
        ForgeQueryWriteCommand::UpdateSymbolicAspects {
            reference: ForgeQuerySymbolicTargetReference::new("edge")
                .unwrap()
                .in_target_collection("topology.edge")
                .unwrap(),
            aspects: vec![ForgeQueryAspectValue::new("capacity", 1).unwrap()],
            metadata: ForgeQueryMutationMetadata::new(),
            naming_intent: None,
            continuity_intent: None,
        },
        ForgeQueryWriteCommand::DeleteSymbolicAspects {
            reference: ForgeQuerySymbolicTargetReference::new("face")
                .unwrap()
                .in_target_collection("topology.face")
                .unwrap(),
            touched_aspect_paths: vec!["boundary".to_string()],
            metadata: ForgeQueryMutationMetadata::new(),
            naming_intent: None,
        },
    ];
    let breadth = ForgeQueryGraphCompositionBreadth::new(2, 0, 0);
    let steps = vec![
        ForgeQueryGraphCompositionProgramStep::new(
            0,
            ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationFollowupMutation,
            "topology.edge",
            Some("edge".to_string()),
        )
        .with_relation_kind_id(KindId(77)),
        ForgeQueryGraphCompositionProgramStep::new(
            1,
            ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement,
            "topology.face",
            Some("face".to_string()),
        )
        .with_relation_kind_id(KindId(88)),
    ];
    let program = ForgeQueryGraphCompositionProgram::new(steps, &breadth);
    ForgeQueryGraphTouchDescriptor::from_authoritative_mutation_batch(&program, &breadth, &commands)
        .unwrap()
}

pub(super) fn catalog(
    registrations: Vec<ForgeQueryGraphObligationRegistration>,
) -> ForgeQueryGraphObligationRegistrationCatalog {
    ForgeQueryGraphObligationRegistrationCatalog::from_registrations(registrations).unwrap()
}

pub(super) fn schema_registration(
    name: &str,
    selector: ForgeQueryGraphTouchSelector,
    world: ForgeQueryGraphObligationOperatingWorldSelector,
) -> ForgeQueryGraphObligationRegistration {
    ForgeQueryGraphObligationRegistration::schema_contract_validator(rule(name), selector, world)
}

pub(super) fn blocking_registration(
    name: &str,
    selector: ForgeQueryGraphTouchSelector,
    world: ForgeQueryGraphObligationOperatingWorldSelector,
) -> ForgeQueryGraphObligationRegistration {
    ForgeQueryGraphObligationRegistration::blocking_invariant(rule(name), selector, world)
}

pub(super) fn relation_kind_id_selector() -> ForgeQueryGraphTouchSelector {
    ForgeQueryGraphTouchSelector::relation_kind_id(77)
}

pub(super) fn collection_selector() -> ForgeQueryGraphTouchSelector {
    ForgeQueryGraphTouchSelector::relation_kind("topology.edge").unwrap()
}

pub(super) fn unrelated_collection_selector() -> ForgeQueryGraphTouchSelector {
    ForgeQueryGraphTouchSelector::relation_kind("topology.face").unwrap()
}

pub(super) fn impossible_collection_selector() -> ForgeQueryGraphTouchSelector {
    ForgeQueryGraphTouchSelector::relation_kind("topology.impossible").unwrap()
}

pub(super) fn lifecycle_selector() -> ForgeQueryGraphTouchSelector {
    ForgeQueryGraphTouchSelector::lifecycle_family(
        ForgeQueryGraphTouchLifecycleFamily::SameBatchSymbolicRelationRetirement,
    )
}

fn rule(name: &str) -> ForgeQueryGraphObligationRuleIdentity {
    ForgeQueryGraphObligationRuleIdentity::new("test.graph-obligation-index", name, "v1").unwrap()
}
