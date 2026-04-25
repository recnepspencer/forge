use super::super::{
    ForgeQueryAuthorityLane, ForgeQueryDerivedViewHandle, ForgeQueryEffectAction,
    ForgeQueryEffectPolicy, ForgeQueryLiveView,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEffectTrigger {
    pub(in crate::runtime::effect) source: ForgeQueryEffectTriggerSource,
    aspects: Vec<String>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::runtime::effect) enum ForgeQueryEffectTriggerSource {
    LiveView { view_name: String },
    ComputedView { view_name: String },
}
impl ForgeQueryEffectTrigger {
    pub fn live_view<T>(
        view: &ForgeQueryLiveView<T>,
        aspects: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            source: ForgeQueryEffectTriggerSource::LiveView {
                view_name: view.name().to_string(),
            },
            aspects: aspects.into_iter().map(Into::into).collect(),
        }
    }
    pub fn computed_view<T>(
        view: &ForgeQueryDerivedViewHandle<T>,
        aspects: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            source: ForgeQueryEffectTriggerSource::ComputedView {
                view_name: view.name().to_string(),
            },
            aspects: aspects.into_iter().map(Into::into).collect(),
        }
    }
    #[cfg(test)]
    pub(in crate::runtime) fn live_view_name(
        view_name: impl Into<String>,
        aspects: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            source: ForgeQueryEffectTriggerSource::LiveView {
                view_name: view_name.into(),
            },
            aspects: aspects.into_iter().map(Into::into).collect(),
        }
    }
    pub fn source_name(&self) -> &str {
        match &self.source {
            ForgeQueryEffectTriggerSource::LiveView { view_name }
            | ForgeQueryEffectTriggerSource::ComputedView { view_name } => view_name,
        }
    }
    pub fn aspects(&self) -> &[String] {
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
        input_aspects: impl IntoIterator<Item = impl Into<String>>,
        output_aspects: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self::Expression(ForgeQueryEffectExpression {
            descriptor: descriptor.into(),
            input_aspects: input_aspects.into_iter().map(Into::into).collect(),
            output_aspects: output_aspects.into_iter().map(Into::into).collect(),
            failure_posture: ForgeQueryEffectExpressionFailurePosture::Admitted,
        })
    }
    pub fn failing_expression(
        descriptor: impl Into<String>,
        input_aspects: impl IntoIterator<Item = impl Into<String>>,
        output_aspects: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self::Expression(ForgeQueryEffectExpression {
            descriptor: descriptor.into(),
            input_aspects: input_aspects.into_iter().map(Into::into).collect(),
            output_aspects: output_aspects.into_iter().map(Into::into).collect(),
            failure_posture: ForgeQueryEffectExpressionFailurePosture::DeterministicFailure,
        })
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryEffectExpression {
    descriptor: String,
    input_aspects: Vec<String>,
    output_aspects: Vec<String>,
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
    pub fn input_aspects(&self) -> &[String] {
        &self.input_aspects
    }
    pub fn output_aspects(&self) -> &[String] {
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
}
impl ForgeQueryEffectDeclaration {
    pub fn deliver(
        name: impl Into<String>,
        trigger: ForgeQueryEffectTrigger,
        target: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            trigger,
            condition: ForgeQueryEffectCondition::Always,
            action: ForgeQueryEffectAction::Deliver,
            target_lane: ForgeQueryAuthorityLane::EffectDeliveryState,
            target: target.into(),
            effect_policy: ForgeQueryEffectPolicy::AuthoritativeAllowed,
            suppression_policy: ForgeQueryEffectSuppressionPolicy::None,
        }
    }
    pub fn with_condition(mut self, condition: ForgeQueryEffectCondition) -> Self {
        self.condition = condition;
        self
    }
    pub fn with_effect_policy(mut self, effect_policy: ForgeQueryEffectPolicy) -> Self {
        self.effect_policy = effect_policy;
        self
    }
    pub fn with_target_lane(mut self, target_lane: ForgeQueryAuthorityLane) -> Self {
        self.target_lane = target_lane;
        self
    }
    pub fn with_meaningful_change_suppression(mut self) -> Self {
        self.suppression_policy = ForgeQueryEffectSuppressionPolicy::MeaningfulSemanticDelta;
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
}
