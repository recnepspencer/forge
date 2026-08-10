use serde::Serialize;

use super::super::PlacementObservationScopeClass;
use super::classification::HotnessClassificationVerdict;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkingSetObservationWindow {
    scope_class: PlacementObservationScopeClass,
    scope_key: String,
    observed_artifact_keys: Vec<String>,
}

impl WorkingSetObservationWindow {
    pub(crate) fn new(
        scope_class: PlacementObservationScopeClass,
        scope_key: impl Into<String>,
        mut observed_artifact_keys: Vec<String>,
    ) -> Self {
        observed_artifact_keys.sort();
        observed_artifact_keys.dedup();
        Self {
            scope_class,
            scope_key: scope_key.into(),
            observed_artifact_keys,
        }
    }

    pub fn scope_class(&self) -> PlacementObservationScopeClass {
        self.scope_class
    }

    pub fn scope_key(&self) -> &str {
        &self.scope_key
    }

    pub fn observed_artifact_keys(&self) -> &[String] {
        &self.observed_artifact_keys
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PlacementDemandSummary {
    scope_class: PlacementObservationScopeClass,
    scope_key: String,
    observed_artifact_count: u64,
    classification_verdict: HotnessClassificationVerdict,
}

impl PlacementDemandSummary {
    pub(crate) fn new(
        scope_class: PlacementObservationScopeClass,
        scope_key: impl Into<String>,
        observed_artifact_count: u64,
        classification_verdict: HotnessClassificationVerdict,
    ) -> Self {
        Self {
            scope_class,
            scope_key: scope_key.into(),
            observed_artifact_count,
            classification_verdict,
        }
    }

    pub fn scope_class(&self) -> PlacementObservationScopeClass {
        self.scope_class
    }

    pub fn scope_key(&self) -> &str {
        &self.scope_key
    }

    pub fn observed_artifact_count(&self) -> u64 {
        self.observed_artifact_count
    }

    pub fn classification_verdict(&self) -> HotnessClassificationVerdict {
        self.classification_verdict
    }
}
