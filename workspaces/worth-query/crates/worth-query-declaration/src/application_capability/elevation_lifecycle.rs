use crate::application_schema::ApplicationOperationRef;

use super::{ApplicationCapabilityContextEntitySlotBinding, ApplicationCapabilityRef};
use super::{ApplicationCapabilityLifecycleEffect, ApplicationCapabilityLifecycleEffectBinding};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityOperationBinding {
    operation: String,
    operation_type: String,
    input_type: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ApplicationCapabilityTransitionBinding {
    capability: String,
    capability_type: String,
    operation: ApplicationCapabilityOperationBinding,
    lifecycle_effect: Option<ApplicationCapabilityLifecycleEffectBinding>,
}

impl ApplicationCapabilityTransitionBinding {
    pub fn from_references<Schema, Capability, Operation, Input>(
        capability: ApplicationCapabilityRef<Schema, Capability>,
        operation: ApplicationOperationRef<Schema, Operation, Input>,
    ) -> Self {
        Self {
            capability: capability.name().to_string(),
            capability_type: capability.marker_type().to_string(),
            operation: ApplicationCapabilityOperationBinding::from_reference(operation),
            lifecycle_effect: None,
        }
    }

    pub fn from_references_with_lifecycle_effect<Schema, Capability, Operation, Input>(
        capability: ApplicationCapabilityRef<Schema, Capability>,
        operation: ApplicationOperationRef<Schema, Operation, Input>,
    ) -> Self
    where
        Input: ApplicationCapabilityLifecycleEffect<Schema, Operation>,
    {
        Self {
            capability: capability.name().to_string(),
            capability_type: capability.marker_type().to_string(),
            operation: ApplicationCapabilityOperationBinding::from_reference(operation),
            lifecycle_effect: Some(ApplicationCapabilityLifecycleEffectBinding::from_input::<
                Schema,
                Operation,
                Input,
            >()),
        }
    }

    pub fn capability(&self) -> &str {
        &self.capability
    }

    pub fn capability_type(&self) -> &str {
        &self.capability_type
    }

    pub const fn operation(&self) -> &ApplicationCapabilityOperationBinding {
        &self.operation
    }

    pub const fn lifecycle_effect(&self) -> Option<&ApplicationCapabilityLifecycleEffectBinding> {
        self.lifecycle_effect.as_ref()
    }
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
    request: ApplicationCapabilityTransitionBinding,
    approve: ApplicationCapabilityTransitionBinding,
    revoke: ApplicationCapabilityTransitionBinding,
    complete_review: ApplicationCapabilityTransitionBinding,
}

impl ApplicationCapabilityElevationLifecycleDefinition {
    pub fn new(
        elevation_slot: ApplicationCapabilityContextEntitySlotBinding,
        review_slot: ApplicationCapabilityContextEntitySlotBinding,
        request: ApplicationCapabilityTransitionBinding,
        approve: ApplicationCapabilityTransitionBinding,
        revoke: ApplicationCapabilityTransitionBinding,
        complete_review: ApplicationCapabilityTransitionBinding,
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

    pub const fn request(&self) -> &ApplicationCapabilityTransitionBinding {
        &self.request
    }

    pub const fn approve(&self) -> &ApplicationCapabilityTransitionBinding {
        &self.approve
    }

    pub const fn revoke(&self) -> &ApplicationCapabilityTransitionBinding {
        &self.revoke
    }

    pub const fn complete_review(&self) -> &ApplicationCapabilityTransitionBinding {
        &self.complete_review
    }

    pub fn transitions(&self) -> [&ApplicationCapabilityTransitionBinding; 4] {
        [
            &self.request,
            &self.approve,
            &self.revoke,
            &self.complete_review,
        ]
    }
}
