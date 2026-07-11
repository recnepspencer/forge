use crate::blob_basis::S8BlobIdentityKeyBasis;

pub(crate) fn blob_identity_digest_bytes(identity: &S8BlobIdentityKeyBasis) -> &[u8] {
    identity.object_digest().as_str().as_bytes()
}
