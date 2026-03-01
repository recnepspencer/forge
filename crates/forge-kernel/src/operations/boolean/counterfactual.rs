//! Counterfactual overrides for Boolean face classification.
//!
//! DOMAIN: Stores forced `FaceClassification` values keyed by decision ID.
//! Used to replay classification outcomes without mutating `ModelingContext`.

use std::collections::BTreeMap;

use forge_core::DecisionId;

use super::classify_schema::FaceClassification;

/// Forced classification overrides used during counterfactual replay.
#[derive(Debug, Clone, Default)]
pub struct CounterfactualOverrides {
    overrides: BTreeMap<u64, FaceClassification>,
}

impl CounterfactualOverrides {
    /// Create an empty override set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert or replace a forced classification for a decision.
    pub fn set(&mut self, decision_id: DecisionId, classification: FaceClassification) {
        self.overrides.insert(decision_id.0, classification);
    }

    /// Get a forced classification, if present.
    pub fn get(&self, decision_id: DecisionId) -> Option<FaceClassification> {
        self.overrides.get(&decision_id.0).copied()
    }

    /// Clear all overrides.
    pub fn clear(&mut self) {
        self.overrides.clear();
    }
}
