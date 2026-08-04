use super::super::telemetry::LivePolicyCounters;
use super::lanes::{LiveCertificationLane, LiveCertificationRejectionLane};
use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MilestoneFiveLiveArtifact {
    pub(in crate::live) suite_name: String,
    pub(in crate::live) certification_digest: String,
    pub(in crate::live) coverage_digest: String,
    pub(in crate::live) counter_snapshot: LivePolicyCounters,
    pub(in crate::live) canonical_lane_count: usize,
    pub(in crate::live) rejection_lane_count: usize,
}

impl MilestoneFiveLiveArtifact {
    pub fn suite_name(&self) -> &str {
        &self.suite_name
    }

    pub fn certification_digest(&self) -> &str {
        &self.certification_digest
    }

    pub fn coverage_digest(&self) -> &str {
        &self.coverage_digest
    }

    pub fn counter_snapshot(&self) -> &LivePolicyCounters {
        &self.counter_snapshot
    }

    pub fn canonical_lane_count(&self) -> usize {
        self.canonical_lane_count
    }

    pub fn rejection_lane_count(&self) -> usize {
        self.rejection_lane_count
    }
}

pub fn build_milestone_five_live_artifact(
    suite_name: impl Into<String>,
    canonical_lanes: &[LiveCertificationLane],
    rejection_lanes: &[LiveCertificationRejectionLane],
) -> MilestoneFiveLiveArtifact {
    let suite_name = suite_name.into();
    let mut certification_parts = vec![format!("suite:{suite_name}")];
    let mut coverage_parts = vec![format!("suite:{suite_name}")];
    let mut counter_snapshot = LivePolicyCounters::default();

    for lane in canonical_lanes {
        certification_parts.push(format!("canonical:{}", lane.lane_name()));
        coverage_parts.push(format!("canonical:{}", lane.lane_name()));
        certification_parts.push(format!(
            "report:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            lane.execution().report().query_digest(),
            lane.execution().report().result_digest(),
            lane.execution().report().delivery_digest(),
            lane.execution().report().replay_digest(),
            lane.execution().report().family().as_str(),
            lane.execution().report().outcome_kind(),
            lane.execution().report().outcome_digest(),
            lane.execution().report().basis_digest(),
            lane.execution().report().subscription_digest()
        ));
        certification_parts.push(format!(
            "patch_envelope:{}:{}:{}",
            lane.execution().patch_envelope().delivery_digest(),
            lane.execution().patch_envelope().replay_digest(),
            lane.execution().patch_envelope().basis_digest()
        ));
        certification_parts.extend(lane.execution().counters().digest_parts("canonical"));
        counter_snapshot.absorb(lane.execution().counters());
    }

    for lane in rejection_lanes {
        certification_parts.push(format!("rejection:{}", lane.lane_name()));
        coverage_parts.push(format!("rejection:{}", lane.lane_name()));
        certification_parts.push(format!(
            "failure:{}:{}",
            lane.failure_class(),
            lane.failure_digest()
        ));
        certification_parts.extend(lane.counters().digest_parts("rejection"));
        counter_snapshot.absorb(lane.counters());
    }

    MilestoneFiveLiveArtifact {
        suite_name,
        certification_digest: hash_parts(&certification_parts),
        coverage_digest: hash_parts(&coverage_parts),
        counter_snapshot,
        canonical_lane_count: canonical_lanes.len(),
        rejection_lane_count: rejection_lanes.len(),
    }
}
