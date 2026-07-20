use std::collections::{BTreeMap, BTreeSet};

use crate::runtime::query_binding::{
    evidence_accumulator::WorthUiQueryBindingEvidenceAccumulator, WorthUiQueryBindingIdentity,
    WorthUiQueryBindingPosture,
};
use crate::runtime::{WorthUiCandidateDependencyMetadata, WorthUiQuerySupportReceipt};
use crate::source::{
    WorthUiArtifact, WorthUiArtifactDependencyDeriver, WorthUiArtifactDependencyGraph,
    WorthUiArtifactDigestor, WorthUiArtifactEquivalenceBasis, WorthUiArtifactNode,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiQueryBindingEvidence {
    identity: WorthUiQueryBindingIdentity,
    posture: WorthUiQueryBindingPosture,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorthUiQueryBindingEvidenceIndex {
    bindings: BTreeMap<String, WorthUiQueryBindingEvidence>,
}

impl WorthUiQueryBindingEvidence {
    pub(super) fn new(
        identity: WorthUiQueryBindingIdentity,
        posture: WorthUiQueryBindingPosture,
    ) -> Self {
        Self { identity, posture }
    }

    pub(crate) fn identity(&self) -> &WorthUiQueryBindingIdentity {
        &self.identity
    }

    pub(crate) fn posture(&self) -> &WorthUiQueryBindingPosture {
        &self.posture
    }
}

impl WorthUiQueryBindingEvidenceIndex {
    pub(crate) fn from_active_artifact(artifact: &WorthUiArtifact) -> Self {
        let report = WorthUiArtifactDependencyDeriver::derive_with_report(artifact);
        let metadata = WorthUiCandidateDependencyMetadata::from_derived_report(
            WorthUiArtifactDigestor::digest(artifact, WorthUiArtifactEquivalenceBasis::semantic()),
            report,
        );
        Self::from_artifact_graph_and_support_receipt(
            artifact,
            metadata.invalidation_basis().dependency_graph(),
            WorthUiQuerySupportReceipt::from_dependency_metadata(&metadata),
        )
    }

    pub(crate) fn from_artifact_graph_and_support_receipt(
        artifact: &WorthUiArtifact,
        graph: &WorthUiArtifactDependencyGraph,
        query_support_receipt: WorthUiQuerySupportReceipt,
    ) -> Self {
        let mut accumulators = BTreeMap::<String, WorthUiQueryBindingEvidenceAccumulator>::new();
        record_artifact_links(artifact, &mut accumulators);
        record_runtime_hooks(graph, &mut accumulators);
        record_query_support_receipt(query_support_receipt, &mut accumulators);
        Self {
            bindings: accumulators
                .into_iter()
                .filter_map(|(binding_id, accumulator)| {
                    accumulator
                        .finish(&binding_id)
                        .map(|evidence| (binding_id, evidence))
                })
                .collect(),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.bindings.len()
    }

    pub(crate) fn get(&self, view_binding_id: &str) -> Option<&WorthUiQueryBindingEvidence> {
        self.bindings.get(view_binding_id)
    }

    pub(crate) fn binding_ids(&self) -> BTreeSet<String> {
        self.bindings.keys().cloned().collect()
    }

    pub(crate) fn entries(
        &self,
    ) -> impl Iterator<Item = (&WorthUiQueryBindingIdentity, &WorthUiQueryBindingPosture)> {
        self.bindings
            .values()
            .map(|evidence| (evidence.identity(), evidence.posture()))
    }
}

fn record_query_support_receipt(
    receipt: WorthUiQuerySupportReceipt,
    accumulators: &mut BTreeMap<String, WorthUiQueryBindingEvidenceAccumulator>,
) {
    for accumulator in accumulators.values_mut() {
        accumulator.record_query_support_receipt(receipt);
    }
}

fn record_runtime_hooks(
    graph: &WorthUiArtifactDependencyGraph,
    accumulators: &mut BTreeMap<String, WorthUiQueryBindingEvidenceAccumulator>,
) {
    for hooks in graph.runtime_hooks().values() {
        for hook in hooks {
            let binding_id = hook.view_binding_id().as_str().to_owned();
            let accumulator = accumulators.entry(binding_id).or_default();
            accumulator.record_runtime_hook(hook);
        }
    }
}

fn record_artifact_links(
    artifact: &WorthUiArtifact,
    accumulators: &mut BTreeMap<String, WorthUiQueryBindingEvidenceAccumulator>,
) {
    for module_id in artifact.module_ids() {
        let Some(module) = artifact.module(module_id) else {
            continue;
        };
        for node in module.nodes() {
            match node {
                WorthUiArtifactNode::Binding(binding) => {
                    let view_binding = binding.view_binding_reference();
                    accumulators
                        .entry(view_binding.view_binding().id().as_str().to_owned())
                        .or_default()
                        .record_bound_view_binding(view_binding);
                }
                WorthUiArtifactNode::Surface(surface) => {
                    if let Some(view_binding) = surface.semantics().view_binding() {
                        accumulators
                            .entry(view_binding.view_binding().id().as_str().to_owned())
                            .or_default()
                            .record_bound_view_binding(view_binding);
                    }
                }
                _ => {}
            }
        }
    }
}
