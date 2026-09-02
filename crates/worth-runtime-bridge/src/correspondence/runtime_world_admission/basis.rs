use std::sync::Arc;

use super::super::{
    BridgeCorrespondenceAdmissionIdentity, BridgeCorrespondenceBasis,
    BridgeInstalledSemanticCorrespondence, BridgeSemanticDependencyCandidate,
};
use worth_proof::AuthorityWitness;

use super::admission::BridgeRuntimeWorldAdmissionAuthorityMarker;

/// Owner-admitted Bridge meaning that may be carried into Runtime World.
///
/// The installed correspondence remains the Bridge authority. This wrapper
/// retains that witness and the exact configuration basis; it is not a second
/// mapping representation and cannot be constructed from a descriptor.
#[derive(Debug, Clone)]
pub struct AdmittedRuntimeWorldCorrespondenceBasis {
    basis: BridgeCorrespondenceBasis,
    dependency: BridgeSemanticDependencyCandidate,
    admission_identity: BridgeCorrespondenceAdmissionIdentity,
    _authority: Arc<AuthorityWitness<BridgeRuntimeWorldAdmissionAuthorityMarker>>,
}

impl PartialEq for AdmittedRuntimeWorldCorrespondenceBasis {
    fn eq(&self, other: &Self) -> bool {
        self.admission_identity == other.admission_identity
    }
}

impl Eq for AdmittedRuntimeWorldCorrespondenceBasis {}

impl AdmittedRuntimeWorldCorrespondenceBasis {
    pub(crate) fn from_installed(
        installed: &BridgeInstalledSemanticCorrespondence,
        authority: AuthorityWitness<BridgeRuntimeWorldAdmissionAuthorityMarker>,
    ) -> Self {
        Self {
            basis: installed.basis().clone(),
            dependency: installed.dependency().clone(),
            admission_identity: installed.admission_identity().clone(),
            _authority: Arc::new(authority),
        }
    }

    pub fn basis(&self) -> &BridgeCorrespondenceBasis {
        &self.basis
    }

    /// Identity issued by Bridge for the installed correspondence admission.
    ///
    /// The identity binds the installed owner path for Runtime World
    /// composition; the descriptive correspondence basis is not the key.
    pub fn admission_identity(&self) -> &BridgeCorrespondenceAdmissionIdentity {
        &self.admission_identity
    }

    pub fn source_installation_generation(&self) -> u64 {
        self.basis.source_installation_generation()
    }

    pub fn signal_graph_instance_id(&self) -> u64 {
        self.basis.signal_graph_instance_id
    }

    pub(crate) fn dependency(&self) -> &BridgeSemanticDependencyCandidate {
        &self.dependency
    }
}
