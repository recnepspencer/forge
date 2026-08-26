use worth_query_installation::facade::*;

use crate::binary_encoding::BinaryEncodingSink;
use crate::binary_input::BinaryInput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};
use crate::record::decode_budget::RecordDecodeAttempt;
use crate::record::sequence::{decode_sequence, require_canonical_sequence, write_sequence};

use super::conditional_node::{decode_nodes, write_nodes};
use super::input_contracts::{
    decode_capabilities, decode_required_domains, write_capabilities, write_required_domains,
};
use super::resource_contract::{decode_resource_contract, write_resource_contract};
use super::semantic_contracts::{decode_evidence, write_evidence};
use super::workflow_value::{decode_value, write_value};

pub(super) fn write_workflow(
    output: &mut dyn BinaryEncodingSink,
    value: &WorthQueryOperationWorkflowContract,
) -> Result<(), Denial> {
    match value {
        WorthQueryOperationWorkflowContract::NotRequired => output.u16(1),
        WorthQueryOperationWorkflowContract::Declared(workflow) => {
            output.u16(2)?;
            output.text(workflow.entry_stage())?;
            write_sequence(output, workflow.stages(), write_stage)
        }
    }
}

pub(super) fn decode_workflow(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<WorthQueryOperationWorkflowContract, Denial> {
    match input.u16()? {
        1 => Ok(WorthQueryOperationWorkflowContract::NotRequired),
        2 => Ok(WorthQueryOperationWorkflowContract::Declared(
            WorthQueryPortableWorkflowDefinition::new(
                input.text()?.to_owned(),
                decode_sequence(input, budget, 24, decode_stage)?,
            ),
        )),
        _ => unsupported(),
    }
}

fn write_stage(
    output: &mut dyn BinaryEncodingSink,
    stage: &WorthQueryPortableWorkflowStage,
) -> Result<(), Denial> {
    output.text(stage.identity())?;
    write_sequence(output, stage.predecessors(), |output, value| {
        output.text(value)
    })?;
    write_bool(output, stage.is_terminal())?;
    write_bool(output, stage.is_publishable())?;
    write_capabilities(output, stage.required_capabilities())?;
    write_semantics(output, stage.semantics())
}

fn decode_stage(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<WorthQueryPortableWorkflowStage, Denial> {
    let identity = input.text()?.to_owned();
    let predecessors = decode_sequence(input, budget, 4, |input, _| Ok(input.text()?.to_owned()))?;
    require_canonical_sequence(&predecessors)?;
    let terminal = decode_bool(input)?;
    let publishable = decode_bool(input)?;
    let capabilities = decode_capabilities(input, budget)?;
    require_canonical_sequence(&capabilities)?;
    let semantics = decode_semantics(input, budget)?;
    Ok(WorthQueryPortableWorkflowStage::new(
        identity,
        predecessors,
        terminal,
        publishable,
        capabilities,
    )
    .with_semantics(semantics))
}

fn write_semantics(
    output: &mut dyn BinaryEncodingSink,
    value: &WorthQueryWorkflowStageSemantics,
) -> Result<(), Denial> {
    write_value(output, &value.input)?;
    write_value(output, &value.output)?;
    write_evidence(output, &value.evidence)?;
    write_required_domains(output, &value.required_domain_roles)?;
    write_sequence(output, &value.graph_read_roles, |output, value| {
        output.text(value)
    })?;
    write_sequence(output, &value.touch_roles, |output, value| {
        output.text(value)
    })?;
    write_sequence(output, &value.effect_roles, |output, value| {
        output.u16(effect_tag(*value))
    })?;
    write_sequence(output, &value.invariant_roles, |output, value| {
        output.text(value)
    })?;
    write_sequence(output, &value.cost_roles, |output, value| {
        output.u16(cost_role_tag(*value))
    })?;
    write_resource_contract(output, &value.resources)?;
    write_sequence(output, &value.terminal_result_states, |output, value| {
        output.u16(result_state_tag(*value))
    })?;
    write_sequence(output, &value.failure_classes, write_failure_class)?;
    write_nodes(output, &value.conditional_nodes)
}

fn decode_semantics(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<WorthQueryWorkflowStageSemantics, Denial> {
    Ok(WorthQueryWorkflowStageSemantics {
        input: decode_value(input)?,
        output: decode_value(input)?,
        evidence: decode_evidence(input)?,
        required_domain_roles: decode_required_domains(input, budget)?,
        graph_read_roles: decode_texts(input, budget)?,
        touch_roles: decode_texts(input, budget)?,
        effect_roles: decode_sequence(input, budget, 2, |input, _| effect(input.u16()?))?,
        invariant_roles: decode_texts(input, budget)?,
        cost_roles: decode_sequence(input, budget, 2, |input, _| cost_role(input.u16()?))?,
        resources: decode_resource_contract(input, budget)?,
        terminal_result_states: decode_sequence(input, budget, 2, |input, _| {
            result_state(input.u16()?)
        })?,
        failure_classes: decode_sequence(input, budget, 2, |input, _| decode_failure_class(input))?,
        conditional_nodes: decode_nodes(input, budget)?,
    })
}

fn decode_texts(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<Vec<String>, Denial> {
    decode_sequence(input, budget, 4, |input, _| Ok(input.text()?.to_owned()))
}

fn write_failure_class(
    output: &mut dyn BinaryEncodingSink,
    value: &WorthQueryOperationFailureClass,
) -> Result<(), Denial> {
    output.u16(match value {
        WorthQueryOperationFailureClass::InvalidInput => 1,
        WorthQueryOperationFailureClass::Unsupported => 2,
        WorthQueryOperationFailureClass::Conflict => 3,
        WorthQueryOperationFailureClass::Dependency => 4,
        WorthQueryOperationFailureClass::Indeterminate => 5,
        WorthQueryOperationFailureClass::Domain(_) => 6,
    })?;
    if let WorthQueryOperationFailureClass::Domain(value) = value {
        output.text(value)?;
    }
    Ok(())
}

fn decode_failure_class(
    input: &mut BinaryInput<'_>,
) -> Result<WorthQueryOperationFailureClass, Denial> {
    match input.u16()? {
        1 => Ok(WorthQueryOperationFailureClass::InvalidInput),
        2 => Ok(WorthQueryOperationFailureClass::Unsupported),
        3 => Ok(WorthQueryOperationFailureClass::Conflict),
        4 => Ok(WorthQueryOperationFailureClass::Dependency),
        5 => Ok(WorthQueryOperationFailureClass::Indeterminate),
        6 => Ok(WorthQueryOperationFailureClass::Domain(
            input.text()?.to_owned(),
        )),
        _ => unsupported(),
    }
}

fn effect_tag(value: WorthQueryOperationEffectFamily) -> u16 {
    match value {
        WorthQueryOperationEffectFamily::Mutation => 1,
        WorthQueryOperationEffectFamily::Merge => 2,
        WorthQueryOperationEffectFamily::Writeback => 3,
    }
}
fn effect(tag: u16) -> Result<WorthQueryOperationEffectFamily, Denial> {
    match tag {
        1 => Ok(WorthQueryOperationEffectFamily::Mutation),
        2 => Ok(WorthQueryOperationEffectFamily::Merge),
        3 => Ok(WorthQueryOperationEffectFamily::Writeback),
        _ => unsupported(),
    }
}
fn cost_role_tag(value: WorthQueryWorkflowCostRole) -> u16 {
    match value {
        WorthQueryWorkflowCostRole::Admission => 1,
        WorthQueryWorkflowCostRole::GraphRead => 2,
        WorthQueryWorkflowCostRole::TouchEffect => 3,
        WorthQueryWorkflowCostRole::CommitAdmission => 4,
        WorthQueryWorkflowCostRole::Effect => 5,
        WorthQueryWorkflowCostRole::Invariant => 6,
        WorthQueryWorkflowCostRole::Execution => 7,
        WorthQueryWorkflowCostRole::ResultValidation => 8,
    }
}
fn cost_role(tag: u16) -> Result<WorthQueryWorkflowCostRole, Denial> {
    match tag {
        1 => Ok(WorthQueryWorkflowCostRole::Admission),
        2 => Ok(WorthQueryWorkflowCostRole::GraphRead),
        3 => Ok(WorthQueryWorkflowCostRole::TouchEffect),
        4 => Ok(WorthQueryWorkflowCostRole::CommitAdmission),
        5 => Ok(WorthQueryWorkflowCostRole::Effect),
        6 => Ok(WorthQueryWorkflowCostRole::Invariant),
        7 => Ok(WorthQueryWorkflowCostRole::Execution),
        8 => Ok(WorthQueryWorkflowCostRole::ResultValidation),
        _ => unsupported(),
    }
}
fn result_state_tag(value: WorthQueryOperationResultState) -> u16 {
    match value {
        WorthQueryOperationResultState::Ready => 1,
        WorthQueryOperationResultState::Advisory => 2,
        WorthQueryOperationResultState::Pending => 3,
        WorthQueryOperationResultState::Partial => 4,
        WorthQueryOperationResultState::Violation => 5,
    }
}
fn result_state(tag: u16) -> Result<WorthQueryOperationResultState, Denial> {
    match tag {
        1 => Ok(WorthQueryOperationResultState::Ready),
        2 => Ok(WorthQueryOperationResultState::Advisory),
        3 => Ok(WorthQueryOperationResultState::Pending),
        4 => Ok(WorthQueryOperationResultState::Partial),
        5 => Ok(WorthQueryOperationResultState::Violation),
        _ => unsupported(),
    }
}
fn write_bool(output: &mut dyn BinaryEncodingSink, value: bool) -> Result<(), Denial> {
    output.u16(u16::from(value))
}
fn decode_bool(input: &mut BinaryInput<'_>) -> Result<bool, Denial> {
    match input.u16()? {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(Denial::new(Kind::InvalidBooleanEncoding)),
    }
}
fn unsupported<T>() -> Result<T, Denial> {
    Err(Denial::new(Kind::UnsupportedRecordVariant))
}
