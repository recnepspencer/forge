use crate::{
    foundational_identity_for_checksum_basis, ChecksumAlgorithmId, ChecksumAlgorithmMismatchDenial,
    ChecksumCompatibilityPosture, ChecksumDetectionModel, ChecksumScopeDeclaration,
    FoundationalChecksumEvidenceIdentity, IntegrityEntryWitness,
};
use forge_store_physical_format::{
    ChecksumCoverageMap, PhysicalFormatIdentity, PhysicalFormatVersion,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChecksumCoverageBasis {
    algorithm_id: ChecksumAlgorithmId,
    physical_format_identity: PhysicalFormatIdentity,
    coverage_map: ChecksumCoverageMap,
    detection_model: ChecksumDetectionModel,
}

impl ChecksumCoverageBasis {
    pub(crate) fn new(
        algorithm_id: ChecksumAlgorithmId,
        scope: ChecksumScopeDeclaration,
        detection_model: ChecksumDetectionModel,
    ) -> Self {
        Self {
            algorithm_id,
            physical_format_identity: scope.physical_format_identity(),
            coverage_map: scope.coverage_map().clone(),
            detection_model,
        }
    }

    pub const fn algorithm_id(&self) -> ChecksumAlgorithmId {
        self.algorithm_id
    }

    pub const fn physical_format_identity(&self) -> PhysicalFormatIdentity {
        self.physical_format_identity
    }

    pub const fn physical_format_version(&self) -> PhysicalFormatVersion {
        self.physical_format_identity.version()
    }

    pub fn coverage_map(&self) -> &ChecksumCoverageMap {
        &self.coverage_map
    }

    pub const fn detection_model(&self) -> ChecksumDetectionModel {
        self.detection_model
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChecksumAlgorithmDeclaration {
    basis: ChecksumCoverageBasis,
    foundational_evidence_identity: FoundationalChecksumEvidenceIdentity,
}

impl ChecksumAlgorithmDeclaration {
    pub(crate) fn declare(
        algorithm_id: ChecksumAlgorithmId,
        scope: ChecksumScopeDeclaration,
        detection_model: ChecksumDetectionModel,
    ) -> Result<Self, ChecksumAlgorithmMismatchDenial> {
        let basis = ChecksumCoverageBasis::new(algorithm_id, scope, detection_model);
        let foundational_evidence_identity = foundational_identity_for_checksum_basis(&basis)?;
        Ok(Self {
            basis,
            foundational_evidence_identity,
        })
    }

    pub fn coverage_basis(&self) -> &ChecksumCoverageBasis {
        &self.basis
    }

    pub fn foundational_evidence_identity(&self) -> &FoundationalChecksumEvidenceIdentity {
        &self.foundational_evidence_identity
    }

    pub fn compatibility_with_coverage(
        &self,
        candidate: &ChecksumCoverageMap,
    ) -> Result<ChecksumCompatibilityPosture, ChecksumAlgorithmMismatchDenial> {
        if self.basis.coverage_map() == candidate {
            return Ok(ChecksumCompatibilityPosture::SameCoverageReused);
        }
        Err(ChecksumAlgorithmMismatchDenial::CompatibilityReadmissionRequired)
    }

    pub fn compatibility_posture_for_coverage(
        &self,
        candidate: &ChecksumCoverageMap,
    ) -> ChecksumCompatibilityPosture {
        if self.basis.coverage_map() == candidate {
            ChecksumCompatibilityPosture::SameCoverageReused
        } else {
            ChecksumCompatibilityPosture::ExplicitReadmissionRequired
        }
    }

    pub fn admit_for_s3_entry(
        self,
        entry_witness: IntegrityEntryWitness,
    ) -> S3ChecksumDeclarationAdmission {
        S3ChecksumDeclarationAdmission {
            declaration: self,
            entry_witness,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S3ChecksumDeclarationAdmission {
    declaration: ChecksumAlgorithmDeclaration,
    entry_witness: IntegrityEntryWitness,
}

impl S3ChecksumDeclarationAdmission {
    pub fn declaration(&self) -> &ChecksumAlgorithmDeclaration {
        &self.declaration
    }

    pub const fn entry_witness(&self) -> IntegrityEntryWitness {
        self.entry_witness
    }
}
