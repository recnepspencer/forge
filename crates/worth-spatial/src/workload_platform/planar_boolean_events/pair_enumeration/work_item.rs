use crate::workload_platform::planar_boolean_events::segment_identity::PlanarBooleanCanonicalSegment;

use super::identity::pair_work_item_identity;
use super::product::PlanarBooleanSegmentCandidateRowReceipt;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanSegmentPairWorkItem {
    left: PlanarBooleanCanonicalSegment,
    right: PlanarBooleanCanonicalSegment,
    segment_pair_identity: String,
}

impl PlanarBooleanSegmentPairWorkItem {
    pub(crate) fn from_candidate_row(row: &PlanarBooleanSegmentCandidateRowReceipt) -> Self {
        Self {
            left: row.left().clone(),
            right: row.right().clone(),
            segment_pair_identity: pair_work_item_identity(row.left(), row.right()),
        }
    }

    pub fn left(&self) -> &PlanarBooleanCanonicalSegment {
        &self.left
    }

    pub fn right(&self) -> &PlanarBooleanCanonicalSegment {
        &self.right
    }

    pub fn segment_pair_identity(&self) -> &str {
        &self.segment_pair_identity
    }
}
