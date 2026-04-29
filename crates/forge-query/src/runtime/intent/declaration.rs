use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryIntentSourceLane {
    UserAuthored,
    EffectTriggered,
    PreviewLocal,
    BranchLocal,
    DerivedRuntime,
}

impl ForgeQueryIntentSourceLane {
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

#[derive(Clone, Debug, PartialEq)]
pub struct ForgeQueryIntentDeclaration {
    name: String,
    strategy_name: String,
    strategy_version: String,
    input_contract: String,
    input: Value,
    source_lane: ForgeQueryIntentSourceLane,
    target_lane: ForgeQueryAuthorityLane,
}

impl ForgeQueryIntentDeclaration {
    pub fn strategy_commit(
        name: impl Into<String>,
        strategy_name: impl Into<String>,
        strategy_version: impl Into<String>,
        input_contract: impl Into<String>,
        input: Value,
    ) -> Self {
        Self {
            name: name.into(),
            strategy_name: strategy_name.into(),
            strategy_version: strategy_version.into(),
            input_contract: input_contract.into(),
            input,
            source_lane: ForgeQueryIntentSourceLane::UserAuthored,
            target_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
        }
    }

    pub(in crate::runtime) fn with_source_lane(
        mut self,
        source_lane: ForgeQueryIntentSourceLane,
    ) -> Self {
        self.source_lane = source_lane;
        self
    }

    pub(in crate::runtime) fn with_target_lane(
        mut self,
        target_lane: ForgeQueryAuthorityLane,
    ) -> Self {
        self.target_lane = target_lane;
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

    pub fn input(&self) -> &Value {
        &self.input
    }

    pub fn source_lane(&self) -> ForgeQueryIntentSourceLane {
        self.source_lane
    }

    pub fn target_lane(&self) -> ForgeQueryAuthorityLane {
        self.target_lane
    }

    pub fn input_digest(&self) -> String {
        let input = serde_json::to_string(&self.input)
            .unwrap_or_else(|error| format!("unserializable-intent-input:{error}"));
        hash_parts(&[
            "forge_query_intent_input_v1".to_string(),
            format!("name:{}", self.name),
            format!("strategy:{}", self.strategy_name),
            format!("version:{}", self.strategy_version),
            format!("contract:{}", self.input_contract),
            format!("input:{input}"),
        ])
    }
}
