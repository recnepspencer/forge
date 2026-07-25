#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalWorkEvidenceDigest([u8; 32]);

impl PhysicalWorkEvidenceDigest {
    pub fn new(bytes: [u8; 32]) -> Option<Self> {
        if bytes == [0; 32] {
            None
        } else {
            Some(Self(bytes))
        }
    }

    pub const fn bytes(self) -> [u8; 32] {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkArtifactBinding {
    path: Box<str>,
    byte_length: u64,
    digest: PhysicalWorkEvidenceDigest,
}

impl PhysicalWorkArtifactBinding {
    pub fn new(
        path: impl Into<Box<str>>,
        byte_length: u64,
        digest: PhysicalWorkEvidenceDigest,
    ) -> Result<Self, PhysicalWorkEvidenceBindingDenial> {
        let path = path.into();
        require_text(&path, PhysicalWorkEvidenceBindingDenial::EmptyArtifactPath)?;
        Ok(Self {
            path,
            byte_length,
            digest,
        })
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub const fn byte_length(&self) -> u64 {
        self.byte_length
    }

    pub const fn digest(&self) -> PhysicalWorkEvidenceDigest {
        self.digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkSourceBinding {
    path: Box<str>,
    digest: PhysicalWorkEvidenceDigest,
}

impl PhysicalWorkSourceBinding {
    pub fn new(
        path: impl Into<Box<str>>,
        digest: PhysicalWorkEvidenceDigest,
    ) -> Result<Self, PhysicalWorkEvidenceBindingDenial> {
        let path = path.into();
        require_text(&path, PhysicalWorkEvidenceBindingDenial::EmptySourcePath)?;
        Ok(Self { path, digest })
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub const fn digest(&self) -> PhysicalWorkEvidenceDigest {
        self.digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalWorkOracleEvidence {
    oracle: Box<str>,
    accepted: bool,
    digest: PhysicalWorkEvidenceDigest,
}

impl PhysicalWorkOracleEvidence {
    pub fn new(
        oracle: impl Into<Box<str>>,
        accepted: bool,
        digest: PhysicalWorkEvidenceDigest,
    ) -> Result<Self, PhysicalWorkEvidenceBindingDenial> {
        let oracle = oracle.into();
        require_text(&oracle, PhysicalWorkEvidenceBindingDenial::EmptyOracle)?;
        Ok(Self {
            oracle,
            accepted,
            digest,
        })
    }

    pub fn oracle(&self) -> &str {
        &self.oracle
    }

    pub const fn accepted(&self) -> bool {
        self.accepted
    }

    pub const fn digest(&self) -> PhysicalWorkEvidenceDigest {
        self.digest
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalWorkEvidenceBindingDenial {
    EmptySourcePath,
    EmptyArtifactPath,
    EmptyOracle,
    EmptyMutantPredicate,
    EmptyMutantSource,
    EmptyMutantProfile,
    EmptyMutantScenario,
    EmptyMutantLocalization,
}

pub(super) fn require_text(
    value: &str,
    denial: PhysicalWorkEvidenceBindingDenial,
) -> Result<(), PhysicalWorkEvidenceBindingDenial> {
    if value.trim().is_empty() {
        Err(denial)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PhysicalWorkEvidenceBindingDenial, PhysicalWorkEvidenceDigest, PhysicalWorkSourceBinding,
    };

    fn digest() -> PhysicalWorkEvidenceDigest {
        PhysicalWorkEvidenceDigest::new([7; 32]).unwrap()
    }

    #[test]
    fn zero_digest_and_empty_source_cannot_enter_evidence() {
        assert!(PhysicalWorkEvidenceDigest::new([0; 32]).is_none());
        assert_eq!(
            PhysicalWorkSourceBinding::new(" ", digest()),
            Err(PhysicalWorkEvidenceBindingDenial::EmptySourcePath)
        );
    }
}
