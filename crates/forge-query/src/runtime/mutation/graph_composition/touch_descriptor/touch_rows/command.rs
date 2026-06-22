use std::collections::BTreeMap;

use crate::runtime::{
    ForgeQueryGraphCompositionProgram, ForgeQueryGraphCompositionProgramStep,
    ForgeQueryWriteCommand,
};

use super::super::lifecycle_family::ForgeQueryGraphTouchLifecycleFamily;
use super::{ForgeQueryGraphTouchDescriptorRow, ForgeQueryGraphTouchDescriptorRowInput};

pub(in super::super) fn derive_command_touch_rows(
    program: &ForgeQueryGraphCompositionProgram,
    commands: &[ForgeQueryWriteCommand],
) -> Vec<ForgeQueryGraphTouchDescriptorRow> {
    let steps_by_component = program
        .steps()
        .iter()
        .map(|step| (step.component_index(), step))
        .collect::<BTreeMap<_, _>>();
    commands
        .iter()
        .enumerate()
        .map(|(component_index, command)| {
            let step = steps_by_component.get(&component_index).copied();
            row_from_command(component_index, command, step)
        })
        .collect()
}

fn row_from_command(
    component_index: usize,
    command: &ForgeQueryWriteCommand,
    step: Option<&ForgeQueryGraphCompositionProgramStep>,
) -> ForgeQueryGraphTouchDescriptorRow {
    let program_step_kind = step.map(ForgeQueryGraphCompositionProgramStep::kind);
    let lifecycle_family =
        program_step_kind.map(ForgeQueryGraphTouchLifecycleFamily::from_program_step_kind);
    let declared_collection = command
        .declared_collection()
        .or_else(|| step.map(|step| step.declared_collection().to_string()));
    let relation_kind_id = step.and_then(ForgeQueryGraphCompositionProgramStep::relation_kind_id);
    let declared_symbol = step.and_then(|step| step.declared_symbol().map(str::to_string));
    ForgeQueryGraphTouchDescriptorRow::new(ForgeQueryGraphTouchDescriptorRowInput {
        component_index,
        mutation_family: command.mutation_family(),
        read_verb: None,
        program_step_kind,
        lifecycle_family,
        declared_collection,
        relation_kind_id,
        declared_symbol,
        declared_aspect_operations: declared_aspect_operations(command),
        touched_aspect_paths: sorted_unique(command.touched_aspect_paths().iter().cloned()),
        has_symbolic_target_reference: command.symbolic_target_reference().is_some(),
        has_existing_truth_binding: command.existing_truth_binding().is_some(),
        symbolic_aspect_reference_count: command.symbolic_aspect_references().len(),
    })
}

fn declared_aspect_operations(command: &ForgeQueryWriteCommand) -> Vec<String> {
    sorted_unique(
        command
            .declared_aspect_operations()
            .into_iter()
            .map(|operation| format!("{}:{}", operation.kind().as_str(), operation.aspect_path())),
    )
}

fn sorted_unique(values: impl IntoIterator<Item = String>) -> Vec<String> {
    values
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect()
}
