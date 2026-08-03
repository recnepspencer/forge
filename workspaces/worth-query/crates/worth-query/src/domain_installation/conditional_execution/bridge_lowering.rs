use std::sync::Arc;

use worth_query_installation::facade::{
    WorthQueryArtifactPosture, WorthQueryArtifactReuseEquivalence, WorthQueryComparatorRequirement,
    WorthQueryConditionalConditionClass, WorthQueryConditionalNodeLocation,
    WorthQueryDeltaComparisonDomain, WorthQueryMaintenancePosture,
    WorthQueryOutputEquivalenceRequirement, WorthQueryPortableConditionalNodeDeclaration,
    WorthQueryQuantityValueFamily, WorthQueryThresholdBoundary,
};
use worth_runtime_bridge::facade::{
    BridgeConditionalCondition, BridgeConditionalContract, BridgeConditionalContractParts,
    BridgeConditionalLocation,
};
use worth_signal::facade::{
    SignalConditionalArtifactReuse, SignalConditionalVersionComparator,
    SignalDeltaThresholdContract, SignalThresholdBoundary, SignalThresholdComparisonDomain,
    SignalThresholdValueFamily,
};

use super::WorthQueryConditionalNodeInstallationDenial;

pub(super) fn lower_bridge_contract(
    declaration: &WorthQueryPortableConditionalNodeDeclaration,
) -> Result<BridgeConditionalContract, WorthQueryConditionalNodeInstallationDenial> {
    validate_supported_postures(declaration)?;
    let condition_dependency_ordinals = declaration
        .condition()
        .dependencies()
        .iter()
        .map(|condition_dependency| {
            declaration
                .dependencies()
                .iter()
                .position(|dependency| dependency == condition_dependency)
                .ok_or(WorthQueryConditionalNodeInstallationDenial::DependencyShape)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(BridgeConditionalContract::new(
        BridgeConditionalContractParts {
            identity: Arc::from(declaration.identity()),
            dependency_count: declaration.dependencies().len(),
            condition_dependency_ordinals,
            condition: lower_condition(declaration)?,
            dependency_comparator: lower_dependency_comparator(declaration.dependency_comparator()),
            output_comparator: lower_output_comparator(declaration.output_equivalence()),
            artifact_reuse: lower_artifact_reuse(declaration.artifact_reuse_equivalence()),
        },
    ))
}

pub(super) fn lower_bridge_location(
    location: &WorthQueryConditionalNodeLocation,
) -> BridgeConditionalLocation {
    match location {
        WorthQueryConditionalNodeLocation::Operation { node_identity } => {
            BridgeConditionalLocation::operation(Arc::from(node_identity.as_str()))
        }
        WorthQueryConditionalNodeLocation::WorkflowStage {
            stage_identity,
            node_identity,
        } => BridgeConditionalLocation::workflow_stage(
            Arc::from(stage_identity.as_str()),
            Arc::from(node_identity.as_str()),
        ),
    }
}

pub(crate) fn query_location_from_bridge_candidate(
    candidate: &worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate,
) -> WorthQueryConditionalNodeLocation {
    match candidate.source_stage_identity() {
        Some(stage_identity) => WorthQueryConditionalNodeLocation::workflow_stage(
            stage_identity,
            candidate.source_node_identity(),
        ),
        None => WorthQueryConditionalNodeLocation::operation(candidate.source_node_identity()),
    }
    .expect("Bridge candidates admitted by Query retain valid Query node identities")
}

fn validate_supported_postures(
    declaration: &WorthQueryPortableConditionalNodeDeclaration,
) -> Result<(), WorthQueryConditionalNodeInstallationDenial> {
    if declaration.maintenance() == WorthQueryMaintenancePosture::EagerOnEligibleInvalidation {
        return Err(WorthQueryConditionalNodeInstallationDenial::UnsupportedMaintenancePosture);
    }
    if declaration.artifact() == WorthQueryArtifactPosture::Durable {
        return Err(WorthQueryConditionalNodeInstallationDenial::UnsupportedArtifactPosture);
    }
    Ok(())
}

fn lower_condition(
    declaration: &WorthQueryPortableConditionalNodeDeclaration,
) -> Result<BridgeConditionalCondition, WorthQueryConditionalNodeInstallationDenial> {
    Ok(match declaration.condition().class() {
        WorthQueryConditionalConditionClass::AlwaysEligible => BridgeConditionalCondition::Always,
        WorthQueryConditionalConditionClass::AspectFiltered => {
            BridgeConditionalCondition::AspectFiltered
        }
        WorthQueryConditionalConditionClass::DeltaThreshold => {
            BridgeConditionalCondition::DeltaThreshold(lower_delta_threshold(declaration)?)
        }
        WorthQueryConditionalConditionClass::OnDemand => BridgeConditionalCondition::OnDemand,
        WorthQueryConditionalConditionClass::DomainSpecific => {
            BridgeConditionalCondition::RuntimePredicate
        }
        WorthQueryConditionalConditionClass::Temporal => BridgeConditionalCondition::TemporalWake,
    })
}

fn lower_delta_threshold(
    declaration: &WorthQueryPortableConditionalNodeDeclaration,
) -> Result<SignalDeltaThresholdContract, WorthQueryConditionalNodeInstallationDenial> {
    let (_, threshold) = declaration
        .condition()
        .delta_threshold_contract()
        .ok_or(WorthQueryConditionalNodeInstallationDenial::InvalidConditionalContract)?;
    Ok(SignalDeltaThresholdContract::new(
        threshold.value().clone(),
        threshold.unit().as_str(),
        match threshold.value_family() {
            WorthQueryQuantityValueFamily::Integer => SignalThresholdValueFamily::Integer,
            WorthQueryQuantityValueFamily::Float32 => SignalThresholdValueFamily::Float32,
            WorthQueryQuantityValueFamily::Float64 => SignalThresholdValueFamily::Float64,
        },
        match threshold.comparison_domain() {
            WorthQueryDeltaComparisonDomain::AbsoluteDifference => {
                SignalThresholdComparisonDomain::AbsoluteDifference
            }
            WorthQueryDeltaComparisonDomain::RelativeRatio => {
                SignalThresholdComparisonDomain::RelativeRatio
            }
        },
        match threshold.boundary() {
            WorthQueryThresholdBoundary::Inclusive => SignalThresholdBoundary::Inclusive,
            WorthQueryThresholdBoundary::Exclusive => SignalThresholdBoundary::Exclusive,
        },
    ))
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
