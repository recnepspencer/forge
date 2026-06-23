use crate::runtime::{
    ForgeQueryAspectValue, ForgeQueryGraphCompositionBreadth, ForgeQueryGraphCompositionProgram,
    ForgeQueryGraphCompositionProgramStep, ForgeQueryGraphCompositionProgramStepKind,
    ForgeQueryGraphTouchDescriptor, ForgeQueryMutationMetadata, ForgeQuerySymbolicTargetReference,
    ForgeQueryWriteCommand,
};
use forge_relational::facade::identity::KindId;

pub(super) fn descriptor_for_step_kind(
    kind: ForgeQueryGraphCompositionProgramStepKind,
) -> ForgeQueryGraphTouchDescriptor {
    let (commands, breadth, program) = if matches!(
        kind,
        ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationFollowupMutation
            | ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityFollowupMutation
            | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetFollowupMutation
            | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetRetarget
            | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetSupersession
            | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedFollowupMutation
            | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetarget
            | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedSupersession
    ) {
        one_step_update_program(kind, "topology.edge", "edge", "weight")
    } else {
        one_step_delete_program(kind, "topology.edge", "edge", vec!["weight"])
    };
    ForgeQueryGraphTouchDescriptor::from_authoritative_mutation_batch(&program, &breadth, &commands)
        .unwrap()
}

pub(super) fn descriptor_for_collection(collection: &str) -> ForgeQueryGraphTouchDescriptor {
    let (commands, breadth, program) = one_step_delete_program(
        ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement,
        collection,
        "edge",
        vec!["weight"],
    );
    ForgeQueryGraphTouchDescriptor::from_authoritative_mutation_batch(&program, &breadth, &commands)
        .unwrap()
}

pub(super) fn descriptor_for_relation_kind_id(
    relation_kind_id: KindId,
) -> ForgeQueryGraphTouchDescriptor {
    let (commands, breadth, program) = one_step_delete_program_with_relation_kind_id(
        ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement,
        "topology.edge",
        "edge",
        vec!["weight"],
        relation_kind_id,
    );
    ForgeQueryGraphTouchDescriptor::from_authoritative_mutation_batch(&program, &breadth, &commands)
        .unwrap()
}

pub(super) fn descriptor_for_touched_paths(
    touched_paths: Vec<&str>,
) -> ForgeQueryGraphTouchDescriptor {
    let (commands, breadth, program) = one_step_delete_program(
        ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement,
        "topology.edge",
        "edge",
        touched_paths,
    );
    ForgeQueryGraphTouchDescriptor::from_authoritative_mutation_batch(&program, &breadth, &commands)
        .unwrap()
}

pub(super) fn one_step_delete_program(
    kind: ForgeQueryGraphCompositionProgramStepKind,
    collection: &str,
    symbol: &str,
    touched_paths: Vec<&str>,
) -> (
    Vec<ForgeQueryWriteCommand>,
    ForgeQueryGraphCompositionBreadth,
    ForgeQueryGraphCompositionProgram,
) {
    let command = ForgeQueryWriteCommand::DeleteSymbolicAspects {
        reference: reference(symbol, collection),
        touched_aspect_paths: touched_paths.into_iter().map(str::to_string).collect(),
        metadata: ForgeQueryMutationMetadata::new(),
        naming_intent: None,
    };
    one_step_program(kind, collection, symbol, command)
}

pub(super) fn one_step_delete_program_with_relation_kind_id(
    kind: ForgeQueryGraphCompositionProgramStepKind,
    collection: &str,
    symbol: &str,
    touched_paths: Vec<&str>,
    relation_kind_id: KindId,
) -> (
    Vec<ForgeQueryWriteCommand>,
    ForgeQueryGraphCompositionBreadth,
    ForgeQueryGraphCompositionProgram,
) {
    let command = ForgeQueryWriteCommand::DeleteSymbolicAspects {
        reference: reference(symbol, collection),
        touched_aspect_paths: touched_paths.into_iter().map(str::to_string).collect(),
        metadata: ForgeQueryMutationMetadata::new(),
        naming_intent: None,
    };
    one_step_program_with_relation_kind_id(
        kind,
        collection,
        symbol,
        command,
        Some(relation_kind_id),
    )
}

pub(super) fn one_step_update_program(
    kind: ForgeQueryGraphCompositionProgramStepKind,
    collection: &str,
    symbol: &str,
    aspect_path: &str,
) -> (
    Vec<ForgeQueryWriteCommand>,
    ForgeQueryGraphCompositionBreadth,
    ForgeQueryGraphCompositionProgram,
) {
    let command = ForgeQueryWriteCommand::UpdateSymbolicAspects {
        reference: reference(symbol, collection),
        aspects: vec![ForgeQueryAspectValue::new(aspect_path, 1).unwrap()],
        metadata: ForgeQueryMutationMetadata::new(),
        naming_intent: None,
        continuity_intent: None,
    };
    one_step_program(kind, collection, symbol, command)
}

fn one_step_program(
    kind: ForgeQueryGraphCompositionProgramStepKind,
    collection: &str,
    symbol: &str,
    command: ForgeQueryWriteCommand,
) -> (
    Vec<ForgeQueryWriteCommand>,
    ForgeQueryGraphCompositionBreadth,
    ForgeQueryGraphCompositionProgram,
) {
    one_step_program_with_relation_kind_id(kind, collection, symbol, command, None)
}

fn one_step_program_with_relation_kind_id(
    kind: ForgeQueryGraphCompositionProgramStepKind,
    collection: &str,
    symbol: &str,
    command: ForgeQueryWriteCommand,
    relation_kind_id: Option<KindId>,
) -> (
    Vec<ForgeQueryWriteCommand>,
    ForgeQueryGraphCompositionBreadth,
    ForgeQueryGraphCompositionProgram,
) {
    let breadth = ForgeQueryGraphCompositionBreadth::new(
        1,
        symbolic_entity_declaration_count(kind),
        symbolic_relation_declaration_count(kind),
    );
    let mut step =
        ForgeQueryGraphCompositionProgramStep::new(0, kind, collection, Some(symbol.to_string()));
    if let Some(relation_kind_id) = relation_kind_id {
        step = step.with_relation_kind_id(relation_kind_id);
    }
    let program = ForgeQueryGraphCompositionProgram::new(vec![step], &breadth);
    (vec![command], breadth, program)
}

fn reference(symbol: &str, collection: &str) -> ForgeQuerySymbolicTargetReference {
    ForgeQuerySymbolicTargetReference::new(symbol)
        .unwrap()
        .in_target_collection(collection)
        .unwrap()
}

fn symbolic_entity_declaration_count(kind: ForgeQueryGraphCompositionProgramStepKind) -> usize {
    if matches!(
        kind,
        ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityDeclaration
    ) {
        1
    } else {
        0
    }
}

fn symbolic_relation_declaration_count(kind: ForgeQueryGraphCompositionProgramStepKind) -> usize {
    if matches!(
        kind,
        ForgeQueryGraphCompositionProgramStepKind::RelationDeclaration
            | ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationDeclaration
    ) {
        1
    } else {
        0
    }
}
