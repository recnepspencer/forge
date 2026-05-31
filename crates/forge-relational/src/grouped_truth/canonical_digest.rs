use forge_foundational::facade::{AspectKey, AspectValue};
use forge_runtime_bridge::facade::TruthSnapshotIdentity;
use sha2::{Digest, Sha256};

use super::grouped_projection::{
    GroupedProjectionContract, RelationalGroupedMemberRow, RelationalGroupedProjectionDigest,
};
use super::row_set::{RelationalAuthoritativeRowArtifact, RelationalRowSetDigest};

pub(super) fn row_set_digest(
    snapshot_identity: &TruthSnapshotIdentity,
    rows: &[RelationalAuthoritativeRowArtifact],
) -> RelationalRowSetDigest {
    let mut bytes = Vec::new();
    bytes.push(1);
    encode_string(&mut bytes, snapshot_identity.as_str());
    for row in rows {
        bytes.push(2);
        encode_string(&mut bytes, row.row_identity().as_str());
        for (aspect_key, value) in row.aspect_values() {
            encode_aspect_entry(&mut bytes, aspect_key, value);
        }
    }
    RelationalRowSetDigest::from_canonical_bytes(&bytes)
}

pub(super) fn grouped_projection_digest(
    row_set_digest: &RelationalRowSetDigest,
    snapshot_identity: &TruthSnapshotIdentity,
    contract: &GroupedProjectionContract,
    members: &[RelationalGroupedMemberRow],
) -> RelationalGroupedProjectionDigest {
    let mut bytes = Vec::new();
    bytes.push(16);
    encode_string(&mut bytes, row_set_digest.as_str());
    encode_string(&mut bytes, snapshot_identity.as_str());
    encode_string(&mut bytes, contract.grouping_aspect().as_str());
    encode_string(&mut bytes, contract.identity_binding_aspect_key().as_str());
    encode_string(&mut bytes, contract.grouping_binding_aspect_key().as_str());
    for member in members {
        bytes.push(17);
        encode_string(&mut bytes, member.row_identity().as_str());
        encode_aspect_value(&mut bytes, member.identity_value());
        encode_aspect_value(&mut bytes, member.grouping_value());
    }
    RelationalGroupedProjectionDigest::from_canonical_bytes(&bytes)
}

fn encode_aspect_entry(bytes: &mut Vec<u8>, aspect_key: &AspectKey, value: &AspectValue) {
    bytes.push(3);
    encode_string(bytes, aspect_key.as_str());
    encode_aspect_value(bytes, value)
}

fn encode_aspect_value(bytes: &mut Vec<u8>, value: &AspectValue) {
    let value_bytes = crate::aspect_wire::encode_aspect_value(value);
    encode_length_prefixed_bytes(bytes, &value_bytes);
}

fn encode_string(bytes: &mut Vec<u8>, value: &str) {
    encode_length_prefixed_bytes(bytes, value.as_bytes());
}

fn encode_length_prefixed_bytes(bytes: &mut Vec<u8>, value: &[u8]) {
    crate::aspect_wire::encode_u32(bytes, value.len() as u32);
    bytes.extend_from_slice(value);
}

pub(super) fn digest_with_prefix(prefix: &str, bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    format!("{prefix}:sha256:{digest:x}")
}
