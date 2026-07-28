use sha2::{Digest, Sha256};

use super::BankProposalDenial;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct BankIdempotencyKey(String);

impl BankIdempotencyKey {
    pub fn new(value: impl Into<String>) -> Result<Self, BankProposalDenial> {
        let value = value.into();
        if value.is_empty()
            || value.len() > 128
            || !value.bytes().all(|byte| byte.is_ascii_graphic())
        {
            return Err(BankProposalDenial::InvalidIdempotencyKey);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BankOperationScopeBinding([u8; 32]);

impl BankOperationScopeBinding {
    /// Carries a descriptive Query operation-scope fingerprint into pure bank
    /// proposal semantics. It is not itself an authorization proof.
    pub const fn from_fingerprint_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub const fn bytes(self) -> [u8; 32] {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BankIdempotencyIntent([u8; 32]);

impl BankIdempotencyIntent {
    pub(crate) fn derive(
        binding: BankOperationScopeBinding,
        key: &BankIdempotencyKey,
        operation: &'static str,
        payload: &[u8],
    ) -> Self {
        let mut hasher = Sha256::new();
        hash_part(&mut hasher, b"WORTH.bank.idempotency-intent.v1");
        hash_part(&mut hasher, &binding.0);
        hash_part(&mut hasher, key.as_str().as_bytes());
        hash_part(&mut hasher, operation.as_bytes());
        hash_part(&mut hasher, payload);
        Self(hasher.finalize().into())
    }

    pub const fn bytes(self) -> [u8; 32] {
        self.0
    }
}

fn hash_part(hasher: &mut Sha256, bytes: &[u8]) {
    hasher.update((bytes.len() as u64).to_be_bytes());
    hasher.update(bytes);
}
