use crate::data::identity::{NamingAnchorId, SpecNodeId, SpecRelationId};
use crate::data::schema::{RelationKind, SpecNodeKind};

#[derive(Debug, Clone)]
pub struct DeterministicIdAllocator {
    namespace: u128,
    next_sequence: u64,
}

impl DeterministicIdAllocator {
    pub fn new(namespace: u128, next_sequence: u64) -> Self {
        Self {
            namespace,
            next_sequence,
        }
    }

    pub fn next_sequence(&self) -> u64 {
        self.next_sequence
    }

    pub fn mint_node_id(&mut self, kind: SpecNodeKind, role: &str) -> SpecNodeId {
        let id = SpecNodeId::new(stable_id(
            self.namespace,
            self.bump_sequence(),
            kind as u16,
            role.as_bytes(),
        ));
        id
    }

    pub fn mint_relation_id(&mut self, kind: RelationKind, role: &str) -> SpecRelationId {
        let id = SpecRelationId::new(stable_id(
            self.namespace,
            self.bump_sequence(),
            10_000 + kind as u16,
            role.as_bytes(),
        ));
        id
    }

    pub fn mint_anchor_id(&mut self, role: &str) -> NamingAnchorId {
        let id = NamingAnchorId::new(stable_id(
            self.namespace,
            self.bump_sequence(),
            20_000,
            role.as_bytes(),
        ));
        id
    }

    fn bump_sequence(&mut self) -> u64 {
        let current = self.next_sequence;
        self.next_sequence += 1;
        current
    }
}

fn stable_id(namespace: u128, sequence: u64, category: u16, role: &[u8]) -> u128 {
    let mut hi = 0xcbf29ce484222325u64;
    let mut lo = 0x9e3779b97f4a7c15u64;

    for byte in namespace.to_be_bytes() {
        hi ^= byte as u64;
        hi = hi.wrapping_mul(0x100000001b3);
        lo ^= (byte as u64).rotate_left(1);
        lo = lo.wrapping_mul(0x100000001b3);
    }
    for byte in sequence.to_be_bytes() {
        hi ^= byte as u64;
        hi = hi.wrapping_mul(0x100000001b3);
        lo ^= (byte as u64).rotate_left(1);
        lo = lo.wrapping_mul(0x100000001b3);
    }
    for byte in category.to_be_bytes() {
        hi ^= byte as u64;
        hi = hi.wrapping_mul(0x100000001b3);
        lo ^= (byte as u64).rotate_left(1);
        lo = lo.wrapping_mul(0x100000001b3);
    }
    for &byte in role {
        hi ^= byte as u64;
        hi = hi.wrapping_mul(0x100000001b3);
        lo ^= (byte as u64).rotate_left(1);
        lo = lo.wrapping_mul(0x100000001b3);
    }

    ((hi as u128) << 64) | lo as u128
}
