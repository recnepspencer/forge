use crate::{
    WorthServerOperationAdmissionPosture, WorthServerOperationConcurrencyClass,
    WorthServerOperationPreconditionPosture, WorthServerOperationSupportPosture,
};

use super::WorthServerProductSessionCoordinationCommand;

#[derive(Clone, Debug)]
pub struct WorthServerLoweredProductSessionCoordinationPlan {
    operation_admission: WorthServerOperationAdmissionPosture,
    command: WorthServerProductSessionCoordinationCommand,
    support_posture: WorthServerOperationSupportPosture,
    precondition_posture: WorthServerOperationPreconditionPosture,
    concurrency_class: WorthServerOperationConcurrencyClass,
    canonical_digest: String,
}

impl WorthServerLoweredProductSessionCoordinationPlan {
    pub(crate) fn new(
        operation_admission: WorthServerOperationAdmissionPosture,
        command: WorthServerProductSessionCoordinationCommand,
        support_posture: WorthServerOperationSupportPosture,
        precondition_posture: WorthServerOperationPreconditionPosture,
        concurrency_class: WorthServerOperationConcurrencyClass,
    ) -> Self {
        let canonical_digest = format!(
            "worth-server-lowered-product-session-coordination-plan-v1|identity={}|metadata={}|footprint={}|support={}|precondition={}|command={}|concurrency={}",
            operation_admission.operation_request().identity().canonical_digest(),
            operation_admission.authority_metadata().canonical_digest(),
            operation_admission.authority_footprint().canonical_digest(),
            support_posture.canonical_digest(),
            precondition_posture.canonical_digest(),
            command.operation_name(),
            concurrency_label(&concurrency_class),
        );
        Self {
            operation_admission,
            command,
            support_posture,
            precondition_posture,
            concurrency_class,
            canonical_digest,
        }
    }

    pub fn operation_admission(&self) -> &WorthServerOperationAdmissionPosture {
        &self.operation_admission
    }

    pub fn command(&self) -> &WorthServerProductSessionCoordinationCommand {
        &self.command
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
