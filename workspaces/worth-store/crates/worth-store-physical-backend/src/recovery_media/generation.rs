#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalRecoveryMediaGeneration([u8; 16]);

impl PhysicalRecoveryMediaGeneration {
    pub(crate) const fn from_owner_attempt(identity: [u8; 16]) -> Self {
        Self(identity)
    }
}
