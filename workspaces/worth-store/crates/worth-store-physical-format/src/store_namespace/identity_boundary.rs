use worth_foundational::{
    admit_foundational_authority_identity, readmit_revalidated_foundational_authority_identity,
    FoundationalAuthorityIdentity, FoundationalBoundaryBridgedIdentity, FoundationalIdentityKind,
};
use worth_proof::{AuthorityMarker, AuthorityWitness};

use super::StableStoreIdentity;

struct StoreNamespaceIdentityAdmissionAuthority(());

impl AuthorityMarker for StoreNamespaceIdentityAdmissionAuthority {}

struct StoreNamespaceIdentityKind;

impl FoundationalIdentityKind for StoreNamespaceIdentityKind {}

type FoundationalStoreNamespaceIdentity = FoundationalAuthorityIdentity<
    StableStoreIdentity,
    StoreNamespaceIdentityAdmissionAuthority,
    StoreNamespaceIdentityKind,
>;
type FoundationalBridgedStoreNamespaceIdentity = FoundationalBoundaryBridgedIdentity<
    StableStoreIdentity,
    StoreNamespaceIdentityAdmissionAuthority,
    StoreNamespaceIdentityKind,
>;

/// Foundational boundary form admitted from a validated Store identity.
///
/// This is portable identity meaning, not authority to open or mutate a root.
pub struct StoreNamespaceIdentityBoundary(FoundationalStoreNamespaceIdentity);

impl StoreNamespaceIdentityBoundary {
    pub fn from_validated_identity(identity: StableStoreIdentity) -> Self {
        Self(admit_foundational_authority_identity(
            identity,
            admission_authority(),
        ))
    }

    pub const fn identity(&self) -> StableStoreIdentity {
        *self.0.value()
    }

    pub fn bridge_trust_boundary(self) -> BridgedStoreNamespaceIdentityBoundary {
        BridgedStoreNamespaceIdentityBoundary(self.0.bridge_trust_boundary())
    }
}

impl Clone for StoreNamespaceIdentityBoundary {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl core::fmt::Debug for StoreNamespaceIdentityBoundary {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("StoreNamespaceIdentityBoundary")
            .field("identity", &"<validated-store-identity>")
            .finish_non_exhaustive()
    }
}

impl PartialEq for StoreNamespaceIdentityBoundary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for StoreNamespaceIdentityBoundary {}

/// Identity observation after crossing a trust boundary.
///
/// It cannot become current again without a freshly decoded Store identity.
pub struct BridgedStoreNamespaceIdentityBoundary(FoundationalBridgedStoreNamespaceIdentity);

impl BridgedStoreNamespaceIdentityBoundary {
    pub const fn observed_identity_bytes(&self) -> [u8; 16] {
        self.0.value().bytes()
    }

    pub fn readmit_after_validation(
        self,
        current: StableStoreIdentity,
    ) -> Result<StoreNamespaceIdentityBoundary, StoreNamespaceIdentityReadmissionDenial> {
        if self.0.value() != &current {
            return Err(StoreNamespaceIdentityReadmissionDenial::IdentityChanged {
                observed: self.0.value().bytes(),
                current: current.bytes(),
            });
        }

        Ok(StoreNamespaceIdentityBoundary(
            readmit_revalidated_foundational_authority_identity(
                self.0,
                current,
                admission_authority(),
            ),
        ))
    }
}

impl core::fmt::Debug for BridgedStoreNamespaceIdentityBoundary {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("BridgedStoreNamespaceIdentityBoundary")
            .field("identity", &"<boundary-observation>")
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreNamespaceIdentityReadmissionDenial {
    IdentityChanged {
        observed: [u8; 16],
        current: [u8; 16],
    },
}

fn admission_authority() -> AuthorityWitness<StoreNamespaceIdentityAdmissionAuthority> {
    AuthorityWitness::from_authority_marker(StoreNamespaceIdentityAdmissionAuthority(()))
}
