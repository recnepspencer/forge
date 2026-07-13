use crate::blob_basis::BlobIdentityKeyBasis;

pub(crate) fn blob_identity_digest_bytes(identity: &BlobIdentityKeyBasis) -> &[u8] {
    identity.object_digest().as_str().as_bytes()
}
