use worth_relational::facade::branch::AdmittedRelationalBranchBasis;
use worth_runtime_bridge::facade::AdmittedRuntimeWorldCorrespondenceBasis;
use worth_signal::facade::branch::AdmittedSignalBranchBasis;

use crate::identity::{CompositeBasisIdentity, RuntimeWorldOwnerIdentity};

/// The exact ordered component tuple that constitutes one Runtime World
/// basis. It contains no ambient currentness lookup or digest substitute.
#[derive(Debug, Clone)]
pub struct CompositeRuntimeWorldBasis {
    owner: RuntimeWorldOwnerIdentity,
    relational: AdmittedRelationalBranchBasis,
    signal: AdmittedSignalBranchBasis,
    correspondence: AdmittedRuntimeWorldCorrespondenceBasis,
    identity: CompositeBasisIdentity,
}

impl PartialEq for CompositeRuntimeWorldBasis {
    fn eq(&self, other: &Self) -> bool {
        self.owner == other.owner
            && self.identity == other.identity
            && self.relational == other.relational
            && self.signal.descriptor() == other.signal.descriptor()
            && self.correspondence == other.correspondence
    }
}

impl Eq for CompositeRuntimeWorldBasis {}

impl CompositeRuntimeWorldBasis {
    pub fn owner_identity(&self) -> RuntimeWorldOwnerIdentity {
        self.owner
    }

    pub fn relational_basis(&self) -> &AdmittedRelationalBranchBasis {
        &self.relational
    }

    pub fn signal_basis(&self) -> &AdmittedSignalBranchBasis {
        &self.signal
    }

    pub fn correspondence_basis(&self) -> &AdmittedRuntimeWorldCorrespondenceBasis {
        &self.correspondence
    }

    pub fn identity(&self) -> &CompositeBasisIdentity {
        &self.identity
    }

    pub(crate) fn admit(
        owner: RuntimeWorldOwnerIdentity,
        relational: AdmittedRelationalBranchBasis,
        signal: AdmittedSignalBranchBasis,
        correspondence: AdmittedRuntimeWorldCorrespondenceBasis,
        identity: CompositeBasisIdentity,
    ) -> Self {
        Self {
            owner,
            relational,
            signal,
            correspondence,
            identity,
        }
    }
}
