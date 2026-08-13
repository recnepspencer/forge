mod admission;
mod application;
mod batching;
mod evidence;
mod output_commit;
mod vocabulary;

#[cfg(test)]
mod tests;

use crate::data::core_profile::StableHashValue;
use crate::data::error::SignalError;
use crate::data::node::NodeState;
use crate::data::reuse::ReuseBasis;
use crate::data::trace::{
    ArtifactWriteDelta, ColdArtifactRecord, HotArtifactWrite, RuntimeArtifactState,
};
use crate::diagnostics::lineage::LineageArtifactId;
use crate::logic::evaluation::{DeferralReason, EvaluationEffect, EvaluationVerdict};

use super::graph::{RuntimeArtifactStructuralDelta, SignalGraph};

#[cfg_attr(not(feature = "parallel"), allow(dead_code))]
pub(crate) use batching::{
    ApplyCommitPacket, OutputCommitPacket, PreparedParallelApplyCommitPacket,
};

impl SignalGraph {
    fn transition_effect_state(
        &mut self,
        effect: &mut EvaluationEffect,
        artifact_write: Option<HotArtifactWrite>,
    ) -> Result<(), SignalError> {
        let prepared_write = self.prepare_effect_artifact_write(artifact_write);
        let state_mutation = self.apply_effect_node_state(effect, prepared_write)?;
        self.record_effect_state_mutation(effect.operational.node, state_mutation);
        Ok(())
    }

    fn prepare_effect_artifact_write(
        &mut self,
        artifact_write: Option<HotArtifactWrite>,
    ) -> PreparedEffectArtifactWrite {
        let Some(write) = artifact_write else {
            return PreparedEffectArtifactWrite::default();
        };
        self.telemetry_mut()
            .storage
            .hot_write_runtime_artifact_count += u64::from(write.runtime.is_some());
        let retained = if write.cold_intent.is_none()
            && vocabulary::runtime_policy_omits_cold_artifacts(self)
        {
            self.telemetry_mut().storage.hot_write_cold_bypass_count += 1;
            self.telemetry_mut()
                .storage
                .deferred_cold_artifact_bypass_count += 1;
            None
        } else {
            self.materialize_retained_artifact(write.cold_intent)
        };
        PreparedEffectArtifactWrite {
            runtime: write.runtime,
            retained,
        }
    }

    fn apply_effect_node_state(
        &mut self,
        effect: &mut EvaluationEffect,
        prepared_write: PreparedEffectArtifactWrite,
    ) -> Result<EffectStateMutation, SignalError> {
        let node = effect.operational.node;
        let previous_artifact =
            PreviousRuntimeArtifactState::from(self.node_runtime_artifact_structural_state(node)?);
        let previous_state = self.node_state(node)?;
        if let Some(causality) = effect.take_causality() {
            self.set_causality(node, Some(causality))?;
        }
        if matches!(effect.operational.verdict, EvaluationVerdict::Recomputed) {
            self.apply_node_aspect_version(
                node,
                effect.operational.aspect_version,
                effect.changed_regions(),
            )?;
        }
        let PreparedEffectArtifactWrite { runtime, retained } = prepared_write;
        let (runtime_artifact_delta, retained_artifact_changed) =
            self.apply_effect_runtime_artifact(node, effect, previous_artifact, runtime, retained)?;
        let state_changed = self.apply_effect_node_lifecycle(node, effect, previous_state)?;
        Ok(EffectStateMutation {
            runtime_artifact_delta,
            retained_artifact_changed,
            state_changed,
        })
    }

    fn apply_effect_runtime_artifact(
        &mut self,
        node: crate::data::handle::NodeId,
        effect: &EvaluationEffect,
        previous: PreviousRuntimeArtifactState,
        runtime: Option<RuntimeArtifactState>,
        retained: Option<ColdArtifactRecord>,
    ) -> Result<(Option<RuntimeArtifactStructuralDelta>, bool), SignalError> {
        if !vocabulary::verdict_retains_runtime_artifact(&effect.operational.verdict) {
            return Ok((None, false));
        }
        let Some(runtime) = runtime else {
            return Ok((None, false));
        };
        let runtime_artifact_delta = Some(RuntimeArtifactStructuralDelta {
            previous_artifact_id: previous.artifact_id,
            next_artifact_id: runtime.lineage_artifact_id().get(),
            previous_output_hash: previous.output_hash,
            next_output_hash: Some(runtime.output_hash()),
            previous_reuse_basis: if matches!(
                effect.operational.verdict,
                EvaluationVerdict::Recomputed
            ) {
                previous.reuse_basis.clone()
            } else {
                previous.reuse_basis
            },
            next_reuse_basis: Some(runtime.reuse_basis().clone_inner()),
        });
        let retained_artifact_changed = self.apply_node_artifact_write_delta(
            node,
            ArtifactWriteDelta {
                runtime: Some(runtime),
                retained,
            },
        )?;
        Ok((runtime_artifact_delta, retained_artifact_changed))
    }

    fn apply_effect_node_lifecycle(
        &mut self,
        node: crate::data::handle::NodeId,
        effect: &EvaluationEffect,
        previous_state: NodeState,
    ) -> Result<bool, SignalError> {
        if vocabulary::verdict_transitions_clean(&effect.operational.verdict) {
            self.transition_node_clean(node)?;
            return Ok(!matches!(previous_state, NodeState::Clean));
        }
        if let EvaluationVerdict::Deferred {
            reason:
                DeferralReason::ConditionNotMet
                | DeferralReason::DependencyPending
                | DeferralReason::OnDemandNotRequested
                | DeferralReason::DebounceWindow
                | DeferralReason::TemporalConditionNotMet,
        } = effect.operational.verdict
        {
            self.set_node_state(node, NodeState::MaybeStale)?;
            return Ok(!matches!(previous_state, NodeState::MaybeStale));
        }
        Ok(false)
    }

    fn record_effect_state_mutation(
        &mut self,
        node: crate::data::handle::NodeId,
        mutation: EffectStateMutation,
    ) {
        if let Some(delta) = mutation.runtime_artifact_delta {
            self.record_branch_mutation_runtime_artifact(node, delta);
        }
        if mutation.retained_artifact_changed {
            self.record_branch_mutation_retained_artifact(node);
        }
        if mutation.state_changed {
            self.record_branch_mutation_state(node);
        }
    }
}

#[derive(Default)]
struct PreparedEffectArtifactWrite {
    runtime: Option<RuntimeArtifactState>,
    retained: Option<ColdArtifactRecord>,
}

#[derive(Default)]
struct EffectStateMutation {
    runtime_artifact_delta: Option<RuntimeArtifactStructuralDelta>,
    retained_artifact_changed: bool,
    state_changed: bool,
}

struct PreviousRuntimeArtifactState {
    artifact_id: Option<LineageArtifactId>,
    output_hash: Option<StableHashValue>,
    reuse_basis: Option<ReuseBasis>,
}

impl PreviousRuntimeArtifactState {
    fn from(
        state: (
            Option<LineageArtifactId>,
            Option<StableHashValue>,
            Option<ReuseBasis>,
        ),
    ) -> Self {
        let (artifact_id, output_hash, reuse_basis) = state;
        Self {
            artifact_id,
            output_hash,
            reuse_basis,
        }
    }
}
