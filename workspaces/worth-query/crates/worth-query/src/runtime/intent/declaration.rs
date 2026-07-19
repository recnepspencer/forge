use super::*;
use crate::identity::hash_parts;
use crate::runtime::{WorthQueryEffectWriteAdjacentTrigger, WorthQueryGraphTouchDescriptor};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryIntentSourceLane {
    UserAuthored,
    EffectTriggered,
    PreviewLocal,
    BranchLocal,
    DerivedRuntime,
}

impl WorthQueryIntentSourceLane {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UserAuthored => "user-authored",
            Self::EffectTriggered => "effect-triggered",
            Self::PreviewLocal => "preview-local",
            Self::BranchLocal => "branch-local",
            Self::DerivedRuntime => "derived-runtime",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentDeclaration {
    name: String,
    strategy_name: String,
    strategy_version: String,
    input_contract: String,
    input: WorthQueryIntentInput,
    source_lane: WorthQueryIntentSourceLane,
    target_lane: WorthQueryAuthorityLane,
    effect_trigger: Option<WorthQueryEffectWriteAdjacentTrigger>,
    graph_touch_descriptor: Option<WorthQueryGraphTouchDescriptor>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryTouchBearingIntentDeclaration {
    declaration: WorthQueryIntentDeclaration,
}

impl WorthQueryIntentDeclaration {
    pub fn strategy_commit(
        name: impl Into<String>,
        strategy_name: impl Into<String>,
        strategy_version: impl Into<String>,
        input_contract: impl Into<String>,
        input: WorthQueryIntentInput,
    ) -> Self {
        Self::strategy_commit_with_input(
            name,
            strategy_name,
            strategy_version,
            input_contract,
            input,
        )
    }

    pub fn strategy_commit_with_input(
        name: impl Into<String>,
        strategy_name: impl Into<String>,
        strategy_version: impl Into<String>,
        input_contract: impl Into<String>,
        input: WorthQueryIntentInput,
    ) -> Self {
        Self {
            name: name.into(),
            strategy_name: strategy_name.into(),
            strategy_version: strategy_version.into(),
            input_contract: input_contract.into(),
            input,
            source_lane: WorthQueryIntentSourceLane::UserAuthored,
            target_lane: WorthQueryAuthorityLane::AuthoritativeTruth,
            effect_trigger: None,
            graph_touch_descriptor: None,
        }
    }

    pub fn with_graph_touch_descriptor(
        mut self,
        graph_touch_descriptor: WorthQueryGraphTouchDescriptor,
    ) -> Self {
        self.graph_touch_descriptor = Some(graph_touch_descriptor);
        self
    }

    pub(in crate::runtime) fn with_source_lane(
        mut self,
        source_lane: WorthQueryIntentSourceLane,
    ) -> Self {
        self.source_lane = source_lane;
        self
    }

    pub(in crate::runtime) fn with_target_lane(
        mut self,
        target_lane: WorthQueryAuthorityLane,
    ) -> Self {
        self.target_lane = target_lane;
        self
    }

    pub(in crate::runtime) fn with_effect_trigger(
        mut self,
        effect_trigger: WorthQueryEffectWriteAdjacentTrigger,
    ) -> Self {
        self.effect_trigger = Some(effect_trigger);
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn strategy_name(&self) -> &str {
        &self.strategy_name
    }

    pub fn strategy_version(&self) -> &str {
        &self.strategy_version
    }

    pub fn input_contract(&self) -> &str {
        &self.input_contract
    }

    pub fn input(&self) -> &WorthQueryIntentInput {
        &self.input
    }

    pub fn input_string_field(&self, field: &str) -> Option<&str> {
        self.input.string_field(field)
    }

    pub fn source_lane(&self) -> WorthQueryIntentSourceLane {
        self.source_lane
    }

    pub fn target_lane(&self) -> WorthQueryAuthorityLane {
        self.target_lane
    }

    pub fn effect_trigger(&self) -> Option<&WorthQueryEffectWriteAdjacentTrigger> {
        self.effect_trigger.as_ref()
    }

    pub fn graph_touch_descriptor(&self) -> Option<&WorthQueryGraphTouchDescriptor> {
        self.graph_touch_descriptor.as_ref()
    }

    pub fn input_digest(&self) -> String {
        let input = self.input.digest_material();
        hash_parts(&[
            "worth_query_intent_input_v1".to_string(),
            format!("name:{}", self.name),
            format!("strategy:{}", self.strategy_name),
            format!("version:{}", self.strategy_version),
            format!("contract:{}", self.input_contract),
            format!("input:{input}"),
            format!(
                "effect-trigger:{}",
                self.effect_trigger
                    .as_ref()
                    .map(WorthQueryEffectWriteAdjacentTrigger::digest)
                    .unwrap_or("none")
            ),
            format!(
                "graph-touch-descriptor:{}",
                self.graph_touch_descriptor
                    .as_ref()
                    .map(WorthQueryGraphTouchDescriptor::descriptor_digest)
                    .unwrap_or("none")
            ),
        ])
    }
}

impl WorthQueryTouchBearingIntentDeclaration {
    pub fn new(
        declaration: WorthQueryIntentDeclaration,
        graph_touch_descriptor: WorthQueryGraphTouchDescriptor,
    ) -> Self {
        Self {
            declaration: declaration.with_graph_touch_descriptor(graph_touch_descriptor),
        }
    }

    pub fn declaration(&self) -> &WorthQueryIntentDeclaration {
        &self.declaration
    }

    pub fn graph_touch_descriptor(&self) -> &WorthQueryGraphTouchDescriptor {
        self.declaration
            .graph_touch_descriptor()
            .expect("touch-bearing intent declaration always carries graph touch descriptor")
    }

    pub fn input_digest(&self) -> String {
        self.declaration.input_digest()
    }

    pub fn into_declaration(self) -> WorthQueryIntentDeclaration {
        self.declaration
    }
}
