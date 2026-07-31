use crate::application_capability::{
    ApplicationCapabilityContextEntitySlotRef, ApplicationCapabilityContextRef,
    ApplicationCapabilityContract, ApplicationCapabilityProvenanceRef,
};

use super::{ApplicationSchemaDeclarationBuilder, ApplicationSchemaMember};

impl<Schema> ApplicationSchemaDeclarationBuilder<Schema> {
    pub fn capability_context<Context>(
        self,
        context: ApplicationCapabilityContextRef<Schema, Context>,
    ) -> Self {
        self.push_member(ApplicationSchemaMember::ApplicationCapabilityContext {
            context: context.name().to_string(),
            context_type: context.marker_type().to_string(),
        })
    }

    pub fn capability_context_entity_slot<Context, Slot, Entity>(
        self,
        slot: ApplicationCapabilityContextEntitySlotRef<Schema, Context, Slot, Entity>,
    ) -> Self {
        self.push_member(
            ApplicationSchemaMember::ApplicationCapabilityContextEntitySlot {
                context: slot.context().to_string(),
                context_type: slot.context_type().to_string(),
                slot: slot.slot().to_string(),
                slot_type: slot.slot_type().to_string(),
                entity: slot.entity().to_string(),
            },
        )
    }

    pub fn capability_provenance<Provenance>(
        self,
        provenance: ApplicationCapabilityProvenanceRef<Schema, Provenance>,
    ) -> Self {
        self.push_member(ApplicationSchemaMember::ApplicationCapabilityProvenance {
            provenance: provenance.name().to_string(),
            provenance_type: provenance.marker_type().to_string(),
        })
    }

    pub fn capability<Capability, Operation, Input>(
        self,
        contract: ApplicationCapabilityContract<Schema, Capability, Operation, Input>,
    ) -> Self {
        self.push_member(ApplicationSchemaMember::ApplicationCapability {
            contract: contract.into_erased(),
        })
    }
}
