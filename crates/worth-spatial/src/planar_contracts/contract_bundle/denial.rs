use super::PlanarContractBundleFamily;
use super::PlanarContractBundleValidationCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarContractBundleDenialKind {
    MissingCertificateFamily,
    MissingTopologyBasis,
    MissingMovementRotationPosture,
    MissingDiagnosticScope,
    MismatchedMovementRotationPosture,
    TopologyBasisMismatch,
    MissingProjectionConsumption,
    MissingRetainedFactDigest,
    MismatchedCertificateFamily,
    BooleanExecutionAlreadyPresent,
}

impl PlanarContractBundleDenialKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingCertificateFamily => "missing-certificate-family",
            Self::MissingTopologyBasis => "missing-topology-basis",
            Self::MissingMovementRotationPosture => "missing-movement-rotation-posture",
            Self::MissingDiagnosticScope => "missing-diagnostic-scope",
            Self::MismatchedMovementRotationPosture => "mismatched-movement-rotation-posture",
            Self::TopologyBasisMismatch => "topology-basis-mismatch",
            Self::MissingProjectionConsumption => "missing-projection-consumption",
            Self::MissingRetainedFactDigest => "missing-retained-fact-digest",
            Self::MismatchedCertificateFamily => "mismatched-certificate-family",
            Self::BooleanExecutionAlreadyPresent => "boolean-execution-already-present",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarContractBundleDenial {
    kind: PlanarContractBundleDenialKind,
    family: Option<PlanarContractBundleFamily>,
    reason: String,
    counters: PlanarContractBundleValidationCounters,
}

impl PlanarContractBundleDenial {
    pub(crate) fn new(
        kind: PlanarContractBundleDenialKind,
        family: Option<PlanarContractBundleFamily>,
        reason: impl Into<String>,
    ) -> Self {
        let counters = if matches!(
            kind,
            PlanarContractBundleDenialKind::MissingCertificateFamily
                | PlanarContractBundleDenialKind::MissingProjectionConsumption
        ) {
            PlanarContractBundleValidationCounters::rejected_missing_family()
        } else {
            PlanarContractBundleValidationCounters::certified(0, 0, 0, 0, 0)
        };
        Self {
            kind,
            family,
            reason: reason.into(),
            counters,
        }
    }

    pub fn kind(&self) -> PlanarContractBundleDenialKind {
        self.kind
    }

    pub fn family(&self) -> Option<PlanarContractBundleFamily> {
        self.family
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn counters(&self) -> PlanarContractBundleValidationCounters {
        self.counters
    }
}
