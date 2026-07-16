use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryAuthorityRegionPosture {
    identity: [u8; 32],
    count: u64,
}

impl RecoveryAuthorityRegionPosture {
    pub fn observed(identity: [u8; 32], count: u64) -> Option<Self> {
        if (count == 0) != (identity == [0; 32]) {
            return None;
        }
        Some(Self { identity, count })
    }

    pub const fn identity(self) -> [u8; 32] {
        self.identity
    }

    pub const fn count(self) -> u64 {
        self.count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveryAuthorityAdmissionPosture {
    identity: [u8; 32],
    verification_identity: [u8; 32],
    trusted: RecoveryAuthorityRegionPosture,
    degraded: RecoveryAuthorityRegionPosture,
    rebuildable: RecoveryAuthorityRegionPosture,
    quarantined: RecoveryAuthorityRegionPosture,
    unavailable: RecoveryAuthorityRegionPosture,
}

impl RecoveryAuthorityAdmissionPosture {
    pub fn from_independent_post_verification(
        verification_identity: [u8; 32],
        regions: [RecoveryAuthorityRegionPosture; 5],
    ) -> Option<Self> {
        if verification_identity == [0; 32]
            || regions
                .iter()
                .try_fold(0_u64, |total, region| total.checked_add(region.count))?
                == 0
        {
            return None;
        }
        let [trusted, degraded, rebuildable, quarantined, unavailable] = regions;
        let mut digest = Sha256::new();
        digest.update(b"worth-store-recovery-authority-admission-posture-v1");
        digest.update(verification_identity);
        for region in regions {
            digest.update(region.identity);
            digest.update(region.count.to_be_bytes());
        }
        Some(Self {
            identity: digest.finalize().into(),
            verification_identity,
            trusted,
            degraded,
            rebuildable,
            quarantined,
            unavailable,
        })
    }

    pub const fn identity(self) -> [u8; 32] {
        self.identity
    }

    pub const fn verification_identity(self) -> [u8; 32] {
        self.verification_identity
    }

    pub const fn trusted(self) -> RecoveryAuthorityRegionPosture {
        self.trusted
    }

    pub const fn degraded(self) -> RecoveryAuthorityRegionPosture {
        self.degraded
    }

    pub const fn rebuildable(self) -> RecoveryAuthorityRegionPosture {
        self.rebuildable
    }

    pub const fn quarantined(self) -> RecoveryAuthorityRegionPosture {
        self.quarantined
    }

    pub const fn unavailable(self) -> RecoveryAuthorityRegionPosture {
        self.unavailable
    }

    pub const fn regions(self) -> [RecoveryAuthorityRegionPosture; 5] {
        [
            self.trusted,
            self.degraded,
            self.rebuildable,
            self.quarantined,
            self.unavailable,
        ]
    }

    pub const fn is_fully_trusted(self) -> bool {
        self.trusted.count() > 0
            && self.degraded.count() == 0
            && self.rebuildable.count() == 0
            && self.quarantined.count() == 0
            && self.unavailable.count() == 0
    }
}
