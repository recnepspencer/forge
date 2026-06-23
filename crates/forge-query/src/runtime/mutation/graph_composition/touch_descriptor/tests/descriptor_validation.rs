use crate::runtime::{
    ForgeQueryGraphCompositionBreadth, ForgeQueryGraphCompositionProgram,
    ForgeQueryGraphCompositionProgramStep, ForgeQueryGraphCompositionProgramStepKind,
    ForgeQueryGraphTouchDescriptor, ForgeQueryGraphTouchDescriptorDenialKind,
    ForgeQueryMutationMetadata, ForgeQueryMutationTargetCollectionIdentity,
    ForgeQuerySymbolicTargetReference, ForgeQueryWriteCommand,
};

use super::fixtures::{one_step_delete_program, touch};

#[test]
fn mismatched_program_command_breadth_is_denied_before_descriptor_identity() {
    let (mut commands, breadth, program) = one_step_delete_program(
        ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement,
        "topology.edge",
        "edge",
        vec!["weight"],
    );
    commands.push(commands[0].clone());
    let denial = ForgeQueryGraphTouchDescriptor::from_authoritative_mutation_batch(
        &program, &breadth, &commands,
    )
    .unwrap_err();

    assert_eq!(
        denial.kind(),
        ForgeQueryGraphTouchDescriptorDenialKind::ProgramComponentCountMismatch
    );
}

#[test]
fn mismatched_symbolic_relation_breadth_is_denied_before_descriptor_identity() {
    let (commands, _breadth, program) = one_step_delete_program(
        ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement,
        "topology.edge",
        "edge",
        vec!["weight"],
    );
    let dishonest_breadth = ForgeQueryGraphCompositionBreadth::new(1, 0, 1);

    let denial = ForgeQueryGraphTouchDescriptor::from_authoritative_mutation_batch(
        &program,
        &dishonest_breadth,
        &commands,
    )
    .unwrap_err();

    assert_eq!(
        denial.kind(),
        ForgeQueryGraphTouchDescriptorDenialKind::SymbolicRelationDeclarationCountMismatch
    );
}

#[test]
fn mismatched_program_command_collection_is_denied_with_matching_counts() {
    let command = ForgeQueryWriteCommand::DeleteSymbolicAspects {
        reference: reference("edge", "topology.edge"),
        touched_aspects: vec![touch("weight")],
        metadata: ForgeQueryMutationMetadata::new(),
        naming_intent: None,
    };
    let breadth = ForgeQueryGraphCompositionBreadth::new(1, 0, 0);
    let program = ForgeQueryGraphCompositionProgram::new(
        vec![ForgeQueryGraphCompositionProgramStep::new(
            0,
            ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement,
            Some(target_collection("topology.face")),
            Some("edge".to_string()),
        )],
        &breadth,
    );

    let denial = ForgeQueryGraphTouchDescriptor::from_authoritative_mutation_batch(
        &program,
        &breadth,
        &[command],
    )
    .unwrap_err();

    assert_eq!(
        denial.kind(),
        ForgeQueryGraphTouchDescriptorDenialKind::ProgramCommandCollectionMismatch
    );
}

#[test]
fn mismatched_program_command_symbol_is_denied_with_matching_counts() {
    let command = ForgeQueryWriteCommand::DeleteSymbolicAspects {
        reference: reference("edge", "topology.edge"),
        touched_aspects: vec![touch("weight")],
        metadata: ForgeQueryMutationMetadata::new(),
        naming_intent: None,
    };
    let breadth = ForgeQueryGraphCompositionBreadth::new(1, 0, 0);
    let program = ForgeQueryGraphCompositionProgram::new(
        vec![ForgeQueryGraphCompositionProgramStep::new(
            0,
            ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement,
            Some(target_collection("topology.edge")),
            Some("other-edge".to_string()),
        )],
        &breadth,
    );

    let denial = ForgeQueryGraphTouchDescriptor::from_authoritative_mutation_batch(
        &program,
        &breadth,
        &[command],
    )
    .unwrap_err();

    assert_eq!(
        denial.kind(),
        ForgeQueryGraphTouchDescriptorDenialKind::ProgramCommandSymbolMismatch
    );
}

fn reference(symbol: &str, collection: &str) -> ForgeQuerySymbolicTargetReference {
    ForgeQuerySymbolicTargetReference::new(symbol)
        .unwrap()
        .in_target_collection(collection)
        .unwrap()
}

fn target_collection(collection: &str) -> ForgeQueryMutationTargetCollectionIdentity {
    ForgeQueryMutationTargetCollectionIdentity::new("graph-composition-test", collection)
}
