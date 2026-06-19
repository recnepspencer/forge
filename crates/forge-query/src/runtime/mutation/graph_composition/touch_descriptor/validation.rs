use std::collections::BTreeSet;

use crate::runtime::{
    ForgeQueryGraphCompositionBreadth, ForgeQueryGraphCompositionProgram,
    ForgeQueryGraphCompositionProgramStepKind, ForgeQueryWriteCommand,
};

use super::denial::{
    ForgeQueryGraphTouchDescriptorDenial, ForgeQueryGraphTouchDescriptorDenialKind,
};

pub(super) fn validate_graph_touch_descriptor_inputs(
    program: &ForgeQueryGraphCompositionProgram,
    breadth: &ForgeQueryGraphCompositionBreadth,
    commands: &[ForgeQueryWriteCommand],
) -> Result<(), ForgeQueryGraphTouchDescriptorDenial> {
    if program.is_empty() {
        return validate_empty_program_breadth(breadth);
    }
    require_equal(
        ForgeQueryGraphTouchDescriptorDenialKind::ProgramComponentCountMismatch,
        program.component_count(),
        commands.len(),
        "graph composition program component count must match command count",
    )?;
    require_equal(
        ForgeQueryGraphTouchDescriptorDenialKind::BreadthProgramComponentCountMismatch,
        breadth.component_count(),
        program.component_count(),
        "graph composition breadth component count must match program component count",
    )?;
    validate_program_step_indexes(program, commands)?;
    validate_program_command_semantics(program, commands)?;
    require_equal(
        ForgeQueryGraphTouchDescriptorDenialKind::SymbolicEntityDeclarationCountMismatch,
        breadth.symbolic_entity_declaration_count(),
        symbolic_entity_declaration_count(program),
        "graph composition breadth symbolic entity count must match program declarations",
    )?;
    require_equal(
        ForgeQueryGraphTouchDescriptorDenialKind::SymbolicRelationDeclarationCountMismatch,
        breadth.symbolic_relation_declaration_count(),
        symbolic_relation_declaration_count(program),
        "graph composition breadth symbolic relation count must match program declarations",
    )
}

fn validate_empty_program_breadth(
    breadth: &ForgeQueryGraphCompositionBreadth,
) -> Result<(), ForgeQueryGraphTouchDescriptorDenial> {
    require_equal(
        ForgeQueryGraphTouchDescriptorDenialKind::BreadthProgramComponentCountMismatch,
        breadth.component_count(),
        0,
        "ordinary batch descriptor may not carry graph breadth without a graph program",
    )?;
    require_equal(
        ForgeQueryGraphTouchDescriptorDenialKind::SymbolicEntityDeclarationCountMismatch,
        breadth.symbolic_entity_declaration_count(),
        0,
        "ordinary batch descriptor may not carry symbolic entity breadth",
    )?;
    require_equal(
        ForgeQueryGraphTouchDescriptorDenialKind::SymbolicRelationDeclarationCountMismatch,
        breadth.symbolic_relation_declaration_count(),
        0,
        "ordinary batch descriptor may not carry symbolic relation breadth",
    )
}

fn validate_program_step_indexes(
    program: &ForgeQueryGraphCompositionProgram,
    commands: &[ForgeQueryWriteCommand],
) -> Result<(), ForgeQueryGraphTouchDescriptorDenial> {
    let mut indexes = BTreeSet::new();
    for step in program.steps() {
        if step.component_index() >= commands.len() {
            return Err(ForgeQueryGraphTouchDescriptorDenial::new(
                ForgeQueryGraphTouchDescriptorDenialKind::ProgramStepIndexOutOfBounds,
                format!(
                    "graph composition program step index {} exceeds command count {}",
                    step.component_index(),
                    commands.len()
                ),
            ));
        }
        if !indexes.insert(step.component_index()) {
            return Err(ForgeQueryGraphTouchDescriptorDenial::new(
                ForgeQueryGraphTouchDescriptorDenialKind::DuplicateProgramStepIndex,
                format!(
                    "graph composition program step index {} appears more than once",
                    step.component_index()
                ),
            ));
        }
    }
    Ok(())
}

fn validate_program_command_semantics(
    program: &ForgeQueryGraphCompositionProgram,
    commands: &[ForgeQueryWriteCommand],
) -> Result<(), ForgeQueryGraphTouchDescriptorDenial> {
    for step in program.steps() {
        let command = &commands[step.component_index()];
        validate_program_command_collection(step, command)?;
        validate_program_command_symbol(step, command)?;
        validate_program_command_mutation_family(step, command)?;
    }
    Ok(())
}

fn validate_program_command_collection(
    step: &crate::runtime::ForgeQueryGraphCompositionProgramStep,
    command: &ForgeQueryWriteCommand,
) -> Result<(), ForgeQueryGraphTouchDescriptorDenial> {
    if let Some(collection) = command.declared_collection_ref() {
        if collection != step.declared_collection() {
            return Err(ForgeQueryGraphTouchDescriptorDenial::new(
                ForgeQueryGraphTouchDescriptorDenialKind::ProgramCommandCollectionMismatch,
                format!(
                    "program collection `{}` does not match command collection `{collection}`",
                    step.declared_collection()
                ),
            ));
        }
    }
    Ok(())
}

fn validate_program_command_symbol(
    step: &crate::runtime::ForgeQueryGraphCompositionProgramStep,
    command: &ForgeQueryWriteCommand,
) -> Result<(), ForgeQueryGraphTouchDescriptorDenial> {
    let Some(declared_symbol) = step.declared_symbol() else {
        return Ok(());
    };
    let Some(reference) = command.symbolic_target_reference() else {
        return Ok(());
    };
    if reference.symbol() == declared_symbol {
        return Ok(());
    }
    Err(ForgeQueryGraphTouchDescriptorDenial::new(
        ForgeQueryGraphTouchDescriptorDenialKind::ProgramCommandSymbolMismatch,
        format!(
            "program symbol `{declared_symbol}` does not match command symbol `{}`",
            reference.symbol()
        ),
    ))
}

fn validate_program_command_mutation_family(
    step: &crate::runtime::ForgeQueryGraphCompositionProgramStep,
    command: &ForgeQueryWriteCommand,
) -> Result<(), ForgeQueryGraphTouchDescriptorDenial> {
    let expected = expected_mutation_family(step.kind());
    if expected == command.mutation_family() {
        return Ok(());
    }
    Err(ForgeQueryGraphTouchDescriptorDenial::new(
        ForgeQueryGraphTouchDescriptorDenialKind::ProgramCommandMutationFamilyMismatch,
        format!(
            "program step `{}` expects mutation family `{}`, got `{}`",
            step.kind().as_str(),
            expected.as_str(),
            command.mutation_family().as_str()
        ),
    ))
}

fn expected_mutation_family(
    kind: ForgeQueryGraphCompositionProgramStepKind,
) -> crate::runtime::ForgeQueryMutationFamily {
    use crate::runtime::ForgeQueryMutationFamily;
    match kind {
        ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityDeclaration
        | ForgeQueryGraphCompositionProgramStepKind::RelationDeclaration
        | ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationDeclaration => {
            ForgeQueryMutationFamily::Insert
        }
        ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityFollowupMutation
        | ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationFollowupMutation
        | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetFollowupMutation
        | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetRetarget
        | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetSupersession
        | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedFollowupMutation
        | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetarget
        | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedSupersession => {
            ForgeQueryMutationFamily::Update
        }
        ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationRetirement
        | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetRetirement
        | ForgeQueryGraphCompositionProgramStepKind::ExistingTargetVerifiedRetirement => {
            ForgeQueryMutationFamily::Delete
        }
    }
}

fn symbolic_entity_declaration_count(program: &ForgeQueryGraphCompositionProgram) -> usize {
    program
        .steps()
        .iter()
        .filter(|step| {
            matches!(
                step.kind(),
                ForgeQueryGraphCompositionProgramStepKind::SymbolicEntityDeclaration
            )
        })
        .count()
}

fn symbolic_relation_declaration_count(program: &ForgeQueryGraphCompositionProgram) -> usize {
    program
        .steps()
        .iter()
        .filter(|step| {
            matches!(
                step.kind(),
                ForgeQueryGraphCompositionProgramStepKind::RelationDeclaration
                    | ForgeQueryGraphCompositionProgramStepKind::SymbolicRelationDeclaration
            )
        })
        .count()
}

fn require_equal(
    kind: ForgeQueryGraphTouchDescriptorDenialKind,
    actual: usize,
    expected: usize,
    message: &'static str,
) -> Result<(), ForgeQueryGraphTouchDescriptorDenial> {
    if actual == expected {
        return Ok(());
    }
    Err(ForgeQueryGraphTouchDescriptorDenial::new(
        kind,
        format!("{message}: actual={actual}; expected={expected}"),
    ))
}
