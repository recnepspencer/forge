use crate::domain_computation::WorthQueryArtifactContentIdentityContract;

use super::{
    portable_text, WorthQueryArtifactContractValidationDenial,
    WorthQueryArtifactContractValidationDenialKind as Kind,
};
use crate::domain_computation::WorthQueryPortableArtifactContract;

pub(super) fn validate(
    contract: &WorthQueryPortableArtifactContract,
) -> Result<(), WorthQueryArtifactContractValidationDenial> {
    let deny = |kind, subject| {
        Err(WorthQueryArtifactContractValidationDenial::new(
            kind, subject,
        ))
    };
    if !contract.family.is_portable() {
        return deny(Kind::InvalidFamilyIdentity, contract.family.as_str());
    }
    if contract.schema_version.get() == 0 {
        return deny(Kind::UnversionedSchema, contract.family.as_str());
    }
    if contract.protocol_version.get() == 0 {
        return deny(Kind::UnversionedProtocol, contract.family.as_str());
    }
    if matches!(
        contract.content_identity,
        WorthQueryArtifactContentIdentityContract::CallerDigestDefined
    ) {
        return deny(Kind::CallerDigestIdentity, contract.family.as_str());
    }
    if matches!(
        &contract.content_identity,
        WorthQueryArtifactContentIdentityContract::OwnerCanonicalProjection {
            projection_family,
            ..
        } if !portable_text(projection_family)
    ) {
        return deny(Kind::InvalidSemanticEvidence, "canonical-projection-family");
    }
    if contract
        .ownership
        .payload_owner()
        .zip(contract.ownership.provider_family())
        .is_none_or(|(owner, provider)| !portable_text(owner) || !portable_text(provider))
    {
        return deny(Kind::AmbiguousOwnership, contract.family.as_str());
    }
    if !contract.evidence.fields_are_portable() {
        return deny(Kind::InvalidSemanticEvidence, contract.family.as_str());
    }
    Ok(())
}
