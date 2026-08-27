use std::collections::BTreeMap;

use worth_query_installation::facade::{
    ApplicationFieldRef, ApplicationFieldUnit, ApplicationOperationProgramTarget, OperationWrites,
    TypedApplicationValue, WritableCapability,
};
use worth_relational::facade::transactions::EntityReference;

use super::{
    denial, WorthQueryApplicationEffectEntity, WorthQueryApplicationEffectProgramBuilder,
    WorthQueryApplicationOptionalFieldWrite, WorthQueryApplicationRealizedEffect,
};
use crate::domain_computation::primary_graph::{
    WorthQueryApplicationAttemptDenial, WorthQueryApplicationAttemptDenialKind,
};

impl<Schema, Operation, Input, Scope>
    WorthQueryApplicationEffectProgramBuilder<Schema, Operation, Input, Scope>
{
    pub fn write_field<Entity, Aspect, Field, Value, Write, Equality, Unit>(
        &mut self,
        target: &WorthQueryApplicationEffectEntity<Schema, Entity>,
        field: ApplicationFieldRef<Schema, Entity, Aspect, Field, Value, Write, Equality, Unit>,
        value: Value,
    ) -> Result<(), WorthQueryApplicationAttemptDenial>
    where
        Field: OperationWrites<Operation>,
        Value: TypedApplicationValue,
        Write: WritableCapability,
        Unit: ApplicationFieldUnit,
    {
        self.validate_target(target, field.entity())?;
        self.admit_program_target(&ApplicationOperationProgramTarget::Write {
            entity: field.entity().to_string(),
            aspect: field.aspect().to_string(),
            field: field.field().to_string(),
        })?;
        let EntityReference::Existing(entity_id) = target.reference else {
            return Err(denial(
                WorthQueryApplicationAttemptDenialKind::ForeignEffectTarget,
                field.field(),
            ));
        };
        let locator = self.field_locator(field.entity(), field.aspect(), field.field())?;
        if let Some(WorthQueryApplicationRealizedEffect::PatchOptionalEntityFields {
            fields, ..
        }) = self.effects.iter_mut().find(|effect| {
            matches!(
                effect,
                WorthQueryApplicationRealizedEffect::PatchOptionalEntityFields {
                    entity,
                    entity_id: candidate,
                    ..
                } if entity == field.entity() && *candidate == entity_id
            )
        }) {
            let contract = self
                .layout
                .aspect_contract(field.entity(), locator.aspect().aspect_key())
                .map(worth_foundational::facade::PortableAspectContractBasis::from_contract)
                .ok_or_else(|| {
                    denial(
                        WorthQueryApplicationAttemptDenialKind::UndeclaredEffect,
                        field.field(),
                    )
                })?;
            fields.insert(
                locator,
                WorthQueryApplicationOptionalFieldWrite {
                    contract,
                    value: Some(value.into_foundational_value()),
                },
            );
            return Ok(());
        }
        match self.effects.iter_mut().find(|effect| {
            matches!(
                effect,
                WorthQueryApplicationRealizedEffect::UpdateEntity {
                    entity,
                    entity_id: candidate,
                    ..
                } if entity == field.entity() && *candidate == entity_id
            )
        }) {
            Some(WorthQueryApplicationRealizedEffect::UpdateEntity {
                entity,
                entity_id: candidate,
                fields,
            }) if entity == field.entity() && *candidate == entity_id => {
                fields.insert(locator, value.into_foundational_value());
            }
            _ => self
                .effects
                .push(WorthQueryApplicationRealizedEffect::UpdateEntity {
                    entity: field.entity().to_string(),
                    entity_id,
                    fields: BTreeMap::from([(locator, value.into_foundational_value())]),
                }),
        }
        Ok(())
    }
}
