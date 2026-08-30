use worth_query_installation::facade::*;

use crate::binary_encoding::BinaryEncodingSink;
use crate::binary_input::BinaryInput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};
use crate::record::decode_budget::RecordDecodeAttempt;
use crate::record::sequence::{decode_sequence, write_sequence};

pub(crate) fn write_replay(
    output: &mut dyn BinaryEncodingSink,
    value: &WorthQueryOperationReplayContract,
) -> Result<(), Denial> {
    match value {
        WorthQueryOperationReplayContract::NotSupported => output.u16(1),
        WorthQueryOperationReplayContract::ReExecutable => output.u16(2),
        WorthQueryOperationReplayContract::CertReplayable { comparator } => {
            output.u16(3)?;
            output.text(comparator.family())
        }
        WorthQueryOperationReplayContract::CertReplayableWithNoise { comparator, noise } => {
            output.u16(4)?;
            output.text(comparator.family())?;
            output.u16(u16::from(noise.diagnostic_warnings))
        }
    }
}

pub(crate) fn decode_replay(
    input: &mut BinaryInput<'_>,
) -> Result<WorthQueryOperationReplayContract, Denial> {
    match input.u16()? {
        1 => Ok(WorthQueryOperationReplayContract::NotSupported),
        2 => Ok(WorthQueryOperationReplayContract::ReExecutable),
        3 => Ok(WorthQueryOperationReplayContract::CertReplayable {
            comparator: decode_comparator(input)?,
        }),
        4 => Ok(WorthQueryOperationReplayContract::CertReplayableWithNoise {
            comparator: decode_comparator(input)?,
            noise: WorthQueryOperationReplayNoiseContract {
                diagnostic_warnings: decode_bool(input)?,
            },
        }),
        _ => unsupported(),
    }
}

pub(crate) fn write_lifecycle(
    output: &mut dyn BinaryEncodingSink,
    lineage: WorthQueryOperationLineageContract,
    promotion: WorthQueryOperationPromotionContract,
    publication: &WorthQueryOperationPublicationContract,
    consumption: WorthQueryOperationProjectionConsumptionContract,
) -> Result<(), Denial> {
    output.u16(match lineage {
        WorthQueryOperationLineageContract::NotRequired => 1,
        WorthQueryOperationLineageContract::Preserve => 2,
        WorthQueryOperationLineageContract::Evolve => 3,
    })?;
    output.u16(match promotion {
        WorthQueryOperationPromotionContract::NotRequired => 1,
        WorthQueryOperationPromotionContract::OnDurableReference => 2,
    })?;
    match publication {
        WorthQueryOperationPublicationContract::NotRequired => output.u16(1)?,
        WorthQueryOperationPublicationContract::DerivedProjection { projection_role } => {
            output.u16(2)?;
            output.text(projection_role.as_str())?;
        }
    }
    output.u16(match consumption {
        WorthQueryOperationProjectionConsumptionContract::NotRequired => 1,
        WorthQueryOperationProjectionConsumptionContract::QueryReadAuthority => 2,
    })
}

pub(crate) fn decode_lifecycle(
    input: &mut BinaryInput<'_>,
) -> Result<
    (
        WorthQueryOperationLineageContract,
        WorthQueryOperationPromotionContract,
        WorthQueryOperationPublicationContract,
        WorthQueryOperationProjectionConsumptionContract,
    ),
    Denial,
> {
    let lineage = match input.u16()? {
        1 => WorthQueryOperationLineageContract::NotRequired,
        2 => WorthQueryOperationLineageContract::Preserve,
        3 => WorthQueryOperationLineageContract::Evolve,
        _ => return unsupported(),
    };
    let promotion = match input.u16()? {
        1 => WorthQueryOperationPromotionContract::NotRequired,
        2 => WorthQueryOperationPromotionContract::OnDurableReference,
        _ => return unsupported(),
    };
    let publication = match input.u16()? {
        1 => WorthQueryOperationPublicationContract::NotRequired,
        2 => WorthQueryOperationPublicationContract::DerivedProjection {
            projection_role: WorthQueryOperationProjectionRole::new(input.text()?.to_owned())
                .map_err(|_| Denial::new(Kind::InvalidRecordShape))?,
        },
        _ => return unsupported(),
    };
    let consumption = match input.u16()? {
        1 => WorthQueryOperationProjectionConsumptionContract::NotRequired,
        2 => WorthQueryOperationProjectionConsumptionContract::QueryReadAuthority,
        _ => return unsupported(),
    };
    Ok((lineage, promotion, publication, consumption))
}

pub(crate) fn write_terminal_cost(
    output: &mut dyn BinaryEncodingSink,
    terminal: &WorthQueryOperationTerminalContract,
    cost: WorthQueryOperationCostContract,
) -> Result<(), Denial> {
    write_sequence(output, &terminal.result_states, |output, value| {
        output.u16(result_state_tag(*value))
    })?;
    write_sequence(output, &terminal.failure_classes, write_failure_class)?;
    for value in [cost.lookup, cost.execution, cost.result_width] {
        output.u16(cost_tag(value))?;
    }
    Ok(())
}

pub(crate) fn write_support_lowering(
    output: &mut dyn BinaryEncodingSink,
    support: WorthQueryOperationSupportRequirements,
    lowering: &WorthQueryOperationLoweringContract,
) -> Result<(), Denial> {
    for value in support_values(support) {
        output.u16(match value {
            WorthQuerySupportRequirement::NotRequired => 1,
            WorthQuerySupportRequirement::Required => 2,
        })?;
    }
    output.text(&lowering.family)?;
    output.u16(u16::from(lowering.deterministic))
}

pub(crate) fn decode_terminal_cost(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<
    (
        WorthQueryOperationTerminalContract,
        WorthQueryOperationCostContract,
    ),
    Denial,
> {
    let terminal = WorthQueryOperationTerminalContract {
        result_states: decode_sequence(input, budget, 2, |input, _| result_state(input.u16()?))?,
        failure_classes: decode_sequence(input, budget, 2, |input, _| decode_failure_class(input))?,
    };
    let cost = WorthQueryOperationCostContract {
        lookup: cost_class(input.u16()?)?,
        execution: cost_class(input.u16()?)?,
        result_width: cost_class(input.u16()?)?,
    };
    Ok((terminal, cost))
}

pub(crate) fn decode_support_lowering(
    input: &mut BinaryInput<'_>,
) -> Result<
    (
        WorthQueryOperationSupportRequirements,
        WorthQueryOperationLoweringContract,
    ),
    Denial,
> {
    let requirements = (0..14)
        .map(|_| support_requirement(input.u16()?))
        .collect::<Result<Vec<_>, Denial>>()?;
    let support = WorthQueryOperationSupportRequirements {
        live: requirements[0],
        continuation: requirements[1],
        async_result_state: requirements[2],
        recovery: requirements[3],
        inspection: requirements[4],
        projection_consumption: requirements[5],
        dependency_impact: requirements[6],
        sharing: requirements[7],
        invalidation: requirements[8],
        collection_delivery: requirements[9],
        conditional_evaluation: requirements[10],
        conditional_comparator: requirements[11],
        conditional_trigger: requirements[12],
        conditional_temporal_or_on_demand: requirements[13],
    };
    let lowering = WorthQueryOperationLoweringContract {
        family: input.text()?.to_owned(),
        deterministic: decode_bool(input)?,
    };
    Ok((support, lowering))
}

fn decode_comparator(
    input: &mut BinaryInput<'_>,
) -> Result<WorthQueryOperationReplayComparatorContract, Denial> {
    WorthQueryOperationReplayComparatorContract::new(input.text()?.to_owned())
        .map_err(|_| Denial::new(Kind::InvalidRecordShape))
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

fn support_values(
    value: WorthQueryOperationSupportRequirements,
) -> [WorthQuerySupportRequirement; 14] {
    [
        value.live,
        value.continuation,
        value.async_result_state,
        value.recovery,
        value.inspection,
        value.projection_consumption,
        value.dependency_impact,
        value.sharing,
        value.invalidation,
        value.collection_delivery,
        value.conditional_evaluation,
        value.conditional_comparator,
        value.conditional_trigger,
        value.conditional_temporal_or_on_demand,
    ]
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
fn cost_tag(value: WorthQueryOperationCostClass) -> u16 {
    match value {
        WorthQueryOperationCostClass::Constant => 1,
        WorthQueryOperationCostClass::DeclaredWidth => 2,
        WorthQueryOperationCostClass::GraphBreadth => 3,
        WorthQueryOperationCostClass::ExternalBoundary => 4,
    }
}
fn cost_class(tag: u16) -> Result<WorthQueryOperationCostClass, Denial> {
    match tag {
        1 => Ok(WorthQueryOperationCostClass::Constant),
        2 => Ok(WorthQueryOperationCostClass::DeclaredWidth),
        3 => Ok(WorthQueryOperationCostClass::GraphBreadth),
        4 => Ok(WorthQueryOperationCostClass::ExternalBoundary),
        _ => unsupported(),
    }
}
fn support_requirement(tag: u16) -> Result<WorthQuerySupportRequirement, Denial> {
    match tag {
        1 => Ok(WorthQuerySupportRequirement::NotRequired),
        2 => Ok(WorthQuerySupportRequirement::Required),
        _ => unsupported(),
    }
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
