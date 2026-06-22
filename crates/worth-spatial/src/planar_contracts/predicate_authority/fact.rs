use worth_math::arithmetic::precision::PrecisionEscalation;
use worth_math::sign::CertifiedTriSign;

use super::{PlanarPredicateAuthorityPosture, PlanarPredicateInputBasis, PlanarPredicateKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarPredicatePerformanceCounters {
    predicate_evaluations: usize,
    input_point_count: usize,
    canonical_basis_part_count: usize,
}

impl PlanarPredicatePerformanceCounters {
    pub(crate) fn orient2d(canonical_basis_part_count: usize) -> Self {
        Self {
            predicate_evaluations: 1,
            input_point_count: 3,
            canonical_basis_part_count,
        }
    }

    pub fn predicate_evaluations(&self) -> usize {
        self.predicate_evaluations
    }

    pub fn input_point_count(&self) -> usize {
        self.input_point_count
    }

    pub fn canonical_basis_part_count(&self) -> usize {
        self.canonical_basis_part_count
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarPredicateFactReceipt {
    predicate_kind: PlanarPredicateKind,
    input_basis: PlanarPredicateInputBasis,
    certified_sign: CertifiedTriSign,
    precision_escalation: PrecisionEscalation,
    posture: PlanarPredicateAuthorityPosture,
    declaration_digest: String,
    envelope_digest: String,
    fact_digest: String,
    counters: PlanarPredicatePerformanceCounters,
}

impl PlanarPredicateFactReceipt {
    pub(crate) fn new(
        predicate_kind: PlanarPredicateKind,
        input_basis: PlanarPredicateInputBasis,
        certified_sign: CertifiedTriSign,
        precision_escalation: PrecisionEscalation,
        declaration_digest: String,
        envelope_digest: String,
        fact_digest: String,
        counters: PlanarPredicatePerformanceCounters,
    ) -> Self {
        Self {
            predicate_kind,
            input_basis,
            certified_sign,
            precision_escalation,
            posture: PlanarPredicateAuthorityPosture::Certified,
            declaration_digest,
            envelope_digest,
            fact_digest,
            counters,
        }
    }

    pub fn predicate_kind(&self) -> PlanarPredicateKind {
        self.predicate_kind
    }

    pub fn input_basis(&self) -> &PlanarPredicateInputBasis {
        &self.input_basis
    }

    pub fn certified_sign(&self) -> CertifiedTriSign {
        self.certified_sign
    }

    pub fn precision_escalation(&self) -> &PrecisionEscalation {
        &self.precision_escalation
    }

    pub fn posture(&self) -> PlanarPredicateAuthorityPosture {
        self.posture
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }

    pub fn fact_digest(&self) -> &str {
        &self.fact_digest
    }

    pub fn counters(&self) -> PlanarPredicatePerformanceCounters {
        self.counters
    }
}
