use std::marker::PhantomData;

use worth_proof::{AuthorityProves, AuthorityWitness, Proof, ProofMarker};

use super::basis::AdmittedRelationalForkSourceBasis;
use super::identity::RelationalBranchIdentity;
use super::reference::RelationalBranchObservation;
use super::version::RelationalBranchVersion;

worth_proof::authority_marker!(pub RelationalBranchObservationAuthorityMarker);
worth_proof::authority_marker!(pub RelationalForkSourceAuthorityMarker);
worth_proof::authority_marker!(pub RelationalLegacyBranchBindingAuthorityMarker);

impl Clone for RelationalLegacyBranchBindingAuthorityMarker {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for RelationalLegacyBranchBindingAuthorityMarker {}

pub type RelationalForkSourceAuthority = AuthorityWitness<RelationalForkSourceAuthorityMarker>;

/// Concrete proof that the Relational owner checked and issued one legacy
/// transaction binding.  The proof names the authority kind; the binding's
/// runtime identity, exact observation, and branch-local version carry the
/// checked facts themselves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RelationalLegacyBranchBindingOwnerProof(PhantomData<()>);

impl ProofMarker for RelationalLegacyBranchBindingOwnerProof {}

impl AuthorityProves<RelationalLegacyBranchBindingOwnerProof>
    for RelationalLegacyBranchBindingAuthorityMarker
{
}

pub type RelationalLegacyBranchBindingProof =
    Proof<RelationalLegacyBranchBindingOwnerProof, RelationalLegacyBranchBindingAuthorityMarker>;

/// Typed denial returned when the Relational owner cannot issue a binding for
/// a branch identity.  The identity is descriptive and therefore may be
/// supplied by a caller, but only an exact runtime-owned cell can be admitted
/// into a transaction authority path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelationalLegacyBranchBindingDenial {
    ForeignRuntime {
        expected_runtime_instance_id: u64,
        actual_runtime_instance_id: u64,
    },
    UnknownBranch(crate::history::data::BranchId),
    IdentityMismatch,
}

/// Private compatibility binding for the pre-detached transaction executor.
/// It is runtime-affine and owner-minted; it has no default, serde, or raw-id
/// constructor and is intentionally distinct from fork/read bases.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RelationalLegacyBranchBinding {
    identity: RelationalBranchIdentity,
    observation: RelationalBranchObservation,
    truth_version: RelationalBranchVersion,
    _proof: RelationalLegacyBranchBindingProof,
}

impl RelationalLegacyBranchBinding {
    pub(crate) fn new(
        identity: RelationalBranchIdentity,
        observation: RelationalBranchObservation,
        truth_version: RelationalBranchVersion,
    ) -> Self {
        Self {
            identity,
            observation,
            truth_version,
            _proof: Proof::from_authority_witness(
                &RelationalLegacyBranchBindingAuthorityMarker::witness(),
            ),
        }
    }

    pub(crate) fn identity(&self) -> &RelationalBranchIdentity {
        &self.identity
    }

    pub(crate) fn observation(&self) -> &RelationalBranchObservation {
        &self.observation
    }

    pub(crate) const fn truth_version(&self) -> RelationalBranchVersion {
        self.truth_version
    }
}

pub(crate) fn admit_relational_fork_source(
    descriptor: super::basis::RelationalForkSourceDescriptor,
) -> AdmittedRelationalForkSourceBasis {
    AdmittedRelationalForkSourceBasis::new(
        descriptor,
        RelationalForkSourceAuthorityMarker::witness(),
    )
}
