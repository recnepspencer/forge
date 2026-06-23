use std::fmt;

use forge_query::facade::{ForgeQueryRuntimeDownstreamDeliveryContract, ForgeQueryWorkspace};

use super::{ForgeServerQueryHandoffOperation, ForgeServerQuerySupportPosture};
use crate::{
    ForgeServerAdmission, ForgeServerOperationAdmissionPosture,
    ForgeServerOperationConcurrencyClass, ForgeServerOperationPreconditionPosture,
    ForgeServerOperationSupportCompositionReceipt, ForgeServerOperationSupportPosture,
};

pub struct ForgeServerQueryHandoff {
    operation_admission: ForgeServerOperationAdmissionPosture,
    operation: ForgeServerQueryHandoffOperation,
    workspace: ForgeQueryWorkspace,
    downstream_delivery_contract: ForgeQueryRuntimeDownstreamDeliveryContract,
    operation_support_posture: ForgeServerOperationSupportPosture,
    support_composition_receipt: ForgeServerOperationSupportCompositionReceipt,
    precondition_posture: ForgeServerOperationPreconditionPosture,
    concurrency_class: ForgeServerOperationConcurrencyClass,
    support_posture: ForgeServerQuerySupportPosture,
    canonical_digest: String,
}

impl fmt::Debug for ForgeServerQueryHandoff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ForgeServerQueryHandoff")
            .field("operation_admission", &self.operation_admission)
            .field("operation", &self.operation)
            .field("workspace_name", &self.workspace.name())
            .field(
                "downstream_delivery_contract",
                &self.downstream_delivery_contract,
            )
            .field("operation_support_posture", &self.operation_support_posture)
            .field(
                "support_composition_receipt",
                &self.support_composition_receipt,
            )
            .field("precondition_posture", &self.precondition_posture)
            .field("concurrency_class", &self.concurrency_class)
            .field("support_posture", &self.support_posture)
            .field("canonical_digest", &self.canonical_digest)
            .finish()
    }
}

impl ForgeServerQueryHandoff {
    pub(crate) fn new(
        operation_admission: ForgeServerOperationAdmissionPosture,
        operation: ForgeServerQueryHandoffOperation,
        workspace: ForgeQueryWorkspace,
        downstream_delivery_contract: ForgeQueryRuntimeDownstreamDeliveryContract,
        operation_support_posture: ForgeServerOperationSupportPosture,
        support_composition_receipt: ForgeServerOperationSupportCompositionReceipt,
        precondition_posture: ForgeServerOperationPreconditionPosture,
        concurrency_class: ForgeServerOperationConcurrencyClass,
        support_posture: ForgeServerQuerySupportPosture,
        canonical_digest: String,
    ) -> Self {
        Self {
            operation_admission,
            operation,
            workspace,
            downstream_delivery_contract,
            operation_support_posture,
            support_composition_receipt,
            precondition_posture,
            concurrency_class,
            support_posture,
            canonical_digest,
        }
    }

    pub fn admission(&self) -> &ForgeServerAdmission {
        self.operation_admission.authorization_proof().admission()
    }

    pub fn operation_admission(&self) -> &ForgeServerOperationAdmissionPosture {
        &self.operation_admission
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

    pub fn operation_support_posture(&self) -> &ForgeServerOperationSupportPosture {
        &self.operation_support_posture
    }

    pub fn support_composition_receipt(&self) -> &ForgeServerOperationSupportCompositionReceipt {
        &self.support_composition_receipt
    }

    pub fn precondition_posture(&self) -> &ForgeServerOperationPreconditionPosture {
        &self.precondition_posture
    }

    pub fn concurrency_class(&self) -> ForgeServerOperationConcurrencyClass {
        self.concurrency_class.clone()
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
