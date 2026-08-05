use worth_foundational::facade::{CanonicalDerivedDigest, CanonicalDigestWorkEvidence};

/// Application-owned identity of effect-integrity input that Query binds to
/// idempotency without treating it as capability authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ApplicationCapabilityGovernedInputIdentity {
    identity: [u8; 32],
    canonical_work: Option<CanonicalDigestWorkEvidence>,
}

impl ApplicationCapabilityGovernedInputIdentity {
    /// Retains an injectively encoded fixed-width identity with no digest work.
    pub fn four_u64(values: [u64; 4]) -> Self {
        let mut identity = [0; 32];
        for (slot, value) in values.into_iter().enumerate() {
            let start = slot * 8;
            identity[start..start + 8].copy_from_slice(&value.to_be_bytes());
        }
        Self {
            identity,
            canonical_work: None,
        }
    }

    /// Retains both a Foundational canonical digest and its exact work.
    pub fn canonical(derived: &CanonicalDerivedDigest) -> Self {
        Self {
            identity: *derived.value().bytes(),
            canonical_work: Some(derived.metadata().work()),
        }
    }

    pub const fn identity(self) -> [u8; 32] {
        self.identity
    }

    pub const fn canonical_work(self) -> Option<CanonicalDigestWorkEvidence> {
        self.canonical_work
    }
}
