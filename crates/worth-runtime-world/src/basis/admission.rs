use std::sync::Arc;

use worth_proof::AuthorityWitness;
use worth_relational::facade::branch::AdmittedRelationalBranchBasis;
use worth_runtime_bridge::facade::AdmittedRuntimeWorldCorrespondenceBasis;
use worth_signal::facade::branch::{
    AdmittedSignalBranchBasis, SignalBranchBasisPort, SignalBranchBasisReadmissionDenial,
};

use super::composite::CompositeRuntimeWorldBasis;
use crate::identity::{
    CompositeBasisIdentity, RuntimeWorldIdentityExhaustion, RuntimeWorldIdentityIssuer,
};

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
    admission: CompositeBasisAdmissionBinding,
}

#[derive(Debug, Clone)]
struct CompositeBasisAdmissionBinding {
    identity: CompositeBasisIdentity,
    _authority: Arc<AuthorityWitness<CompositeBasisAdmissionAuthorityMarker>>,
}

impl PartialEq for AdmittedCompositeRuntimeWorldBasis {
    fn eq(&self, other: &Self) -> bool {
        self.inner.admission.identity == other.inner.admission.identity
    }
}

impl Eq for AdmittedCompositeRuntimeWorldBasis {}

impl AdmittedCompositeRuntimeWorldBasis {
    fn new(basis: CompositeRuntimeWorldBasis, identity: CompositeBasisIdentity) -> Self {
        let authority =
            AuthorityWitness::from_authority_marker(CompositeBasisAdmissionAuthorityMarker::seal());
        Self {
            inner: Arc::new(AdmittedCompositeRuntimeWorldBasisInner {
                basis,
                admission: CompositeBasisAdmissionBinding {
                    identity,
                    _authority: Arc::new(authority),
                },
            }),
        }
    }

    pub fn owner_identity(&self) -> crate::identity::RuntimeWorldOwnerIdentity {
        self.identity().owner_identity()
    }

    pub fn identity(&self) -> &CompositeBasisIdentity {
        &self.inner.admission.identity
    }

    pub fn relational_basis(&self) -> &AdmittedRelationalBranchBasis {
        self.inner.basis.relational_basis()
    }

    pub fn signal_basis(&self) -> &AdmittedSignalBranchBasis {
        self.inner.basis.signal_basis()
    }

    pub fn correspondence_basis(&self) -> &AdmittedRuntimeWorldCorrespondenceBasis {
        self.inner.basis.correspondence_basis()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CompositeBasisAdmissionDenial {
    Signal(SignalBranchBasisReadmissionDenial),
    IdentityExhausted(RuntimeWorldIdentityExhaustion),
}

/// Admit a live component tuple only after the Signal owner has checked the
/// exact basis against its current owner cell. The World identity is issued
/// only after that owner-side admission succeeds.
pub(crate) fn admit_current<D, I, T>(
    identities: &mut RuntimeWorldIdentityIssuer,
    signal_port: &SignalBranchBasisPort<D, I, T>,
    relational: AdmittedRelationalBranchBasis,
    signal: AdmittedSignalBranchBasis,
    correspondence: AdmittedRuntimeWorldCorrespondenceBasis,
) -> Result<AdmittedCompositeRuntimeWorldBasis, CompositeBasisAdmissionDenial>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    signal_port
        .compare_current_exact(&signal)
        .map_err(CompositeBasisAdmissionDenial::Signal)?;
    let identity = identities
        .composite_basis()
        .map_err(CompositeBasisAdmissionDenial::IdentityExhausted)?;
    let basis = CompositeRuntimeWorldBasis::admit(relational, signal, correspondence);
    Ok(AdmittedCompositeRuntimeWorldBasis::new(basis, identity))
}

#[cfg(test)]
#[path = "admission_tests.rs"]
mod tests;
