use serde::{Deserialize, Serialize};

use super::reference::SignalBranchObservation;
use super::SignalBranchBasisLifecyclePosture;
use crate::state::SignalBranchId;

pub const SIGNAL_BRANCH_BASIS_DESCRIPTOR_SCHEMA_VERSION: u16 = 1;

/// Serializable description of one exact Signal branch observation.
///
/// A descriptor carries no owner proof and cannot open any Signal operation.
/// Deserialization deliberately produces this weak form; the Signal runtime
/// must readmit it before it becomes operational again.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalBranchBasisDescriptor {
    schema_version: u16,
    owner_branch_id: SignalBranchId,
    observation: SignalBranchObservation,
    lifecycle_posture: SignalBranchBasisLifecyclePosture,
}

impl SignalBranchBasisDescriptor {
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    pub fn observation(&self) -> &SignalBranchObservation {
        &self.observation
    }

    pub const fn lifecycle_posture(&self) -> SignalBranchBasisLifecyclePosture {
        self.lifecycle_posture
    }

    pub const fn branch_id(&self) -> SignalBranchId {
        self.owner_branch_id
    }

    pub(crate) const fn owner_branch_id(&self) -> SignalBranchId {
        self.branch_id()
    }

    pub(crate) fn owner_issued(
        owner_branch_id: SignalBranchId,
        observation: SignalBranchObservation,
    ) -> Self {
        Self {
            schema_version: SIGNAL_BRANCH_BASIS_DESCRIPTOR_SCHEMA_VERSION,
            owner_branch_id,
            observation,
            lifecycle_posture: SignalBranchBasisLifecyclePosture::Live,
        }
    }
}
