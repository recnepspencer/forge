use crate::graph::{UiGraphEvidenceRefKind, UiGraphInspectionTargetKind, UiGraphInspectionStopPoint};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphInspectionSupportReport {
    target_kinds: &'static [UiGraphInspectionTargetKind],
    evidence_ref_kinds: &'static [UiGraphEvidenceRefKind],
    stop_points: &'static [UiGraphInspectionStopPoint],
}

impl UiGraphInspectionSupportReport {
    pub(crate) const fn new(
        target_kinds: &'static [UiGraphInspectionTargetKind],
        evidence_ref_kinds: &'static [UiGraphEvidenceRefKind],
        stop_points: &'static [UiGraphInspectionStopPoint],
    ) -> Self {
        Self {
            target_kinds,
            evidence_ref_kinds,
            stop_points,
        }
    }

    pub const fn target_kinds(self) -> &'static [UiGraphInspectionTargetKind] {
        self.target_kinds
    }

    pub const fn evidence_ref_kinds(self) -> &'static [UiGraphEvidenceRefKind] {
        self.evidence_ref_kinds
    }

    pub const fn stop_points(self) -> &'static [UiGraphInspectionStopPoint] {
        self.stop_points
    }
}
