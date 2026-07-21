use super::{
    WorthQueryCompletedWorkflowTrace, WorthQueryWorkflowRunCounters,
    WorthQueryWorkflowSemanticValue, WorthQueryWorkflowStageWarning,
};
use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::WorthQueryConditionalOutcomeClass;
use crate::identity_evolution::InstalledIdentityEvolutionOutcome;
use crate::memory_workspace::{WorthQueryEntityIdentity, WorthQueryMutationDelta};
use crate::runtime::{
    WorthQueryAspectMutationOperation, WorthQueryContinuityMutationEvidence,
    WorthQueryNamingMutationEvidence,
};
use worth_query_installation::facade::{
    WorthQueryOperationEffectFamily, WorthQueryOperationResultState,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConditionalTraceMeaning {
    location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
    declaration: worth_query_installation::facade::WorthQueryPortableConditionalNodeDeclaration,
    outcome: WorthQueryConditionalOutcomeClass,
    artifact_reuse_admitted: bool,
    signal_identity: String,
    observations: Vec<WorthQueryConditionalObservationMeaning>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConditionalObservationMeaning {
    dependency_ordinal: usize,
    previous: Option<worth_foundational::facade::ContractValidatedAspectArtifact>,
    current: worth_foundational::facade::ContractValidatedAspectArtifact,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryMutationTraceMeaning {
    target_entity: Option<WorthQueryEntityIdentity>,
    target_collection: Option<String>,
    deltas: Vec<WorthQueryMutationDelta>,
    declared_aspect_operations: Vec<WorthQueryAspectMutationOperation>,
    declared_aspect_value_digest: Option<String>,
    naming: Option<WorthQueryNamingMutationEvidence>,
    continuity: Option<WorthQueryContinuityMutationEvidence>,
}

impl WorthQueryMutationTraceMeaning {
    pub fn target_entity(&self) -> Option<&WorthQueryEntityIdentity> {
        self.target_entity.as_ref()
    }

    pub fn target_collection(&self) -> Option<&str> {
        self.target_collection.as_deref()
    }

    pub fn deltas(&self) -> &[WorthQueryMutationDelta] {
        &self.deltas
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryEffectTraceMeaning {
    family: WorthQueryOperationEffectFamily,
    mutation: Option<WorthQueryMutationTraceMeaning>,
}

impl WorthQueryEffectTraceMeaning {
    pub const fn family(&self) -> WorthQueryOperationEffectFamily {
        self.family
    }

    pub fn mutation(&self) -> Option<&WorthQueryMutationTraceMeaning> {
        self.mutation.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryLineageTraceMeaning {
    outcome: InstalledIdentityEvolutionOutcome,
    effect_indices: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryPublicationTraceMeaning {
    projection_role: String,
    stage_identity: String,
    output: WorthQueryWorkflowSemanticValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInvariantTraceMeaning {
    invariant_role: String,
    installed_invariant_identity: String,
    effect_indices: Vec<usize>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryWorkflowStageTraceSemantics {
    stage_identity: String,
    predecessor_stage_identities: Vec<String>,
    input: WorthQueryWorkflowSemanticValue,
    output: WorthQueryWorkflowSemanticValue,
    result_state: Option<WorthQueryOperationResultState>,
    warnings: Vec<WorthQueryWorkflowStageWarning>,
    effects: Vec<WorthQueryEffectTraceMeaning>,
    invariants: Vec<WorthQueryInvariantTraceMeaning>,
    conditional_path: Vec<WorthQueryConditionalTraceMeaning>,
    lineage: Vec<WorthQueryLineageTraceMeaning>,
}

impl WorthQueryWorkflowStageTraceSemantics {
    pub fn stage_identity(&self) -> &str {
        &self.stage_identity
    }
    pub fn input(&self) -> &WorthQueryWorkflowSemanticValue {
        &self.input
    }
    pub fn output(&self) -> &WorthQueryWorkflowSemanticValue {
        &self.output
    }
    pub fn result_state(&self) -> Option<WorthQueryOperationResultState> {
        self.result_state
    }
    pub fn effects(&self) -> &[WorthQueryEffectTraceMeaning] {
        &self.effects
    }
    pub fn conditional_path(&self) -> &[WorthQueryConditionalTraceMeaning] {
        &self.conditional_path
    }
    pub fn lineage(&self) -> &[WorthQueryLineageTraceMeaning] {
        &self.lineage
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthQueryWorkflowTraceSemantics {
    operation_identity: String,
    stages: Vec<WorthQueryWorkflowStageTraceSemantics>,
    publication: Option<WorthQueryPublicationTraceMeaning>,
}

impl WorthQueryWorkflowTraceSemantics {
    pub fn operation_identity(&self) -> &str {
        &self.operation_identity
    }
    pub fn stages(&self) -> &[WorthQueryWorkflowStageTraceSemantics] {
        &self.stages
    }
}

pub type WorthQueryReplayNoiseContract =
    worth_query_installation::facade::WorthQueryOperationReplayNoiseContract;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryReplayDivergence {
    Operation,
    StageSet,
    PredecessorTopology { stage: String },
    Input { stage: String },
    Output { stage: String },
    ResultState { stage: String },
    Diagnostic { stage: String },
    Effect { stage: String },
    Invariant { stage: String },
    ConditionalPath { stage: String },
    Lineage { stage: String },
    Publication,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryReplayComparison {
    Equivalent,
    Diverged(WorthQueryReplayDivergence),
}

impl<D, O, F, L: BasisOperationLane> WorthQueryCompletedWorkflowTrace<D, O, F, L> {
    pub fn semantics(&self) -> WorthQueryWorkflowTraceSemantics {
        let mut lineage_by_stage = std::collections::BTreeMap::<&str, Vec<_>>::new();
        if let Some(report) = self.lineage_report() {
            for evidence in report.evidence() {
                lineage_by_stage
                    .entry(evidence.stage_identity())
                    .or_default()
                    .push(evidence);
            }
        }
        let mut stages = self
            .stage_receipts()
            .iter()
            .map(|receipt| {
                let effect_index = receipt
                    .effect_evidence()
                    .iter()
                    .enumerate()
                    .map(|(index, effect)| (effect.receipt_identity(), index))
                    .collect::<std::collections::BTreeMap<_, _>>();
                WorthQueryWorkflowStageTraceSemantics {
                    stage_identity: receipt.stage_identity().to_owned(),
                    predecessor_stage_identities: receipt.predecessor_stage_identities().to_vec(),
                    input: receipt.input().clone(),
                    output: receipt.output().semantic_value(),
                    result_state: receipt.result_state(),
                    warnings: receipt.warnings().to_vec(),
                    effects: receipt
                        .effect_evidence()
                        .iter()
                        .map(effect_meaning)
                        .collect(),
                    invariants: receipt
                        .invariant_outcomes()
                        .iter()
                        .map(|outcome| WorthQueryInvariantTraceMeaning {
                            invariant_role: outcome.invariant_role().to_owned(),
                            installed_invariant_identity: outcome
                                .installed_invariant_identity()
                                .to_owned(),
                            effect_indices: {
                                let mut indices = outcome
                                    .effect_receipt_identities()
                                    .iter()
                                    .map(|identity| {
                                        *effect_index.get(identity.as_str()).expect(
                                            "invariant admission validated every effect receipt identity",
                                        )
                                    })
                                    .collect::<Vec<_>>();
                                indices.sort_unstable();
                                indices
                            },
                        })
                        .collect(),
                    conditional_path: receipt
                        .conditional_provenance()
                        .iter()
                        .map(|item| WorthQueryConditionalTraceMeaning {
                            location: item.location().clone(),
                            declaration: item.declaration().clone(),
                            outcome: item.class(),
                            artifact_reuse_admitted: item.artifact_reuse_admitted(),
                            signal_identity: item.signal_identity().to_owned(),
                            observations: (0..item.semantic_observation_count())
                                .filter_map(|ordinal| item.semantic_observation(ordinal))
                                .map(|observation| WorthQueryConditionalObservationMeaning {
                                    dependency_ordinal: observation.dependency_ordinal(),
                                    previous: observation.previous().cloned(),
                                    current: observation.current().clone(),
                                })
                                .collect(),
                        })
                        .collect(),
                    lineage: lineage_by_stage
                        .get(receipt.stage_identity())
                        .into_iter()
                        .flat_map(|evidence| evidence.iter().copied())
                        .map(|evidence| WorthQueryLineageTraceMeaning {
                            outcome: evidence.outcome().clone(),
                            effect_indices: evidence
                                .effect_receipt_identities()
                                .iter()
                                .map(|identity| {
                                    *effect_index.get(identity.as_str()).expect(
                                        "lineage binding validated every effect receipt identity",
                                    )
                                })
                                .collect(),
                        })
                        .collect(),
                }
            })
            .collect::<Vec<_>>();
        stages.sort_by(|left, right| left.stage_identity.cmp(&right.stage_identity));
        let publication = publication_meaning(self, &stages);
        WorthQueryWorkflowTraceSemantics {
            operation_identity: self.run.bound.definition().canonical_identity().to_owned(),
            stages,
            publication,
        }
    }

    pub(crate) fn exact_counters(&self) -> WorthQueryWorkflowRunCounters {
        self.counters()
    }
}

pub fn compare_exact_workflow_traces(
    original: &WorthQueryWorkflowTraceSemantics,
    candidate: &WorthQueryWorkflowTraceSemantics,
    noise: WorthQueryReplayNoiseContract,
) -> WorthQueryReplayComparison {
    compare_exact_workflow_traces_counted(original, candidate, noise).0
}

pub(crate) fn compare_exact_workflow_traces_counted(
    original: &WorthQueryWorkflowTraceSemantics,
    candidate: &WorthQueryWorkflowTraceSemantics,
    noise: WorthQueryReplayNoiseContract,
) -> (WorthQueryReplayComparison, usize) {
    use WorthQueryReplayDivergence as D;
    if original.operation_identity != candidate.operation_identity {
        return (WorthQueryReplayComparison::Diverged(D::Operation), 0);
    }
    if original.stages.len() != candidate.stages.len()
        || original
            .stages
            .iter()
            .zip(&candidate.stages)
            .any(|(left, right)| left.stage_identity != right.stage_identity)
    {
        return (WorthQueryReplayComparison::Diverged(D::StageSet), 0);
    }
    for (index, (left, right)) in original.stages.iter().zip(&candidate.stages).enumerate() {
        let stage = left.stage_identity.clone();
        let divergence = if left.predecessor_stage_identities != right.predecessor_stage_identities
        {
            Some(D::PredecessorTopology { stage })
        } else if left.input != right.input {
            Some(D::Input { stage })
        } else if left.output != right.output {
            Some(D::Output { stage })
        } else if left.result_state != right.result_state {
            Some(D::ResultState { stage })
        } else if !noise.diagnostic_warnings && left.warnings != right.warnings {
            Some(D::Diagnostic { stage })
        } else if left.effects != right.effects {
            Some(D::Effect { stage })
        } else if left.invariants != right.invariants {
            Some(D::Invariant { stage })
        } else if left.conditional_path != right.conditional_path {
            Some(D::ConditionalPath { stage })
        } else if left.lineage != right.lineage {
            Some(D::Lineage { stage })
        } else {
            None
        };
        if let Some(divergence) = divergence {
            return (WorthQueryReplayComparison::Diverged(divergence), index + 1);
        }
    }
    if original.publication != candidate.publication {
        return (
            WorthQueryReplayComparison::Diverged(D::Publication),
            original.stages.len(),
        );
    }
    (
        WorthQueryReplayComparison::Equivalent,
        original.stages.len(),
    )
}

fn effect_meaning(
    effect: &super::WorthQueryWorkflowEffectEvidence,
) -> WorthQueryEffectTraceMeaning {
    let mutation = effect
        .mutation_receipt()
        .map(|receipt| WorthQueryMutationTraceMeaning {
            target_entity: receipt.target_entity_identity().cloned(),
            target_collection: receipt
                .target_collection_identity()
                .map(|identity| identity.as_str().to_owned()),
            deltas: receipt.deltas().to_vec(),
            declared_aspect_operations: receipt.declared_aspect_operations().to_vec(),
            declared_aspect_value_digest: receipt.declared_aspect_value_digest().map(str::to_owned),
            naming: receipt.naming_mutation_evidence().cloned(),
            continuity: receipt.continuity_mutation_evidence().cloned(),
        });
    WorthQueryEffectTraceMeaning {
        family: effect.family(),
        mutation,
    }
}

fn publication_meaning<D, O, F, L: BasisOperationLane>(
    trace: &WorthQueryCompletedWorkflowTrace<D, O, F, L>,
    stages: &[WorthQueryWorkflowStageTraceSemantics],
) -> Option<WorthQueryPublicationTraceMeaning> {
    let worth_query_installation::facade::WorthQueryOperationPublicationContract::DerivedProjection {
        projection_role,
    } = &trace.bound().definition().semantics().publication
    else {
        return None;
    };
    trace
        .run
        .graph
        .stages()
        .iter()
        .find(|stage| stage.is_publishable())
        .and_then(|publication_stage| {
            stages
                .iter()
                .find(|stage| stage.stage_identity == publication_stage.identity())
        })
        .map(|stage| WorthQueryPublicationTraceMeaning {
            projection_role: projection_role.as_str().to_owned(),
            stage_identity: stage.stage_identity.clone(),
            output: stage.output.clone(),
        })
}

#[cfg(test)]
#[path = "semantic_trace_tests.rs"]
mod tests;
