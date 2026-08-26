use worth_foundational::facade::FoundationalPerformanceCounterName as CounterName;
use worth_query_installation::facade::{
    WorthQueryStructuralCounterAggregation as Aggregation,
    WorthQueryStructuralCounterContract as CounterContract,
    WorthQueryStructuralCounterMonotonicity as Monotonicity,
    WorthQueryStructuralCounterReplayPosture as Replay,
    WorthQueryStructuralCounterRequiredness as Requiredness,
    WorthQueryStructuralCounterResetBoundary as ResetBoundary,
    WorthQueryStructuralCounterRole as Role, WorthQueryStructuralCounterSchema as CounterSchema,
    WorthQueryStructuralCounterScope as Scope, WorthQueryStructuralCounterUnit as Unit,
};

use crate::binary_encoding::BinaryEncodingSink;
use crate::binary_input::BinaryInput;
use crate::denial::{
    WorthQueryPackageArchiveDenial as Denial, WorthQueryPackageArchiveDenialKind as Kind,
};
use crate::record::decode_budget::RecordDecodeAttempt;
use crate::record::sequence::{
    decode_sequence, require_canonical_sequence, require_canonical_sequence_by, write_sequence,
};

pub(super) fn write_counters(
    output: &mut dyn BinaryEncodingSink,
    contract: &CounterContract,
) -> Result<(), Denial> {
    write_sequence(output, contract.rows(), |output, row| {
        output.text(row.name().as_str())?;
        output.u16(role_tag(row.role()))?;
        write_unit(output, row.unit())?;
        write_aggregation(output, row.aggregation())?;
        output.u16(monotonicity_tag(row.monotonicity()))?;
        output.u16(scope_tag(row.scope()))?;
        output.u16(reset_boundary_tag(row.reset_boundary()))?;
        output.u16(requiredness_tag(row.requiredness()))?;
        output.u16(replay_tag(row.replay()))
    })
}

pub(super) fn decode_counters(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<CounterContract, Denial> {
    let rows = decode_sequence(input, budget, 22, |input, budget| {
        let name = decode_counter_name(input)?;
        let role = role_from_tag(input.u16()?)?;
        let unit = decode_unit(input)?;
        let aggregation = decode_aggregation(input, budget)?;
        let monotonicity = monotonicity_from_tag(input.u16()?)?;
        let scope = scope_from_tag(input.u16()?)?;
        let reset_boundary = reset_boundary_from_tag(input.u16()?)?;
        let requiredness = requiredness_from_tag(input.u16()?)?;
        let replay = replay_from_tag(input.u16()?)?;
        Ok(CounterSchema::new(
            name,
            role,
            unit,
            aggregation,
            monotonicity,
            scope,
            reset_boundary,
            requiredness,
            replay,
        ))
    })?;
    require_canonical_sequence_by(&rows, |row| row.name())?;
    Ok(CounterContract::declare(rows))
}

fn write_unit(output: &mut dyn BinaryEncodingSink, unit: &Unit) -> Result<(), Denial> {
    match unit {
        Unit::Bytes => output.u16(1),
        Unit::Elements => output.u16(2),
        Unit::Operations => output.u16(3),
        Unit::Comparisons => output.u16(4),
        Unit::Iterations => output.u16(5),
        Unit::Neighborhoods => output.u16(6),
        Unit::Domain(value) => {
            output.u16(7)?;
            output.text(value)
        }
    }
}

fn decode_unit(input: &mut BinaryInput<'_>) -> Result<Unit, Denial> {
    match input.u16()? {
        1 => Ok(Unit::Bytes),
        2 => Ok(Unit::Elements),
        3 => Ok(Unit::Operations),
        4 => Ok(Unit::Comparisons),
        5 => Ok(Unit::Iterations),
        6 => Ok(Unit::Neighborhoods),
        7 => Ok(Unit::Domain(input.text()?.to_owned())),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

fn write_aggregation(
    output: &mut dyn BinaryEncodingSink,
    aggregation: &Aggregation,
) -> Result<(), Denial> {
    let (tag, sources) = match aggregation {
        Aggregation::Independent => {
            output.u16(1)?;
            return Ok(());
        }
        Aggregation::SumOf(sources) => (2, sources),
        Aggregation::MaximumOf(sources) => (3, sources),
        Aggregation::MinimumOf(sources) => (4, sources),
    };
    output.u16(tag)?;
    write_sequence(output, sources, |output, source| {
        output.text(source.as_str())
    })
}

fn decode_aggregation(
    input: &mut BinaryInput<'_>,
    budget: &mut RecordDecodeAttempt,
) -> Result<Aggregation, Denial> {
    let tag = input.u16()?;
    if tag == 1 {
        return Ok(Aggregation::Independent);
    }
    let sources = decode_sequence(input, budget, 4, |input, _| decode_counter_name(input))?;
    require_canonical_sequence(&sources)?;
    match tag {
        2 => Ok(Aggregation::SumOf(sources)),
        3 => Ok(Aggregation::MaximumOf(sources)),
        4 => Ok(Aggregation::MinimumOf(sources)),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

fn decode_counter_name(input: &mut BinaryInput<'_>) -> Result<CounterName, Denial> {
    CounterName::new(input.text()?.to_owned()).map_err(|_| Denial::new(Kind::InvalidRecordShape))
}

const fn role_tag(value: Role) -> u16 {
    match value {
        Role::Bytes => 1,
        Role::Elements => 2,
        Role::StructuralWork => 3,
        Role::DomainWork => 4,
    }
}

fn role_from_tag(tag: u16) -> Result<Role, Denial> {
    match tag {
        1 => Ok(Role::Bytes),
        2 => Ok(Role::Elements),
        3 => Ok(Role::StructuralWork),
        4 => Ok(Role::DomainWork),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

const fn monotonicity_tag(value: Monotonicity) -> u16 {
    match value {
        Monotonicity::Unconstrained => 1,
        Monotonicity::NonDecreasing => 2,
    }
}

fn monotonicity_from_tag(tag: u16) -> Result<Monotonicity, Denial> {
    match tag {
        1 => Ok(Monotonicity::Unconstrained),
        2 => Ok(Monotonicity::NonDecreasing),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

const fn scope_tag(value: Scope) -> u16 {
    match value {
        Scope::Operation => 1,
        Scope::Run => 2,
        Scope::Stage => 3,
        Scope::Attempt => 4,
        Scope::ArtifactOccurrence => 5,
    }
}

fn scope_from_tag(tag: u16) -> Result<Scope, Denial> {
    match tag {
        1 => Ok(Scope::Operation),
        2 => Ok(Scope::Run),
        3 => Ok(Scope::Stage),
        4 => Ok(Scope::Attempt),
        5 => Ok(Scope::ArtifactOccurrence),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

const fn reset_boundary_tag(value: ResetBoundary) -> u16 {
    match value {
        ResetBoundary::Operation => 1,
        ResetBoundary::Run => 2,
        ResetBoundary::Stage => 3,
        ResetBoundary::Attempt => 4,
        ResetBoundary::ArtifactOccurrence => 5,
    }
}

fn reset_boundary_from_tag(tag: u16) -> Result<ResetBoundary, Denial> {
    match tag {
        1 => Ok(ResetBoundary::Operation),
        2 => Ok(ResetBoundary::Run),
        3 => Ok(ResetBoundary::Stage),
        4 => Ok(ResetBoundary::Attempt),
        5 => Ok(ResetBoundary::ArtifactOccurrence),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

const fn requiredness_tag(value: Requiredness) -> u16 {
    match value {
        Requiredness::RequiredCore => 1,
        Requiredness::OptionalSidecar => 2,
    }
}

fn requiredness_from_tag(tag: u16) -> Result<Requiredness, Denial> {
    match tag {
        1 => Ok(Requiredness::RequiredCore),
        2 => Ok(Requiredness::OptionalSidecar),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}

const fn replay_tag(value: Replay) -> u16 {
    match value {
        Replay::Exact => 1,
        Replay::NonDecreasing => 2,
        Replay::ProviderCertified => 3,
        Replay::NotCompared => 4,
    }
}

fn replay_from_tag(tag: u16) -> Result<Replay, Denial> {
    match tag {
        1 => Ok(Replay::Exact),
        2 => Ok(Replay::NonDecreasing),
        3 => Ok(Replay::ProviderCertified),
        4 => Ok(Replay::NotCompared),
        _ => Err(Denial::new(Kind::UnsupportedRecordVariant)),
    }
}
