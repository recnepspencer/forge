use crate::{
    ForgeServerOperationAdmissionPosture, ForgeServerOperationConcurrencyClass,
    ForgeServerOperationPreconditionPosture, ForgeServerOperationSupportPosture,
};

use super::ForgeServerProductSessionCoordinationCommand;

#[derive(Clone, Debug)]
pub struct ForgeServerLoweredProductSessionCoordinationPlan {
    operation_admission: ForgeServerOperationAdmissionPosture,
    command: ForgeServerProductSessionCoordinationCommand,
    support_posture: ForgeServerOperationSupportPosture,
    precondition_posture: ForgeServerOperationPreconditionPosture,
    concurrency_class: ForgeServerOperationConcurrencyClass,
    canonical_digest: String,
}

impl ForgeServerLoweredProductSessionCoordinationPlan {
    pub(crate) fn new(
        operation_admission: ForgeServerOperationAdmissionPosture,
        command: ForgeServerProductSessionCoordinationCommand,
        support_posture: ForgeServerOperationSupportPosture,
        precondition_posture: ForgeServerOperationPreconditionPosture,
        concurrency_class: ForgeServerOperationConcurrencyClass,
    ) -> Self {
        let canonical_digest = format!(
            "forge-server-lowered-product-session-coordination-plan-v1|identity={}|metadata={}|footprint={}|support={}|precondition={}|command={}|concurrency={}",
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

    pub fn operation_admission(&self) -> &ForgeServerOperationAdmissionPosture {
        &self.operation_admission
    }

    pub fn command(&self) -> &ForgeServerProductSessionCoordinationCommand {
        &self.command
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
