use super::super::{
    ForgeQueryAuthorityLane, ForgeQueryDerivedViewHandle, ForgeQueryEffectAction,
    ForgeQueryEffectPolicy, ForgeQueryLiveView,
};
use super::follow_on::{
    ForgeQueryEffectWriteAdjacentTrigger, ForgeQueryEffectWriteAdjacentTriggerClass,
};
use crate::evidence_identity::ForgeQueryEvidenceIdentity;
use crate::runtime::ForgeQueryAspectTouch;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEffectTrigger {
    pub(in crate::runtime::effect) source: ForgeQueryEffectTriggerSource,
    aspects: Vec<ForgeQueryAspectTouch>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::runtime::effect) enum ForgeQueryEffectTriggerSource {
    LiveView { view_name: String },
    ComputedView { view_name: String },
}
impl ForgeQueryEffectTrigger {
    pub fn live_view<T>(
        view: &ForgeQueryLiveView<T>,
        aspects: impl IntoIterator<Item = ForgeQueryAspectTouch>,
    ) -> Self {
        Self {
            source: ForgeQueryEffectTriggerSource::LiveView {
                view_name: view.name().to_string(),
            },
            aspects: aspects.into_iter().collect(),
        }
    }
    pub fn computed_view<T>(
        view: &ForgeQueryDerivedViewHandle<T>,
        aspects: impl IntoIterator<Item = ForgeQueryAspectTouch>,
    ) -> Self {
        Self {
            source: ForgeQueryEffectTriggerSource::ComputedView {
                view_name: view.name().to_string(),
            },
            aspects: aspects.into_iter().collect(),
        }
    }
    #[cfg(test)]
    pub(in crate::runtime) fn live_view_name(
        view_name: impl Into<String>,
        aspects: impl IntoIterator<Item = ForgeQueryAspectTouch>,
    ) -> Self {
        Self {
            source: ForgeQueryEffectTriggerSource::LiveView {
                view_name: view_name.into(),
            },
            aspects: aspects.into_iter().collect(),
        }
    }
    pub fn source_name(&self) -> &str {
        match &self.source {
            ForgeQueryEffectTriggerSource::LiveView { view_name }
            | ForgeQueryEffectTriggerSource::ComputedView { view_name } => view_name,
        }
    }
    pub fn aspect_touches(&self) -> &[ForgeQueryAspectTouch] {
        &self.aspects
    }
    pub fn source_kind(&self) -> ForgeQueryEffectTriggerSourceKind {
        match self.source {
            ForgeQueryEffectTriggerSource::LiveView { .. } => {
                ForgeQueryEffectTriggerSourceKind::LiveView
            }
            ForgeQueryEffectTriggerSource::ComputedView { .. } => {
                ForgeQueryEffectTriggerSourceKind::ComputedView
            }
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryEffectTriggerSourceKind {
    LiveView,
    ComputedView,
}
impl ForgeQueryEffectTriggerSourceKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LiveView => "live-view",
            Self::ComputedView => "computed-view",
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryEffectCondition {
    Always,
    Expression(ForgeQueryEffectExpression),
}
impl ForgeQueryEffectCondition {
    pub fn always() -> Self {
        Self::Always
    }
    pub fn expression(
        descriptor: impl Into<String>,
        input_aspects: impl IntoIterator<Item = ForgeQueryAspectTouch>,
        output_aspects: impl IntoIterator<Item = ForgeQueryAspectTouch>,
    ) -> Self {
        Self::Expression(ForgeQueryEffectExpression {
            descriptor: descriptor.into(),
            input_aspects: input_aspects.into_iter().collect(),
            output_aspects: output_aspects.into_iter().collect(),
            failure_posture: ForgeQueryEffectExpressionFailurePosture::Admitted,
        })
    }
    pub fn failing_expression(
        descriptor: impl Into<String>,
        input_aspects: impl IntoIterator<Item = ForgeQueryAspectTouch>,
        output_aspects: impl IntoIterator<Item = ForgeQueryAspectTouch>,
    ) -> Self {
        Self::Expression(ForgeQueryEffectExpression {
            descriptor: descriptor.into(),
            input_aspects: input_aspects.into_iter().collect(),
            output_aspects: output_aspects.into_iter().collect(),
            failure_posture: ForgeQueryEffectExpressionFailurePosture::DeterministicFailure,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEffectExpression {
    descriptor: String,
    input_aspects: Vec<ForgeQueryAspectTouch>,
    output_aspects: Vec<ForgeQueryAspectTouch>,
    failure_posture: ForgeQueryEffectExpressionFailurePosture,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryEffectExpressionFailurePosture {
    Admitted,
    DeterministicFailure,
}
impl ForgeQueryEffectExpression {
    pub fn descriptor(&self) -> &str {
        &self.descriptor
    }
    pub fn input_aspect_touches(&self) -> &[ForgeQueryAspectTouch] {
        &self.input_aspects
    }
    pub fn output_aspect_touches(&self) -> &[ForgeQueryAspectTouch] {
        &self.output_aspects
    }
    pub fn failure_posture(&self) -> ForgeQueryEffectExpressionFailurePosture {
        self.failure_posture
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryEffectSuppressionPolicy {
    None,
    MeaningfulSemanticDelta,
}
impl ForgeQueryEffectSuppressionPolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::MeaningfulSemanticDelta => "meaningful-semantic-delta",
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEffectDeclaration {
    name: String,
    trigger: ForgeQueryEffectTrigger,
    condition: ForgeQueryEffectCondition,
    action: ForgeQueryEffectAction,
    target_lane: ForgeQueryAuthorityLane,
    target: String,
    effect_policy: ForgeQueryEffectPolicy,
    suppression_policy: ForgeQueryEffectSuppressionPolicy,
    write_adjacent_trigger: ForgeQueryEffectWriteAdjacentTrigger,
}
impl ForgeQueryEffectDeclaration {
    pub fn deliver(
        name: impl Into<String>,
        trigger: ForgeQueryEffectTrigger,
        target: impl Into<String>,
    ) -> Self {
        let name = name.into();
        Self {
            write_adjacent_trigger: ForgeQueryEffectWriteAdjacentTrigger::ordinary(&name),
            name,
            trigger,
            condition: ForgeQueryEffectCondition::Always,
            action: ForgeQueryEffectAction::Deliver,
            target_lane: ForgeQueryAuthorityLane::EffectDeliveryState,
            target: target.into(),
            effect_policy: ForgeQueryEffectPolicy::AuthoritativeAllowed,
            suppression_policy: ForgeQueryEffectSuppressionPolicy::None,
        }
    }
    pub fn write_intent(
        name: impl Into<String>,
        trigger: ForgeQueryEffectTrigger,
        intent: impl Into<String>,
    ) -> Self {
        let name = name.into();
        Self {
            write_adjacent_trigger: ForgeQueryEffectWriteAdjacentTrigger::ordinary(&name),
            name,
            trigger,
            condition: ForgeQueryEffectCondition::Always,
            action: ForgeQueryEffectAction::WriteIntent,
            target_lane: ForgeQueryAuthorityLane::PendingWriteIntent,
            target: intent.into(),
            effect_policy: ForgeQueryEffectPolicy::SandboxedWriteIntent,
            suppression_policy: ForgeQueryEffectSuppressionPolicy::None,
        }
    }
    pub fn with_condition(mut self, condition: ForgeQueryEffectCondition) -> Self {
        self.condition = condition;
        self
    }
    #[cfg(test)]
    pub(in crate::runtime) fn with_effect_policy(
        mut self,
        effect_policy: ForgeQueryEffectPolicy,
    ) -> Self {
        self.effect_policy = effect_policy;
        self
    }
    #[cfg(test)]
    pub(in crate::runtime) fn with_target_lane(
        mut self,
        target_lane: ForgeQueryAuthorityLane,
    ) -> Self {
        self.target_lane = target_lane;
        self
    }
    pub fn with_meaningful_change_suppression(mut self) -> Self {
        self.suppression_policy = ForgeQueryEffectSuppressionPolicy::MeaningfulSemanticDelta;
        self
    }
    pub fn with_write_adjacent_trigger(
        mut self,
        class: ForgeQueryEffectWriteAdjacentTriggerClass,
        origin_identity: ForgeQueryEvidenceIdentity,
    ) -> Self {
        self.write_adjacent_trigger =
            ForgeQueryEffectWriteAdjacentTrigger::new(class, origin_identity);
        self
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn trigger(&self) -> &ForgeQueryEffectTrigger {
        &self.trigger
    }
    pub fn condition(&self) -> &ForgeQueryEffectCondition {
        &self.condition
    }
    pub fn action(&self) -> ForgeQueryEffectAction {
        self.action
    }
    pub fn target_lane(&self) -> ForgeQueryAuthorityLane {
        self.target_lane
    }
    pub fn target(&self) -> &str {
        &self.target
    }
    pub fn effect_policy(&self) -> ForgeQueryEffectPolicy {
        self.effect_policy
    }
    pub fn suppression_policy(&self) -> ForgeQueryEffectSuppressionPolicy {
        self.suppression_policy
    }
    pub fn write_adjacent_trigger(&self) -> &ForgeQueryEffectWriteAdjacentTrigger {
        &self.write_adjacent_trigger
    }
}
