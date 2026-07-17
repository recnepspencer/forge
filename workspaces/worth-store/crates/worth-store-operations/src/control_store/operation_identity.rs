#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OperationalOperationId(String);

impl OperationalOperationId {
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidOperationalIdentity> {
        validated(value.into()).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn stable_fingerprint(&self) -> [u8; 32] {
        use sha2::{Digest, Sha256};

        let mut digest = Sha256::new();
        digest.update(b"worth-store-operational-operation-id-v1");
        digest.update((self.0.len() as u64).to_be_bytes());
        digest.update(self.0.as_bytes());
        digest.finalize().into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperationalTransitionId(String);

impl OperationalTransitionId {
    pub fn new(value: impl Into<String>) -> Result<Self, InvalidOperationalIdentity> {
        validated(value.into()).map(Self)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn recovery_staging_completed() -> Self {
        Self("recovery-staging-completed".to_owned())
    }

    pub(crate) fn recovery_publication_published() -> Self {
        Self("recovery-publication-published".to_owned())
    }

    pub(crate) fn recovery_publication_fence_released() -> Self {
        Self("recovery-publication-fence-released".to_owned())
    }

    pub(crate) fn repair_recovery_abandoned() -> Self {
        Self("repair-recovery-abandoned".to_owned())
    }

    pub(crate) fn repair_recovery_isolated() -> Self {
        Self("repair-recovery-isolated".to_owned())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidOperationalIdentity;

fn validated(value: String) -> Result<String, InvalidOperationalIdentity> {
    if value.trim().is_empty() || value.len() > 512 || value.chars().any(char::is_control) {
        Err(InvalidOperationalIdentity)
    } else {
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{OperationalOperationId, OperationalTransitionId};

    #[test]
    fn control_characters_cannot_alias_the_durable_transition_delimiter() {
        assert!(OperationalOperationId::new("operation\0transition").is_err());
        assert!(OperationalTransitionId::new("transition\0suffix").is_err());
        assert!(OperationalOperationId::new("operation\nnext").is_err());
        assert!(OperationalTransitionId::new("ordinary-transition").is_ok());
    }
}
