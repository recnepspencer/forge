use sha2::{Digest, Sha256};
use worth_relational::facade::identity::{EntityId, KindId, RelationId};
use worth_relational::facade::storage::authoritative_aspect_value_field_comparison_key;

use super::observations::{
    WorthQueryPrincipalMappingObservation, WorthQueryPrincipalTargetObservation,
};

pub(super) fn principal_binding_freshness_digest(
    binding: &str,
    mapping: &WorthQueryPrincipalMappingObservation,
    target: &WorthQueryPrincipalTargetObservation,
) -> [u8; 32] {
    let mut hash = Sha256::new();
    hash.update(b"worth-query-application-principal-freshness-v1");
    encode_bytes(&mut hash, binding.as_bytes());
    encode_entity_id(&mut hash, mapping.entity_id);
    encode_kind_id(&mut hash, mapping.kind_id);
    let identity = authoritative_aspect_value_field_comparison_key(&mapping.identity);
    encode_bytes(&mut hash, identity.canonical_value_bytes());
    hash.update([u8::from(mapping.enabled)]);
    encode_relation_id(&mut hash, target.relation_id);
    encode_kind_id(&mut hash, target.relation_kind);
    encode_entity_id(&mut hash, target.source);
    encode_entity_id(&mut hash, target.target);
    encode_kind_id(&mut hash, target.principal_kind);
    let principal_identity =
        authoritative_aspect_value_field_comparison_key(&target.principal_identity);
    encode_bytes(&mut hash, principal_identity.canonical_value_bytes());
    hash.finalize().into()
}

fn encode_entity_id(hash: &mut Sha256, identity: EntityId) {
    hash.update(identity.partition_value().to_le_bytes());
    hash.update(identity.local_slot_value().to_le_bytes());
    hash.update(identity.generation_value().to_le_bytes());
}

fn encode_relation_id(hash: &mut Sha256, identity: RelationId) {
    hash.update(identity.partition_value().to_le_bytes());
    hash.update(identity.local_slot_value().to_le_bytes());
    hash.update(identity.generation_value().to_le_bytes());
}

fn encode_kind_id(hash: &mut Sha256, identity: KindId) {
    hash.update(identity.as_u32().to_le_bytes());
}

fn encode_bytes(hash: &mut Sha256, value: &[u8]) {
    let length = u64::try_from(value.len())
        .expect("primary graph identity components are bounded below u64::MAX bytes");
    hash.update(length.to_le_bytes());
    hash.update(value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_sequence_length_prefix_is_fixed_width_and_canonical() {
        let mut hash = Sha256::new();
        encode_bytes(&mut hash, b"abc");
        let digest: [u8; 32] = hash.finalize().into();
        assert_eq!(
            hex(&digest),
            "ce91dc5eec0139adf091900d225971d6ad246a845bad791b5693a9d0d55dd391"
        );
    }

    fn hex(bytes: &[u8]) -> String {
        bytes.iter().map(|byte| format!("{byte:02x}")).collect()
    }
}
