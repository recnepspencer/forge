use worth_query_declaration::facade::application_schema::ApplicationSchemaBindingIdentity;

use crate::authority_cryptography::{
    AuthoritySeal, AuthoritySealDomain, AuthorityTranscript, PackageAuthorityKey,
};

pub(super) fn derive_capability_authority_seal(
    key: &PackageAuthorityKey,
    binding: &ApplicationSchemaBindingIdentity,
    capability_identity: &[u8; 32],
    capability_type: &str,
    operation: &str,
    operation_type: &str,
    input_type: &str,
) -> AuthoritySeal {
    authority_transcript(
        key,
        binding,
        capability_identity,
        capability_type,
        operation,
        operation_type,
        input_type,
    )
    .finish()
}

pub(super) fn verify_capability_authority_seal(
    seal: &AuthoritySeal,
    key: &PackageAuthorityKey,
    binding: &ApplicationSchemaBindingIdentity,
    capability_identity: &[u8; 32],
    capability_type: &str,
    operation: &str,
    operation_type: &str,
    input_type: &str,
) -> bool {
    authority_transcript(
        key,
        binding,
        capability_identity,
        capability_type,
        operation,
        operation_type,
        input_type,
    )
    .verifies(seal)
}

fn authority_transcript(
    key: &PackageAuthorityKey,
    binding: &ApplicationSchemaBindingIdentity,
    capability_identity: &[u8; 32],
    capability_type: &str,
    operation: &str,
    operation_type: &str,
    input_type: &str,
) -> AuthorityTranscript {
    let mut transcript =
        AuthorityTranscript::new(key, AuthoritySealDomain::InstalledApplicationCapability);
    transcript.bytes("package", binding.package_identity().bytes());
    transcript.bytes("schema", binding.schema_identity().bytes());
    transcript.bytes("capability", capability_identity);
    transcript.text("capability-type", capability_type);
    transcript.text("operation", operation);
    transcript.text("operation-type", operation_type);
    transcript.text("input-type", input_type);
    transcript
}
