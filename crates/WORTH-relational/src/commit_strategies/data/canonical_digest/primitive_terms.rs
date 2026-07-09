use sha2::{Digest, Sha256};

use crate::identity::data::{EntityId, KindId, PartitionId, RelationId, VersionId};
use crate::symbols::data::ClientKey;
use worth_foundational::facade::AspectFieldLocator;

pub(super) struct StrategyDigestBytes {
    bytes: Vec<u8>,
}

impl StrategyDigestBytes {
    pub(super) fn digest(domain: &'static str, fill: impl FnOnce(&mut Self)) -> [u8; 32] {
        let mut bytes = Self::new(domain);
        fill(&mut bytes);
        Sha256::digest(bytes.bytes).into()
    }

    pub(super) fn hex_digest(domain: &'static str, fill: impl FnOnce(&mut Self)) -> String {
        let digest = Self::digest(domain, fill);
        digest.iter().map(|byte| format!("{byte:02x}")).collect()
    }

    fn new(domain: &'static str) -> Self {
        let mut bytes = Self { bytes: Vec::new() };
        bytes.string(domain);
        bytes
    }

    pub(super) fn tag(&mut self, tag: u8) {
        self.bytes.push(tag);
    }

    pub(super) fn bool(&mut self, value: bool) {
        self.tag(u8::from(value));
    }

    pub(super) fn u16(&mut self, value: u16) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub(super) fn u32(&mut self, value: u32) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub(super) fn u64(&mut self, value: u64) {
        self.bytes.extend_from_slice(&value.to_le_bytes());
    }

    pub(super) fn usize(&mut self, value: usize) {
        self.u64(value as u64);
    }

    pub(super) fn string(&mut self, value: &str) {
        self.u32(value.len() as u32);
        self.bytes.extend_from_slice(value.as_bytes());
    }

    pub(super) fn bytes(&mut self, value: &[u8]) {
        self.u32(value.len() as u32);
        self.bytes.extend_from_slice(value);
    }

    pub(super) fn digest_bytes(&mut self, value: &[u8; 32]) {
        self.bytes.extend_from_slice(value);
    }

    pub(super) fn optional<T>(&mut self, value: Option<T>, write: impl FnOnce(&mut Self, T)) {
        match value {
            Some(value) => {
                self.tag(1);
                write(self, value);
            }
            None => self.tag(0),
        }
    }

    pub(super) fn partition_id(&mut self, value: PartitionId) {
        self.u32(value.as_u32());
    }

    pub(super) fn kind_id(&mut self, value: KindId) {
        self.u32(value.as_u32());
    }

    pub(super) fn version_id(&mut self, value: VersionId) {
        self.u64(value.as_u64());
    }

    pub(super) fn entity_id(&mut self, value: EntityId) {
        self.partition_id(value.partition_id);
        self.u64(value.local_slot_value());
        self.u32(value.generation_value());
    }

    pub(super) fn relation_id(&mut self, value: RelationId) {
        self.partition_id(value.partition_id);
        self.u64(value.local_slot_value());
        self.u32(value.generation_value());
    }

    pub(super) fn client_key(&mut self, value: &ClientKey) {
        if let Some(raw) = value.as_raw_str() {
            self.tag(1);
            self.string(raw);
        } else if let Some(symbol) = value.as_symbol() {
            self.tag(2);
            self.u32(symbol.0);
        } else {
            self.tag(3);
            self.string(value.canonical_text().as_ref());
        }
    }

    pub(super) fn aspect_field_locator(&mut self, locator: &AspectFieldLocator) {
        self.bytes(&crate::aspect_wire::encode_aspect_field_locator(locator));
    }
}
