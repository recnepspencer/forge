use worth_query_declaration::facade::application_schema::ApplicationSchemaBindingIdentity;

use crate::application_query::WorthQueryInstalledApplicationQueryIdentity;
use crate::authority_cryptography::{
    AuthoritySeal, AuthoritySealDomain, AuthorityTranscript, PackageAuthorityKey,
};

pub(super) fn derive_installed_query_authority_seal(
    key: &PackageAuthorityKey,
    binding: &ApplicationSchemaBindingIdentity,
    query_identity: &WorthQueryInstalledApplicationQueryIdentity,
) -> AuthoritySeal {
    authority_transcript(key, binding, query_identity).finish()
}

pub(super) fn verify_installed_query_authority_seal(
    seal: &AuthoritySeal,
    key: &PackageAuthorityKey,
    binding: &ApplicationSchemaBindingIdentity,
    query_identity: &WorthQueryInstalledApplicationQueryIdentity,
) -> bool {
    authority_transcript(key, binding, query_identity).verifies(seal)
}

fn authority_transcript(
    key: &PackageAuthorityKey,
    binding: &ApplicationSchemaBindingIdentity,
    query_identity: &WorthQueryInstalledApplicationQueryIdentity,
) -> AuthorityTranscript {
    let mut transcript =
        AuthorityTranscript::new(key, AuthoritySealDomain::InstalledApplicationQuery);
    transcript.bytes("package", binding.package_identity().bytes());
    transcript.bytes("schema", binding.schema_identity().bytes());
    transcript.bytes("query", query_identity.as_bytes());
    transcript
}
