mod inspection;
mod read;
mod routing;
mod unified_inspection;

use crate::identity::hash_parts;
use crate::runtime::{
    ForgeQueryEffectDelivery, ForgeQueryIntentDeclaration, ForgeQueryIntentSourceLane,
    ForgeQueryWriteCommand,
};

use super::{
    ForgeQueryAuthoritativeIntentExecutionHandoff,
    ForgeQueryAuthoritativeMutationBatchExecutionHandoff,
    ForgeQueryAuthoritativeMutationExecutionHandoff,
    ForgeQueryEffectTriggeredIntentExecutionHandoff, ForgeQueryIntentAdmissionCoveredEntrypoint,
    ForgeQueryIntentAdmissionExecutionSeam, ForgeQueryIntentAdmissionFamily,
};
pub use inspection::{
    ForgeQueryDerivedInspectionExecutionBinding, ForgeQueryDerivedMaterializationExecutionBinding,
};
pub use read::{ForgeQueryLiveReadExecutionBinding, ForgeQueryReadExecutionBinding};
pub use routing::ForgeQueryExistingTruthProbeExecutionBinding;
pub use unified_inspection::ForgeQueryUnifiedInspectionExecutionBinding;

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryAuthoritativeIntentExecutionBinding {
    handoff: ForgeQueryAuthoritativeIntentExecutionHandoff,
    binding_digest: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryEffectTriggeredIntentExecutionBinding {
    handoff: ForgeQueryEffectTriggeredIntentExecutionHandoff,
    effect_name: String,
    trigger_commit_identity: String,
    pending_delivery_digest: String,
    binding_digest: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryAuthoritativeMutationExecutionBinding {
    handoff: ForgeQueryAuthoritativeMutationExecutionHandoff,
    binding_digest: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryAuthoritativeMutationBatchExecutionBinding {
    handoff: ForgeQueryAuthoritativeMutationBatchExecutionHandoff,
    binding_digest: String,
}

impl ForgeQueryAuthoritativeIntentExecutionBinding {
    pub(crate) fn from_handoff(handoff: ForgeQueryAuthoritativeIntentExecutionHandoff) -> Self {
        let binding_digest = hash_parts(&[
            "forge_query_authoritative_intent_execution_binding_v1".to_string(),
            format!("handoff:{}", handoff.handoff_digest()),
        ]);
        Self {
            handoff,
            binding_digest,
        }
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.handoff.family()
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.handoff.entrypoint()
    }

    pub fn execution_seam(&self) -> ForgeQueryIntentAdmissionExecutionSeam {
        self.handoff.execution_seam()
    }

    pub fn declaration(&self) -> &ForgeQueryIntentDeclaration {
        self.handoff.declaration()
    }

    pub fn handoff(&self) -> &ForgeQueryAuthoritativeIntentExecutionHandoff {
        &self.handoff
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }
}

impl ForgeQueryEffectTriggeredIntentExecutionBinding {
    pub(crate) fn from_handoff_and_delivery(
        handoff: ForgeQueryEffectTriggeredIntentExecutionHandoff,
        pending_delivery: &ForgeQueryEffectDelivery,
    ) -> Self {
        let pending_delivery_digest = hash_effect_pending_delivery(pending_delivery);
        let binding_digest = hash_parts(&[
            "forge_query_effect_intent_execution_binding_v1".to_string(),
            format!("handoff:{}", handoff.handoff_digest()),
            format!("pending_delivery:{pending_delivery_digest}"),
        ]);
        Self {
            handoff,
            effect_name: pending_delivery.effect_name().to_string(),
            trigger_commit_identity: pending_delivery.commit_identity().to_string(),
            pending_delivery_digest,
            binding_digest,
        }
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.handoff.family()
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.handoff.entrypoint()
    }

    pub fn execution_seam(&self) -> ForgeQueryIntentAdmissionExecutionSeam {
        self.handoff.execution_seam()
    }

    pub fn declaration(&self) -> &ForgeQueryIntentDeclaration {
        self.handoff.declaration()
    }

    pub fn handoff(&self) -> &ForgeQueryEffectTriggeredIntentExecutionHandoff {
        &self.handoff
    }

    pub fn effect_name(&self) -> &str {
        &self.effect_name
    }

    pub fn trigger_commit_identity(&self) -> &str {
        &self.trigger_commit_identity
    }

    pub fn pending_delivery_digest(&self) -> &str {
        &self.pending_delivery_digest
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }

    pub(crate) fn matches_pending_delivery(
        &self,
        pending_delivery: &ForgeQueryEffectDelivery,
    ) -> bool {
        self.effect_name == pending_delivery.effect_name()
            && self.trigger_commit_identity == pending_delivery.commit_identity()
            && self.pending_delivery_digest == hash_effect_pending_delivery(pending_delivery)
    }
}

impl ForgeQueryAuthoritativeMutationExecutionBinding {
    pub(crate) fn from_handoff(handoff: ForgeQueryAuthoritativeMutationExecutionHandoff) -> Self {
        let binding_digest = hash_parts(&[
            "forge_query_authoritative_mutation_execution_binding_v1".to_string(),
            format!("handoff:{}", handoff.handoff_digest()),
        ]);
        Self {
            handoff,
            binding_digest,
        }
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.handoff.family()
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.handoff.entrypoint()
    }

    pub fn execution_seam(&self) -> ForgeQueryIntentAdmissionExecutionSeam {
        self.handoff.execution_seam()
    }

    pub fn command(&self) -> &ForgeQueryWriteCommand {
        self.handoff.command()
    }

    pub fn handoff(&self) -> &ForgeQueryAuthoritativeMutationExecutionHandoff {
        &self.handoff
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }
}

impl ForgeQueryAuthoritativeMutationBatchExecutionBinding {
    pub(crate) fn from_handoff(
        handoff: ForgeQueryAuthoritativeMutationBatchExecutionHandoff,
    ) -> Self {
        let binding_digest = hash_parts(&[
            "forge_query_authoritative_mutation_batch_execution_binding_v1".to_string(),
            format!("handoff:{}", handoff.handoff_digest()),
        ]);
        Self {
            handoff,
            binding_digest,
        }
    }

    pub fn family(&self) -> ForgeQueryIntentAdmissionFamily {
        self.handoff.family()
    }

    pub fn entrypoint(&self) -> ForgeQueryIntentAdmissionCoveredEntrypoint {
        self.handoff.entrypoint()
    }

    pub fn execution_seam(&self) -> ForgeQueryIntentAdmissionExecutionSeam {
        self.handoff.execution_seam()
    }

    pub fn handoff(&self) -> &ForgeQueryAuthoritativeMutationBatchExecutionHandoff {
        &self.handoff
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }
}

fn hash_effect_pending_delivery(pending_delivery: &ForgeQueryEffectDelivery) -> String {
    let phase_digest = pending_delivery
        .phase_evidence()
        .phases()
        .iter()
        .map(|phase| phase.as_str())
        .collect::<Vec<_>>()
        .join(">");
    hash_parts(&[
        "forge_query_effect_pending_delivery_v1".to_string(),
        format!("effect:{}", pending_delivery.effect_name()),
        format!("commit:{}", pending_delivery.commit_identity()),
        format!("trigger-source:{}", pending_delivery.trigger_source()),
        format!(
            "trigger-source-kind:{}",
            pending_delivery.trigger_source_kind().as_str()
        ),
        format!("target:{}", pending_delivery.target()),
        format!("action:{}", pending_delivery.action().as_str()),
        format!("authority:{}", pending_delivery.authority_lane()),
        format!("policy:{}", pending_delivery.effect_policy().as_str()),
        format!(
            "suppression:{}",
            pending_delivery.suppression_policy().as_str()
        ),
        format!("family:{:?}", pending_delivery.family()),
        format!("phases:{phase_digest}"),
        format!(
            "loop-prevention:{}",
            pending_delivery.phase_evidence().loop_prevention().as_str()
        ),
        format!(
            "source-lane:{}",
            ForgeQueryIntentSourceLane::EffectTriggered.as_str()
        ),
        format!("aspects:{}", pending_delivery.aspect_paths().join("|")),
        format!("payload:{}", pending_delivery.payload()),
        format!("reason:{}", pending_delivery.reason().unwrap_or("none")),
    ])
}
