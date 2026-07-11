use crate::{ChecksumAlgorithmMismatchDenial, ChecksumDetectionModel, ChecksumScopeDeclaration};
use forge_store_contracts::StableDigest;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChecksumAlgorithmId {
    kind: ChecksumAlgorithmKind,
}

impl ChecksumAlgorithmId {
    pub const fn crc32c() -> Self {
        Self {
            kind: ChecksumAlgorithmKind::Crc32c,
        }
    }

    pub const fn crc64_nvme() -> Self {
        Self {
            kind: ChecksumAlgorithmKind::Crc64Nvme,
        }
    }

    pub fn admit_claim(
        claim: ChecksumAlgorithmClaim<'_>,
    ) -> Result<Self, ChecksumAlgorithmMismatchDenial> {
        match claim {
            ChecksumAlgorithmClaim::Declared(id) => Ok(id),
            ChecksumAlgorithmClaim::DeclaredText("crc32c") => Ok(Self::crc32c()),
            ChecksumAlgorithmClaim::DeclaredText("crc64-nvme") => Ok(Self::crc64_nvme()),
            ChecksumAlgorithmClaim::DeclaredText(_) => {
                Err(ChecksumAlgorithmMismatchDenial::UnknownAlgorithm)
            }
            ChecksumAlgorithmClaim::ArtifactDigest => {
                Err(ChecksumAlgorithmMismatchDenial::DigestAsChecksumSubstitution)
            }
            ChecksumAlgorithmClaim::AuthenticityClaim => {
                Err(ChecksumAlgorithmMismatchDenial::ChecksumAsAuthenticityClaim)
            }
        }
    }

    pub fn require_matches(self, expected: Self) -> Result<Self, ChecksumAlgorithmMismatchDenial> {
        if self == expected {
            Ok(self)
        } else {
            Err(ChecksumAlgorithmMismatchDenial::AlgorithmIdMismatch)
        }
    }

    pub fn declare_for_scope(
        self,
        scope: ChecksumScopeDeclaration,
    ) -> Result<crate::ChecksumAlgorithmDeclaration, ChecksumAlgorithmMismatchDenial> {
        crate::ChecksumAlgorithmDeclaration::declare(self, scope, self.detection_model())
    }

    pub const fn as_str(self) -> &'static str {
        match self.kind {
            ChecksumAlgorithmKind::Crc32c => "crc32c",
            ChecksumAlgorithmKind::Crc64Nvme => "crc64-nvme",
        }
    }

    pub const fn detection_model(self) -> ChecksumDetectionModel {
        match self.kind {
            ChecksumAlgorithmKind::Crc32c => ChecksumDetectionModel::crc32c_physical_bytes(),
            ChecksumAlgorithmKind::Crc64Nvme => ChecksumDetectionModel::crc64_nvme_physical_bytes(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum ChecksumAlgorithmKind {
    Crc32c,
    Crc64Nvme,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChecksumAlgorithmClaim<'a> {
    Declared(ChecksumAlgorithmId),
    DeclaredText(&'a str),
    ArtifactDigest,
    AuthenticityClaim,
}

impl<'a> ChecksumAlgorithmClaim<'a> {
    pub const fn declared_text(algorithm_id: &'a str) -> Self {
        Self::DeclaredText(algorithm_id)
    }

    pub const fn artifact_digest_substitution(_digest: &'a StableDigest) -> Self {
        Self::ArtifactDigest
    }

    pub const fn checksum_as_authenticity_claim() -> Self {
        Self::AuthenticityClaim
    }
}
