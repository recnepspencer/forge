use std::collections::BTreeMap;

use worth_query_installation::facade::{
    WorthQueryStructuralCounterAggregation, WorthQueryStructuralCounterContract,
    WorthQueryStructuralCounterMonotonicity, WorthQueryStructuralCounterReplayPosture,
    WorthQueryStructuralCounterRequiredness,
};

use super::{
    WorthQueryAdmittedStructuralCounter, WorthQueryDomainEvidenceAdmissionDenial,
    WorthQueryDomainEvidenceAdmissionDenialKind, WorthQueryStructuralCounterObservation,
};

pub(super) fn admit_counters(
    contract: &WorthQueryStructuralCounterContract,
    mut observations: Vec<WorthQueryStructuralCounterObservation>,
) -> Result<Vec<WorthQueryAdmittedStructuralCounter>, WorthQueryDomainEvidenceAdmissionDenial> {
    observations.sort_by(|left, right| left.name().cmp(right.name()));
    if observations
        .windows(2)
        .any(|pair| pair[0].name() == pair[1].name())
    {
        return Err(denial(
            WorthQueryDomainEvidenceAdmissionDenialKind::DuplicateCounter,
            "duplicate-counter",
        ));
    }
    for schema in contract.rows() {
        if schema.requiredness() == WorthQueryStructuralCounterRequiredness::RequiredCore
            && observations
                .binary_search_by(|candidate| candidate.name().cmp(schema.name()))
                .is_err()
        {
            return Err(denial(
                WorthQueryDomainEvidenceAdmissionDenialKind::MissingRequiredCounter,
                schema.name().as_str(),
            ));
        }
    }
    let by_name = observations
        .iter()
        .map(|observation| (observation.name().as_str(), observation))
        .collect::<BTreeMap<_, _>>();
    let mut admitted = Vec::with_capacity(observations.len());
    for observation in &observations {
        let schema = contract.row(observation.name()).ok_or_else(|| {
            denial(
                WorthQueryDomainEvidenceAdmissionDenialKind::UndeclaredCounter,
                observation.name().as_str(),
            )
        })?;
        validate_observation(schema, observation)?;
        validate_aggregate(schema, observation, &by_name)?;
        admitted.push(WorthQueryAdmittedStructuralCounter::new(
            schema.clone(),
            observation.initial(),
            observation.observed(),
            observation.provider_certification().map(str::to_owned),
        ));
    }
    Ok(admitted)
}

fn validate_observation(
    schema: &worth_query_installation::facade::WorthQueryStructuralCounterSchema,
    observation: &WorthQueryStructuralCounterObservation,
) -> Result<(), WorthQueryDomainEvidenceAdmissionDenial> {
    if schema.monotonicity() == WorthQueryStructuralCounterMonotonicity::NonDecreasing
        && observation.observed() < observation.initial()
    {
        return Err(denial(
            WorthQueryDomainEvidenceAdmissionDenialKind::CounterMovedBackward,
            schema.name().as_str(),
        ));
    }
    if schema.replay() == WorthQueryStructuralCounterReplayPosture::ProviderCertified
        && observation
            .provider_certification()
            .is_none_or(|identity| !portable(identity))
    {
        return Err(denial(
            WorthQueryDomainEvidenceAdmissionDenialKind::ProviderCertificationMissing,
            schema.name().as_str(),
        ));
    }
    Ok(())
}

fn validate_aggregate(
    schema: &worth_query_installation::facade::WorthQueryStructuralCounterSchema,
    observation: &WorthQueryStructuralCounterObservation,
    by_name: &BTreeMap<&str, &WorthQueryStructuralCounterObservation>,
) -> Result<(), WorthQueryDomainEvidenceAdmissionDenial> {
    let sources = schema.aggregation().sources();
    if matches!(
        schema.aggregation(),
        WorthQueryStructuralCounterAggregation::Independent
    ) {
        return Ok(());
    }
    let initial = aggregate(schema.aggregation(), sources, by_name, |source| {
        source.initial()
    });
    let observed = aggregate(schema.aggregation(), sources, by_name, |source| {
        source.observed()
    });
    if initial != Some(observation.initial()) || observed != Some(observation.observed()) {
        return Err(denial(
            WorthQueryDomainEvidenceAdmissionDenialKind::CounterAggregateMismatch,
            schema.name().as_str(),
        ));
    }
    Ok(())
}

fn aggregate(
    law: &WorthQueryStructuralCounterAggregation,
    sources: &[worth_foundational::facade::FoundationalPerformanceCounterName],
    by_name: &BTreeMap<&str, &WorthQueryStructuralCounterObservation>,
    value: impl Fn(&WorthQueryStructuralCounterObservation) -> u64,
) -> Option<u64> {
    let values = sources
        .iter()
        .map(|source| by_name.get(source.as_str()).copied().map(&value))
        .collect::<Option<Vec<_>>>()?;
    match law {
        WorthQueryStructuralCounterAggregation::Independent => None,
        WorthQueryStructuralCounterAggregation::SumOf(_) => values
            .into_iter()
            .try_fold(0_u64, |total, next| total.checked_add(next)),
        WorthQueryStructuralCounterAggregation::MaximumOf(_) => values.into_iter().max(),
        WorthQueryStructuralCounterAggregation::MinimumOf(_) => values.into_iter().min(),
    }
}

fn denial(
    kind: WorthQueryDomainEvidenceAdmissionDenialKind,
    subject: impl Into<String>,
) -> WorthQueryDomainEvidenceAdmissionDenial {
    WorthQueryDomainEvidenceAdmissionDenial::new(kind, subject)
}

fn portable(value: &str) -> bool {
    !value.trim().is_empty() && value.trim() == value && !value.chars().any(char::is_whitespace)
}
