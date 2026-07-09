use crate::{
    WorthServerOperationAdmissionPosture, WorthServerOperationConcurrencyClass,
    WorthServerOperationPreconditionPosture, WorthServerOperationSupportPosture,
};

use super::{WorthServerProductOperationDeclaration, WorthServerProductOperationPayload};

#[derive(Clone, Debug)]
pub struct WorthServerLoweredProductOperationPlan {
    operation_admission: WorthServerOperationAdmissionPosture,
    declaration: WorthServerProductOperationDeclaration,
    payload: WorthServerProductOperationPayload,
    support_posture: WorthServerOperationSupportPosture,
    precondition_posture: WorthServerOperationPreconditionPosture,
    concurrency_class: WorthServerOperationConcurrencyClass,
    canonical_digest: String,
}

impl WorthServerLoweredProductOperationPlan {
    pub(crate) fn new(
        operation_admission: WorthServerOperationAdmissionPosture,
        declaration: WorthServerProductOperationDeclaration,
        payload: WorthServerProductOperationPayload,
        support_posture: WorthServerOperationSupportPosture,
        precondition_posture: WorthServerOperationPreconditionPosture,
        concurrency_class: WorthServerOperationConcurrencyClass,
    ) -> Self {
        let canonical_digest = format!(
            "worth-server-lowered-product-operation-plan-v1|identity={}|metadata={}|footprint={}|support={}|precondition={}|payload={}|concurrency={}",
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

    pub fn operation_admission(&self) -> &WorthServerOperationAdmissionPosture {
        &self.operation_admission
    }

    pub fn declaration(&self) -> &WorthServerProductOperationDeclaration {
        &self.declaration
    }

    pub fn payload(&self) -> &WorthServerProductOperationPayload {
        &self.payload
    }

    pub fn support_posture(&self) -> &WorthServerOperationSupportPosture {
        &self.support_posture
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

fn concurrency_label(concurrency_class: &WorthServerOperationConcurrencyClass) -> &'static str {
    match concurrency_class {
        WorthServerOperationConcurrencyClass::ConcurrentSharedRead => "shared-read",
        WorthServerOperationConcurrencyClass::SerializeDeterministically => {
            "serialize-deterministically"
        }
    }
}
