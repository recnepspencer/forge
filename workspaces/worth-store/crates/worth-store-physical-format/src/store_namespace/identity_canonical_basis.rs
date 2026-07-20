use worth_foundational::canonicalization_api::lower_lane::basis::{
    prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisReadyArtifact, CanonicalBasisValue,
    CanonicalIntegerWidth, CanonicalizationRuleVersion,
};
use worth_foundational::{CanonicalBasisConstructionDenial, InternedString};
use worth_proof::TransitionOutcome;

use super::identity_record::STORE_NAMESPACE_IDENTITY_ENCODING_VERSION;
use super::{StableStoreIdentity, StoreNamespaceVersion};

const IDENTITY_DOMAIN: CanonicalBasisDomain = CanonicalBasisDomain::Identity;
const IDENTITY_FIELD: CanonicalBasisEntryKind = CanonicalBasisEntryKind::Field;

pub type StoreNamespaceIdentityCanonicalBasisOutcome =
    TransitionOutcome<CanonicalBasisReadyArtifact, CanonicalBasisConstructionDenial>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreNamespaceIdentityPublicationPosture {
    StagedCandidate,
    Published,
}

/// Complete semantic input to the namespace-identity canonical boundary.
///
/// Public construction is intentionally limited to a validated, published
/// `StableStoreIdentity`. The other posture remains in the grammar so future
/// format evolution cannot silently omit publication meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreNamespaceIdentityCanonicalMeaning {
    namespace_version: u16,
    encoding_version: u16,
    identity: [u8; 16],
    publication: StoreNamespaceIdentityPublicationPosture,
}

/// Descriptive namespace identity decoded outside Store's trust boundary.
///
/// This value deliberately carries no `StableStoreIdentity`. It can participate
/// in canonical comparison, but it cannot open, re-admit, or mutate a Store.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternalStoreNamespaceIdentityMeaning {
    namespace_version: u16,
    encoding_version: u16,
    identity: [u8; 16],
    publication: StoreNamespaceIdentityPublicationPosture,
}

impl ExternalStoreNamespaceIdentityMeaning {
    pub const fn observed_published(
        namespace_version: u16,
        encoding_version: u16,
        identity: [u8; 16],
    ) -> Self {
        Self {
            namespace_version,
            encoding_version,
            identity,
            publication: StoreNamespaceIdentityPublicationPosture::Published,
        }
    }

    pub const fn with_publication_posture(
        mut self,
        publication: StoreNamespaceIdentityPublicationPosture,
    ) -> Self {
        self.publication = publication;
        self
    }

    const fn into_canonical_meaning(self) -> StoreNamespaceIdentityCanonicalMeaning {
        StoreNamespaceIdentityCanonicalMeaning {
            namespace_version: self.namespace_version,
            encoding_version: self.encoding_version,
            identity: self.identity,
            publication: self.publication,
        }
    }
}

impl StoreNamespaceIdentityCanonicalMeaning {
    pub const fn from_published_identity(identity: StableStoreIdentity) -> Self {
        Self {
            namespace_version: StoreNamespaceVersion::CURRENT.value(),
            encoding_version: STORE_NAMESPACE_IDENTITY_ENCODING_VERSION,
            identity: identity.bytes(),
            publication: StoreNamespaceIdentityPublicationPosture::Published,
        }
    }

    pub const fn namespace_version(self) -> u16 {
        self.namespace_version
    }

    pub const fn encoding_version(self) -> u16 {
        self.encoding_version
    }

    pub const fn identity(self) -> [u8; 16] {
        self.identity
    }

    pub const fn publication(self) -> StoreNamespaceIdentityPublicationPosture {
        self.publication
    }

    #[cfg(test)]
    pub(super) const fn for_test(
        namespace_version: u16,
        encoding_version: u16,
        identity: [u8; 16],
        publication: StoreNamespaceIdentityPublicationPosture,
    ) -> Self {
        Self {
            namespace_version,
            encoding_version,
            identity,
            publication,
        }
    }
}

pub fn prepare_store_namespace_identity_canonical_basis(
    version: CanonicalizationRuleVersion,
    meaning: StoreNamespaceIdentityCanonicalMeaning,
) -> StoreNamespaceIdentityCanonicalBasisOutcome {
    prepare_canonical_basis_sequence(version, IDENTITY_DOMAIN, canonical_entries(meaning))
}

pub fn prepare_external_store_namespace_identity_canonical_basis(
    version: CanonicalizationRuleVersion,
    meaning: ExternalStoreNamespaceIdentityMeaning,
) -> StoreNamespaceIdentityCanonicalBasisOutcome {
    prepare_store_namespace_identity_canonical_basis(version, meaning.into_canonical_meaning())
}

fn canonical_entries(meaning: StoreNamespaceIdentityCanonicalMeaning) -> Vec<CanonicalBasisEntry> {
    vec![
        text_entry("source.kind", "store-namespace-identity"),
        u16_entry("namespace.version", meaning.namespace_version()),
        u16_entry("encoding.version", meaning.encoding_version()),
        CanonicalBasisEntry::new(
            IDENTITY_DOMAIN,
            CanonicalBasisLocus::Named("identity".into()),
            CanonicalBasisEntryKind::Identity,
            CanonicalBasisValue::UnsignedInteger {
                width: CanonicalIntegerWidth::Bits128,
                value: u128::from_be_bytes(meaning.identity()),
            },
        ),
        text_entry(
            "publication.posture",
            publication_token(meaning.publication()),
        ),
    ]
}

fn text_entry(
    locus: impl Into<InternedString>,
    value: impl Into<InternedString>,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        IDENTITY_DOMAIN,
        CanonicalBasisLocus::Named(locus.into()),
        IDENTITY_FIELD,
        CanonicalBasisValue::ExactText(value.into()),
    )
}

fn u16_entry(locus: impl Into<InternedString>, value: u16) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        IDENTITY_DOMAIN,
        CanonicalBasisLocus::Named(locus.into()),
        IDENTITY_FIELD,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits16,
            value: u128::from(value),
        },
    )
}

const fn publication_token(posture: StoreNamespaceIdentityPublicationPosture) -> &'static str {
    match posture {
        StoreNamespaceIdentityPublicationPosture::StagedCandidate => "staged-candidate",
        StoreNamespaceIdentityPublicationPosture::Published => "published",
    }
}
