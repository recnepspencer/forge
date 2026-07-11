use forge_store_physical_backend::{
    AdmittedBackendCapabilityWitness, BackendCapabilityKind, BackendTargetProfile,
};

use super::{
    BackendQualificationMatrixDenial, BackendQualificationRow, BackendQualificationRowIdentity,
    QualificationCapabilityProofAuthority, QualificationHarnessProof, QualificationResidualDebt,
};
use crate::IoPressureHarnessEvidence;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackendQualificationMatrix {
    rows: Vec<BackendQualificationRow>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct QualificationMatrixPublisher {
    rows: Vec<BackendQualificationRow>,
}

impl QualificationMatrixPublisher {
    pub fn from_executed_store_evidence() -> Self {
        Self { rows: Vec::new() }
    }

    fn with_row(
        mut self,
        row: BackendQualificationRow,
    ) -> Result<Self, BackendQualificationMatrixDenial> {
        if self.rows.iter().any(|existing| same_claim(existing, &row)) {
            return Err(BackendQualificationMatrixDenial::DuplicateRow {
                profile: row.profile(),
                capability: row.capability(),
                evidence_class: row.evidence_class(),
            });
        }
        self.rows.push(row);
        Ok(self)
    }

    pub fn with_executed_buffered_file_row(
        self,
        witness: &AdmittedBackendCapabilityWitness,
        evidence: &IoPressureHarnessEvidence,
    ) -> Result<Self, BackendQualificationMatrixDenial> {
        self.with_executed_capability_row(
            witness,
            BackendCapabilityKind::BufferedFile,
            evidence,
            QualificationHarnessProof::from_executed_buffered_file_evidence(
                QualificationCapabilityProofAuthority::from_executed_store_evidence(),
                evidence,
            ),
        )
    }

    pub fn with_executed_buffered_file_row_and_residual_debt(
        self,
        witness: &AdmittedBackendCapabilityWitness,
        evidence: &IoPressureHarnessEvidence,
        residual_debt: QualificationResidualDebt,
    ) -> Result<Self, BackendQualificationMatrixDenial> {
        self.with_executed_capability_row_and_residual_debt(
            witness,
            BackendCapabilityKind::BufferedFile,
            evidence,
            QualificationHarnessProof::from_executed_buffered_file_evidence(
                QualificationCapabilityProofAuthority::from_executed_store_evidence(),
                evidence,
            ),
            residual_debt,
        )
    }

    pub fn with_executed_direct_io_row(
        self,
        witness: &AdmittedBackendCapabilityWitness,
        evidence: &IoPressureHarnessEvidence,
    ) -> Result<Self, BackendQualificationMatrixDenial> {
        self.with_executed_capability_row(
            witness,
            BackendCapabilityKind::DirectIo,
            evidence,
            QualificationHarnessProof::from_executed_direct_io_evidence(
                QualificationCapabilityProofAuthority::from_executed_store_evidence(),
                evidence,
            ),
        )
    }

    pub fn with_executed_direct_io_row_and_residual_debt(
        self,
        witness: &AdmittedBackendCapabilityWitness,
        evidence: &IoPressureHarnessEvidence,
        residual_debt: QualificationResidualDebt,
    ) -> Result<Self, BackendQualificationMatrixDenial> {
        self.with_executed_capability_row_and_residual_debt(
            witness,
            BackendCapabilityKind::DirectIo,
            evidence,
            QualificationHarnessProof::from_executed_direct_io_evidence(
                QualificationCapabilityProofAuthority::from_executed_store_evidence(),
                evidence,
            ),
            residual_debt,
        )
    }

    pub fn with_executed_mmap_row(
        self,
        witness: &AdmittedBackendCapabilityWitness,
        evidence: &IoPressureHarnessEvidence,
    ) -> Result<Self, BackendQualificationMatrixDenial> {
        self.with_executed_capability_row(
            witness,
            BackendCapabilityKind::Mmap,
            evidence,
            QualificationHarnessProof::from_executed_mmap_evidence(
                QualificationCapabilityProofAuthority::from_executed_store_evidence(),
                evidence,
            ),
        )
    }

    pub fn with_executed_mmap_row_and_residual_debt(
        self,
        witness: &AdmittedBackendCapabilityWitness,
        evidence: &IoPressureHarnessEvidence,
        residual_debt: QualificationResidualDebt,
    ) -> Result<Self, BackendQualificationMatrixDenial> {
        self.with_executed_capability_row_and_residual_debt(
            witness,
            BackendCapabilityKind::Mmap,
            evidence,
            QualificationHarnessProof::from_executed_mmap_evidence(
                QualificationCapabilityProofAuthority::from_executed_store_evidence(),
                evidence,
            ),
            residual_debt,
        )
    }

    pub fn with_executed_async_io_row(
        self,
        witness: &AdmittedBackendCapabilityWitness,
        evidence: &IoPressureHarnessEvidence,
    ) -> Result<Self, BackendQualificationMatrixDenial> {
        self.with_executed_capability_row(
            witness,
            BackendCapabilityKind::AsyncIo,
            evidence,
            QualificationHarnessProof::from_executed_async_io_evidence(
                QualificationCapabilityProofAuthority::from_executed_store_evidence(),
                evidence,
            ),
        )
    }

    pub fn with_executed_async_io_row_and_residual_debt(
        self,
        witness: &AdmittedBackendCapabilityWitness,
        evidence: &IoPressureHarnessEvidence,
        residual_debt: QualificationResidualDebt,
    ) -> Result<Self, BackendQualificationMatrixDenial> {
        self.with_executed_capability_row_and_residual_debt(
            witness,
            BackendCapabilityKind::AsyncIo,
            evidence,
            QualificationHarnessProof::from_executed_async_io_evidence(
                QualificationCapabilityProofAuthority::from_executed_store_evidence(),
                evidence,
            ),
            residual_debt,
        )
    }

    pub fn with_executed_flush_durability_row(
        self,
        witness: &AdmittedBackendCapabilityWitness,
        evidence: &IoPressureHarnessEvidence,
    ) -> Result<Self, BackendQualificationMatrixDenial> {
        self.with_executed_capability_row(
            witness,
            BackendCapabilityKind::Fsync,
            evidence,
            QualificationHarnessProof::from_executed_flush_durability_evidence(
                QualificationCapabilityProofAuthority::from_executed_store_evidence(),
                evidence,
            ),
        )
    }

    pub fn with_executed_flush_durability_row_and_residual_debt(
        self,
        witness: &AdmittedBackendCapabilityWitness,
        evidence: &IoPressureHarnessEvidence,
        residual_debt: QualificationResidualDebt,
    ) -> Result<Self, BackendQualificationMatrixDenial> {
        self.with_executed_capability_row_and_residual_debt(
            witness,
            BackendCapabilityKind::Fsync,
            evidence,
            QualificationHarnessProof::from_executed_flush_durability_evidence(
                QualificationCapabilityProofAuthority::from_executed_store_evidence(),
                evidence,
            ),
            residual_debt,
        )
    }

    pub fn with_executed_directory_sync_row(
        self,
        witness: &AdmittedBackendCapabilityWitness,
        evidence: &IoPressureHarnessEvidence,
    ) -> Result<Self, BackendQualificationMatrixDenial> {
        self.with_executed_capability_row(
            witness,
            BackendCapabilityKind::DirectorySync,
            evidence,
            QualificationHarnessProof::from_executed_directory_sync_evidence(
                QualificationCapabilityProofAuthority::from_executed_store_evidence(),
                evidence,
            ),
        )
    }

    pub fn with_executed_directory_sync_row_and_residual_debt(
        self,
        witness: &AdmittedBackendCapabilityWitness,
        evidence: &IoPressureHarnessEvidence,
        residual_debt: QualificationResidualDebt,
    ) -> Result<Self, BackendQualificationMatrixDenial> {
        self.with_executed_capability_row_and_residual_debt(
            witness,
            BackendCapabilityKind::DirectorySync,
            evidence,
            QualificationHarnessProof::from_executed_directory_sync_evidence(
                QualificationCapabilityProofAuthority::from_executed_store_evidence(),
                evidence,
            ),
            residual_debt,
        )
    }

    pub fn with_executed_durable_rename_row(
        self,
        witness: &AdmittedBackendCapabilityWitness,
        evidence: &IoPressureHarnessEvidence,
    ) -> Result<Self, BackendQualificationMatrixDenial> {
        self.with_executed_capability_row(
            witness,
            BackendCapabilityKind::DurableRename,
            evidence,
            QualificationHarnessProof::from_executed_durable_rename_evidence(
                QualificationCapabilityProofAuthority::from_executed_store_evidence(),
                evidence,
            ),
        )
    }

    pub fn with_executed_durable_rename_row_and_residual_debt(
        self,
        witness: &AdmittedBackendCapabilityWitness,
        evidence: &IoPressureHarnessEvidence,
        residual_debt: QualificationResidualDebt,
    ) -> Result<Self, BackendQualificationMatrixDenial> {
        self.with_executed_capability_row_and_residual_debt(
            witness,
            BackendCapabilityKind::DurableRename,
            evidence,
            QualificationHarnessProof::from_executed_durable_rename_evidence(
                QualificationCapabilityProofAuthority::from_executed_store_evidence(),
                evidence,
            ),
            residual_debt,
        )
    }

    pub fn with_executed_secure_frame_io_row(
        self,
        witness: &AdmittedBackendCapabilityWitness,
        evidence: &IoPressureHarnessEvidence,
    ) -> Result<Self, BackendQualificationMatrixDenial> {
        self.with_executed_capability_row(
            witness,
            BackendCapabilityKind::SecureFrameIo,
            evidence,
            QualificationHarnessProof::from_executed_secure_frame_io_evidence(
                QualificationCapabilityProofAuthority::from_executed_store_evidence(),
                evidence,
            ),
        )
    }

    pub fn with_executed_secure_frame_io_row_and_residual_debt(
        self,
        witness: &AdmittedBackendCapabilityWitness,
        evidence: &IoPressureHarnessEvidence,
        residual_debt: QualificationResidualDebt,
    ) -> Result<Self, BackendQualificationMatrixDenial> {
        self.with_executed_capability_row_and_residual_debt(
            witness,
            BackendCapabilityKind::SecureFrameIo,
            evidence,
            QualificationHarnessProof::from_executed_secure_frame_io_evidence(
                QualificationCapabilityProofAuthority::from_executed_store_evidence(),
                evidence,
            ),
            residual_debt,
        )
    }

    pub fn publish(self) -> Result<BackendQualificationMatrix, BackendQualificationMatrixDenial> {
        Ok(BackendQualificationMatrix { rows: self.rows })
    }

    fn with_executed_capability_row(
        self,
        witness: &AdmittedBackendCapabilityWitness,
        capability: BackendCapabilityKind,
        evidence: &IoPressureHarnessEvidence,
        harness_proof: QualificationHarnessProof,
    ) -> Result<Self, BackendQualificationMatrixDenial> {
        self.with_row(
            BackendQualificationRow::from_admitted_backend_witness_with_proof(
                witness,
                capability,
                evidence,
                harness_proof,
            )?,
        )
    }

    fn with_executed_capability_row_and_residual_debt(
        self,
        witness: &AdmittedBackendCapabilityWitness,
        capability: BackendCapabilityKind,
        evidence: &IoPressureHarnessEvidence,
        harness_proof: QualificationHarnessProof,
        residual_debt: QualificationResidualDebt,
    ) -> Result<Self, BackendQualificationMatrixDenial> {
        self.with_row(
            BackendQualificationRow::from_admitted_backend_witness_with_proof_and_residual_debt(
                witness,
                capability,
                evidence,
                harness_proof,
                residual_debt,
            )?,
        )
    }
}

impl BackendQualificationMatrix {
    pub fn rows(&self) -> &[BackendQualificationRow] {
        &self.rows
    }

    pub fn iter(&self) -> impl Iterator<Item = &BackendQualificationRow> {
        self.rows.iter()
    }

    pub fn require_row(
        &self,
        identity: BackendQualificationRowIdentity,
    ) -> Result<&BackendQualificationRow, BackendQualificationMatrixDenial> {
        self.rows
            .iter()
            .find(|row| row.identity() == identity)
            .ok_or(BackendQualificationMatrixDenial::RowNotFound {
                profile: identity.profile(),
                capability: identity.capability(),
            })
    }

    pub fn rows_for_claim(
        &self,
        profile: BackendTargetProfile,
        capability: BackendCapabilityKind,
    ) -> impl Iterator<Item = &BackendQualificationRow> {
        self.rows
            .iter()
            .filter(move |row| row.profile() == profile && row.capability() == capability)
    }
}

fn same_claim(left: &BackendQualificationRow, right: &BackendQualificationRow) -> bool {
    left.identity() == right.identity()
}
