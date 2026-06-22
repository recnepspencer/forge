#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkloadCertificationContextDenial {
    kind: WorkloadCertificationContextDenialKind,
    reason: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkloadCertificationContextDenialKind {
    MissingTransformReceipts,
    MismatchedTransformReceipts,
    MismatchedMotionBinding,
    PredicateCertificationFailed,
    PrecisionBasisDenied,
    PrecisionCertificationFailed,
    LocalFrameBasisDenied,
    LocalFrameCertificationFailed,
}

impl WorkloadCertificationContextDenial {
    pub(crate) fn new(
        kind: WorkloadCertificationContextDenialKind,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            reason: reason.into(),
        }
    }

    pub fn kind(&self) -> WorkloadCertificationContextDenialKind {
        self.kind
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}
