use std::fmt;

use worth_query::facade::{WorthQueryRuntimeDownstreamDeliveryContract, WorthQueryWorkspace};

use super::{WorthServerQueryHandoffOperation, WorthServerQuerySupportPosture};
use crate::{
    WorthServerAdmission, WorthServerOperationAdmissionPosture,
    WorthServerOperationConcurrencyClass, WorthServerOperationPreconditionPosture,
    WorthServerOperationSupportCompositionReceipt, WorthServerOperationSupportPosture,
};

pub struct WorthServerQueryHandoff {
    operation_admission: WorthServerOperationAdmissionPosture,
    operation: WorthServerQueryHandoffOperation,
    workspace: WorthQueryWorkspace,
    downstream_delivery_contract: WorthQueryRuntimeDownstreamDeliveryContract,
    operation_support_posture: WorthServerOperationSupportPosture,
    support_composition_receipt: WorthServerOperationSupportCompositionReceipt,
    precondition_posture: WorthServerOperationPreconditionPosture,
    concurrency_class: WorthServerOperationConcurrencyClass,
    support_posture: WorthServerQuerySupportPosture,
    canonical_digest: String,
}

impl fmt::Debug for WorthServerQueryHandoff {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WorthServerQueryHandoff")
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

impl WorthServerQueryHandoff {
    pub(crate) fn new(
        operation_admission: WorthServerOperationAdmissionPosture,
        operation: WorthServerQueryHandoffOperation,
        workspace: WorthQueryWorkspace,
        downstream_delivery_contract: WorthQueryRuntimeDownstreamDeliveryContract,
        operation_support_posture: WorthServerOperationSupportPosture,
        support_composition_receipt: WorthServerOperationSupportCompositionReceipt,
        precondition_posture: WorthServerOperationPreconditionPosture,
        concurrency_class: WorthServerOperationConcurrencyClass,
        support_posture: WorthServerQuerySupportPosture,
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

    pub fn admission(&self) -> &WorthServerAdmission {
        self.operation_admission.authorization_proof().admission()
    }

    pub fn operation_admission(&self) -> &WorthServerOperationAdmissionPosture {
        &self.operation_admission
    }

    pub fn operation(&self) -> &WorthServerQueryHandoffOperation {
        &self.operation
    }

    pub fn workspace(&self) -> &WorthQueryWorkspace {
        &self.workspace
    }

    pub(crate) fn workspace_mut(&mut self) -> &mut WorthQueryWorkspace {
        &mut self.workspace
    }

    pub fn downstream_delivery_contract(&self) -> &WorthQueryRuntimeDownstreamDeliveryContract {
        &self.downstream_delivery_contract
    }

    pub fn support_posture(&self) -> &WorthServerQuerySupportPosture {
        &self.support_posture
    }

    pub fn operation_support_posture(&self) -> &WorthServerOperationSupportPosture {
        &self.operation_support_posture
    }

    pub fn support_composition_receipt(&self) -> &WorthServerOperationSupportCompositionReceipt {
        &self.support_composition_receipt
    }

    pub fn precondition_posture(&self) -> &WorthServerOperationPreconditionPosture {
        &self.precondition_posture
    }

    pub fn concurrency_class(&self) -> WorthServerOperationConcurrencyClass {
        self.concurrency_class.clone()
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }
}
