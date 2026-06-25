use super::digest::digest_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiHostObservationAdmissionDenialCode {
    StaleMountedProductView,
    UnknownMountedNode,
    DuplicateObservationRow,
    InvalidMetricBasis,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiHostObservationAdmissionDenial {
    code: WorthUiHostObservationAdmissionDenialCode,
    subject: String,
    denial_digest: u64,
}

impl WorthUiHostObservationAdmissionDenial {
    pub(super) fn new(
        code: WorthUiHostObservationAdmissionDenialCode,
        subject: impl Into<String>,
    ) -> Self {
        let subject = subject.into();
        let denial_digest = digest_parts(["host_observation_denial", code.token(), &subject]);
        Self {
            code,
            subject,
            denial_digest,
        }
    }

    pub fn code(&self) -> WorthUiHostObservationAdmissionDenialCode {
        self.code
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn denial_digest(&self) -> u64 {
        self.denial_digest
    }
}

impl WorthUiHostObservationAdmissionDenialCode {
    pub fn token(self) -> &'static str {
        match self {
            Self::StaleMountedProductView => "stale_mounted_product_view",
            Self::UnknownMountedNode => "unknown_mounted_node",
            Self::DuplicateObservationRow => "duplicate_observation_row",
            Self::InvalidMetricBasis => "invalid_metric_basis",
        }
    }
}
