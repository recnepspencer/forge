#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupDiagnosticWitnessKind {
    SpatialTouchStageReceiptAndQueryPosture,
    SpatialTouchStageReceiptOnly,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupDiagnosticWitnessShape {
    kind: EvidenceLookupDiagnosticWitnessKind,
}

impl EvidenceLookupDiagnosticWitnessShape {
    pub(crate) const fn spatial_touch_stage_receipt_and_query_posture() -> Self {
        Self {
            kind: EvidenceLookupDiagnosticWitnessKind::SpatialTouchStageReceiptAndQueryPosture,
        }
    }

    pub(crate) const fn spatial_touch_stage_receipt_only() -> Self {
        Self {
            kind: EvidenceLookupDiagnosticWitnessKind::SpatialTouchStageReceiptOnly,
        }
    }

    pub const fn kind(&self) -> EvidenceLookupDiagnosticWitnessKind {
        self.kind
    }
}
