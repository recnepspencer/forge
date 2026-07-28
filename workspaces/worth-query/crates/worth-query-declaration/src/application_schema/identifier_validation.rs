use super::authorization_policy::ApplicationAuthorizationPath;
use super::declaration::ApplicationSchemaDeclarationDenial;
use super::schema_member::{ApplicationOperationProgramTarget, ApplicationSchemaMember};

pub(super) fn validate_schema_header(
    owner: &str,
    name: &str,
) -> Result<(), ApplicationSchemaDeclarationDenial> {
    if owner.starts_with('.') || owner.ends_with('.') || owner.contains("..") {
        return Err(ApplicationSchemaDeclarationDenial::InvalidIdentifier);
    }
    for segment in owner.split('.') {
        validate_simple_identifier(segment)?;
    }
    validate_simple_identifier(name)?;
    Ok(())
}

fn validate_simple_identifier(value: &str) -> Result<(), ApplicationSchemaDeclarationDenial> {
    if value.is_empty()
        || value.trim() != value
        || value.chars().any(char::is_whitespace)
        || value.contains('.')
    {
        return Err(ApplicationSchemaDeclarationDenial::InvalidIdentifier);
    }
    Ok(())
}

pub(super) fn validate_member_identifiers(
    members: &[ApplicationSchemaMember],
) -> Result<(), ApplicationSchemaDeclarationDenial> {
    for member in members {
        match member {
            ApplicationSchemaMember::Entity { entity } => validate_simple_identifier(entity)?,
            ApplicationSchemaMember::Aspect { entity, aspect } => {
                validate_identifiers([entity, aspect])?;
            }
            ApplicationSchemaMember::Field {
                entity,
                aspect,
                field,
                currency,
                ..
            } => {
                validate_identifiers([entity, aspect, field])?;
                if let Some(currency) = currency {
                    validate_simple_identifier(currency)?;
                }
            }
            ApplicationSchemaMember::Relation { relation, from, to } => {
                validate_identifiers([relation, from, to])?;
            }
            ApplicationSchemaMember::PrincipalBinding {
                binding,
                mapping_entity,
                identity_aspect,
                identity_field,
                status_aspect,
                status_field,
                target_relation,
                principal_entity,
                principal_identity_aspect,
                principal_identity_field,
                ..
            } => validate_identifiers([
                binding,
                mapping_entity,
                identity_aspect,
                identity_field,
                status_aspect,
                status_field,
                target_relation,
                principal_entity,
                principal_identity_aspect,
                principal_identity_field,
            ])?,
            ApplicationSchemaMember::Operation { operation, .. } => {
                validate_simple_identifier(operation)?;
            }
            ApplicationSchemaMember::OperationProgram { operation, target } => {
                validate_simple_identifier(operation)?;
                validate_program_target_identifiers(target)?;
            }
            ApplicationSchemaMember::Policy { policy } => validate_simple_identifier(policy)?,
            ApplicationSchemaMember::Ability {
                ability,
                scope_entity,
            } => validate_identifiers([ability, scope_entity])?,
            ApplicationSchemaMember::OperationAbility {
                operation,
                ability,
                scope_entity,
            } => validate_identifiers([operation, ability, scope_entity])?,
            ApplicationSchemaMember::AbilityPolicy {
                ability,
                scope_entity,
                policy,
                paths,
            } => {
                validate_identifiers([ability, scope_entity, policy])?;
                for path in paths {
                    validate_authorization_path(path)?;
                }
            }
            ApplicationSchemaMember::Currency { currency } => {
                validate_simple_identifier(currency)?;
            }
            ApplicationSchemaMember::Effect { effect, .. } => {
                validate_simple_identifier(effect)?;
            }
        }
    }
    Ok(())
}

fn validate_authorization_path(
    path: &ApplicationAuthorizationPath,
) -> Result<(), ApplicationSchemaDeclarationDenial> {
    validate_simple_identifier(path.principal_entity())?;
    validate_simple_identifier(path.scope_entity())?;
    for traversal in path.traversals() {
        validate_simple_identifier(traversal.relation())?;
        validate_simple_identifier(traversal.from())?;
        validate_simple_identifier(traversal.to())?;
    }
    for predicate in path.predicates() {
        validate_simple_identifier(predicate.entity())?;
        validate_simple_identifier(predicate.aspect())?;
        validate_simple_identifier(predicate.field())?;
    }
    Ok(())
}

fn validate_program_target_identifiers(
    target: &ApplicationOperationProgramTarget,
) -> Result<(), ApplicationSchemaDeclarationDenial> {
    match target {
        ApplicationOperationProgramTarget::Create { entity }
        | ApplicationOperationProgramTarget::Delete { entity } => {
            validate_simple_identifier(entity)
        }
        ApplicationOperationProgramTarget::Write {
            entity,
            aspect,
            field,
        } => validate_identifiers([entity, aspect, field]),
        ApplicationOperationProgramTarget::Link { relation, from, to }
        | ApplicationOperationProgramTarget::Unlink { relation, from, to } => {
            validate_identifiers([relation, from, to])
        }
        ApplicationOperationProgramTarget::Emit { effect } => validate_simple_identifier(effect),
    }
}

fn validate_identifiers<'a>(
    identifiers: impl IntoIterator<Item = &'a String>,
) -> Result<(), ApplicationSchemaDeclarationDenial> {
    for identifier in identifiers {
        validate_simple_identifier(identifier)?;
    }
    Ok(())
}
