use super::super::lineage::FoundationalBoundaryEvidenceLineageSubject;
use super::super::provenance::FoundationalBoundaryEvidenceProvenanceArtifact;
use super::super::receipts::{
    FoundationalBoundaryEvidenceCompletedReceiptArtifact,
    FoundationalBoundaryEvidenceExecutedReceiptArtifact, FoundationalBoundaryEvidenceReceiptKind,
};
use super::definitions::{
    FoundationalBoundaryEvidenceSupportBasisDisclosure,
    FoundationalBoundaryEvidenceSupportRecoveryPosture,
    FoundationalBoundaryEvidenceSupportResidualDebtKind,
    FoundationalBoundaryEvidenceSupportTruthKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceSupportResidualDebtSet(
    Vec<FoundationalBoundaryEvidenceSupportResidualDebtKind>,
);

impl FoundationalBoundaryEvidenceSupportResidualDebtSet {
    pub fn new(
        mut kinds: Vec<FoundationalBoundaryEvidenceSupportResidualDebtKind>,
    ) -> Result<Self, FoundationalBoundaryEvidenceSupportConstructionDenial> {
        kinds.sort();
        kinds.dedup();

        if kinds.is_empty() {
            return Err(
                FoundationalBoundaryEvidenceSupportConstructionDenial::ResidualDebtSetMustNotBeEmpty,
            );
        }

        Ok(Self(kinds))
    }

    pub fn kinds(&self) -> &[FoundationalBoundaryEvidenceSupportResidualDebtKind] {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalBoundaryEvidenceSupportConstructionDenial {
    SupportPublicationReceiptRequired,
    SupportCloseoutReceiptRequired,
    ResidualDebtSetMustNotBeEmpty,
    RebuildRequiredSupportRequiresResidualDebt,
    QuarantinedSupportRequiresResidualDebt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidencePublishedSupportArtifact {
    support_truth_kind: FoundationalBoundaryEvidenceSupportTruthKind,
    support_publication_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    basis_disclosure: FoundationalBoundaryEvidenceSupportBasisDisclosure,
    recovery_posture: Option<FoundationalBoundaryEvidenceSupportRecoveryPosture>,
    residual_debt: Option<FoundationalBoundaryEvidenceSupportResidualDebtSet>,
}

impl FoundationalBoundaryEvidencePublishedSupportArtifact {
    pub(crate) fn new(
        support_truth_kind: FoundationalBoundaryEvidenceSupportTruthKind,
        support_publication_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
        basis_disclosure: FoundationalBoundaryEvidenceSupportBasisDisclosure,
        recovery_posture: Option<FoundationalBoundaryEvidenceSupportRecoveryPosture>,
        residual_debt: Option<FoundationalBoundaryEvidenceSupportResidualDebtSet>,
    ) -> Result<Self, FoundationalBoundaryEvidenceSupportConstructionDenial> {
        if support_publication_receipt.receipt_kind()
            != FoundationalBoundaryEvidenceReceiptKind::SupportPublication
        {
            return Err(
                FoundationalBoundaryEvidenceSupportConstructionDenial::SupportPublicationReceiptRequired,
            );
        }

        validate_support_posture(recovery_posture, residual_debt.as_ref())?;

        Ok(Self {
            support_truth_kind,
            support_publication_receipt,
            basis_disclosure,
            recovery_posture,
            residual_debt,
        })
    }

    pub const fn support_truth_kind(&self) -> FoundationalBoundaryEvidenceSupportTruthKind {
        self.support_truth_kind
    }

    pub fn support_publication_receipt(
        &self,
    ) -> &FoundationalBoundaryEvidenceExecutedReceiptArtifact {
        &self.support_publication_receipt
    }

    pub const fn basis_disclosure(&self) -> FoundationalBoundaryEvidenceSupportBasisDisclosure {
        self.basis_disclosure
    }

    pub const fn recovery_posture(
        &self,
    ) -> Option<FoundationalBoundaryEvidenceSupportRecoveryPosture> {
        self.recovery_posture
    }

    pub fn residual_debt(&self) -> Option<&FoundationalBoundaryEvidenceSupportResidualDebtSet> {
        self.residual_debt.as_ref()
    }

    pub fn provenance(&self) -> &FoundationalBoundaryEvidenceProvenanceArtifact {
        self.support_publication_receipt.provenance()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceSupportCloseoutArtifact {
    closeout_receipt: FoundationalBoundaryEvidenceCompletedReceiptArtifact,
    basis_disclosure: FoundationalBoundaryEvidenceSupportBasisDisclosure,
    recovery_posture: Option<FoundationalBoundaryEvidenceSupportRecoveryPosture>,
    residual_debt: Option<FoundationalBoundaryEvidenceSupportResidualDebtSet>,
}

impl FoundationalBoundaryEvidenceSupportCloseoutArtifact {
    pub(crate) fn new(
        closeout_receipt: FoundationalBoundaryEvidenceCompletedReceiptArtifact,
        basis_disclosure: FoundationalBoundaryEvidenceSupportBasisDisclosure,
        recovery_posture: Option<FoundationalBoundaryEvidenceSupportRecoveryPosture>,
        residual_debt: Option<FoundationalBoundaryEvidenceSupportResidualDebtSet>,
    ) -> Result<Self, FoundationalBoundaryEvidenceSupportConstructionDenial> {
        if closeout_receipt.receipt_kind() != FoundationalBoundaryEvidenceReceiptKind::Closeout {
            return Err(
                FoundationalBoundaryEvidenceSupportConstructionDenial::SupportCloseoutReceiptRequired,
            );
        }

        validate_support_posture(recovery_posture, residual_debt.as_ref())?;

        Ok(Self {
            closeout_receipt,
            basis_disclosure,
            recovery_posture,
            residual_debt,
        })
    }

    pub const fn support_truth_kind(&self) -> FoundationalBoundaryEvidenceSupportTruthKind {
        FoundationalBoundaryEvidenceSupportTruthKind::DegradedRecoveryReport
    }

    pub fn closeout_receipt(&self) -> &FoundationalBoundaryEvidenceCompletedReceiptArtifact {
        &self.closeout_receipt
    }

    pub const fn basis_disclosure(&self) -> FoundationalBoundaryEvidenceSupportBasisDisclosure {
        self.basis_disclosure
    }

    pub const fn recovery_posture(
        &self,
    ) -> Option<FoundationalBoundaryEvidenceSupportRecoveryPosture> {
        self.recovery_posture
    }

    pub fn residual_debt(&self) -> Option<&FoundationalBoundaryEvidenceSupportResidualDebtSet> {
        self.residual_debt.as_ref()
    }

    pub fn provenance(&self) -> &FoundationalBoundaryEvidenceProvenanceArtifact {
        self.closeout_receipt.provenance()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryEvidenceTransientLifecycleSupportArtifact {
    subject: FoundationalBoundaryEvidenceLineageSubject,
    executed_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    basis_disclosure: FoundationalBoundaryEvidenceSupportBasisDisclosure,
    residual_debt: Option<FoundationalBoundaryEvidenceSupportResidualDebtSet>,
}

impl FoundationalBoundaryEvidenceTransientLifecycleSupportArtifact {
    pub(crate) fn new(
        subject: FoundationalBoundaryEvidenceLineageSubject,
        executed_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
        basis_disclosure: FoundationalBoundaryEvidenceSupportBasisDisclosure,
        residual_debt: Option<FoundationalBoundaryEvidenceSupportResidualDebtSet>,
    ) -> Self {
        Self {
            subject,
            executed_receipt,
            basis_disclosure,
            residual_debt,
        }
    }

    pub const fn support_truth_kind(&self) -> FoundationalBoundaryEvidenceSupportTruthKind {
        FoundationalBoundaryEvidenceSupportTruthKind::TransientLifecycleEvidence
    }

    pub const fn subject(&self) -> FoundationalBoundaryEvidenceLineageSubject {
        self.subject
    }

    pub fn executed_receipt(&self) -> &FoundationalBoundaryEvidenceExecutedReceiptArtifact {
        &self.executed_receipt
    }

    pub const fn basis_disclosure(&self) -> FoundationalBoundaryEvidenceSupportBasisDisclosure {
        self.basis_disclosure
    }

    pub fn residual_debt(&self) -> Option<&FoundationalBoundaryEvidenceSupportResidualDebtSet> {
        self.residual_debt.as_ref()
    }

    pub fn provenance(&self) -> &FoundationalBoundaryEvidenceProvenanceArtifact {
        self.executed_receipt.provenance()
    }
}

fn validate_support_posture(
    recovery_posture: Option<FoundationalBoundaryEvidenceSupportRecoveryPosture>,
    residual_debt: Option<&FoundationalBoundaryEvidenceSupportResidualDebtSet>,
) -> Result<(), FoundationalBoundaryEvidenceSupportConstructionDenial> {
    match recovery_posture {
        Some(FoundationalBoundaryEvidenceSupportRecoveryPosture::RebuildRequired)
            if residual_debt.is_none() =>
        {
            Err(
                FoundationalBoundaryEvidenceSupportConstructionDenial::RebuildRequiredSupportRequiresResidualDebt,
            )
        }
        Some(FoundationalBoundaryEvidenceSupportRecoveryPosture::Quarantined)
            if residual_debt.is_none() =>
        {
            Err(
                FoundationalBoundaryEvidenceSupportConstructionDenial::QuarantinedSupportRequiresResidualDebt,
            )
        }
        _ => Ok(()),
    }
}
