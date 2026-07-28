use std::collections::BTreeSet;

use worth_foundational::facade::ScalarAspectType;

use crate::authentication::{
    WorthQueryExternalPrincipalIdentity, WorthQueryPrincipalMappingStatus,
};

use super::authorization_policy::{
    ApplicationAuthorizationPath, ApplicationAuthorizationPathEffect,
    ApplicationAuthorizationTraversalDirection,
};
use super::{
    ApplicationOperationProgramTarget, ApplicationSchemaDeclarationDenial, ApplicationSchemaMember,
};

mod member_collections;

use member_collections::{
    collect_abilities, collect_aspects, collect_currencies, collect_effects, collect_entities,
    collect_fields, collect_operations, collect_policies, collect_principal_entities,
    collect_relations,
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
    members: &'a [ApplicationSchemaMember],
    entities: BTreeSet<&'a str>,
    aspects: BTreeSet<(&'a str, &'a str)>,
    currencies: BTreeSet<&'a str>,
    operations: BTreeSet<&'a str>,
    abilities: BTreeSet<(&'a str, &'a str)>,
    policies: BTreeSet<&'a str>,
    principal_entities: BTreeSet<&'a str>,
    fields: BTreeSet<(&'a str, &'a str, &'a str)>,
    relations: BTreeSet<(&'a str, &'a str, &'a str)>,
    effects: BTreeSet<&'a str>,
}

impl<'a> ClosureIndex<'a> {
    fn new(members: &'a [ApplicationSchemaMember]) -> Self {
        Self {
            members,
            entities: collect_entities(members),
            aspects: collect_aspects(members),
            currencies: collect_currencies(members),
            operations: collect_operations(members),
            abilities: collect_abilities(members),
            policies: collect_policies(members),
            principal_entities: collect_principal_entities(members),
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
            ApplicationSchemaMember::PrincipalBinding {
                mapping_entity,
                identity_aspect,
                identity_field,
                status_aspect,
                status_field,
                target_relation,
                principal_entity,
                principal_identity_aspect,
                principal_identity_field,
                principal_identity_scalar_family,
                principal_identity_value_type,
                ..
            } if !self.principal_binding_dependencies_exist(
                mapping_entity,
                identity_aspect,
                identity_field,
                status_aspect,
                status_field,
                target_relation,
                principal_entity,
                principal_identity_aspect,
                principal_identity_field,
                *principal_identity_scalar_family,
                principal_identity_value_type,
            ) =>
            {
                Err(ApplicationSchemaDeclarationDenial::MissingPrincipalBindingDependency)
            }
            ApplicationSchemaMember::OperationProgram { operation, target }
                if !self.operations.contains(operation.as_str())
                    || !self.program_target_exists(target) =>
            {
                Err(ApplicationSchemaDeclarationDenial::MissingOperationProgramDependency)
            }
            ApplicationSchemaMember::Ability { scope_entity, .. }
                if !self.entities.contains(scope_entity.as_str()) =>
            {
                Err(ApplicationSchemaDeclarationDenial::MissingAbilityDependency)
            }
            ApplicationSchemaMember::OperationAbility {
                operation,
                ability,
                scope_entity,
            } if !self.operations.contains(operation.as_str())
                || !self
                    .abilities
                    .contains(&(ability.as_str(), scope_entity.as_str())) =>
            {
                Err(ApplicationSchemaDeclarationDenial::MissingAbilityDependency)
            }
            ApplicationSchemaMember::AbilityPolicy {
                ability,
                scope_entity,
                policy,
                paths,
            } if !self
                .abilities
                .contains(&(ability.as_str(), scope_entity.as_str()))
                || !self.policies.contains(policy.as_str()) =>
            {
                Err(ApplicationSchemaDeclarationDenial::MissingAbilityPolicyDependency)
            }
            ApplicationSchemaMember::AbilityPolicy {
                scope_entity,
                paths,
                ..
            } if !self.authorization_paths_are_closed(scope_entity, paths) => {
                Err(ApplicationSchemaDeclarationDenial::InvalidAbilityPolicy)
            }
            _ => Ok(()),
        }
    }

    fn authorization_paths_are_closed(
        &self,
        scope_entity: &str,
        paths: &[ApplicationAuthorizationPath],
    ) -> bool {
        !paths.is_empty()
            && paths
                .iter()
                .any(|path| path.effect() == ApplicationAuthorizationPathEffect::Allow)
            && paths
                .iter()
                .all(|path| self.authorization_path_is_closed(scope_entity, path))
    }

    fn authorization_path_is_closed(
        &self,
        scope_entity: &str,
        path: &ApplicationAuthorizationPath,
    ) -> bool {
        if path.scope_entity() != scope_entity
            || !self.principal_entities.contains(path.principal_entity())
        {
            return false;
        }
        let mut current = path.principal_entity();
        let mut entity_by_ordinal = vec![current];
        for traversal in path.traversals() {
            if !self
                .relations
                .contains(&(traversal.relation(), traversal.from(), traversal.to()))
            {
                return false;
            }
            current = match traversal.direction() {
                ApplicationAuthorizationTraversalDirection::Forward
                    if current == traversal.from() =>
                {
                    traversal.to()
                }
                ApplicationAuthorizationTraversalDirection::Reverse
                    if current == traversal.to() =>
                {
                    traversal.from()
                }
                _ => return false,
            };
            entity_by_ordinal.push(current);
        }
        current == scope_entity
            && path.predicates().iter().all(|predicate| {
                entity_by_ordinal
                    .get(predicate.traversal_ordinal())
                    .is_some_and(|entity| *entity == predicate.entity())
                    && self.fields.contains(&(
                        predicate.entity(),
                        predicate.aspect(),
                        predicate.field(),
                    ))
                    && self.field_is_equality_queryable(
                        predicate.entity(),
                        predicate.aspect(),
                        predicate.field(),
                    )
            })
    }

    fn field_is_equality_queryable(&self, entity: &str, aspect: &str, field: &str) -> bool {
        self.members.iter().any(|member| {
            matches!(
                member,
                ApplicationSchemaMember::Field {
                    entity: candidate_entity,
                    aspect: candidate_aspect,
                    field: candidate_field,
                    equality_queryable: true,
                    ..
                } if candidate_entity == entity
                    && candidate_aspect == aspect
                    && candidate_field == field
            )
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn principal_binding_dependencies_exist(
        &self,
        mapping_entity: &str,
        identity_aspect: &str,
        identity_field: &str,
        status_aspect: &str,
        status_field: &str,
        target_relation: &str,
        principal_entity: &str,
        principal_identity_aspect: &str,
        principal_identity_field: &str,
        principal_identity_scalar_family: ScalarAspectType,
        principal_identity_value_type: &str,
    ) -> bool {
        self.entities.contains(mapping_entity)
            && self.entities.contains(principal_entity)
            && self.field_matches(
                mapping_entity,
                identity_aspect,
                identity_field,
                ScalarAspectType::String,
                std::any::type_name::<WorthQueryExternalPrincipalIdentity>(),
                false,
                true,
            )
            && self.field_matches(
                mapping_entity,
                status_aspect,
                status_field,
                ScalarAspectType::Bool,
                std::any::type_name::<WorthQueryPrincipalMappingStatus>(),
                true,
                false,
            )
            && self
                .relations
                .contains(&(target_relation, mapping_entity, principal_entity))
            && self.field_matches(
                principal_entity,
                principal_identity_aspect,
                principal_identity_field,
                principal_identity_scalar_family,
                principal_identity_value_type,
                false,
                true,
            )
    }

    fn field_matches(
        &self,
        entity_name: &str,
        aspect_name: &str,
        field_name: &str,
        expected_family: ScalarAspectType,
        expected_value_type: &str,
        expected_writable: bool,
        equality_required: bool,
    ) -> bool {
        self.members.iter().any(|member| {
            matches!(
                member,
                ApplicationSchemaMember::Field {
                    entity,
                    aspect,
                    field,
                    scalar_family,
                    value_type,
                    writable,
                    equality_queryable,
                    ..
                } if entity == entity_name
                    && aspect == aspect_name
                    && field == field_name
                    && *scalar_family == expected_family
                    && value_type == expected_value_type
                    && *writable == expected_writable
                    && (!equality_required || *equality_queryable)
            )
        })
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
