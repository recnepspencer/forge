use sha2::{Digest, Sha256};

use crate::{OperationalTruthRegion, OperationalTruthReport};

use super::StagedRecoveryPostVerificationDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum RegionPostureClass {
    Trusted,
    Degraded,
    Rebuildable,
    Quarantined,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StagedRecoveryRegionPosture {
    identity: [u8; 32],
    count: u64,
}

impl StagedRecoveryRegionPosture {
    const EMPTY: Self = Self {
        identity: [0; 32],
        count: 0,
    };

    pub const fn identity(self) -> [u8; 32] {
        self.identity
    }

    pub const fn count(self) -> u64 {
        self.count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StagedRecoveryAuthorityPosture {
    trusted: StagedRecoveryRegionPosture,
    degraded: StagedRecoveryRegionPosture,
    rebuildable: StagedRecoveryRegionPosture,
    quarantined: StagedRecoveryRegionPosture,
    unavailable: StagedRecoveryRegionPosture,
}

impl StagedRecoveryAuthorityPosture {
    pub(super) fn from_truth(
        truth: &OperationalTruthReport,
    ) -> Result<Self, StagedRecoveryPostVerificationDenial> {
        let mut classified = Vec::new();
        classified
            .try_reserve_exact(truth.regions().len())
            .map_err(|_| StagedRecoveryPostVerificationDenial::AllocationFailed)?;
        for region in truth.regions() {
            classified.push((class(region), region_identity(region)));
        }
        classified.sort_unstable();
        Ok(Self {
            trusted: set(&classified, RegionPostureClass::Trusted),
            degraded: set(&classified, RegionPostureClass::Degraded),
            rebuildable: set(&classified, RegionPostureClass::Rebuildable),
            quarantined: set(&classified, RegionPostureClass::Quarantined),
            unavailable: set(&classified, RegionPostureClass::Unavailable),
        })
    }

    pub const fn trusted(self) -> StagedRecoveryRegionPosture {
        self.trusted
    }

    pub const fn degraded(self) -> StagedRecoveryRegionPosture {
        self.degraded
    }

    pub const fn rebuildable(self) -> StagedRecoveryRegionPosture {
        self.rebuildable
    }

    pub const fn quarantined(self) -> StagedRecoveryRegionPosture {
        self.quarantined
    }

    pub const fn unavailable(self) -> StagedRecoveryRegionPosture {
        self.unavailable
    }
}

const fn class(region: &OperationalTruthRegion) -> RegionPostureClass {
    match region {
        OperationalTruthRegion::TrustedAuthorityRegion(_) => RegionPostureClass::Trusted,
        OperationalTruthRegion::DegradedDerivedRegion(_) => RegionPostureClass::Degraded,
        OperationalTruthRegion::RebuildableRegion(_) => RegionPostureClass::Rebuildable,
        OperationalTruthRegion::QuarantinedRegion(_) => RegionPostureClass::Quarantined,
        OperationalTruthRegion::UnrecoverableAuthorityRegion(_)
        | OperationalTruthRegion::IndeterminateTruthRegion(_)
        | OperationalTruthRegion::AliasGroup { .. }
        | OperationalTruthRegion::OverlapConflict { .. } => RegionPostureClass::Unavailable,
    }
}

fn region_identity(region: &OperationalTruthRegion) -> [u8; 32] {
    let evidence = region.evidence();
    let (start, end) = evidence.range();
    let mut digest = Sha256::new();
    digest.update(b"worth-store-staged-authority-region-v1");
    digest.update([class(region) as u8]);
    digest.update(evidence.source().as_os_str().to_string_lossy().as_bytes());
    digest.update(start.to_be_bytes());
    digest.update(end.to_be_bytes());
    digest.update(evidence.content_digest());
    digest.finalize().into()
}

fn set(
    classified: &[(RegionPostureClass, [u8; 32])],
    expected: RegionPostureClass,
) -> StagedRecoveryRegionPosture {
    let mut count = 0_u64;
    let mut digest = Sha256::new();
    digest.update(b"worth-store-staged-authority-region-set-v1");
    digest.update([expected as u8]);
    for (_, identity) in classified.iter().filter(|(class, _)| *class == expected) {
        count += 1;
        digest.update(identity);
    }
    if count == 0 {
        StagedRecoveryRegionPosture::EMPTY
    } else {
        StagedRecoveryRegionPosture {
            identity: digest.finalize().into(),
            count,
        }
    }
}
