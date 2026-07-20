use worth_query_installation::facade::{
    WorthQueryArtifactReuseEquivalence, WorthQueryComparatorRequirement,
    WorthQueryConditionalConditionClass, WorthQueryOutputEquivalenceRequirement,
    WorthQueryPortableConditionalNodeDeclaration,
};
use worth_signal::facade::{
    AspectMask, SignalConditionalArtifactReuse, SignalConditionalCondition,
    SignalConditionalContractDefinition, SignalConditionalVersionComparator,
};

pub(super) fn lower_signal_contract(
    declaration: &WorthQueryPortableConditionalNodeDeclaration,
    dependency_aspects: AspectMask,
    condition_aspects: AspectMask,
) -> Result<SignalConditionalContractDefinition, super::BridgeConditionalDenial> {
    let trigger_aspects = condition_aspect_mask(declaration, dependency_aspects, condition_aspects);
    let condition = match declaration.condition().class() {
        WorthQueryConditionalConditionClass::AlwaysEligible => SignalConditionalCondition::Always,
        WorthQueryConditionalConditionClass::AspectFiltered => {
            SignalConditionalCondition::AspectFilter(trigger_aspects)
        }
        WorthQueryConditionalConditionClass::OnDemand => SignalConditionalCondition::OnDemand,
        WorthQueryConditionalConditionClass::DeltaThreshold => {
            SignalConditionalCondition::DeltaThreshold(lower_delta_threshold(declaration)?)
        }
        WorthQueryConditionalConditionClass::DomainSpecific => {
            SignalConditionalCondition::RuntimePredicate
        }
        WorthQueryConditionalConditionClass::Temporal => SignalConditionalCondition::TemporalWake,
    };
    Ok(SignalConditionalContractDefinition {
        condition,
        dependency_aspects,
        trigger_aspects,
        dependency_comparator: lower_dependency_comparator(declaration.dependency_comparator()),
        output_comparator: lower_output_comparator(declaration.output_equivalence()),
        artifact_reuse: lower_artifact_reuse(declaration.artifact_reuse_equivalence()),
    })
}

fn lower_delta_threshold(
    declaration: &WorthQueryPortableConditionalNodeDeclaration,
) -> Result<worth_signal::facade::SignalDeltaThresholdContract, super::BridgeConditionalDenial> {
    use worth_query_installation::facade::{
        WorthQueryDeltaComparisonDomain as Domain, WorthQueryQuantityValueFamily as Family,
        WorthQueryThresholdBoundary as Boundary,
    };
    let (_, threshold) = declaration
        .condition()
        .delta_threshold_contract()
        .ok_or_else(|| {
            super::BridgeConditionalDenial::new(
                super::BridgeConditionalDenialKind::SignalContractInstallation,
                "delta-threshold declaration lost its typed threshold contract",
            )
        })?;
    Ok(worth_signal::facade::SignalDeltaThresholdContract::new(
        threshold.value().clone(),
        threshold.unit().as_str(),
        match threshold.value_family() {
            Family::Integer => worth_signal::facade::SignalThresholdValueFamily::Integer,
            Family::Float32 => worth_signal::facade::SignalThresholdValueFamily::Float32,
            Family::Float64 => worth_signal::facade::SignalThresholdValueFamily::Float64,
        },
        match threshold.comparison_domain() {
            Domain::AbsoluteDifference => {
                worth_signal::facade::SignalThresholdComparisonDomain::AbsoluteDifference
            }
            Domain::RelativeRatio => {
                worth_signal::facade::SignalThresholdComparisonDomain::RelativeRatio
            }
        },
        match threshold.boundary() {
            Boundary::Inclusive => worth_signal::facade::SignalThresholdBoundary::Inclusive,
            Boundary::Exclusive => worth_signal::facade::SignalThresholdBoundary::Exclusive,
        },
    ))
}

fn condition_aspect_mask(
    declaration: &WorthQueryPortableConditionalNodeDeclaration,
    dependency_aspects: AspectMask,
    condition_aspects: AspectMask,
) -> AspectMask {
    match declaration.condition().class() {
        WorthQueryConditionalConditionClass::OnDemand
        | WorthQueryConditionalConditionClass::Temporal => AspectMask::EMPTY,
        WorthQueryConditionalConditionClass::AlwaysEligible
        | WorthQueryConditionalConditionClass::DomainSpecific => dependency_aspects,
        WorthQueryConditionalConditionClass::AspectFiltered
        | WorthQueryConditionalConditionClass::DeltaThreshold => condition_aspects,
    }
}

fn lower_artifact_reuse(
    requirement: &WorthQueryArtifactReuseEquivalence,
) -> SignalConditionalArtifactReuse {
    match requirement {
        WorthQueryArtifactReuseEquivalence::NotReusable => {
            SignalConditionalArtifactReuse::NotReusable
        }
        WorthQueryArtifactReuseEquivalence::DependencyAndOutputEquivalent => {
            SignalConditionalArtifactReuse::DependencyAndOutputEquivalent
        }
        WorthQueryArtifactReuseEquivalence::OutputEquivalent => {
            SignalConditionalArtifactReuse::OutputEquivalent
        }
        WorthQueryArtifactReuseEquivalence::Registered(_) => {
            SignalConditionalArtifactReuse::RuntimeResolved
        }
    }
}

fn lower_dependency_comparator(
    requirement: &WorthQueryComparatorRequirement,
) -> SignalConditionalVersionComparator {
    match requirement {
        WorthQueryComparatorRequirement::ExactCanonicalValue
        | WorthQueryComparatorRequirement::FoundationalContractEquivalence => {
            SignalConditionalVersionComparator::Exact
        }
        WorthQueryComparatorRequirement::Registered(_) => {
            SignalConditionalVersionComparator::RuntimeResolved
        }
    }
}

fn lower_output_comparator(
    requirement: &WorthQueryOutputEquivalenceRequirement,
) -> SignalConditionalVersionComparator {
    match requirement {
        WorthQueryOutputEquivalenceRequirement::ExactCanonicalValue
        | WorthQueryOutputEquivalenceRequirement::FoundationalContractEquivalence => {
            SignalConditionalVersionComparator::Exact
        }
        WorthQueryOutputEquivalenceRequirement::OutputIdentity => {
            SignalConditionalVersionComparator::OutputIdentity
        }
        WorthQueryOutputEquivalenceRequirement::Registered(_) => {
            SignalConditionalVersionComparator::RuntimeResolved
        }
    }
}
