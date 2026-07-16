use super::{
    ModelActionFamily, OwnerOperationFamily, OwnerOutcomeSource, OwnerSourcePolymorphism,
    ProductionOwner, ProtocolFamily,
};
use crate::protocol_bindings::OwnerEvidenceClass;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OwnerBoundaryBinding {
    protocol: ProtocolFamily,
    owner: ProductionOwner,
    operation: OwnerOperationFamily,
    model_action_family: ModelActionFamily,
    source: OwnerOutcomeSource,
}

impl OwnerBoundaryBinding {
    pub(super) fn to<T>(
        protocol: ProtocolFamily,
        owner: ProductionOwner,
        operation: OwnerOperationFamily,
        evidence_class: OwnerEvidenceClass,
    ) -> Self {
        Self {
            protocol,
            owner,
            operation,
            model_action_family: operation.model_action_family(),
            source: OwnerOutcomeSource::of::<T>(evidence_class),
        }
    }

    pub(super) fn to_polymorphic<T>(
        protocol: ProtocolFamily,
        owner: ProductionOwner,
        operation: OwnerOperationFamily,
        evidence_class: OwnerEvidenceClass,
        polymorphism: OwnerSourcePolymorphism,
    ) -> Self {
        Self {
            protocol,
            owner,
            operation,
            model_action_family: operation.model_action_family(),
            source: OwnerOutcomeSource::polymorphic::<T>(evidence_class, polymorphism),
        }
    }

    pub const fn protocol(self) -> ProtocolFamily {
        self.protocol
    }

    pub const fn owner(self) -> ProductionOwner {
        self.owner
    }

    pub const fn operation(self) -> OwnerOperationFamily {
        self.operation
    }

    pub const fn model_action_family(self) -> ModelActionFamily {
        self.model_action_family
    }

    pub const fn source(self) -> OwnerOutcomeSource {
        self.source
    }
}
