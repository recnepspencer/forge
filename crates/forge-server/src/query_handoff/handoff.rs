use std::fmt;

use forge_query::facade::{ForgeQueryRuntimeDownstreamDeliveryContract, ForgeQueryWorkspace};

use super::{ForgeServerQueryHandoffOperation, ForgeServerQuerySupportPosture};
use crate::ForgeServerAdmission;

pub struct ForgeServerQueryHandoff {
    admission: ForgeServerAdmission,
    operation: ForgeServerQueryHandoffOperation,
    workspace: ForgeQueryWorkspace,
    downstream_delivery_contract: ForgeQueryRuntimeDownstreamDeliveryContract,
    support_posture: ForgeServerQuerySupportPosture,
    canonical_digest: String,
}

impl fmt::Debug for ForgeServerQueryHandoff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ForgeServerQueryHandoff")
            .field("admission", &self.admission)
            .field("operation", &self.operation)
            .field("workspace_name", &self.workspace.name())
            .field(
                "downstream_delivery_contract",
                &self.downstream_delivery_contract,
            )
            .field("support_posture", &self.support_posture)
            .field("canonical_digest", &self.canonical_digest)
            .finish()
    }
}

impl ForgeServerQueryHandoff {
    pub(crate) fn new(
        admission: ForgeServerAdmission,
        operation: ForgeServerQueryHandoffOperation,
        workspace: ForgeQueryWorkspace,
        downstream_delivery_contract: ForgeQueryRuntimeDownstreamDeliveryContract,
        support_posture: ForgeServerQuerySupportPosture,
        canonical_digest: String,
    ) -> Self {
        Self {
            admission,
            operation,
            workspace,
            downstream_delivery_contract,
            support_posture,
            canonical_digest,
        }
    }

    pub fn admission(&self) -> &ForgeServerAdmission {
        &self.admission
    }

    pub fn operation(&self) -> &ForgeServerQueryHandoffOperation {
        &self.operation
    }

    pub fn workspace(&self) -> &ForgeQueryWorkspace {
        &self.workspace
    }

    pub(crate) fn workspace_mut(&mut self) -> &mut ForgeQueryWorkspace {
        &mut self.workspace
    }

    pub fn downstream_delivery_contract(&self) -> &ForgeQueryRuntimeDownstreamDeliveryContract {
        &self.downstream_delivery_contract
    }

    pub fn support_posture(&self) -> &ForgeServerQuerySupportPosture {
        &self.support_posture
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
