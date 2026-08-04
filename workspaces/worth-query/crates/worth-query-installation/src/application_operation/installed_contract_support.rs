use worth_query_declaration::facade::application_schema::{
    ApplicationSchema, ApplicationSchemaBindingIdentity,
};

use crate::application_schema::WorthQueryInstalledApplicationSchema;
use crate::authority_cryptography::{
    AuthoritySeal, AuthoritySealDomain, AuthorityTranscript, PackageAuthorityKey,
};
use crate::graph_obligation::{
    WorthQueryGraphObligationInstallationDenial, WorthQueryInstalledGraphCapabilityRequirement,
    WorthQueryInstalledGraphObligationSetIdentity,
};

use super::{
    WorthQueryApplicationOperationInstallationDenial,
    WorthQueryApplicationOperationInstallationDenialKind,
    WorthQueryInstalledApplicationOperationAuthorization,
};

pub(super) fn operation_authorization(
    operation: &str,
    ability_count: usize,
    capability_count: usize,
) -> Result<
    WorthQueryInstalledApplicationOperationAuthorization,
    WorthQueryApplicationOperationInstallationDenial,
> {
    match (ability_count > 0, capability_count > 0) {
        (true, true) => Err(operation_denial(
            WorthQueryApplicationOperationInstallationDenialKind::ConflictingAuthorizationContract,
            operation,
        )),
        (true, false) => Ok(WorthQueryInstalledApplicationOperationAuthorization::Abilities),
        (false, true) => Ok(WorthQueryInstalledApplicationOperationAuthorization::Capability),
        (false, false) => Ok(WorthQueryInstalledApplicationOperationAuthorization::Principal),
    }
}

pub(super) fn authority_identity(
    key: &PackageAuthorityKey,
    identity: &ApplicationSchemaBindingIdentity,
    operation: &str,
    input_type: &str,
    obligations: &WorthQueryInstalledGraphObligationSetIdentity,
) -> AuthoritySeal {
    authority_transcript(key, identity, operation, input_type, obligations).finish()
}

pub(super) fn authority_transcript(
    key: &PackageAuthorityKey,
    identity: &ApplicationSchemaBindingIdentity,
    operation: &str,
    input_type: &str,
    obligations: &WorthQueryInstalledGraphObligationSetIdentity,
) -> AuthorityTranscript {
    let mut transcript =
        AuthorityTranscript::new(key, AuthoritySealDomain::InstalledApplicationOperation);
    transcript.bytes("package", identity.package_identity().bytes());
    transcript.bytes("schema", identity.schema_identity().bytes());
    transcript.text("operation", operation);
    transcript.text("input-type", input_type);
    transcript.bytes("graph-obligations", obligations.bytes());
    transcript
}

pub(super) fn operation_capability_requirements<Schema>(
    schema: &WorthQueryInstalledApplicationSchema<Schema>,
    operation: &str,
    input_type: &str,
) -> Vec<WorthQueryInstalledGraphCapabilityRequirement>
where
    Schema: ApplicationSchema,
{
    schema
        .capability_registry
        .values()
        .filter(|compiled| {
            compiled.contract().operation() == operation
                && compiled.contract().input_type() == input_type
        })
        .map(|compiled| {
            WorthQueryInstalledGraphCapabilityRequirement::new(
                compiled.identity().clone(),
                compiled.contract().clone(),
            )
        })
        .collect()
}

pub(super) fn graph_obligation_denial(
    operation: &str,
    denial: WorthQueryGraphObligationInstallationDenial,
) -> WorthQueryApplicationOperationInstallationDenial {
    let kind = match denial {
        WorthQueryGraphObligationInstallationDenial::InvalidContract => {
            WorthQueryApplicationOperationInstallationDenialKind::InvalidGraphObligationContract
        }
        WorthQueryGraphObligationInstallationDenial::Canonical(
            worth_foundational::facade::CanonicalDigestDerivationDenial::EntryLimitExceeded {
                ..
            },
        ) => WorthQueryApplicationOperationInstallationDenialKind::CanonicalEntryBudgetExceeded,
        WorthQueryGraphObligationInstallationDenial::Canonical(
            worth_foundational::facade::CanonicalDigestDerivationDenial::EncodedByteLimitExceeded {
                ..
            },
        ) => {
            WorthQueryApplicationOperationInstallationDenialKind::CanonicalEncodedByteBudgetExceeded
        }
        WorthQueryGraphObligationInstallationDenial::Canonical(_) => {
            WorthQueryApplicationOperationInstallationDenialKind::CanonicalDigestSlotRejected
        }
    };
    operation_denial(kind, operation)
}

pub(super) fn operation_denial(
    kind: WorthQueryApplicationOperationInstallationDenialKind,
    operation: &str,
) -> WorthQueryApplicationOperationInstallationDenial {
    WorthQueryApplicationOperationInstallationDenial::new(kind, operation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authorization_mode_is_an_exclusive_installed_lattice() {
        assert_eq!(
            operation_authorization("operation", 0, 0).unwrap(),
            WorthQueryInstalledApplicationOperationAuthorization::Principal
        );
        assert_eq!(
            operation_authorization("operation", 1, 0).unwrap(),
            WorthQueryInstalledApplicationOperationAuthorization::Abilities
        );
        assert_eq!(
            operation_authorization("operation", 0, 1).unwrap(),
            WorthQueryInstalledApplicationOperationAuthorization::Capability
        );
        let denial = operation_authorization("operation", 1, 1).unwrap_err();
        assert_eq!(
            denial.kind(),
            WorthQueryApplicationOperationInstallationDenialKind::ConflictingAuthorizationContract
        );
    }
}
