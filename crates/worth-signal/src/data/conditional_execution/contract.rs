use crate::data::aspect::{AspectMask, InstalledSignalNodeCapability, SignalAspectLoweringOwner};
use crate::data::comparator::{
    InstalledSignalComparatorIdentity, InstalledSignalComparatorRole, VersionComparatorPolicy,
};
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::{
    EvaluationCondition, InstalledSignalConditionIdentity, InstalledSignalConditionRole,
    NodeEvaluationConfig,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignalConditionalCondition {
    Always,
    AspectFilter(AspectMask),
    DeltaThreshold(SignalDeltaThresholdContract),
    OnDemand,
    RuntimePredicate,
    TemporalWake,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalThresholdValueFamily {
    Integer,
    Float32,
    Float64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalThresholdComparisonDomain {
    AbsoluteDifference,
    RelativeRatio,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalThresholdBoundary {
    Inclusive,
    Exclusive,
}

/// Signal-owned, lossless threshold meaning installed by the lowering owner.
/// The runtime predicate identity remains opaque, while this contract retains
/// the typed semantic parameters that identity is authorized to evaluate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignalDeltaThresholdContract {
    threshold: worth_foundational::facade::AspectValue,
    unit_identity: String,
    value_family: SignalThresholdValueFamily,
    comparison_domain: SignalThresholdComparisonDomain,
    boundary: SignalThresholdBoundary,
}

impl SignalDeltaThresholdContract {
    pub fn new(
        threshold: worth_foundational::facade::AspectValue,
        unit_identity: impl Into<String>,
        value_family: SignalThresholdValueFamily,
        comparison_domain: SignalThresholdComparisonDomain,
        boundary: SignalThresholdBoundary,
    ) -> Self {
        Self {
            threshold,
            unit_identity: unit_identity.into(),
            value_family,
            comparison_domain,
            boundary,
        }
    }

    pub fn threshold(&self) -> &worth_foundational::facade::AspectValue {
        &self.threshold
    }

    pub fn unit_identity(&self) -> &str {
        &self.unit_identity
    }

    pub const fn value_family(&self) -> SignalThresholdValueFamily {
        self.value_family
    }

    pub const fn comparison_domain(&self) -> SignalThresholdComparisonDomain {
        self.comparison_domain
    }

    pub const fn boundary(&self) -> SignalThresholdBoundary {
        self.boundary
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalConditionalVersionComparator {
    Exact,
    Tolerance(u64),
    OutputIdentity,
    RuntimeResolved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalConditionalArtifactReuse {
    NotReusable,
    DependencyAndOutputEquivalent,
    OutputEquivalent,
    RuntimeResolved,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignalConditionalContractDefinition {
    pub condition: SignalConditionalCondition,
    pub dependency_aspects: AspectMask,
    pub trigger_aspects: AspectMask,
    pub dependency_comparator: SignalConditionalVersionComparator,
    pub output_comparator: SignalConditionalVersionComparator,
    pub artifact_reuse: SignalConditionalArtifactReuse,
}

#[derive(Debug, Clone)]
pub enum SignalConditionalArtifactReusePolicy {
    NotReusable,
    DependencyAndOutputEquivalent,
    OutputEquivalent,
    Installed(InstalledSignalComparatorIdentity),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignalConditionalContractDenial {
    ForeignGraph,
    ForeignLoweringOwner,
    StaleNode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstalledSignalComparatorUse {
    DependencyVersion,
    OutputEquivalence,
    ArtifactReuse,
}

pub(super) struct InstalledSignalConditionalAuthority {
    _owner_seal: (),
}

/// Opaque installed contract. Construction requires both the exact graph-local
/// node capability and the graph's admitted lowering owner.
pub struct InstalledSignalConditionalContract {
    pub(super) authority: std::sync::Arc<InstalledSignalConditionalAuthority>,
    graph_instance_id: u64,
    node: NodeId,
    condition: EvaluationCondition,
    semantic_condition: SignalConditionalCondition,
    dependency_aspects: AspectMask,
    trigger_aspects: AspectMask,
    dependency_comparator: VersionComparatorPolicy,
    output_comparator: VersionComparatorPolicy,
    artifact_reuse: SignalConditionalArtifactReusePolicy,
}

impl InstalledSignalConditionalContract {
    pub const fn graph_instance_id(&self) -> u64 {
        self.graph_instance_id
    }

    pub const fn node(&self) -> NodeId {
        self.node
    }

    pub fn condition(&self) -> &EvaluationCondition {
        &self.condition
    }

    pub fn semantic_condition(&self) -> &SignalConditionalCondition {
        &self.semantic_condition
    }

    pub fn dependency_comparator(&self) -> &VersionComparatorPolicy {
        &self.dependency_comparator
    }

    pub const fn dependency_aspects(&self) -> AspectMask {
        self.dependency_aspects
    }

    pub const fn trigger_aspects(&self) -> AspectMask {
        self.trigger_aspects
    }

    pub fn output_comparator(&self) -> &VersionComparatorPolicy {
        &self.output_comparator
    }

    pub fn artifact_reuse(&self) -> &SignalConditionalArtifactReusePolicy {
        &self.artifact_reuse
    }

    pub fn accepts_condition_identity(&self, candidate: &InstalledSignalConditionIdentity) -> bool {
        matches!(
            &self.condition,
            EvaluationCondition::Installed(installed)
                if installed.is_same_installed_identity(candidate)
        )
    }

    pub fn classify_comparator_identity(
        &self,
        candidate: &InstalledSignalComparatorIdentity,
    ) -> Option<InstalledSignalComparatorUse> {
        if installed_comparator_identity(&self.dependency_comparator)
            .is_some_and(|installed| installed.is_same_installed_identity(candidate))
        {
            return Some(InstalledSignalComparatorUse::DependencyVersion);
        }
        if installed_comparator_identity(&self.output_comparator)
            .is_some_and(|installed| installed.is_same_installed_identity(candidate))
        {
            return Some(InstalledSignalComparatorUse::OutputEquivalence);
        }
        if matches!(
            &self.artifact_reuse,
            SignalConditionalArtifactReusePolicy::Installed(installed)
                if installed.is_same_installed_identity(candidate)
        ) {
            return Some(InstalledSignalComparatorUse::ArtifactReuse);
        }
        None
    }

    pub fn retains_decision(&self, evidence: &super::SignalConditionalDecisionEvidence) -> bool {
        std::sync::Arc::ptr_eq(&self.authority, &evidence.contract_authority)
    }
}

impl SignalGraph {
    pub fn install_conditional_contract(
        &mut self,
        owner: &SignalAspectLoweringOwner,
        node: InstalledSignalNodeCapability,
        definition: SignalConditionalContractDefinition,
    ) -> Result<InstalledSignalConditionalContract, SignalConditionalContractDenial> {
        if node.graph_instance_id() != self.runtime_instance_id() {
            return Err(SignalConditionalContractDenial::ForeignGraph);
        }
        if !self
            .aspect_lowering_owner
            .as_ref()
            .is_some_and(|installed| installed.is_same_owner(owner))
        {
            return Err(SignalConditionalContractDenial::ForeignLoweringOwner);
        }
        self.get_contract(node.node())
            .map_err(|_| SignalConditionalContractDenial::StaleNode)?;
        let semantic_condition = definition.condition;
        let condition =
            lower_condition(self.runtime_instance_id(), node.node(), &semantic_condition);
        let dependency_comparator = lower_comparator(
            self.runtime_instance_id(),
            node.node(),
            InstalledSignalComparatorRole::DependencyVersion,
            definition.dependency_comparator,
        );
        let output_comparator = lower_comparator(
            self.runtime_instance_id(),
            node.node(),
            InstalledSignalComparatorRole::OutputEquivalence,
            definition.output_comparator,
        );
        let artifact_reuse = lower_artifact_reuse(
            self.runtime_instance_id(),
            node.node(),
            definition.artifact_reuse,
        );
        let mut config = self
            .node_eval_config(node.node())
            .map_err(|_| SignalConditionalContractDenial::StaleNode)?
            .clone();
        config.condition = condition.clone();
        config.comparator = Some(output_comparator.clone());
        install_node_evaluation_config(self, node.node(), config)?;
        Ok(InstalledSignalConditionalContract {
            authority: std::sync::Arc::new(InstalledSignalConditionalAuthority { _owner_seal: () }),
            graph_instance_id: self.runtime_instance_id(),
            node: node.node(),
            condition,
            semantic_condition,
            dependency_aspects: definition.dependency_aspects,
            trigger_aspects: definition.trigger_aspects,
            dependency_comparator,
            output_comparator,
            artifact_reuse,
        })
    }
}

fn installed_comparator_identity(
    policy: &VersionComparatorPolicy,
) -> Option<&InstalledSignalComparatorIdentity> {
    match policy {
        VersionComparatorPolicy::Installed { identity } => Some(identity),
        _ => None,
    }
}

fn lower_artifact_reuse(
    graph: u64,
    node: NodeId,
    reuse: SignalConditionalArtifactReuse,
) -> SignalConditionalArtifactReusePolicy {
    match reuse {
        SignalConditionalArtifactReuse::NotReusable => {
            SignalConditionalArtifactReusePolicy::NotReusable
        }
        SignalConditionalArtifactReuse::DependencyAndOutputEquivalent => {
            SignalConditionalArtifactReusePolicy::DependencyAndOutputEquivalent
        }
        SignalConditionalArtifactReuse::OutputEquivalent => {
            SignalConditionalArtifactReusePolicy::OutputEquivalent
        }
        SignalConditionalArtifactReuse::RuntimeResolved => {
            SignalConditionalArtifactReusePolicy::Installed(InstalledSignalComparatorIdentity::new(
                graph,
                node,
                InstalledSignalComparatorRole::ArtifactReuse,
            ))
        }
    }
}

fn install_node_evaluation_config(
    graph: &mut SignalGraph,
    node: NodeId,
    config: NodeEvaluationConfig,
) -> Result<(), SignalConditionalContractDenial> {
    graph
        .get_entry_mut(node)
        .map_err(|_| SignalConditionalContractDenial::StaleNode)?
        .set_eval_config(config);
    Ok(())
}

fn lower_condition(
    graph: u64,
    node: NodeId,
    condition: &SignalConditionalCondition,
) -> EvaluationCondition {
    match condition {
        SignalConditionalCondition::Always => EvaluationCondition::Always,
        SignalConditionalCondition::AspectFilter(mask) => EvaluationCondition::AspectFilter(*mask),
        SignalConditionalCondition::OnDemand => EvaluationCondition::OnDemand,
        SignalConditionalCondition::DeltaThreshold(_)
        | SignalConditionalCondition::RuntimePredicate => {
            EvaluationCondition::Installed(InstalledSignalConditionIdentity::new(
                graph,
                node,
                InstalledSignalConditionRole::Predicate,
            ))
        }
        SignalConditionalCondition::TemporalWake => {
            EvaluationCondition::Installed(InstalledSignalConditionIdentity::new(
                graph,
                node,
                InstalledSignalConditionRole::TemporalWake,
            ))
        }
    }
}

fn lower_comparator(
    graph: u64,
    node: NodeId,
    role: InstalledSignalComparatorRole,
    comparator: SignalConditionalVersionComparator,
) -> VersionComparatorPolicy {
    match comparator {
        SignalConditionalVersionComparator::Exact => VersionComparatorPolicy::Exact,
        SignalConditionalVersionComparator::Tolerance(epsilon) => {
            VersionComparatorPolicy::Tolerance { epsilon }
        }
        SignalConditionalVersionComparator::OutputIdentity => {
            VersionComparatorPolicy::OutputIdentity
        }
        SignalConditionalVersionComparator::RuntimeResolved => VersionComparatorPolicy::Installed {
            identity: InstalledSignalComparatorIdentity::new(graph, node, role),
        },
    }
}
