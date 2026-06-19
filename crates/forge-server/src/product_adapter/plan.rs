use crate::{
    ForgeServerOperationAdmissionPosture, ForgeServerOperationConcurrencyClass,
    ForgeServerOperationPreconditionPosture, ForgeServerOperationSupportPosture,
};

use super::{ForgeServerProductOperationDeclaration, ForgeServerProductOperationPayload};

#[derive(Clone, Debug)]
pub struct ForgeServerLoweredProductOperationPlan {
    operation_admission: ForgeServerOperationAdmissionPosture,
    declaration: ForgeServerProductOperationDeclaration,
    payload: ForgeServerProductOperationPayload,
    support_posture: ForgeServerOperationSupportPosture,
    precondition_posture: ForgeServerOperationPreconditionPosture,
    concurrency_class: ForgeServerOperationConcurrencyClass,
    canonical_digest: String,
}

impl ForgeServerLoweredProductOperationPlan {
    pub(crate) fn new(
        operation_admission: ForgeServerOperationAdmissionPosture,
        declaration: ForgeServerProductOperationDeclaration,
        payload: ForgeServerProductOperationPayload,
        support_posture: ForgeServerOperationSupportPosture,
        precondition_posture: ForgeServerOperationPreconditionPosture,
        concurrency_class: ForgeServerOperationConcurrencyClass,
    ) -> Self {
        let canonical_digest = format!(
            "forge-server-lowered-product-operation-plan-v1|identity={}|metadata={}|footprint={}|support={}|precondition={}|payload={}|concurrency={}",
            operation_admission
                .operation_request()
                .identity()
                .canonical_digest(),
            operation_admission.authority_metadata().canonical_digest(),
            operation_admission.authority_footprint().canonical_digest(),
            support_posture.canonical_digest(),
            precondition_posture.canonical_digest(),
            payload.envelope().canonical_digest(),
            concurrency_label(&concurrency_class),
        );
        Self {
            operation_admission,
            declaration,
            payload,
            support_posture,
            precondition_posture,
            concurrency_class,
            canonical_digest,
        }
    }

    pub fn operation_admission(&self) -> &ForgeServerOperationAdmissionPosture {
        &self.operation_admission
    }

    pub fn declaration(&self) -> &ForgeServerProductOperationDeclaration {
        &self.declaration
    }

    pub fn payload(&self) -> &ForgeServerProductOperationPayload {
        &self.payload
    }

    pub fn support_posture(&self) -> &ForgeServerOperationSupportPosture {
        &self.support_posture
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

fn concurrency_label(concurrency_class: &ForgeServerOperationConcurrencyClass) -> &'static str {
    match concurrency_class {
        ForgeServerOperationConcurrencyClass::ConcurrentSharedRead => "shared-read",
        ForgeServerOperationConcurrencyClass::SerializeDeterministically => {
            "serialize-deterministically"
        }
    }
}
