use crate::{
    IntegrityEvidenceCounters, IntegrityEvidenceLocality, IntegrityEvidenceOutcome,
    PhysicalIntegrityEvidenceBundle, QuarantineRecord, StoreIntegrityBoundaryClaim,
};
use crate::{IntegrityHandoffDenial, IntegrityHandoffDenialKind};
use worth_foundational::{FoundationalBoundaryArtifactCategory, FoundationalBoundaryArtifactRole};
use worth_store_aspect_native::StoreDigestEvidence;
use worth_store_contracts::StableDigest;
use worth_store_physical_format::PhysicalReferenceScope;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecoveryIntegrityHandoffReceipt {
    basis: StableDigest,
    category: FoundationalBoundaryArtifactCategory,
    role: FoundationalBoundaryArtifactRole,
    outcome: IntegrityEvidenceOutcome,
    locality: IntegrityEvidenceLocality,
    counters: IntegrityEvidenceCounters,
    physical_authority_basis: Option<StoreDigestEvidence>,
    receipt_evidence_basis: Option<StableDigest>,
}

impl RecoveryIntegrityHandoffReceipt {
    pub fn from_executed_evidence(
        evidence: &PhysicalIntegrityEvidenceBundle,
    ) -> Result<Self, IntegrityHandoffDenial> {
        if evidence.category() != FoundationalBoundaryArtifactCategory::Artifact {
            return Err(IntegrityHandoffDenial::new(
                IntegrityHandoffDenialKind::EvidenceIsNotAuthoritativeCurrent,
            ));
        }
        if evidence.boundary_role() != FoundationalBoundaryArtifactRole::AuthoritativeCurrent {
            return Err(IntegrityHandoffDenial::new(
                IntegrityHandoffDenialKind::EvidenceIsNotAuthoritativeCurrent,
            ));
        }
        if !matches!(
            evidence.integrity_outcome(),
            IntegrityEvidenceOutcome::IntactPhysicalBoundary
        ) {
            return Err(IntegrityHandoffDenial::new(
                IntegrityHandoffDenialKind::EvidenceIsNotIntactPhysicalBoundary,
            ));
        }
        Ok(Self::from_evidence(evidence))
    }

    pub fn from_quarantine_receipt_evidence(
        evidence: &PhysicalIntegrityEvidenceBundle,
    ) -> Result<Self, IntegrityHandoffDenial> {
        if evidence.category() != FoundationalBoundaryArtifactCategory::Receipt {
            return Err(IntegrityHandoffDenial::new(
                IntegrityHandoffDenialKind::EvidenceIsNotReceiptEvidence,
            ));
        }
        if evidence.boundary_role() != FoundationalBoundaryArtifactRole::ReceiptEvidence {
            return Err(IntegrityHandoffDenial::new(
                IntegrityHandoffDenialKind::EvidenceIsNotReceiptEvidence,
            ));
        }
        Ok(Self::from_evidence(evidence))
    }

    pub fn basis(&self) -> &StableDigest {
        &self.basis
    }

    pub const fn category(&self) -> FoundationalBoundaryArtifactCategory {
        self.category
    }

    pub const fn role(&self) -> FoundationalBoundaryArtifactRole {
        self.role
    }

    pub const fn outcome(&self) -> &IntegrityEvidenceOutcome {
        &self.outcome
    }

    pub const fn locality(&self) -> IntegrityEvidenceLocality {
        self.locality
    }

    pub const fn counters(&self) -> IntegrityEvidenceCounters {
        self.counters
    }

    pub fn physical_authority_basis(&self) -> Option<&StoreDigestEvidence> {
        self.physical_authority_basis.as_ref()
    }

    pub(crate) fn require_scope(
        &self,
        scope: PhysicalReferenceScope,
    ) -> Result<(), IntegrityHandoffDenial> {
        if self.locality == IntegrityEvidenceLocality::PhysicalScope(scope) {
            Ok(())
        } else {
            Err(IntegrityHandoffDenial::new(
                IntegrityHandoffDenialKind::ReceiptScopeMismatch,
            ))
        }
    }

    pub(crate) fn require_counters(
        &self,
        counters: IntegrityEvidenceCounters,
    ) -> Result<(), IntegrityHandoffDenial> {
        if self.counters == counters {
            Ok(())
        } else {
            Err(IntegrityHandoffDenial::new(
                IntegrityHandoffDenialKind::ReceiptCounterMismatch,
            ))
        }
    }

    pub(crate) fn require_physical_authority_basis(
        &self,
        basis: &StoreDigestEvidence,
    ) -> Result<(), IntegrityHandoffDenial> {
        if self.physical_authority_basis.as_ref() == Some(basis) {
            Ok(())
        } else {
            Err(IntegrityHandoffDenial::new(
                IntegrityHandoffDenialKind::ReceiptBasisMismatch,
            ))
        }
    }

    pub(crate) fn require_receipt_evidence(&self) -> Result<(), IntegrityHandoffDenial> {
        if self.category == FoundationalBoundaryArtifactCategory::Receipt
            && self.role == FoundationalBoundaryArtifactRole::ReceiptEvidence
        {
            Ok(())
        } else {
            Err(IntegrityHandoffDenial::new(
                IntegrityHandoffDenialKind::EvidenceIsNotReceiptEvidence,
            ))
        }
    }

    pub fn require_quarantine_record_basis(
        &self,
        record: &QuarantineRecord,
    ) -> Result<(), IntegrityHandoffDenial> {
        self.require_receipt_evidence()?;
        let expected = record.receipt().foundational_basis().digest();
        if self.receipt_evidence_basis.as_ref() == Some(expected) {
            Ok(())
        } else {
            Err(IntegrityHandoffDenial::new(
                IntegrityHandoffDenialKind::ReceiptBasisMismatch,
            ))
        }
    }

    fn from_evidence(evidence: &PhysicalIntegrityEvidenceBundle) -> Self {
        Self {
            basis: evidence.certification_receipt().basis().clone(),
            category: evidence.category(),
            role: evidence.boundary_role(),
            outcome: *evidence.integrity_outcome(),
            locality: evidence.locality(),
            counters: evidence.counters(),
            physical_authority_basis: physical_authority_basis(evidence),
            receipt_evidence_basis: receipt_evidence_basis(evidence),
        }
    }
}

fn physical_authority_basis(
    evidence: &PhysicalIntegrityEvidenceBundle,
) -> Option<StoreDigestEvidence> {
    match evidence.store_claim() {
        StoreIntegrityBoundaryClaim::PhysicalAuthority(claim) => Some(claim.basis().clone()),
        _ => None,
    }
}

fn receipt_evidence_basis(evidence: &PhysicalIntegrityEvidenceBundle) -> Option<StableDigest> {
    match evidence.store_claim() {
        StoreIntegrityBoundaryClaim::ReceiptEvidence(claim) => Some(claim.basis().clone()),
        _ => None,
    }
}
