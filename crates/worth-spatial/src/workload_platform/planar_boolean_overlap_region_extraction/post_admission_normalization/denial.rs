use super::counters::PlanarBooleanPostAdmissionNormalizationCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanPostAdmissionNormalizationDenialKind {
    InputIdentityMismatchDenied,
    AmbiguousCanonicalWindingDenied,
    AmbiguousCanonicalBoundaryDenied,
    BoundaryWitnessMismatchDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanPostAdmissionNormalizationDenial {
    kind: PlanarBooleanPostAdmissionNormalizationDenialKind,
    rejected_identity: String,
    counters: PlanarBooleanPostAdmissionNormalizationCounters,
    message: &'static str,
}

impl PlanarBooleanPostAdmissionNormalizationDenial {
    pub(crate) fn new(
        kind: PlanarBooleanPostAdmissionNormalizationDenialKind,
        rejected_identity: impl Into<String>,
        counters: PlanarBooleanPostAdmissionNormalizationCounters,
        message: &'static str,
    ) -> Self {
        Self {
            kind,
            rejected_identity: rejected_identity.into(),
            counters,
            message,
        }
    }

    pub fn kind(&self) -> PlanarBooleanPostAdmissionNormalizationDenialKind {
        self.kind
    }

    pub fn rejected_identity(&self) -> &str {
        &self.rejected_identity
    }

    pub fn counters(&self) -> PlanarBooleanPostAdmissionNormalizationCounters {
        self.counters
    }

    pub fn message(&self) -> &'static str {
        self.message
    }
}
