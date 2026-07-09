use worth_proof::{AuthorityMarker, AuthorityWitness};

use crate::canonicalization::CanonicalDerivedDigest;

use super::current::{
    FoundationalAdmittedIdentityValue, FoundationalAuthorityIdentity,
    FoundationalBoundaryBridgedIdentity, FoundationalRevalidatedIdentityValue,
};
use super::derived::{
    FoundationalDigestIdentityEvidence, FoundationalIdentityDigestDerivationEvidence,
    FoundationalIdentityProjectionEvidence, FoundationalProjectionIdentity,
};
use super::markers::{FoundationalIdentityBasis, FoundationalIdentityKind};

pub fn admit_foundational_authority_identity<Value, Authority, Kind>(
    value: Value,
    authority: AuthorityWitness<Authority>,
) -> FoundationalAuthorityIdentity<Value, Authority, Kind>
where
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    let admitted = FoundationalAdmittedIdentityValue::admit(value, authority);
    FoundationalAuthorityIdentity::from_admitted(admitted)
}

pub fn readmit_foundational_authority_identity<Value, Authority, Kind>(
    bridged: FoundationalBoundaryBridgedIdentity<Value, Authority, Kind>,
    authority: AuthorityWitness<Authority>,
) -> FoundationalAuthorityIdentity<Value, Authority, Kind>
where
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    let revalidated = bridged.revalidate_current_value(authority);
    FoundationalAuthorityIdentity::readmit(revalidated)
}

pub fn readmit_revalidated_foundational_authority_identity<Value, Authority, Kind>(
    bridged: FoundationalBoundaryBridgedIdentity<Value, Authority, Kind>,
    value: Value,
    authority: AuthorityWitness<Authority>,
) -> FoundationalAuthorityIdentity<Value, Authority, Kind>
where
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    let revalidated = bridged.revalidate_replacement_value(value, authority);
    FoundationalAuthorityIdentity::readmit(revalidated)
}

pub fn project_foundational_identity<Label, Value, Authority, Kind>(
    identity: &FoundationalAuthorityIdentity<Value, Authority, Kind>,
    label: Label,
    authority: AuthorityWitness<Authority>,
) -> FoundationalProjectionIdentity<Label, Kind>
where
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    let evidence =
        FoundationalIdentityProjectionEvidence::derive_from_authority(identity, label, authority);
    FoundationalProjectionIdentity::from_projection_evidence(evidence)
}

pub fn derive_foundational_digest_identity_evidence<Basis, Value, Authority, Kind>(
    identity: &FoundationalAuthorityIdentity<Value, Authority, Kind>,
    digest: CanonicalDerivedDigest,
    authority: AuthorityWitness<Authority>,
) -> FoundationalDigestIdentityEvidence<Basis, Authority, Kind>
where
    Basis: FoundationalIdentityBasis,
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    let evidence = FoundationalIdentityDigestDerivationEvidence::derive_from_authority(
        identity, digest, authority,
    );
    FoundationalDigestIdentityEvidence::from_derivation_evidence(evidence)
}

pub fn admit_foundational_external_identity_token<Value, Authority, Kind>(
    token: super::external::FoundationalExternalIdentityToken<Value, Kind>,
    authority: AuthorityWitness<Authority>,
) -> FoundationalAuthorityIdentity<Value, Authority, Kind>
where
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    let admitted = token.admit_with_authority(authority);
    FoundationalAuthorityIdentity::from_admitted(admitted)
}

pub fn admitted_foundational_identity_value<Value, Authority, Kind>(
    value: Value,
    authority: AuthorityWitness<Authority>,
) -> FoundationalAdmittedIdentityValue<Value, Authority, Kind>
where
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    FoundationalAdmittedIdentityValue::admit(value, authority)
}

pub fn revalidated_foundational_identity_value<Value, Authority, Kind>(
    bridged: FoundationalBoundaryBridgedIdentity<Value, Authority, Kind>,
    authority: AuthorityWitness<Authority>,
) -> FoundationalRevalidatedIdentityValue<Value, Authority, Kind>
where
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    bridged.revalidate_current_value(authority)
}
