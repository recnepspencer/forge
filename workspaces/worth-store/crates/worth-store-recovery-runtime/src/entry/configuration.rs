use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalRecoveryStaticConfiguration {
    identity: [u8; 32],
}

impl PhysicalRecoveryStaticConfiguration {
    pub fn current() -> Self {
        let mut digest = Sha256::new();
        digest.update(b"worth.store.physical.recovery.configuration@1");
        Self {
            identity: digest.finalize().into(),
        }
    }

    pub(crate) const fn identity(&self) -> [u8; 32] {
        self.identity
    }
}
