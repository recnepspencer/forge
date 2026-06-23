use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::memory_workspace::ForgeQueryCommitIdentity;
use crate::runtime::ForgeQueryAspectTouch;

use super::super::ForgeQueryEffectPolicy;
use super::super::{ForgeQueryAuthorityLane, ForgeQueryEffectAction};
use super::declaration::{
    ForgeQueryEffectDeclaration, ForgeQueryEffectSuppressionPolicy,
    ForgeQueryEffectTriggerSourceKind,
};
use super::delivery_helpers::{
    effect_trigger_commit_evidence_identity, terminal_touch_digest_projection_sequence,
};
use super::follow_on::ForgeQueryEffectWriteAdjacentTrigger;
use super::phase::ForgeQueryEffectPhaseEvidence;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ForgeQueryEffectCounters {
    pub(in crate::runtime::effect) considered: usize,
    pub(in crate::runtime::effect) delivered: usize,
    pub(in crate::runtime::effect) pending_write_intents: usize,
    pub(in crate::runtime::effect) suppressed: usize,
    pub(in crate::runtime::effect) meaningful_suppressions: usize,
    pub(in crate::runtime::effect) expression_failures: usize,
}
impl ForgeQueryEffectCounters {
    pub fn considered(&self) -> usize {
        self.considered
    }
    pub fn delivered(&self) -> usize {
        self.delivered
    }
    pub fn pending_write_intents(&self) -> usize {
        self.pending_write_intents
    }
    pub fn suppressed(&self) -> usize {
        self.suppressed
    }
    pub fn meaningful_suppressions(&self) -> usize {
        self.meaningful_suppressions
    }
    pub fn expression_failures(&self) -> usize {
        self.expression_failures
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryEffectDeliveryFamily {
    Delivered,
    PendingWriteIntent,
    Suppressed,
    ExpressionFailed,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEffectPayload {
    condition: Option<String>,
    input_aspects: Vec<ForgeQueryAspectTouch>,
    output_aspects: Vec<ForgeQueryAspectTouch>,
    changed_aspects: Vec<ForgeQueryAspectTouch>,
}

impl ForgeQueryEffectPayload {
    pub(in crate::runtime::effect) fn always(changed_aspects: &[ForgeQueryAspectTouch]) -> Self {
        Self {
            condition: Some("always".to_string()),
            input_aspects: Vec::new(),
            output_aspects: Vec::new(),
            changed_aspects: changed_aspects.to_vec(),
        }
    }

    pub(in crate::runtime::effect) fn expression(
        condition: impl Into<String>,
        input_aspects: &[ForgeQueryAspectTouch],
        output_aspects: &[ForgeQueryAspectTouch],
        changed_aspects: &[ForgeQueryAspectTouch],
    ) -> Self {
        Self {
            condition: Some(condition.into()),
            input_aspects: input_aspects.to_vec(),
            output_aspects: output_aspects.to_vec(),
            changed_aspects: changed_aspects.to_vec(),
        }
    }

    pub(in crate::runtime::effect) fn empty() -> Self {
        Self {
            condition: None,
            input_aspects: Vec::new(),
            output_aspects: Vec::new(),
            changed_aspects: Vec::new(),
        }
    }

    pub fn condition(&self) -> Option<&str> {
        self.condition.as_deref()
    }

    pub fn input_aspect_touches(&self) -> &[ForgeQueryAspectTouch] {
        &self.input_aspects
    }

    pub fn output_aspect_touches(&self) -> &[ForgeQueryAspectTouch] {
        &self.output_aspects
    }

    pub fn changed_aspect_touches(&self) -> &[ForgeQueryAspectTouch] {
        &self.changed_aspects
    }

    pub(crate) fn terminal_digest_material(&self) -> String {
        let Some(condition) = self.condition.as_deref() else {
            return "condition:<none>".to_string();
        };
        [
            format!("condition:{condition}"),
            format!(
                "input:{}",
                terminal_touch_digest_projection_sequence(&self.input_aspects)
            ),
            format!(
                "output:{}",
                terminal_touch_digest_projection_sequence(&self.output_aspects)
            ),
            format!(
                "changed:{}",
                terminal_touch_digest_projection_sequence(&self.changed_aspects)
            ),
        ]
        .join("|")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryEffectDelivery {
    effect_name: String,
    commit_identity: ForgeQueryCommitIdentity,
    trigger_commit_evidence_identity: ForgeQueryEvidenceIdentity,
    trigger_source: String,
    trigger_source_kind: ForgeQueryEffectTriggerSourceKind,
    target: String,
    action: ForgeQueryEffectAction,
    authority_lane: ForgeQueryAuthorityLane,
    effect_policy: ForgeQueryEffectPolicy,
    aspect_touches: Vec<ForgeQueryAspectTouch>,
    family: ForgeQueryEffectDeliveryFamily,
    suppression_policy: ForgeQueryEffectSuppressionPolicy,
    phase_evidence: ForgeQueryEffectPhaseEvidence,
    write_adjacent_trigger: ForgeQueryEffectWriteAdjacentTrigger,
    payload: ForgeQueryEffectPayload,
    reason: Option<String>,
}
impl ForgeQueryEffectDelivery {
    pub(in crate::runtime::effect) fn delivered(
        declaration: &ForgeQueryEffectDeclaration,
        commit_identity: ForgeQueryCommitIdentity,
        trigger_source: impl Into<String>,
        trigger_source_kind: ForgeQueryEffectTriggerSourceKind,
        aspect_touches: Vec<ForgeQueryAspectTouch>,
        payload: ForgeQueryEffectPayload,
    ) -> Self {
        let trigger_commit_evidence_identity =
            effect_trigger_commit_evidence_identity(&commit_identity);
        Self {
            effect_name: declaration.name().to_string(),
            commit_identity,
            trigger_commit_evidence_identity,
            trigger_source: trigger_source.into(),
            trigger_source_kind,
            target: declaration.target().to_string(),
            action: declaration.action(),
            authority_lane: declaration.target_lane(),
            effect_policy: declaration.effect_policy(),
            aspect_touches,
            family: ForgeQueryEffectDeliveryFamily::Delivered,
            suppression_policy: declaration.suppression_policy(),
            phase_evidence: ForgeQueryEffectPhaseEvidence::delivery(),
            write_adjacent_trigger: declaration.write_adjacent_trigger().clone(),
            payload,
            reason: None,
        }
    }
    pub(in crate::runtime::effect) fn pending_write_intent(
        declaration: &ForgeQueryEffectDeclaration,
        commit_identity: ForgeQueryCommitIdentity,
        trigger_source: impl Into<String>,
        trigger_source_kind: ForgeQueryEffectTriggerSourceKind,
        aspect_touches: Vec<ForgeQueryAspectTouch>,
        payload: ForgeQueryEffectPayload,
    ) -> Self {
        let trigger_commit_evidence_identity =
            effect_trigger_commit_evidence_identity(&commit_identity);
        Self {
            effect_name: declaration.name().to_string(),
            commit_identity,
            trigger_commit_evidence_identity,
            trigger_source: trigger_source.into(),
            trigger_source_kind,
            target: declaration.target().to_string(),
            action: declaration.action(),
            authority_lane: declaration.target_lane(),
            effect_policy: declaration.effect_policy(),
            aspect_touches,
            family: ForgeQueryEffectDeliveryFamily::PendingWriteIntent,
            suppression_policy: declaration.suppression_policy(),
            phase_evidence: ForgeQueryEffectPhaseEvidence::pending_write_intent(),
            write_adjacent_trigger: declaration.write_adjacent_trigger().clone(),
            payload,
            reason: Some(
                "effect lowered to pending write intent; commit execution awaits intent authority"
                    .to_string(),
            ),
        }
    }
    pub(in crate::runtime::effect) fn suppressed(
        declaration: &ForgeQueryEffectDeclaration,
        commit_identity: ForgeQueryCommitIdentity,
        trigger_source: impl Into<String>,
        trigger_source_kind: ForgeQueryEffectTriggerSourceKind,
        reason: impl Into<String>,
    ) -> Self {
        let trigger_commit_evidence_identity =
            effect_trigger_commit_evidence_identity(&commit_identity);
        Self {
            effect_name: declaration.name().to_string(),
            commit_identity,
            trigger_commit_evidence_identity,
            trigger_source: trigger_source.into(),
            trigger_source_kind,
            target: declaration.target().to_string(),
            action: declaration.action(),
            authority_lane: declaration.target_lane(),
            effect_policy: declaration.effect_policy(),
            aspect_touches: Vec::new(),
            family: ForgeQueryEffectDeliveryFamily::Suppressed,
            suppression_policy: declaration.suppression_policy(),
            phase_evidence: ForgeQueryEffectPhaseEvidence::suppressed(),
            write_adjacent_trigger: declaration.write_adjacent_trigger().clone(),
            payload: ForgeQueryEffectPayload::empty(),
            reason: Some(reason.into()),
        }
    }
    pub(in crate::runtime::effect) fn expression_failed(
        declaration: &ForgeQueryEffectDeclaration,
        commit_identity: ForgeQueryCommitIdentity,
        trigger_source: impl Into<String>,
        trigger_source_kind: ForgeQueryEffectTriggerSourceKind,
        aspect_touches: Vec<ForgeQueryAspectTouch>,
        reason: impl Into<String>,
    ) -> Self {
        let trigger_commit_evidence_identity =
            effect_trigger_commit_evidence_identity(&commit_identity);
        Self {
            effect_name: declaration.name().to_string(),
            commit_identity,
            trigger_commit_evidence_identity,
            trigger_source: trigger_source.into(),
            trigger_source_kind,
            target: declaration.target().to_string(),
            action: declaration.action(),
            authority_lane: declaration.target_lane(),
            effect_policy: declaration.effect_policy(),
            aspect_touches,
            family: ForgeQueryEffectDeliveryFamily::ExpressionFailed,
            suppression_policy: declaration.suppression_policy(),
            phase_evidence: ForgeQueryEffectPhaseEvidence::expression_failure(),
            write_adjacent_trigger: declaration.write_adjacent_trigger().clone(),
            payload: ForgeQueryEffectPayload::empty(),
            reason: Some(reason.into()),
        }
    }
    pub fn effect_name(&self) -> &str {
        &self.effect_name
    }
    pub fn commit_identity(&self) -> &ForgeQueryCommitIdentity {
        &self.commit_identity
    }
    pub fn trigger_commit_evidence_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.trigger_commit_evidence_identity
    }
    pub fn trigger_source(&self) -> &str {
        &self.trigger_source
    }
    pub fn trigger_source_kind(&self) -> ForgeQueryEffectTriggerSourceKind {
        self.trigger_source_kind
    }
    pub fn target(&self) -> &str {
        &self.target
    }
    pub fn action(&self) -> ForgeQueryEffectAction {
        self.action
    }
    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }
    pub fn effect_policy(&self) -> ForgeQueryEffectPolicy {
        self.effect_policy
    }
    pub fn aspect_touches(&self) -> &[ForgeQueryAspectTouch] {
        &self.aspect_touches
    }

    pub fn family(&self) -> &ForgeQueryEffectDeliveryFamily {
        &self.family
    }
    pub fn suppression_policy(&self) -> ForgeQueryEffectSuppressionPolicy {
        self.suppression_policy
    }
    pub fn phase_evidence(&self) -> &ForgeQueryEffectPhaseEvidence {
        &self.phase_evidence
    }
    pub fn write_adjacent_trigger(&self) -> &ForgeQueryEffectWriteAdjacentTrigger {
        &self.write_adjacent_trigger
    }
    pub fn payload(&self) -> &ForgeQueryEffectPayload {
        &self.payload
    }
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }
}
