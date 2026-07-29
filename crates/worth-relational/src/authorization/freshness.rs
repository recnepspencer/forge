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
            .and_then(|plan| self.observe_authorization(plan).ok())
        else {
            return RelationalAuthorizationObservationFreshness::Stale;
        };
        if current.decision() == expected.decision() && current.paths() == expected.paths() {
            RelationalAuthorizationObservationFreshness::Fresh
        } else {
            RelationalAuthorizationObservationFreshness::Stale
        }
    }
}
