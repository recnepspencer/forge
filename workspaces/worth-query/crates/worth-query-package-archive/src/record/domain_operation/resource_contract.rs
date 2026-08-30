use worth_query_declaration::facade::domain_computation::{
    WorthQueryCancellationSafePointFamily as SafePoint,
    WorthQueryExecutionDegradation as Degradation, WorthQueryExecutionMode as Mode,
    WorthQueryPartialEffectPosture as PartialEffect,
    WorthQueryResourceDimension as ResourceDimension,
    WorthQueryResourceLimitRequest as ResourceLimits,
    WorthQueryRetainedProgressPosture as RetainedProgress,
    WorthQuerySemanticScaleAxis as ScaleAxis, WorthQuerySemanticScaleRequest as ScaleLimits,
    WorthQueryYieldedStatePosture as YieldedState,
};
use worth_query_installation::facade::{
    WorthQueryExecutionAccessProductFamily as AccessProduct,
    WorthQueryExecutionAllocatorFamily as Allocator, WorthQueryExecutionProviderFamily as Provider,
    WorthQueryExecutionProviderRequirements as ProviderRequirements,
    WorthQueryExecutionResourceContract as ResourceContract,
    WorthQueryExecutionResourceEnvelope as Envelope,
    WorthQueryExecutionStrategyContract as Strategy,
    WorthQueryExecutionStrategyName as StrategyName,
};

use crate::binary_encoding::BinaryEncodingSink;
use crate::binary_input::BinaryInput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};
use crate::record::decode_budget::RecordDecodeAttempt;
use crate::record::sequence::{decode_sequence, write_sequence};

pub(super) fn write_resource_contract(
    output: &mut dyn BinaryEncodingSink,
    contract: &ResourceContract,
) -> Result<(), Denial> {
    match contract {
        ResourceContract::Undeclared => output.u16(1),
        ResourceContract::Declared { strategies } => {
            output.u16(2)?;
            write_sequence(output, strategies, write_strategy)
        }
    }
}

pub(super) fn decode_resource_contract(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<ResourceContract, Denial> {
    match input.u16()? {
        1 => Ok(ResourceContract::Undeclared),
        2 => {
            let strategies = decode_sequence(input, budget, 90, |input, _| decode_strategy(input))?;
            if strategies
                .windows(2)
                .any(|pair| pair[0].name() >= pair[1].name())
            {
                return Err(Denial::new(Kind::NonCanonicalRecordSequence));
            }
            ResourceContract::declared(strategies)
                .map_err(|_| Denial::new(Kind::InvalidRecordShape))
        }
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

fn write_strategy(output: &mut dyn BinaryEncodingSink, strategy: &Strategy) -> Result<(), Denial> {
    output.text(strategy.name().as_str())?;
    write_envelope(output, strategy.envelope())?;
    let providers = strategy.provider_requirements();
    output.text(providers.provider().as_str())?;
    output.text(providers.access_product().as_str())?;
    output.text(providers.allocator().as_str())
}

fn decode_strategy(input: &mut BinaryInput<'_>) -> Result<Strategy, Denial> {
    let name = StrategyName::new(input.text()?.to_owned())
        .map_err(|_| Denial::new(Kind::InvalidRecordShape))?;
    let envelope = decode_envelope(input)?;
    let providers = ProviderRequirements::new(
        Provider::new(input.text()?.to_owned())
            .map_err(|_| Denial::new(Kind::InvalidRecordShape))?,
        AccessProduct::new(input.text()?.to_owned())
            .map_err(|_| Denial::new(Kind::InvalidRecordShape))?,
        Allocator::new(input.text()?.to_owned())
            .map_err(|_| Denial::new(Kind::InvalidRecordShape))?,
    );
    Ok(Strategy::new(name, envelope, providers))
}

fn write_envelope(output: &mut dyn BinaryEncodingSink, envelope: &Envelope) -> Result<(), Denial> {
    for axis in ScaleAxis::ALL {
        output.u64(envelope.scale_ceiling(axis))?;
    }
    for dimension in ResourceDimension::ALL {
        output.u64(envelope.resource_ceiling(dimension))?;
    }
    output.u16(match envelope.mode() {
        Mode::Synchronous => 1,
        Mode::Asynchronous => 2,
    })?;
    output.u16(match envelope.degradation() {
        None => 1,
        Some(Degradation::PartialResult) => 2,
    })?;
    output.u16(match envelope.partial_effect_posture() {
        PartialEffect::EffectFree => 1,
        PartialEffect::PartialEffectsMayRemain => 2,
    })?;
    output.u16(match envelope.yielded_state_posture() {
        YieldedState::NotYieldable => 1,
        YieldedState::ProviderCheckpoint => 2,
    })?;
    output.u16(match envelope.retained_progress_posture() {
        RetainedProgress::ReleaseAfterAttempt => 1,
        RetainedProgress::RetainAttemptCapacity => 2,
    })?;
    output.text(envelope.cancellation_safe_point().as_str())
}

fn decode_envelope(input: &mut BinaryInput<'_>) -> Result<Envelope, Denial> {
    let mut scale = ScaleLimits::bounded(input.u64()?);
    for axis in ScaleAxis::ALL.into_iter().skip(1) {
        scale = scale.with(axis, input.u64()?);
    }
    let mut resources = ResourceLimits::bounded(input.u64()?);
    for dimension in ResourceDimension::ALL.into_iter().skip(1) {
        resources = resources.with(dimension, input.u64()?);
    }
    let mode = match input.u16()? {
        1 => Mode::Synchronous,
        2 => Mode::Asynchronous,
        _ => return Err(Denial::new(Kind::UnsupportedRecordVariant)),
    };
    let degradation = match input.u16()? {
        1 => None,
        2 => Some(Degradation::PartialResult),
        _ => return Err(Denial::new(Kind::UnsupportedRecordVariant)),
    };
    let partial_effect = match input.u16()? {
        1 => PartialEffect::EffectFree,
        2 => PartialEffect::PartialEffectsMayRemain,
        _ => return Err(Denial::new(Kind::UnsupportedRecordVariant)),
    };
    let yielded_state = match input.u16()? {
        1 => YieldedState::NotYieldable,
        2 => YieldedState::ProviderCheckpoint,
        _ => return Err(Denial::new(Kind::UnsupportedRecordVariant)),
    };
    let retained_progress = match input.u16()? {
        1 => RetainedProgress::ReleaseAfterAttempt,
        2 => RetainedProgress::RetainAttemptCapacity,
        _ => return Err(Denial::new(Kind::UnsupportedRecordVariant)),
    };
    let safe_point = SafePoint::new(input.text()?.to_owned())
        .map_err(|_| Denial::new(Kind::InvalidRecordShape))?;
    Ok(
        Envelope::new(scale, resources, mode, degradation, safe_point)
            .with_partial_effect_posture(partial_effect)
            .with_yielded_state_posture(yielded_state)
            .with_retained_progress_posture(retained_progress),
    )
}
