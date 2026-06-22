mod inspection;
mod read;
mod routing;
mod unified_inspection;

use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQueryCommitIdentity;
use crate::runtime::{
    ForgeQueryAspectTouch, ForgeQueryAuthoritativeMutationObligationDispatch,
    ForgeQueryEffectDelivery, ForgeQueryEffectWriteAdjacentTrigger, ForgeQueryIntentDeclaration,
    ForgeQueryIntentSourceLane, ForgeQueryWriteCommand,
};

use super::{
    ForgeQueryAuthoritativeIntentExecutionHandoff,
    ForgeQueryAuthoritativeMutationBatchExecutionHandoff,
    ForgeQueryAuthoritativeMutationExecutionHandoff,
    ForgeQueryEffectTriggeredIntentExecutionHandoff,
};
use crate::intent_admission::{
    ForgeQueryIntentAdmissionCoveredEntrypoint, ForgeQueryIntentAdmissionExecutionSeam,
    ForgeQueryIntentAdmissionFamily,
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
    trigger_commit_identity: ForgeQueryCommitIdentity,
    write_adjacent_trigger: ForgeQueryEffectWriteAdjacentTrigger,
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
        let binding_digest = intent_execution_binding_identity(
            "authoritative-intent-execution",
            handoff.handoff_digest(),
            None,
        );
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
        let binding_digest = intent_execution_binding_identity(
            "effect-intent-execution",
            handoff.handoff_digest(),
            Some(&pending_delivery_digest),
        );
        Self {
            handoff,
            effect_name: pending_delivery.effect_name().to_string(),
            trigger_commit_identity: pending_delivery.commit_identity().clone(),
            write_adjacent_trigger: pending_delivery.write_adjacent_trigger().clone(),
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

    pub fn trigger_commit_identity(&self) -> &ForgeQueryCommitIdentity {
        &self.trigger_commit_identity
    }

    pub fn pending_delivery_digest(&self) -> &str {
        &self.pending_delivery_digest
    }

    pub fn write_adjacent_trigger(&self) -> &ForgeQueryEffectWriteAdjacentTrigger {
        &self.write_adjacent_trigger
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }

    pub(crate) fn matches_pending_delivery(
        &self,
        pending_delivery: &ForgeQueryEffectDelivery,
    ) -> bool {
        self.effect_name == pending_delivery.effect_name()
            && &self.trigger_commit_identity == pending_delivery.commit_identity()
            && self.pending_delivery_digest == hash_effect_pending_delivery(pending_delivery)
    }
}

impl ForgeQueryAuthoritativeMutationExecutionBinding {
    pub(crate) fn from_handoff(handoff: ForgeQueryAuthoritativeMutationExecutionHandoff) -> Self {
        let binding_digest = intent_execution_binding_identity(
            "authoritative-mutation-execution",
            handoff.handoff_digest(),
            None,
        );
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

    pub fn obligation_dispatch(
        &self,
    ) -> Option<&ForgeQueryAuthoritativeMutationObligationDispatch> {
        self.handoff.obligation_dispatch()
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }
}

impl ForgeQueryAuthoritativeMutationBatchExecutionBinding {
    pub(crate) fn from_handoff(
        handoff: ForgeQueryAuthoritativeMutationBatchExecutionHandoff,
    ) -> Self {
        let binding_digest = intent_execution_binding_identity(
            "authoritative-mutation-batch-execution",
            handoff.handoff_digest(),
            None,
        );
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

    pub fn obligation_dispatch(
        &self,
    ) -> Option<&ForgeQueryAuthoritativeMutationObligationDispatch> {
        self.handoff.obligation_dispatch()
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }
}

fn hash_effect_pending_delivery(pending_delivery: &ForgeQueryEffectDelivery) -> String {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::EffectIntentReceipt)
        .field_shape(ForgeQueryEvidenceTag::new("role"), "pending-delivery")
        .field_value(
            ForgeQueryEvidenceTag::new("effect"),
            pending_delivery.effect_name(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("commit"),
            &pending_delivery.commit_identity().evidence_identity(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("trigger_source"),
            pending_delivery.trigger_source(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("trigger_source_kind"),
            pending_delivery.trigger_source_kind().as_str(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("target"),
            pending_delivery.target(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("action"),
            pending_delivery.action().as_str(),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("authority"),
            pending_delivery.authority_lane().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("policy"),
            pending_delivery.effect_policy().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("suppression"),
            pending_delivery.suppression_policy().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("family"),
            effect_delivery_family_label(pending_delivery.family()),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("phase"),
            pending_delivery
                .phase_evidence()
                .phases()
                .iter()
                .map(|phase| phase.as_str()),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("loop_prevention"),
            pending_delivery.phase_evidence().loop_prevention().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("source_lane"),
            ForgeQueryIntentSourceLane::EffectTriggered.as_str(),
        )
        .field_value_sequence(
            ForgeQueryEvidenceTag::new("aspect"),
            native_aspect_digest_parts(pending_delivery.aspect_touches())
                .iter()
                .map(String::as_str),
        )
        .field_value(
            ForgeQueryEvidenceTag::new("payload"),
            pending_delivery.payload().native_digest_material(),
        )
        .optional_value(
            ForgeQueryEvidenceTag::new("reason"),
            pending_delivery.reason(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("write_adjacent_trigger"),
            pending_delivery.write_adjacent_trigger().identity(),
        )
        .seal()
        .as_str()
        .to_string()
}

fn native_aspect_digest_parts(touches: &[ForgeQueryAspectTouch]) -> Vec<String> {
    touches
        .iter()
        .map(ForgeQueryAspectTouch::admitted_touch_digest_part)
        .collect()
}

fn effect_delivery_family_label(
    family: &crate::runtime::ForgeQueryEffectDeliveryFamily,
) -> &'static str {
    match family {
        crate::runtime::ForgeQueryEffectDeliveryFamily::Delivered => "delivered",
        crate::runtime::ForgeQueryEffectDeliveryFamily::PendingWriteIntent => {
            "pending_write_intent"
        }
        crate::runtime::ForgeQueryEffectDeliveryFamily::Suppressed => "suppressed",
        crate::runtime::ForgeQueryEffectDeliveryFamily::ExpressionFailed => "expression_failed",
    }
}

fn intent_execution_binding_identity(
    role: &'static str,
    handoff_digest: &str,
    pending_delivery_digest: Option<&str>,
) -> String {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::EffectIntentReceipt)
        .field_shape(ForgeQueryEvidenceTag::new("role"), role)
        .field_value(ForgeQueryEvidenceTag::new("handoff"), handoff_digest)
        .optional_value(
            ForgeQueryEvidenceTag::new("pending_delivery"),
            pending_delivery_digest,
        )
        .seal()
        .as_str()
        .to_string()
}

pub(super) fn handoff_execution_binding_identity(
    role: &'static str,
    handoff_digest: &str,
) -> String {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::EffectIntentReceipt)
        .field_shape(ForgeQueryEvidenceTag::new("role"), role)
        .field_value(ForgeQueryEvidenceTag::new("handoff"), handoff_digest)
        .seal()
        .as_str()
        .to_string()
}
