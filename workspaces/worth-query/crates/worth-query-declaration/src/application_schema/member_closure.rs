use std::collections::BTreeSet;

use super::{
    ApplicationOperationProgramTarget, ApplicationSchemaDeclarationDenial, ApplicationSchemaMember,
};

pub(super) fn validate_member_closure(
    members: &[ApplicationSchemaMember],
) -> Result<(), ApplicationSchemaDeclarationDenial> {
    let index = ClosureIndex::new(members);
    for member in members {
        index.validate(member)?;
    }
    Ok(())
}

struct ClosureIndex<'a> {
    entities: BTreeSet<&'a str>,
    aspects: BTreeSet<(&'a str, &'a str)>,
    currencies: BTreeSet<&'a str>,
    operations: BTreeSet<&'a str>,
    fields: BTreeSet<(&'a str, &'a str, &'a str)>,
    relations: BTreeSet<(&'a str, &'a str, &'a str)>,
    effects: BTreeSet<&'a str>,
}

impl<'a> ClosureIndex<'a> {
    fn new(members: &'a [ApplicationSchemaMember]) -> Self {
        Self {
            entities: collect_entities(members),
            aspects: collect_aspects(members),
            currencies: collect_currencies(members),
            operations: collect_operations(members),
            fields: collect_fields(members),
            relations: collect_relations(members),
            effects: collect_effects(members),
        }
    }

    fn validate(
        &self,
        member: &ApplicationSchemaMember,
    ) -> Result<(), ApplicationSchemaDeclarationDenial> {
        match member {
            ApplicationSchemaMember::Aspect { entity, .. }
                if !self.entities.contains(entity.as_str()) =>
            {
                Err(ApplicationSchemaDeclarationDenial::MissingEntity)
            }
            ApplicationSchemaMember::Field { entity, aspect, .. }
                if !self.aspects.contains(&(entity.as_str(), aspect.as_str())) =>
            {
                Err(ApplicationSchemaDeclarationDenial::MissingAspect)
            }
            ApplicationSchemaMember::Field {
                currency: Some(currency),
                ..
            } if !self.currencies.contains(currency.as_str()) => {
                Err(ApplicationSchemaDeclarationDenial::MissingCurrency)
            }
            ApplicationSchemaMember::Relation { from, to, .. }
                if !self.entities.contains(from.as_str())
                    || !self.entities.contains(to.as_str()) =>
            {
                Err(ApplicationSchemaDeclarationDenial::MissingEntity)
            }
            ApplicationSchemaMember::OperationProgram { operation, target }
                if !self.operations.contains(operation.as_str())
                    || !self.program_target_exists(target) =>
            {
                Err(ApplicationSchemaDeclarationDenial::MissingOperationProgramDependency)
            }
            _ => Ok(()),
        }
    }

    fn program_target_exists(&self, target: &ApplicationOperationProgramTarget) -> bool {
        match target {
            ApplicationOperationProgramTarget::Create { entity }
            | ApplicationOperationProgramTarget::Delete { entity } => {
                self.entities.contains(entity.as_str())
            }
            ApplicationOperationProgramTarget::Write {
                entity,
                aspect,
                field,
            } => self
                .fields
                .contains(&(entity.as_str(), aspect.as_str(), field.as_str())),
            ApplicationOperationProgramTarget::Link { relation, from, to }
            | ApplicationOperationProgramTarget::Unlink { relation, from, to } => self
                .relations
                .contains(&(relation.as_str(), from.as_str(), to.as_str())),
            ApplicationOperationProgramTarget::Emit { effect } => {
                self.effects.contains(effect.as_str())
            }
        }
    }
}

fn collect_entities(members: &[ApplicationSchemaMember]) -> BTreeSet<&str> {
    members
        .iter()
        .filter_map(|member| match member {
            ApplicationSchemaMember::Entity { entity } => Some(entity.as_str()),
            _ => None,
        })
        .collect()
}

fn collect_aspects(members: &[ApplicationSchemaMember]) -> BTreeSet<(&str, &str)> {
    members
        .iter()
        .filter_map(|member| match member {
            ApplicationSchemaMember::Aspect { entity, aspect } => {
                Some((entity.as_str(), aspect.as_str()))
            }
            _ => None,
        })
        .collect()
}

fn collect_currencies(members: &[ApplicationSchemaMember]) -> BTreeSet<&str> {
    members
        .iter()
        .filter_map(|member| match member {
            ApplicationSchemaMember::Currency { currency } => Some(currency.as_str()),
            _ => None,
        })
        .collect()
}

fn collect_operations(members: &[ApplicationSchemaMember]) -> BTreeSet<&str> {
    members
        .iter()
        .filter_map(|member| match member {
            ApplicationSchemaMember::Operation { operation, .. } => Some(operation.as_str()),
            _ => None,
        })
        .collect()
}

fn collect_fields(members: &[ApplicationSchemaMember]) -> BTreeSet<(&str, &str, &str)> {
    members
        .iter()
        .filter_map(|member| match member {
            ApplicationSchemaMember::Field {
                entity,
                aspect,
                field,
                ..
            } => Some((entity.as_str(), aspect.as_str(), field.as_str())),
            _ => None,
        })
        .collect()
}

fn collect_relations(members: &[ApplicationSchemaMember]) -> BTreeSet<(&str, &str, &str)> {
    members
        .iter()
        .filter_map(|member| match member {
            ApplicationSchemaMember::Relation { relation, from, to } => {
                Some((relation.as_str(), from.as_str(), to.as_str()))
            }
            _ => None,
        })
        .collect()
}

fn collect_effects(members: &[ApplicationSchemaMember]) -> BTreeSet<&str> {
    members
        .iter()
        .filter_map(|member| match member {
            ApplicationSchemaMember::Effect { effect, .. } => Some(effect.as_str()),
            _ => None,
        })
        .collect()
}
