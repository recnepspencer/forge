use worth_query_declaration::facade::application_schema::ApplicationSchemaBindingIdentity;

use crate::application_query::WorthQueryInstalledApplicationQueryIdentity;
use crate::authority_cryptography::{
    AuthoritySeal, AuthoritySealDomain, AuthorityTranscript, PackageAuthorityKey,
};
use crate::graph_obligation::WorthQueryInstalledGraphObligationSetIdentity;

pub(super) fn derive_installed_query_authority_seal(
    key: &PackageAuthorityKey,
    binding: &ApplicationSchemaBindingIdentity,
    query_identity: &WorthQueryInstalledApplicationQueryIdentity,
    obligations: &WorthQueryInstalledGraphObligationSetIdentity,
) -> AuthoritySeal {
    authority_transcript(key, binding, query_identity, obligations).finish()
}

fn authority_transcript(
    key: &PackageAuthorityKey,
    binding: &ApplicationSchemaBindingIdentity,
    query_identity: &WorthQueryInstalledApplicationQueryIdentity,
    obligations: &WorthQueryInstalledGraphObligationSetIdentity,
) -> AuthorityTranscript {
    let mut transcript =
        AuthorityTranscript::new(key, AuthoritySealDomain::InstalledApplicationQuery);
    transcript.bytes("package", binding.package_identity().bytes());
    transcript.bytes("schema", binding.schema_identity().bytes());
    transcript.bytes("query", query_identity.as_bytes());
    transcript.bytes("graph-obligations", obligations.bytes());
    transcript
}
