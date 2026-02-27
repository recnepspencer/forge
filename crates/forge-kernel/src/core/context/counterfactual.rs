//! Classification overrides for counterfactual replay.
//!
//! DOMAIN: Forced face classifications that override computed results during replay.

use crate::operations::boolean::FaceClassification;
use forge_core::DecisionId;

use super::schema::ModelingContext;

impl ModelingContext {
    /// Set a forced classification override for counterfactual replay.
    ///
    /// When the classify phase encounters a decision with this ID,
    /// it uses the forced `FaceClassification` instead of computing
    /// the result from ray-casting. This enables re-executing the
    /// Boolean pipeline with different classification outcomes.
    pub fn set_classification_override(
        &mut self,
        decision_id: DecisionId,
        classification: FaceClassification,
    ) {
        self.classification_overrides
            .insert(decision_id.0, classification);
    }

    /// Check if a classification override exists for a decision ID.
    ///
    /// Returns the forced `FaceClassification` if one was set via
    /// `set_classification_override`, or `None` for normal execution.
    pub fn get_classification_override(
        &self,
        decision_id: DecisionId,
    ) -> Option<FaceClassification> {
        self.classification_overrides.get(&decision_id.0).copied()
    }

    /// Remove all classification overrides.
    pub fn clear_classification_overrides(&mut self) {
        self.classification_overrides.clear();
    }
}
