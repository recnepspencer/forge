use super::lane_builders::{build_lanes, PreviewCertificationLanes};
use super::rejection_evidence::{build_rejection_evidence, PreviewRejectionEvidence};

pub(super) struct PreviewCertificationEvidence {
    pub(super) lanes: PreviewCertificationLanes,
    pub(super) rejections: PreviewRejectionEvidence,
}

pub(super) fn build_preview_certification_evidence() -> PreviewCertificationEvidence {
    PreviewCertificationEvidence {
        lanes: build_lanes(),
        rejections: build_rejection_evidence(),
    }
}
