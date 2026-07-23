use std::collections::BTreeSet;

use super::WorthQueryWorkflowValue;

/// Replayable input vocabulary. Projection capabilities are intentionally not
/// representable: rebuilding one would be Store/re-admission work, not Query
/// intent normalization.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryWorkflowIntentValue {
    NotRequired,
    Bool(bool),
    I64(i64),
    U64(u64),
    Text(String),
    EntityIdentity(String),
    CurrentEntityIdentity(crate::memory_workspace::WorthQueryEntityIdentity),
}

impl WorthQueryWorkflowIntentValue {
    pub(crate) fn runtime_value(&self) -> WorthQueryWorkflowValue {
        match self {
            Self::NotRequired => WorthQueryWorkflowValue::NotRequired,
            Self::Bool(value) => WorthQueryWorkflowValue::Bool(*value),
            Self::I64(value) => WorthQueryWorkflowValue::I64(*value),
            Self::U64(value) => WorthQueryWorkflowValue::U64(*value),
            Self::Text(value) => WorthQueryWorkflowValue::Text(value.clone()),
            Self::EntityIdentity(value) => WorthQueryWorkflowValue::EntityIdentity(value.clone()),
            Self::CurrentEntityIdentity(value) => {
                WorthQueryWorkflowValue::CurrentEntityIdentity(value.clone())
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryWorkflowIntentStage {
    stage_identity: String,
    input: WorthQueryWorkflowIntentValue,
}

impl WorthQueryWorkflowIntentStage {
    pub fn new(stage_identity: impl Into<String>, input: WorthQueryWorkflowIntentValue) -> Self {
        Self {
            stage_identity: stage_identity.into(),
            input,
        }
    }
    pub fn stage_identity(&self) -> &str {
        &self.stage_identity
    }
    pub fn input(&self) -> &WorthQueryWorkflowIntentValue {
        &self.input
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryNormalizedWorkflowIntent {
    stages: Vec<WorthQueryWorkflowIntentStage>,
}

impl WorthQueryNormalizedWorkflowIntent {
    pub fn new(stages: Vec<WorthQueryWorkflowIntentStage>) -> Result<Self, &'static str> {
        if stages.is_empty() {
            return Err("empty-workflow-intent");
        }
        let mut identities = BTreeSet::new();
        if stages.iter().any(|stage| {
            stage.stage_identity.trim().is_empty()
                || !identities.insert(stage.stage_identity.as_str())
        }) {
            return Err("invalid-or-duplicate-workflow-intent-stage");
        }
        Ok(Self { stages })
    }
    pub fn stages(&self) -> &[WorthQueryWorkflowIntentStage] {
        &self.stages
    }
}
