use crate::application_schema::ApplicationOperationRef;

use super::ApplicationCapabilityContextEntitySlotBinding;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityOperationBinding {
    operation: String,
    operation_type: String,
    input_type: String,
}

impl ApplicationCapabilityOperationBinding {
    pub fn from_reference<Schema, Operation, Input>(
        operation: ApplicationOperationRef<Schema, Operation, Input>,
    ) -> Self {
        Self {
            operation: operation.name().to_string(),
            operation_type: std::any::type_name::<Operation>().to_string(),
            input_type: std::any::type_name::<Input>().to_string(),
        }
    }

    pub fn operation(&self) -> &str {
        &self.operation
    }

    pub fn operation_type(&self) -> &str {
        &self.operation_type
    }

    pub fn input_type(&self) -> &str {
        &self.input_type
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityElevationLifecycleDefinition {
    elevation_slot: ApplicationCapabilityContextEntitySlotBinding,
    review_slot: ApplicationCapabilityContextEntitySlotBinding,
    request: ApplicationCapabilityOperationBinding,
    approve: ApplicationCapabilityOperationBinding,
    revoke: ApplicationCapabilityOperationBinding,
    complete_review: ApplicationCapabilityOperationBinding,
}

impl ApplicationCapabilityElevationLifecycleDefinition {
    pub fn new(
        elevation_slot: ApplicationCapabilityContextEntitySlotBinding,
        review_slot: ApplicationCapabilityContextEntitySlotBinding,
        request: ApplicationCapabilityOperationBinding,
        approve: ApplicationCapabilityOperationBinding,
        revoke: ApplicationCapabilityOperationBinding,
        complete_review: ApplicationCapabilityOperationBinding,
    ) -> Self {
        Self {
            elevation_slot,
            review_slot,
            request,
            approve,
            revoke,
            complete_review,
        }
    }

    pub const fn elevation_slot(&self) -> &ApplicationCapabilityContextEntitySlotBinding {
        &self.elevation_slot
    }

    pub const fn review_slot(&self) -> &ApplicationCapabilityContextEntitySlotBinding {
        &self.review_slot
    }

    pub const fn request(&self) -> &ApplicationCapabilityOperationBinding {
        &self.request
    }

    pub const fn approve(&self) -> &ApplicationCapabilityOperationBinding {
        &self.approve
    }

    pub const fn revoke(&self) -> &ApplicationCapabilityOperationBinding {
        &self.revoke
    }

    pub const fn complete_review(&self) -> &ApplicationCapabilityOperationBinding {
        &self.complete_review
    }

    pub fn operations(&self) -> [&ApplicationCapabilityOperationBinding; 4] {
        [
            &self.request,
            &self.approve,
            &self.revoke,
            &self.complete_review,
        ]
    }
}
