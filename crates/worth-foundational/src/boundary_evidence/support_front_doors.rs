use worth_proof::TransitionOutcome;

use super::lineage::FoundationalBoundaryEvidenceLineageSubject;
use super::receipts::{
    FoundationalBoundaryEvidenceCompletedReceiptArtifact,
    FoundationalBoundaryEvidenceExecutedReceiptArtifact,
};
use super::support::{
    FoundationalBoundaryEvidencePublishedSupportArtifact,
    FoundationalBoundaryEvidenceSupportBasisDisclosure,
    FoundationalBoundaryEvidenceSupportCloseoutArtifact,
    FoundationalBoundaryEvidenceSupportConstructionDenial,
    FoundationalBoundaryEvidenceSupportRecoveryPosture,
    FoundationalBoundaryEvidenceSupportResidualDebtSet,
    FoundationalBoundaryEvidenceSupportTruthKind,
    FoundationalBoundaryEvidenceTransientLifecycleSupportArtifact,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FoundationalBoundaryEvidenceSupportFrontDoor;

impl FoundationalBoundaryEvidenceSupportFrontDoor {
    pub fn published_evidence(self) -> FoundationalBoundaryEvidencePublishedSupportStep {
        FoundationalBoundaryEvidencePublishedSupportStep::new(
            FoundationalBoundaryEvidenceSupportTruthKind::EvidenceBundle,
        )
    }

    pub fn certification_summary(self) -> FoundationalBoundaryEvidencePublishedSupportStep {
        FoundationalBoundaryEvidencePublishedSupportStep::new(
            FoundationalBoundaryEvidenceSupportTruthKind::CertificationSummary,
        )
    }

    pub fn parity_artifact(self) -> FoundationalBoundaryEvidencePublishedSupportStep {
        FoundationalBoundaryEvidencePublishedSupportStep::new(
            FoundationalBoundaryEvidenceSupportTruthKind::ParityArtifact,
        )
    }

    pub fn stale_basis_disclosure(self) -> FoundationalBoundaryEvidencePublishedSupportStep {
        FoundationalBoundaryEvidencePublishedSupportStep::new(
            FoundationalBoundaryEvidenceSupportTruthKind::StaleBasisDisclosure,
        )
    }

    pub fn residual_debt_statement(self) -> FoundationalBoundaryEvidencePublishedSupportStep {
        FoundationalBoundaryEvidencePublishedSupportStep::new(
            FoundationalBoundaryEvidenceSupportTruthKind::ResidualDebtStatement,
        )
    }

    pub fn degraded_recovery_report(self) -> FoundationalBoundaryEvidenceSupportCloseoutStep {
        FoundationalBoundaryEvidenceSupportCloseoutStep::new()
    }

    pub fn transient_lifecycle(
        self,
        subject: FoundationalBoundaryEvidenceLineageSubject,
    ) -> FoundationalBoundaryEvidenceTransientLifecycleStep {
        FoundationalBoundaryEvidenceTransientLifecycleStep::new(subject)
    }
}

#[derive(Debug, Clone)]
pub struct FoundationalBoundaryEvidencePublishedSupportStep {
    support_truth_kind: FoundationalBoundaryEvidenceSupportTruthKind,
}

impl FoundationalBoundaryEvidencePublishedSupportStep {
    fn new(support_truth_kind: FoundationalBoundaryEvidenceSupportTruthKind) -> Self {
        Self { support_truth_kind }
    }

    pub fn with_basis_disclosure(
        self,
        basis_disclosure: FoundationalBoundaryEvidenceSupportBasisDisclosure,
    ) -> FoundationalBoundaryEvidenceDisclosedPublishedSupportStep {
        FoundationalBoundaryEvidenceDisclosedPublishedSupportStep {
            support_truth_kind: self.support_truth_kind,
            basis_disclosure,
            recovery_posture: None,
            residual_debt: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FoundationalBoundaryEvidenceDisclosedPublishedSupportStep {
    support_truth_kind: FoundationalBoundaryEvidenceSupportTruthKind,
    basis_disclosure: FoundationalBoundaryEvidenceSupportBasisDisclosure,
    recovery_posture: Option<FoundationalBoundaryEvidenceSupportRecoveryPosture>,
    residual_debt: Option<FoundationalBoundaryEvidenceSupportResidualDebtSet>,
}

impl FoundationalBoundaryEvidenceDisclosedPublishedSupportStep {
    pub fn with_recovery_posture(
        mut self,
        recovery_posture: FoundationalBoundaryEvidenceSupportRecoveryPosture,
    ) -> Self {
        self.recovery_posture = Some(recovery_posture);
        self
    }

    pub fn with_residual_debt(
        mut self,
        residual_debt: FoundationalBoundaryEvidenceSupportResidualDebtSet,
    ) -> Self {
        self.residual_debt = Some(residual_debt);
        self
    }

    pub fn attested_by(
        self,
        support_publication_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    ) -> TransitionOutcome<
        FoundationalBoundaryEvidencePublishedSupportArtifact,
        FoundationalBoundaryEvidenceSupportConstructionDenial,
    > {
        match FoundationalBoundaryEvidencePublishedSupportArtifact::new(
            self.support_truth_kind,
            support_publication_receipt,
            self.basis_disclosure,
            self.recovery_posture,
            self.residual_debt,
        ) {
            Ok(artifact) => TransitionOutcome::success(artifact),
            Err(denial) => TransitionOutcome::denied(denial),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FoundationalBoundaryEvidenceSupportCloseoutStep;

impl FoundationalBoundaryEvidenceSupportCloseoutStep {
    fn new() -> Self {
        Self
    }

    pub fn with_basis_disclosure(
        self,
        basis_disclosure: FoundationalBoundaryEvidenceSupportBasisDisclosure,
    ) -> FoundationalBoundaryEvidenceDisclosedSupportCloseoutStep {
        FoundationalBoundaryEvidenceDisclosedSupportCloseoutStep {
            basis_disclosure,
            recovery_posture: None,
            residual_debt: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FoundationalBoundaryEvidenceDisclosedSupportCloseoutStep {
    basis_disclosure: FoundationalBoundaryEvidenceSupportBasisDisclosure,
    recovery_posture: Option<FoundationalBoundaryEvidenceSupportRecoveryPosture>,
    residual_debt: Option<FoundationalBoundaryEvidenceSupportResidualDebtSet>,
}

impl FoundationalBoundaryEvidenceDisclosedSupportCloseoutStep {
    pub fn with_recovery_posture(
        mut self,
        recovery_posture: FoundationalBoundaryEvidenceSupportRecoveryPosture,
    ) -> Self {
        self.recovery_posture = Some(recovery_posture);
        self
    }

    pub fn with_residual_debt(
        mut self,
        residual_debt: FoundationalBoundaryEvidenceSupportResidualDebtSet,
    ) -> Self {
        self.residual_debt = Some(residual_debt);
        self
    }

    pub fn closed_out_by(
        self,
        closeout_receipt: FoundationalBoundaryEvidenceCompletedReceiptArtifact,
    ) -> TransitionOutcome<
        FoundationalBoundaryEvidenceSupportCloseoutArtifact,
        FoundationalBoundaryEvidenceSupportConstructionDenial,
    > {
        match FoundationalBoundaryEvidenceSupportCloseoutArtifact::new(
            closeout_receipt,
            self.basis_disclosure,
            self.recovery_posture,
            self.residual_debt,
        ) {
            Ok(artifact) => TransitionOutcome::success(artifact),
            Err(denial) => TransitionOutcome::denied(denial),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FoundationalBoundaryEvidenceTransientLifecycleStep {
    subject: FoundationalBoundaryEvidenceLineageSubject,
}

impl FoundationalBoundaryEvidenceTransientLifecycleStep {
    fn new(subject: FoundationalBoundaryEvidenceLineageSubject) -> Self {
        Self { subject }
    }

    pub fn with_basis_disclosure(
        self,
        basis_disclosure: FoundationalBoundaryEvidenceSupportBasisDisclosure,
    ) -> FoundationalBoundaryEvidenceDisclosedTransientLifecycleStep {
        FoundationalBoundaryEvidenceDisclosedTransientLifecycleStep {
            subject: self.subject,
            basis_disclosure,
            residual_debt: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FoundationalBoundaryEvidenceDisclosedTransientLifecycleStep {
    subject: FoundationalBoundaryEvidenceLineageSubject,
    basis_disclosure: FoundationalBoundaryEvidenceSupportBasisDisclosure,
    residual_debt: Option<FoundationalBoundaryEvidenceSupportResidualDebtSet>,
}

impl FoundationalBoundaryEvidenceDisclosedTransientLifecycleStep {
    pub fn with_residual_debt(
        mut self,
        residual_debt: FoundationalBoundaryEvidenceSupportResidualDebtSet,
    ) -> Self {
        self.residual_debt = Some(residual_debt);
        self
    }

    pub fn opened_and_closed_within(
        self,
        executed_receipt: FoundationalBoundaryEvidenceExecutedReceiptArtifact,
    ) -> FoundationalBoundaryEvidenceTransientLifecycleSupportArtifact {
        FoundationalBoundaryEvidenceTransientLifecycleSupportArtifact::new(
            self.subject,
            executed_receipt,
            self.basis_disclosure,
            self.residual_debt,
        )
    }
}
