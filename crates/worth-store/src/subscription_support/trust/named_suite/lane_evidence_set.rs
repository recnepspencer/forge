use super::super::failure::SupportTrustFailure;
use super::digest::stable_digest;
use super::lane_evidence::SubscriptionSupportAccuracyLaneEvidence;
use super::lane_validation::validate_required_lane_evidence;
use super::row_kind::SubscriptionSupportAccuracyCertificationRowKind;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportAccuracyLaneEvidenceSet {
    lanes: Vec<SubscriptionSupportAccuracyLaneEvidence>,
    lane_evidence_set_digest: String,
}

impl SubscriptionSupportAccuracyLaneEvidenceSet {
    pub fn new(
        mut lanes: Vec<SubscriptionSupportAccuracyLaneEvidence>,
    ) -> Result<Self, SupportTrustFailure> {
        lanes.sort_by_key(SubscriptionSupportAccuracyLaneEvidence::row_kind);
        validate_required_lane_evidence(&lanes)?;
        let mut evidence_set = Self {
            lanes,
            lane_evidence_set_digest: String::new(),
        };
        evidence_set.lane_evidence_set_digest =
            stable_digest(&SubscriptionSupportAccuracyLaneEvidenceSetDigestBasis {
                lane_digests: &evidence_set
                    .lanes
                    .iter()
                    .map(SubscriptionSupportAccuracyLaneEvidence::evidence_digest)
                    .collect::<Vec<_>>(),
            })?;
        Ok(evidence_set)
    }

    pub fn lanes(&self) -> &[SubscriptionSupportAccuracyLaneEvidence] {
        &self.lanes
    }

    pub fn lane_evidence_set_digest(&self) -> &str {
        &self.lane_evidence_set_digest
    }

    pub(super) fn evidence_for(
        &self,
        row_kind: SubscriptionSupportAccuracyCertificationRowKind,
    ) -> Option<&SubscriptionSupportAccuracyLaneEvidence> {
        self.lanes.iter().find(|lane| lane.row_kind() == row_kind)
    }
}

#[derive(Serialize)]
struct SubscriptionSupportAccuracyLaneEvidenceSetDigestBasis<'a> {
    lane_digests: &'a [&'a str],
}
