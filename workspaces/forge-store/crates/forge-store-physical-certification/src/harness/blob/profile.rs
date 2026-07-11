use forge_store_blob_chunks::{BlobHarnessChunkSizeClass, BlobHarnessSizeClass};
use forge_store_budgets::BlobHarnessEnvelopeDeclaration;

use crate::PhysicalSimulationProfile;

use super::foundational_profile::BlobHarnessMaterializedProfile;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlobHarnessProfile {
    Local,
    CiMemoryEnvelopeExceeding,
    HeavyMultiGb,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlobHarnessProfileSet {
    profiles: Vec<BlobHarnessProfile>,
}

impl BlobHarnessProfile {
    pub const fn local() -> Self {
        Self::Local
    }

    pub const fn ci_memory_envelope_exceeding() -> Self {
        Self::CiMemoryEnvelopeExceeding
    }

    pub const fn heavy_multi_gb() -> Self {
        Self::HeavyMultiGb
    }

    pub const fn physical_profile(self) -> PhysicalSimulationProfile {
        match self {
            Self::Local => PhysicalSimulationProfile::DeveloperSmoke,
            Self::CiMemoryEnvelopeExceeding => PhysicalSimulationProfile::CiCertification,
            Self::HeavyMultiGb => PhysicalSimulationProfile::ReleaseCertification,
        }
    }

    pub const fn size_class(self) -> BlobHarnessSizeClass {
        match self {
            Self::Local => BlobHarnessSizeClass::LocalDeterministic,
            Self::CiMemoryEnvelopeExceeding => BlobHarnessSizeClass::MemoryEnvelopeExceeding,
            Self::HeavyMultiGb => BlobHarnessSizeClass::HeavyMultiGbDeclared,
        }
    }

    pub const fn chunk_size_class(self) -> BlobHarnessChunkSizeClass {
        match self {
            Self::Local => BlobHarnessChunkSizeClass::Fixed64KiB,
            Self::CiMemoryEnvelopeExceeding => BlobHarnessChunkSizeClass::Fixed64KiB,
            Self::HeavyMultiGb => BlobHarnessChunkSizeClass::Fixed8MiB,
        }
    }

    pub const fn envelope(self) -> BlobHarnessEnvelopeDeclaration {
        match self {
            Self::Local => BlobHarnessEnvelopeDeclaration::local(),
            Self::CiMemoryEnvelopeExceeding => {
                BlobHarnessEnvelopeDeclaration::ci_memory_envelope_exceeding()
            }
            Self::HeavyMultiGb => BlobHarnessEnvelopeDeclaration::heavy_multi_gb(),
        }
    }

    pub fn materialize_foundational_profile(self) -> BlobHarnessMaterializedProfile {
        BlobHarnessMaterializedProfile::for_blob_profile(self)
    }
}

impl BlobHarnessProfileSet {
    pub fn required_qualification_profiles() -> Self {
        Self {
            profiles: vec![
                BlobHarnessProfile::Local,
                BlobHarnessProfile::CiMemoryEnvelopeExceeding,
                BlobHarnessProfile::HeavyMultiGb,
            ],
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = BlobHarnessProfile> + '_ {
        self.profiles.iter().copied()
    }
}
