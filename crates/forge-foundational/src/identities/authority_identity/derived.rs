use std::fmt;
use std::marker::PhantomData;

use forge_proof::{AuthorityMarker, AuthorityWitness};

use crate::canonicalization::CanonicalDerivedDigest;

use super::current::FoundationalAuthorityIdentity;
use super::markers::{FoundationalIdentityBasis, FoundationalIdentityKind};

pub struct FoundationalIdentityProjectionEvidence<Label, Authority, Kind>
where
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    label: Label,
    _authority: PhantomData<fn() -> Authority>,
    _kind: PhantomData<fn() -> Kind>,
}

impl<Label, Authority, Kind> FoundationalIdentityProjectionEvidence<Label, Authority, Kind>
where
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    pub fn derive_from_authority<Value>(
        _identity: &FoundationalAuthorityIdentity<Value, Authority, Kind>,
        label: Label,
        _authority: AuthorityWitness<Authority>,
    ) -> Self {
        Self {
            label,
            _authority: PhantomData,
            _kind: PhantomData,
        }
    }

    pub const fn label(&self) -> &Label {
        &self.label
    }

    fn into_label(self) -> Label {
        self.label
    }
}

pub struct FoundationalProjectionIdentity<Label, Kind>
where
    Kind: FoundationalIdentityKind,
{
    label: Label,
    _kind: PhantomData<fn() -> Kind>,
}

impl<Label, Kind> FoundationalProjectionIdentity<Label, Kind>
where
    Kind: FoundationalIdentityKind,
{
    pub fn from_projection_evidence<Authority>(
        evidence: FoundationalIdentityProjectionEvidence<Label, Authority, Kind>,
    ) -> Self
    where
        Authority: AuthorityMarker,
    {
        Self {
            label: evidence.into_label(),
            _kind: PhantomData,
        }
    }

    pub const fn label(&self) -> &Label {
        &self.label
    }

    pub fn into_label(self) -> Label {
        self.label
    }
}

impl<Label, Kind> Clone for FoundationalProjectionIdentity<Label, Kind>
where
    Label: Clone,
    Kind: FoundationalIdentityKind,
{
    fn clone(&self) -> Self {
        Self {
            label: self.label.clone(),
            _kind: PhantomData,
        }
    }
}

impl<Label, Kind> fmt::Debug for FoundationalProjectionIdentity<Label, Kind>
where
    Label: fmt::Debug,
    Kind: FoundationalIdentityKind,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FoundationalProjectionIdentity")
            .field("label", &self.label)
            .finish_non_exhaustive()
    }
}

impl<Label, Kind> PartialEq for FoundationalProjectionIdentity<Label, Kind>
where
    Label: PartialEq,
    Kind: FoundationalIdentityKind,
{
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl<Label, Kind> Eq for FoundationalProjectionIdentity<Label, Kind>
where
    Label: Eq,
    Kind: FoundationalIdentityKind,
{
}

pub struct FoundationalIdentityDigestDerivationEvidence<Basis, Authority, Kind>
where
    Basis: FoundationalIdentityBasis,
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    digest: CanonicalDerivedDigest,
    _basis: PhantomData<fn() -> Basis>,
    _authority: PhantomData<fn() -> Authority>,
    _kind: PhantomData<fn() -> Kind>,
}

impl<Basis, Authority, Kind> FoundationalIdentityDigestDerivationEvidence<Basis, Authority, Kind>
where
    Basis: FoundationalIdentityBasis,
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    pub fn derive_from_authority<Value>(
        _identity: &FoundationalAuthorityIdentity<Value, Authority, Kind>,
        digest: CanonicalDerivedDigest,
        _authority: AuthorityWitness<Authority>,
    ) -> Self {
        Self {
            digest,
            _basis: PhantomData,
            _authority: PhantomData,
            _kind: PhantomData,
        }
    }

    fn into_digest(self) -> CanonicalDerivedDigest {
        self.digest
    }
}

pub struct FoundationalDigestIdentityEvidence<Basis, Authority, Kind>
where
    Basis: FoundationalIdentityBasis,
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    digest: CanonicalDerivedDigest,
    _basis: PhantomData<fn() -> Basis>,
    _authority: PhantomData<fn() -> Authority>,
    _kind: PhantomData<fn() -> Kind>,
}

impl<Basis, Authority, Kind> FoundationalDigestIdentityEvidence<Basis, Authority, Kind>
where
    Basis: FoundationalIdentityBasis,
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    pub fn from_derivation_evidence(
        evidence: FoundationalIdentityDigestDerivationEvidence<Basis, Authority, Kind>,
    ) -> Self {
        Self {
            digest: evidence.into_digest(),
            _basis: PhantomData,
            _authority: PhantomData,
            _kind: PhantomData,
        }
    }

    pub const fn digest(&self) -> &CanonicalDerivedDigest {
        &self.digest
    }

    pub fn into_digest(self) -> CanonicalDerivedDigest {
        self.digest
    }
}

impl<Basis, Authority, Kind> fmt::Debug
    for FoundationalDigestIdentityEvidence<Basis, Authority, Kind>
where
    Basis: FoundationalIdentityBasis,
    Authority: AuthorityMarker,
    Kind: FoundationalIdentityKind,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FoundationalDigestIdentityEvidence")
            .field("digest", &self.digest)
            .finish_non_exhaustive()
    }
}
