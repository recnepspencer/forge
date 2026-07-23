use worth_proof::TransitionOutcome;

use crate::data::aspect::{Aspect, AspectMask, SignalAspectLoweringOwner};
use crate::data::graph::SignalGraph;

use super::super::super::{
    InstalledSignalConditionalContract, SignalConditionalArtifactReuse, SignalConditionalCondition,
    SignalConditionalContractDefinition, SignalConditionalVersionComparator,
    SignalDeltaThresholdContract, SignalThresholdBoundary, SignalThresholdComparisonDomain,
    SignalThresholdValueFamily,
};
use super::super::{SignalConditionalComparatorPosition, SignalConditionalSemanticMismatch};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum SemanticMismatchKind {
    ConditionClass,
    AspectFilterMask,
    ThresholdValue,
    ThresholdUnitIdentity,
    ThresholdValueFamily,
    ThresholdComparisonDomain,
    ThresholdBoundary,
    InstalledConditionMeaningUnproven,
    DependencyAspects,
    TriggerAspects,
    ComparatorClass(SignalConditionalComparatorPosition),
    ComparatorTolerance(SignalConditionalComparatorPosition),
    InstalledComparatorMeaningUnproven(SignalConditionalComparatorPosition),
    ArtifactReuseClass,
    InstalledArtifactReuseMeaningUnproven,
}

pub(super) fn install_fresh(
    definition: SignalConditionalContractDefinition,
) -> InstalledSignalConditionalContract {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let owner = claim_owner(&mut graph);
    install_at(&mut graph, &owner, node, definition)
}

pub(super) fn claim_owner(graph: &mut SignalGraph) -> SignalAspectLoweringOwner {
    let owner = SignalAspectLoweringOwner::fresh();
    graph.claim_aspect_lowering_owner(&owner).unwrap();
    owner
}

pub(super) fn install_at(
    graph: &mut SignalGraph,
    owner: &SignalAspectLoweringOwner,
    node: crate::data::handle::NodeId,
    definition: SignalConditionalContractDefinition,
) -> InstalledSignalConditionalContract {
    let TransitionOutcome::Success(capability) = graph.admit_installed_node(node) else {
        panic!("live node should admit")
    };
    graph
        .install_conditional_contract(owner, capability, definition)
        .unwrap()
}

pub(super) fn base_definition() -> SignalConditionalContractDefinition {
    SignalConditionalContractDefinition {
        condition: SignalConditionalCondition::DeltaThreshold(threshold(
            10,
            "millimeter",
            integer(),
            absolute(),
            inclusive(),
        )),
        dependency_aspects: mask(1),
        trigger_aspects: mask(2),
        dependency_comparator: SignalConditionalVersionComparator::Exact,
        output_comparator: SignalConditionalVersionComparator::Exact,
        artifact_reuse: SignalConditionalArtifactReuse::NotReusable,
    }
}

pub(super) fn portable_definition() -> SignalConditionalContractDefinition {
    with_condition(SignalConditionalCondition::Always)
}

pub(super) fn threshold(
    value: u64,
    unit: &str,
    family: SignalThresholdValueFamily,
    comparison: SignalThresholdComparisonDomain,
    boundary: SignalThresholdBoundary,
) -> SignalDeltaThresholdContract {
    SignalDeltaThresholdContract::new(
        worth_foundational::facade::AspectValue::UInt64(value),
        unit,
        family,
        comparison,
        boundary,
    )
}

pub(super) fn with_threshold(
    threshold: SignalDeltaThresholdContract,
) -> SignalConditionalContractDefinition {
    with_condition(SignalConditionalCondition::DeltaThreshold(threshold))
}

pub(super) fn with_condition(
    condition: SignalConditionalCondition,
) -> SignalConditionalContractDefinition {
    let mut definition = base_definition();
    definition.condition = condition;
    definition
}

pub(super) fn with_dependency_aspects(aspects: AspectMask) -> SignalConditionalContractDefinition {
    let mut definition = base_definition();
    definition.dependency_aspects = aspects;
    definition
}

pub(super) fn with_trigger_aspects(aspects: AspectMask) -> SignalConditionalContractDefinition {
    let mut definition = base_definition();
    definition.trigger_aspects = aspects;
    definition
}

pub(super) fn with_dependency_comparator(
    comparator: SignalConditionalVersionComparator,
) -> SignalConditionalContractDefinition {
    let mut definition = base_definition();
    definition.dependency_comparator = comparator;
    definition
}

pub(super) fn with_output_comparator(
    comparator: SignalConditionalVersionComparator,
) -> SignalConditionalContractDefinition {
    let mut definition = base_definition();
    definition.output_comparator = comparator;
    definition
}

pub(super) fn with_artifact_reuse(
    reuse: SignalConditionalArtifactReuse,
) -> SignalConditionalContractDefinition {
    let mut definition = base_definition();
    definition.artifact_reuse = reuse;
    definition
}

pub(super) fn mismatch_kind(mismatch: &SignalConditionalSemanticMismatch) -> SemanticMismatchKind {
    match mismatch {
        SignalConditionalSemanticMismatch::ConditionClass { .. } => {
            SemanticMismatchKind::ConditionClass
        }
        SignalConditionalSemanticMismatch::AspectFilterMask { .. } => {
            SemanticMismatchKind::AspectFilterMask
        }
        SignalConditionalSemanticMismatch::ThresholdValue { .. } => {
            SemanticMismatchKind::ThresholdValue
        }
        SignalConditionalSemanticMismatch::ThresholdUnitIdentity { .. } => {
            SemanticMismatchKind::ThresholdUnitIdentity
        }
        SignalConditionalSemanticMismatch::ThresholdValueFamily { .. } => {
            SemanticMismatchKind::ThresholdValueFamily
        }
        SignalConditionalSemanticMismatch::ThresholdComparisonDomain { .. } => {
            SemanticMismatchKind::ThresholdComparisonDomain
        }
        SignalConditionalSemanticMismatch::ThresholdBoundary { .. } => {
            SemanticMismatchKind::ThresholdBoundary
        }
        SignalConditionalSemanticMismatch::InstalledConditionMeaningUnproven => {
            SemanticMismatchKind::InstalledConditionMeaningUnproven
        }
        SignalConditionalSemanticMismatch::DependencyAspects { .. } => {
            SemanticMismatchKind::DependencyAspects
        }
        SignalConditionalSemanticMismatch::TriggerAspects { .. } => {
            SemanticMismatchKind::TriggerAspects
        }
        SignalConditionalSemanticMismatch::ComparatorClass { position, .. } => {
            SemanticMismatchKind::ComparatorClass(*position)
        }
        SignalConditionalSemanticMismatch::ComparatorTolerance { position, .. } => {
            SemanticMismatchKind::ComparatorTolerance(*position)
        }
        SignalConditionalSemanticMismatch::ComparatorCustomKey { position, .. } => {
            SemanticMismatchKind::ComparatorClass(*position)
        }
        SignalConditionalSemanticMismatch::InstalledComparatorMeaningUnproven { position } => {
            SemanticMismatchKind::InstalledComparatorMeaningUnproven(*position)
        }
        SignalConditionalSemanticMismatch::ArtifactReuseClass { .. } => {
            SemanticMismatchKind::ArtifactReuseClass
        }
        SignalConditionalSemanticMismatch::InstalledArtifactReuseMeaningUnproven => {
            SemanticMismatchKind::InstalledArtifactReuseMeaningUnproven
        }
    }
}

pub(super) const fn mask(aspect: u8) -> AspectMask {
    AspectMask::from_aspect(Aspect::new(aspect))
}

pub(super) const fn integer() -> SignalThresholdValueFamily {
    SignalThresholdValueFamily::Integer
}

pub(super) const fn float64() -> SignalThresholdValueFamily {
    SignalThresholdValueFamily::Float64
}

pub(super) const fn absolute() -> SignalThresholdComparisonDomain {
    SignalThresholdComparisonDomain::AbsoluteDifference
}

pub(super) const fn relative() -> SignalThresholdComparisonDomain {
    SignalThresholdComparisonDomain::RelativeRatio
}

pub(super) const fn inclusive() -> SignalThresholdBoundary {
    SignalThresholdBoundary::Inclusive
}

pub(super) const fn exclusive() -> SignalThresholdBoundary {
    SignalThresholdBoundary::Exclusive
}
