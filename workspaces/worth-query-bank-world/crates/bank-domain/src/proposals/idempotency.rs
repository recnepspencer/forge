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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BankIdempotencyKeyIdentity([u8; 32]);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct BankIdempotencyClaim {
    key: BankIdempotencyKeyIdentity,
    intent: BankIdempotencyIntent,
}

impl BankIdempotencyClaim {
    pub(crate) fn derive(
        binding: BankOperationScopeBinding,
        key: &BankIdempotencyKey,
        operation: &'static str,
        payload: &[u8],
    ) -> Self {
        let mut key_hasher = Sha256::new();
        hash_part(&mut key_hasher, b"WORTH.bank.idempotency-key-identity.v1");
        hash_part(&mut key_hasher, &binding.0);
        hash_part(&mut key_hasher, operation.as_bytes());
        hash_part(&mut key_hasher, key.as_str().as_bytes());
        let key = BankIdempotencyKeyIdentity(key_hasher.finalize().into());
        let mut hasher = Sha256::new();
        hash_part(&mut hasher, b"WORTH.bank.idempotency-intent.v1");
        hash_part(&mut hasher, &key.0);
        hash_part(&mut hasher, payload);
        Self {
            key,
            intent: BankIdempotencyIntent(hasher.finalize().into()),
        }
    }

    pub const fn key(self) -> BankIdempotencyKeyIdentity {
        self.key
    }

    pub const fn intent(self) -> BankIdempotencyIntent {
        self.intent
    }
}

impl BankIdempotencyIntent {
    pub const fn bytes(self) -> [u8; 32] {
        self.0
    }

    pub(crate) fn canonical_text(self) -> String {
        canonical_hex(self.0)
    }

    pub(crate) fn from_canonical_text(value: &str) -> Option<Self> {
        decode_hex(value).map(Self)
    }
}

impl BankIdempotencyKeyIdentity {
    pub const fn bytes(self) -> [u8; 32] {
        self.0
    }

    pub(crate) fn canonical_text(self) -> String {
        canonical_hex(self.0)
    }

    pub(crate) fn from_canonical_text(value: &str) -> Option<Self> {
        decode_hex(value).map(Self)
    }
}

fn hash_part(hasher: &mut Sha256, bytes: &[u8]) {
    hasher.update((bytes.len() as u64).to_be_bytes());
    hasher.update(bytes);
}

fn canonical_hex(bytes: [u8; 32]) -> String {
    use std::fmt::Write;

    let mut encoded = String::with_capacity(64);
    for byte in bytes {
        write!(&mut encoded, "{byte:02x}").expect("writing to String cannot fail");
    }
    encoded
}

fn decode_hex(value: &str) -> Option<[u8; 32]> {
    if value.len() != 64 {
        return None;
    }
    let mut decoded = [0_u8; 32];
    for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
        decoded[index] = (hex_nibble(pair[0])? << 4) | hex_nibble(pair[1])?;
    }
    Some(decoded)
}

const fn hex_nibble(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        _ => None,
    }
}
