use std::collections::BTreeSet;

use crate::application_capability::ApplicationCapabilityContextEntitySlotBinding;

use super::super::{ApplicationSchemaDeclarationDenial, ApplicationSchemaMember};

pub(super) struct DeclaredCapabilityDimensions<'a> {
    contexts: BTreeSet<(&'a str, &'a str)>,
    provenance: BTreeSet<(&'a str, &'a str)>,
    entity_slots: BTreeSet<(&'a str, &'a str, &'a str, &'a str, &'a str)>,
}

impl<'a> DeclaredCapabilityDimensions<'a> {
    pub(super) fn validate(
        members: &'a [ApplicationSchemaMember],
    ) -> Result<Self, ApplicationSchemaDeclarationDenial> {
        let dimensions = Self::collect(members);
        dimensions.reject_ambiguous_contexts(members)?;
        dimensions.reject_ambiguous_provenance(members)?;
        dimensions.reject_invalid_entity_slots(members)?;
        Ok(dimensions)
    }

    fn collect(members: &'a [ApplicationSchemaMember]) -> Self {
        let mut contexts = BTreeSet::new();
        let mut provenance = BTreeSet::new();
        let mut entity_slots = BTreeSet::new();
        for member in members {
            match member {
                ApplicationSchemaMember::ApplicationCapabilityContext {
                    context,
                    context_type,
                } => {
                    contexts.insert((context.as_str(), context_type.as_str()));
                }
                ApplicationSchemaMember::ApplicationCapabilityContextEntitySlot {
                    context,
                    context_type,
                    slot,
                    slot_type,
                    entity,
                } => {
                    entity_slots.insert((
                        context.as_str(),
                        context_type.as_str(),
                        slot.as_str(),
                        slot_type.as_str(),
                        entity.as_str(),
                    ));
                }
                ApplicationSchemaMember::ApplicationCapabilityProvenance {
                    provenance: name,
                    provenance_type,
                } => {
                    provenance.insert((name.as_str(), provenance_type.as_str()));
                }
                _ => {}
            }
        }
        Self {
            contexts,
            provenance,
            entity_slots,
        }
    }

    fn reject_ambiguous_contexts(
        &self,
        members: &[ApplicationSchemaMember],
    ) -> Result<(), ApplicationSchemaDeclarationDenial> {
        let mut names = BTreeSet::new();
        let mut types = BTreeSet::new();
        for member in members {
            let ApplicationSchemaMember::ApplicationCapabilityContext {
                context,
                context_type,
            } = member
            else {
                continue;
            };
            if !names.insert(context) || !types.insert(context_type) {
                return Err(
                    ApplicationSchemaDeclarationDenial::DuplicateApplicationCapabilityContext,
                );
            }
        }
        Ok(())
    }

    fn reject_ambiguous_provenance(
        &self,
        members: &[ApplicationSchemaMember],
    ) -> Result<(), ApplicationSchemaDeclarationDenial> {
        let mut names = BTreeSet::new();
        let mut types = BTreeSet::new();
        for member in members {
            let ApplicationSchemaMember::ApplicationCapabilityProvenance {
                provenance,
                provenance_type,
            } = member
            else {
                continue;
            };
            if !names.insert(provenance) || !types.insert(provenance_type) {
                return Err(
                    ApplicationSchemaDeclarationDenial::DuplicateApplicationCapabilityProvenance,
                );
            }
        }
        Ok(())
    }

    fn reject_invalid_entity_slots(
        &self,
        members: &[ApplicationSchemaMember],
    ) -> Result<(), ApplicationSchemaDeclarationDenial> {
        let entities = members
            .iter()
            .filter_map(|member| match member {
                ApplicationSchemaMember::Entity { entity } => Some(entity.as_str()),
                _ => None,
            })
            .collect::<BTreeSet<_>>();
        let mut names = BTreeSet::new();
        let mut types = BTreeSet::new();
        for &(context, context_type, slot, slot_type, entity) in &self.entity_slots {
            if !self.contexts.contains(&(context, context_type)) || !entities.contains(entity) {
                return Err(
                    ApplicationSchemaDeclarationDenial::MissingApplicationCapabilityContextDependency,
                );
            }
            if !names.insert((context, context_type, slot))
                || !types.insert((context, context_type, slot_type))
            {
                return Err(
                    ApplicationSchemaDeclarationDenial::DuplicateApplicationCapabilityContextSlot,
                );
            }
        }
        Ok(())
    }

    pub(super) fn context_exists(&self, name: &str, marker_type: &str) -> bool {
        self.contexts.contains(&(name, marker_type))
    }

    pub(super) fn provenance_exists(&self, name: &str, marker_type: &str) -> bool {
        self.provenance.contains(&(name, marker_type))
    }

    pub(super) fn entity_slot_exists(
        &self,
        binding: &ApplicationCapabilityContextEntitySlotBinding,
    ) -> bool {
        self.entity_slots.contains(&(
            binding.context(),
            binding.context_type(),
            binding.slot(),
            binding.slot_type(),
            binding.entity(),
        ))
    }
}
