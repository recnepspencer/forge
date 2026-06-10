use super::PredicateCertificateConsumptionCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PredicateCertificateConsumptionDenialKind {
    MissingPredicateAuthority,
    MissingPredicateConsumer,
    DuplicatePredicateReceipt,
    UnconsumedPredicateReceipt,
    MissingConsumedPredicateReceipt,
    MissingPrecisionMetadata,
    TopologyBasisMismatch,
    MovementRotationPostureMismatch,
    LocalFrameMismatch,
    TolerancePolicyMismatch,
    SubstitutePredicateEvidence,
}

impl PredicateCertificateConsumptionDenialKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingPredicateAuthority => "missing-predicate-authority",
            Self::MissingPredicateConsumer => "missing-predicate-consumer",
            Self::DuplicatePredicateReceipt => "duplicate-predicate-receipt",
            Self::UnconsumedPredicateReceipt => "unconsumed-predicate-receipt",
            Self::MissingConsumedPredicateReceipt => "missing-consumed-predicate-receipt",
            Self::MissingPrecisionMetadata => "missing-precision-metadata",
            Self::TopologyBasisMismatch => "topology-basis-mismatch",
            Self::MovementRotationPostureMismatch => "movement-rotation-posture-mismatch",
            Self::LocalFrameMismatch => "local-frame-mismatch",
            Self::TolerancePolicyMismatch => "tolerance-policy-mismatch",
            Self::SubstitutePredicateEvidence => "substitute-predicate-evidence",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PredicateCertificateConsumptionDenial {
    kind: PredicateCertificateConsumptionDenialKind,
    reason: String,
    counters: PredicateCertificateConsumptionCounters,
}

impl PredicateCertificateConsumptionDenial {
    pub(crate) fn new(
        kind: PredicateCertificateConsumptionDenialKind,
        reason: impl Into<String>,
    ) -> Self {
        let counters = if matches!(
            kind,
            PredicateCertificateConsumptionDenialKind::SubstitutePredicateEvidence
        ) {
            PredicateCertificateConsumptionCounters::rejected_substitute()
        } else {
            PredicateCertificateConsumptionCounters::certified(0, 0, 0)
        };
        Self {
            kind,
            reason: reason.into(),
            counters,
        }
    }

    pub fn kind(&self) -> PredicateCertificateConsumptionDenialKind {
        self.kind
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn counters(&self) -> PredicateCertificateConsumptionCounters {
        self.counters
    }
}
