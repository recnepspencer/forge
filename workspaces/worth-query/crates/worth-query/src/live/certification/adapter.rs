use super::super::identity::{LiveChangeOrdinal, LiveProgressBasis, LiveProgressError};
use super::super::patches::{LivePatchEnvelope, LivePatchPayload};
use super::super::promotion::LiveQueryPlan;
use super::super::refresh::{LiveCoalescingError, LiveRefreshError, RefreshAdmissionClass};
use super::super::relevance::BridgeChangeSummary;
use super::super::{
    execute_live_change, live_execution_report, patch_envelope_from_payload,
    replay_bundle_from_patch_envelope, LiveExecutionEnvelope, LiveExecutionError,
    LivePatchConstructionBasis, LivePolicyCounters,
};
use super::artifact::{build_milestone_five_live_artifact, MilestoneFiveLiveArtifact};
use super::lanes::{
    LiveCertificationLane, LiveCertificationRejectionLane, LiveExpectedRejectionError,
};
use crate::basis::ResolvedSnapshotBasis;

pub struct MilestoneFiveLiveAdapter;

impl MilestoneFiveLiveAdapter {
    pub fn detail_patch_lane(
        live: &LiveQueryPlan,
        change: &BridgeChangeSummary,
    ) -> Result<LiveCertificationLane, LiveExecutionError> {
        Self::canonical_lane("detail-live-patch-parity", live, change)
    }

    pub fn suppression_lane(
        live: &LiveQueryPlan,
        change: &BridgeChangeSummary,
    ) -> Result<LiveCertificationLane, LiveExecutionError> {
        Self::canonical_lane("irrelevant-update-suppression", live, change)
    }

    pub fn ordered_collection_patch_lane(
        live: &LiveQueryPlan,
        change: &BridgeChangeSummary,
    ) -> Result<LiveCertificationLane, LiveExecutionError> {
        Self::canonical_lane("ordered-collection-live-patch-parity", live, change)
    }

    pub fn bounded_materialization_patch_lane(
        live: &LiveQueryPlan,
        change: &BridgeChangeSummary,
    ) -> Result<LiveCertificationLane, LiveExecutionError> {
        Self::canonical_lane("bounded-materialization-live-patch-parity", live, change)
    }

    pub fn progress_advance_lane(
        live: &LiveQueryPlan,
        next_ordinal: LiveChangeOrdinal,
        next_basis: ResolvedSnapshotBasis,
    ) -> Result<LiveCertificationLane, LiveProgressError> {
        let progress = Self::advance_progress(live, next_ordinal, next_basis)?;
        let execution = Self::progress_execution(live, &progress);
        Ok(LiveCertificationLane::new(
            "live-progress-basis-parity",
            execution,
        ))
    }

    fn advance_progress(
        live: &LiveQueryPlan,
        next_ordinal: LiveChangeOrdinal,
        next_basis: ResolvedSnapshotBasis,
    ) -> Result<LiveProgressBasis, LiveProgressError> {
        live.progress_basis().advance(
            live.progress_basis().change_sequence_id(),
            next_ordinal,
            next_basis,
        )
    }

    fn progress_execution(
        live: &LiveQueryPlan,
        progress: &LiveProgressBasis,
    ) -> LiveExecutionEnvelope {
        let outcome_kind = "progress_advance".to_string();
        let outcome_digest = Self::progress_outcome_digest(progress);
        let patch_envelope =
            Self::progress_patch_envelope(live, progress, &outcome_kind, &outcome_digest);
        let counters = LivePolicyCounters::from_progress_advance();
        LiveExecutionEnvelope {
            report: live_execution_report(live, outcome_kind, outcome_digest),
            replay_bundle: replay_bundle_from_patch_envelope(
                patch_envelope.clone(),
                counters.clone(),
            ),
            patch_envelope,
            counters,
        }
    }

    fn progress_outcome_digest(progress: &LiveProgressBasis) -> String {
        format!(
            "ordinal:{}:replay:{}",
            progress.last_ordinal().value(),
            progress.replay_digest().as_str()
        )
    }

    fn progress_patch_envelope(
        live: &LiveQueryPlan,
        progress: &LiveProgressBasis,
        outcome_kind: &str,
        outcome_digest: &str,
    ) -> LivePatchEnvelope {
        patch_envelope_from_payload(
            live,
            LivePatchPayload::ProgressAdvance {
                ordinal: progress.last_ordinal().value(),
            },
            LivePatchConstructionBasis {
                outcome_kind: outcome_kind.to_string(),
                outcome_digest: outcome_digest.to_string(),
                basis_digest: progress
                    .current_basis()
                    .proof()
                    .digest()
                    .as_str()
                    .to_string(),
                replay_digest: progress.replay_digest().as_str().to_string(),
            },
        )
    }

    pub fn refresh_fallback_lane(
        live: &LiveQueryPlan,
        admission_class: RefreshAdmissionClass,
    ) -> Result<LiveCertificationLane, LiveRefreshError> {
        let fallback = live.request_refresh_fallback(admission_class)?;
        Ok(LiveCertificationLane::new(
            "refresh-fallback-equivalence",
            Self::refresh_execution(live, &fallback),
        ))
    }

    fn refresh_execution(
        live: &LiveQueryPlan,
        fallback: &super::super::refresh::RefreshFallback,
    ) -> LiveExecutionEnvelope {
        let outcome_kind = "refresh".to_string();
        let outcome_digest = Self::refresh_outcome_digest(fallback);
        let patch_envelope =
            Self::refresh_patch_envelope(live, fallback, &outcome_kind, &outcome_digest);
        let counters = LivePolicyCounters::from_refresh_fallback(fallback);
        LiveExecutionEnvelope {
            report: live_execution_report(live, outcome_kind, outcome_digest),
            replay_bundle: replay_bundle_from_patch_envelope(
                patch_envelope.clone(),
                counters.clone(),
            ),
            patch_envelope,
            counters,
        }
    }

    fn refresh_outcome_digest(fallback: &super::super::refresh::RefreshFallback) -> String {
        format!(
            "refresh:{}:{}",
            fallback.admission_class().as_str(),
            fallback.admission_status().as_str()
        )
    }

    fn refresh_patch_envelope(
        live: &LiveQueryPlan,
        fallback: &super::super::refresh::RefreshFallback,
        outcome_kind: &str,
        outcome_digest: &str,
    ) -> LivePatchEnvelope {
        patch_envelope_from_payload(
            live,
            LivePatchPayload::Refresh(fallback.clone()),
            LivePatchConstructionBasis {
                outcome_kind: outcome_kind.to_string(),
                outcome_digest: outcome_digest.to_string(),
                basis_digest: live
                    .progress_basis()
                    .current_basis()
                    .proof()
                    .digest()
                    .as_str()
                    .to_string(),
                replay_digest: live.progress_basis().replay_digest().as_str().to_string(),
            },
        )
    }

    pub fn coalesced_delivery_lane(
        live: &LiveQueryPlan,
        bundle_count: usize,
    ) -> Result<LiveCertificationLane, LiveCoalescingError> {
        let decision = live.request_coalesced_delivery(bundle_count)?;
        Ok(LiveCertificationLane::new(
            "coalesced-sequence-replay-parity",
            LiveExecutionEnvelope {
                report: live_execution_report(
                    live,
                    "coalesced_delivery".to_string(),
                    format!("{decision:?}"),
                ),
                patch_envelope: patch_envelope_from_payload(
                    live,
                    LivePatchPayload::Coalesced(decision.clone()),
                    LivePatchConstructionBasis {
                        outcome_kind: "coalesced_delivery".to_string(),
                        outcome_digest: format!("{decision:?}"),
                        basis_digest: live
                            .progress_basis()
                            .current_basis()
                            .proof()
                            .digest()
                            .as_str()
                            .to_string(),
                        replay_digest: live.progress_basis().replay_digest().as_str().to_string(),
                    },
                ),
                replay_bundle: replay_bundle_from_patch_envelope(
                    patch_envelope_from_payload(
                        live,
                        LivePatchPayload::Coalesced(decision.clone()),
                        LivePatchConstructionBasis {
                            outcome_kind: "coalesced_delivery".to_string(),
                            outcome_digest: format!("{decision:?}"),
                            basis_digest: live
                                .progress_basis()
                                .current_basis()
                                .proof()
                                .digest()
                                .as_str()
                                .to_string(),
                            replay_digest: live
                                .progress_basis()
                                .replay_digest()
                                .as_str()
                                .to_string(),
                        },
                    ),
                    LivePolicyCounters::from_coalescing_decision(&decision),
                ),
                counters: LivePolicyCounters::from_coalescing_decision(&decision),
            },
        ))
    }

    pub fn canonical_lane(
        lane_name: impl Into<String>,
        live: &LiveQueryPlan,
        change: &BridgeChangeSummary,
    ) -> Result<LiveCertificationLane, LiveExecutionError> {
        let execution = execute_live_change(live, change)?;
        Ok(LiveCertificationLane::new(lane_name, execution))
    }

    pub fn refresh_rejection_lane(
        lane_name: impl Into<String>,
        live: &LiveQueryPlan,
        admission_class: RefreshAdmissionClass,
    ) -> Result<LiveCertificationRejectionLane, LiveExpectedRejectionError> {
        match live.request_refresh_fallback(admission_class.clone()) {
            Ok(fallback) => Err(LiveExpectedRejectionError::UnexpectedRefreshAdmission {
                admission_class: fallback.admission_class().clone(),
                admission_status: fallback.admission_status().clone(),
            }),
            Err(error) => Ok(LiveCertificationRejectionLane::new(
                lane_name,
                "forbidden-refresh-escape-hatch",
                format!("{error:?}"),
                LivePolicyCounters::from_refresh_error(&error),
            )),
        }
    }

    pub fn coalescing_rejection_lane(
        lane_name: impl Into<String>,
        live: &LiveQueryPlan,
        bundle_count: usize,
    ) -> Result<LiveCertificationRejectionLane, LiveExpectedRejectionError> {
        match live.request_coalesced_delivery(bundle_count) {
            Ok(decision) => {
                Err(LiveExpectedRejectionError::UnexpectedCoalescingAdmission { decision })
            }
            Err(error) => Ok(LiveCertificationRejectionLane::new(
                lane_name,
                "forbidden-coalescing-class",
                format!("{error:?}"),
                LivePolicyCounters::from_coalescing_error(&error),
            )),
        }
    }

    pub fn progress_rejection_lane(
        lane_name: impl Into<String>,
        live: &LiveQueryPlan,
        next_ordinal: LiveChangeOrdinal,
        next_basis: ResolvedSnapshotBasis,
    ) -> Result<LiveCertificationRejectionLane, LiveExpectedRejectionError> {
        match live.progress_basis().advance(
            live.progress_basis().change_sequence_id(),
            next_ordinal,
            next_basis,
        ) {
            Ok(progress) => Err(LiveExpectedRejectionError::UnexpectedProgressAdvance {
                ordinal: progress.last_ordinal().value(),
                replay_digest: progress.replay_digest().as_str().to_string(),
            }),
            Err(error) => Ok(LiveCertificationRejectionLane::new(
                lane_name,
                "non-monotonic-change-sequence",
                format!("{error:?}"),
                LivePolicyCounters::from_progress_error(&error),
            )),
        }
    }

    pub fn artifact(
        suite_name: impl Into<String>,
        canonical_lanes: &[LiveCertificationLane],
        rejection_lanes: &[LiveCertificationRejectionLane],
    ) -> MilestoneFiveLiveArtifact {
        build_milestone_five_live_artifact(suite_name, canonical_lanes, rejection_lanes)
    }

    pub fn forbidden_refresh_rejection_lane(
        live: &LiveQueryPlan,
        admission_class: RefreshAdmissionClass,
    ) -> Result<LiveCertificationRejectionLane, LiveExpectedRejectionError> {
        Self::refresh_rejection_lane("forbidden-refresh-escape-hatch", live, admission_class)
    }

    pub fn forbidden_coalescing_rejection_lane(
        live: &LiveQueryPlan,
        bundle_count: usize,
    ) -> Result<LiveCertificationRejectionLane, LiveExpectedRejectionError> {
        Self::coalescing_rejection_lane("forbidden-coalescing-class", live, bundle_count)
    }

    pub fn non_monotonic_progress_rejection_lane(
        live: &LiveQueryPlan,
        next_ordinal: LiveChangeOrdinal,
        next_basis: ResolvedSnapshotBasis,
    ) -> Result<LiveCertificationRejectionLane, LiveExpectedRejectionError> {
        Self::progress_rejection_lane(
            "non-monotonic-change-sequence",
            live,
            next_ordinal,
            next_basis,
        )
    }
}
