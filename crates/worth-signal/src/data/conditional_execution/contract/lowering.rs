use crate::data::comparator::{
    InstalledSignalComparatorIdentity, InstalledSignalComparatorRole, VersionComparatorPolicy,
};
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::{
    EvaluationCondition, InstalledSignalConditionIdentity, InstalledSignalConditionRole,
    NodeEvaluationConfig,
};
use crate::data::output_equivalence::OutputEquivalencePolicy;

use super::{
    SignalConditionalArtifactReuse, SignalConditionalArtifactReusePolicy,
    SignalConditionalCondition, SignalConditionalContractDenial,
    SignalConditionalVersionComparator,
};

pub(super) fn installed_comparator_identity(
    policy: &VersionComparatorPolicy,
) -> Option<&InstalledSignalComparatorIdentity> {
    match policy {
        VersionComparatorPolicy::Installed { identity } => Some(identity),
        _ => None,
    }
}

pub(super) fn installed_output_equivalence_identity(
    policy: &OutputEquivalencePolicy,
) -> Option<&InstalledSignalComparatorIdentity> {
    match policy {
        OutputEquivalencePolicy::Installed { identity } => Some(identity.comparator_identity()),
        _ => None,
    }
}

pub(super) fn lower_artifact_reuse(
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

pub(super) fn install_node_evaluation_config(
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

pub(super) fn lower_condition(
    graph: u64,
    node: NodeId,
    condition: &SignalConditionalCondition,
) -> EvaluationCondition {
    match condition {
        SignalConditionalCondition::Always => EvaluationCondition::Always,
        SignalConditionalCondition::AspectFilter(mask) => EvaluationCondition::AspectFilter(*mask),
        SignalConditionalCondition::OnDemand => EvaluationCondition::OnDemand,
        SignalConditionalCondition::DeltaThreshold(_)
        | SignalConditionalCondition::RuntimePredicate => installed_predicate(graph, node),
        SignalConditionalCondition::TemporalWake => {
            EvaluationCondition::Installed(InstalledSignalConditionIdentity::new(
                graph,
                node,
                InstalledSignalConditionRole::TemporalWake,
            ))
        }
    }
}

fn installed_predicate(graph: u64, node: NodeId) -> EvaluationCondition {
    EvaluationCondition::Installed(InstalledSignalConditionIdentity::new(
        graph,
        node,
        InstalledSignalConditionRole::Predicate,
    ))
}

pub(super) fn lower_comparator(
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
