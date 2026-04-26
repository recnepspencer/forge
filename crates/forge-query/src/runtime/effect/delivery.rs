use std::marker::PhantomData;

use serde_json::Value;

use super::super::{ForgeQueryAuthorityLane, ForgeQueryEffectAction};
use super::declaration::{
    ForgeQueryEffectDeclaration, ForgeQueryEffectSuppressionPolicy,
    ForgeQueryEffectTriggerSourceKind,
};
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
#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryEffectDelivery {
    effect_name: String,
    commit_identity: String,
    trigger_source: String,
    trigger_source_kind: ForgeQueryEffectTriggerSourceKind,
    target: String,
    action: ForgeQueryEffectAction,
    authority_lane: ForgeQueryAuthorityLane,
    aspect_paths: Vec<String>,
    family: ForgeQueryEffectDeliveryFamily,
    suppression_policy: ForgeQueryEffectSuppressionPolicy,
    phase_evidence: ForgeQueryEffectPhaseEvidence,
    payload: Value,
    reason: Option<String>,
}
impl ForgeQueryEffectDelivery {
    pub(in crate::runtime::effect) fn delivered(
        declaration: &ForgeQueryEffectDeclaration,
        commit_identity: impl Into<String>,
        trigger_source: impl Into<String>,
        trigger_source_kind: ForgeQueryEffectTriggerSourceKind,
        aspect_paths: Vec<String>,
        payload: Value,
    ) -> Self {
        Self {
            effect_name: declaration.name().to_string(),
            commit_identity: commit_identity.into(),
            trigger_source: trigger_source.into(),
            trigger_source_kind,
            target: declaration.target().to_string(),
            action: declaration.action(),
            authority_lane: declaration.target_lane(),
            aspect_paths,
            family: ForgeQueryEffectDeliveryFamily::Delivered,
            suppression_policy: declaration.suppression_policy(),
            phase_evidence: ForgeQueryEffectPhaseEvidence::delivery(),
            payload,
            reason: None,
        }
    }
    pub(in crate::runtime::effect) fn pending_write_intent(
        declaration: &ForgeQueryEffectDeclaration,
        commit_identity: impl Into<String>,
        trigger_source: impl Into<String>,
        trigger_source_kind: ForgeQueryEffectTriggerSourceKind,
        aspect_paths: Vec<String>,
        payload: Value,
    ) -> Self {
        Self {
            effect_name: declaration.name().to_string(),
            commit_identity: commit_identity.into(),
            trigger_source: trigger_source.into(),
            trigger_source_kind,
            target: declaration.target().to_string(),
            action: declaration.action(),
            authority_lane: declaration.target_lane(),
            aspect_paths,
            family: ForgeQueryEffectDeliveryFamily::PendingWriteIntent,
            suppression_policy: declaration.suppression_policy(),
            phase_evidence: ForgeQueryEffectPhaseEvidence::pending_write_intent(),
            payload,
            reason: Some(
                "effect lowered to pending write intent; commit execution awaits intent authority"
                    .to_string(),
            ),
        }
    }
    pub(in crate::runtime::effect) fn suppressed(
        declaration: &ForgeQueryEffectDeclaration,
        commit_identity: impl Into<String>,
        trigger_source: impl Into<String>,
        trigger_source_kind: ForgeQueryEffectTriggerSourceKind,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            effect_name: declaration.name().to_string(),
            commit_identity: commit_identity.into(),
            trigger_source: trigger_source.into(),
            trigger_source_kind,
            target: declaration.target().to_string(),
            action: declaration.action(),
            authority_lane: declaration.target_lane(),
            aspect_paths: Vec::new(),
            family: ForgeQueryEffectDeliveryFamily::Suppressed,
            suppression_policy: declaration.suppression_policy(),
            phase_evidence: ForgeQueryEffectPhaseEvidence::suppressed(),
            payload: Value::Null,
            reason: Some(reason.into()),
        }
    }
    pub(in crate::runtime::effect) fn expression_failed(
        declaration: &ForgeQueryEffectDeclaration,
        commit_identity: impl Into<String>,
        trigger_source: impl Into<String>,
        trigger_source_kind: ForgeQueryEffectTriggerSourceKind,
        aspect_paths: Vec<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            effect_name: declaration.name().to_string(),
            commit_identity: commit_identity.into(),
            trigger_source: trigger_source.into(),
            trigger_source_kind,
            target: declaration.target().to_string(),
            action: declaration.action(),
            authority_lane: declaration.target_lane(),
            aspect_paths,
            family: ForgeQueryEffectDeliveryFamily::ExpressionFailed,
            suppression_policy: declaration.suppression_policy(),
            phase_evidence: ForgeQueryEffectPhaseEvidence::expression_failure(),
            payload: Value::Null,
            reason: Some(reason.into()),
        }
    }
    pub fn effect_name(&self) -> &str {
        &self.effect_name
    }
    pub fn commit_identity(&self) -> &str {
        &self.commit_identity
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
    pub fn aspect_paths(&self) -> &[String] {
        &self.aspect_paths
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
    pub fn payload(&self) -> &Value {
        &self.payload
    }
    pub fn reason(&self) -> Option<&str> {
        self.reason.as_deref()
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEffectHandle<T = Value> {
    name: String,
    authority_lane: ForgeQueryAuthorityLane,
    marker: PhantomData<T>,
}
impl<T> ForgeQueryEffectHandle<T> {
    pub(in crate::runtime) fn new(
        name: impl Into<String>,
        authority_lane: ForgeQueryAuthorityLane,
    ) -> Self {
        Self {
            name: name.into(),
            authority_lane,
            marker: PhantomData,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }
}
