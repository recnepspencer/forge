use std::sync::Arc;

use worth_proof::AuthorityWitness;
use worth_relational::facade::branch::AdmittedRelationalBranchBasis;
use worth_runtime_bridge::facade::AdmittedRuntimeWorldCorrespondenceBasis;
use worth_signal::facade::branch::AdmittedSignalBranchBasis;

use super::CompositeRuntimeWorldBasis;
use crate::identity::{CompositeBasisIdentity, RuntimeWorldOwnerIdentity};

worth_proof::authority_marker!(pub(crate) CompositeBasisAdmissionAuthorityMarker);

/// Owner-admitted composite basis. The private proof prevents descriptors,
/// equal-looking component values, or a consumer projection from minting it.
#[derive(Debug, Clone)]
pub struct AdmittedCompositeRuntimeWorldBasis {
    inner: Arc<AdmittedCompositeRuntimeWorldBasisInner>,
}

#[derive(Debug, Clone)]
struct AdmittedCompositeRuntimeWorldBasisInner {
    basis: CompositeRuntimeWorldBasis,
    _authority: Arc<AuthorityWitness<CompositeBasisAdmissionAuthorityMarker>>,
}

impl PartialEq for AdmittedCompositeRuntimeWorldBasis {
    fn eq(&self, other: &Self) -> bool {
        self.inner.basis == other.inner.basis
    }
}

impl Eq for AdmittedCompositeRuntimeWorldBasis {}

impl AdmittedCompositeRuntimeWorldBasis {
    pub(crate) fn new(
        basis: CompositeRuntimeWorldBasis,
        authority: AuthorityWitness<CompositeBasisAdmissionAuthorityMarker>,
    ) -> Self {
        Self {
            inner: Arc::new(AdmittedCompositeRuntimeWorldBasisInner {
                basis,
                _authority: Arc::new(authority),
            }),
        }
    }

    pub fn basis(&self) -> &CompositeRuntimeWorldBasis {
        &self.inner.basis
    }

    pub fn owner_identity(&self) -> RuntimeWorldOwnerIdentity {
        self.basis().owner_identity()
    }

    pub fn identity(&self) -> &CompositeBasisIdentity {
        self.basis().identity()
    }

    pub fn relational_basis(&self) -> &AdmittedRelationalBranchBasis {
        self.basis().relational_basis()
    }

    pub fn signal_basis(&self) -> &AdmittedSignalBranchBasis {
        self.basis().signal_basis()
    }

    pub fn correspondence_basis(&self) -> &AdmittedRuntimeWorldCorrespondenceBasis {
        self.basis().correspondence_basis()
    }
}
