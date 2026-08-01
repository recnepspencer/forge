use crate::logic::runtime::RelationalRuntime;
use crate::snapshots::data::SnapshotHandle;

use super::{
    RelationalAuthorizationObservationEvidence, RelationalAuthorizationObservationFreshness,
};

impl RelationalRuntime {
    pub fn compare_authorization_observation(
        &self,
        expected: &RelationalAuthorizationObservationEvidence,
        snapshot: SnapshotHandle,
    ) -> RelationalAuthorizationObservationFreshness {
        let Some(current) = expected
            .comparison_plan(snapshot)
            .ok()
            .and_then(|plan| self.evaluate_authorization_plan(&plan).ok())
        else {
            return RelationalAuthorizationObservationFreshness::Stale;
        };
        let semantically_equal = current.paths.len() == expected.paths().len()
            && current
                .paths
                .iter()
                .zip(expected.paths())
                .all(|(current, expected)| current.has_same_decision_and_witness(expected));
        if semantically_equal {
            RelationalAuthorizationObservationFreshness::Fresh
        } else {
            RelationalAuthorizationObservationFreshness::Stale
        }
    }
}
