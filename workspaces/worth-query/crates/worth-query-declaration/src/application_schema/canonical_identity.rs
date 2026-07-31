use worth_foundational::facade::{
    prepare_canonical_basis_sequence, CanonicalBasisEntryKind, CanonicalizationRuleVersion,
};

use super::canonical_authorization_identity::append_authorization_path;
use super::canonical_basis::{ApplicationSchemaCanonicalBasis, APPLICATION_SCHEMA_DOMAIN};
use super::canonical_capability_identity::append_capability_contract;
use super::canonical_decision_read_identity::append_decision_read_target;
use super::canonical_operation_identity::append_operation_target;
use super::{ApplicationSchemaIdentity, ApplicationSchemaMember};

const RULE_VERSION: &str = "worth-query-application-schema-v9";

pub(super) struct ApplicationSchemaCanonicalHeader<'a> {
    pub owner: &'a str,
    pub name: &'a str,
    pub major: u32,
    pub minor: u32,
}

pub(super) fn canonical_identity(
    header: ApplicationSchemaCanonicalHeader<'_>,
    members: &[ApplicationSchemaMember],
) -> ApplicationSchemaIdentity {
    let mut canonical = ApplicationSchemaCanonicalBasis::with_member_capacity(members.len());
    canonical.text("header.owner", header.owner);
    canonical.text("header.name", header.name);
    canonical.u32("header.major", header.major);
    canonical.u32("header.minor", header.minor);
    canonical.usize("member-count", members.len());
    for (index, member) in members.iter().enumerate() {
        append_member(&mut canonical, index, member);
    }
    let version =
        CanonicalizationRuleVersion::new(RULE_VERSION).expect("the schema identity rule is valid");
    let basis = prepare_canonical_basis_sequence(
        version,
        APPLICATION_SCHEMA_DOMAIN,
        canonical.into_entries(),
    )
    .into_result()
    .expect("schema identity loci are unique and typed");
    ApplicationSchemaIdentity::from_canonical_basis(basis)
}

fn append_member(
    basis: &mut ApplicationSchemaCanonicalBasis,
    index: usize,
    member: &ApplicationSchemaMember,
) {
    let prefix = format!("member[{index}]");
    match member {
        ApplicationSchemaMember::Entity { entity } => {
            basis.text(format!("{prefix}.kind"), "entity");
            basis.text(format!("{prefix}.entity"), entity);
        }
        ApplicationSchemaMember::Aspect { entity, aspect } => {
            basis.text(format!("{prefix}.kind"), "aspect");
            basis.text(format!("{prefix}.entity"), entity);
            basis.text(format!("{prefix}.aspect"), aspect);
        }
        ApplicationSchemaMember::Field { .. } => append_schema_field(basis, &prefix, member),
        ApplicationSchemaMember::Relation { relation, from, to } => {
            basis.text(format!("{prefix}.kind"), "relation");
            basis.text(format!("{prefix}.relation"), relation);
            basis.text(format!("{prefix}.from"), from);
            basis.text(format!("{prefix}.to"), to);
        }
        ApplicationSchemaMember::PrincipalBinding { .. } => {
            append_principal_binding(basis, &prefix, member);
        }
        ApplicationSchemaMember::ApplicationQuery { definition } => {
            basis.text(format!("{prefix}.kind"), "application-query");
            basis.extend(definition.canonical_basis().embedded_entries(
                APPLICATION_SCHEMA_DOMAIN,
                &format!("{prefix}.query-meaning"),
                CanonicalBasisEntryKind::Identity,
            ));
        }
        ApplicationSchemaMember::ApplicationCapability { contract } => {
            basis.text(format!("{prefix}.kind"), "application-capability");
            append_capability_contract(basis, &format!("{prefix}.contract"), contract);
        }
        ApplicationSchemaMember::ApplicationCapabilityContext {
            context,
            context_type,
        } => {
            basis.text(format!("{prefix}.kind"), "application-capability-context");
            basis.text(format!("{prefix}.context"), context);
            basis.text(format!("{prefix}.context-type"), context_type);
        }
        ApplicationSchemaMember::ApplicationCapabilityContextEntitySlot {
            context,
            context_type,
            slot,
            slot_type,
            entity,
        } => {
            basis.text(
                format!("{prefix}.kind"),
                "application-capability-context-entity-slot",
            );
            basis.text(format!("{prefix}.context"), context);
            basis.text(format!("{prefix}.context-type"), context_type);
            basis.text(format!("{prefix}.slot"), slot);
            basis.text(format!("{prefix}.slot-type"), slot_type);
            basis.text(format!("{prefix}.entity"), entity);
        }
        ApplicationSchemaMember::ApplicationCapabilityProvenance {
            provenance,
            provenance_type,
        } => {
            basis.text(
                format!("{prefix}.kind"),
                "application-capability-provenance",
            );
            basis.text(format!("{prefix}.provenance"), provenance);
            basis.text(format!("{prefix}.provenance-type"), provenance_type);
        }
        ApplicationSchemaMember::Operation {
            operation,
            input_type,
        } => {
            basis.text(format!("{prefix}.kind"), "operation");
            basis.text(format!("{prefix}.operation"), operation);
            basis.text(format!("{prefix}.input-type"), input_type);
        }
        ApplicationSchemaMember::OperationProgram { operation, target } => {
            basis.text(format!("{prefix}.kind"), "operation-program");
            basis.text(format!("{prefix}.operation"), operation);
            append_operation_target(basis, &format!("{prefix}.target"), target);
        }
        ApplicationSchemaMember::OperationDecisionRead { operation, target } => {
            basis.text(format!("{prefix}.kind"), "operation-decision-read");
            basis.text(format!("{prefix}.operation"), operation);
            append_decision_read_target(basis, &format!("{prefix}.target"), target);
        }
        ApplicationSchemaMember::OperationMutationPrecondition { operation, target } => {
            basis.text(format!("{prefix}.kind"), "operation-mutation-precondition");
            basis.text(format!("{prefix}.operation"), operation);
            basis.text(format!("{prefix}.family"), target.family().canonical_name());
            basis.text(format!("{prefix}.entity"), target.entity());
            basis.text(format!("{prefix}.aspect"), target.aspect());
            basis.text(format!("{prefix}.field"), target.field_name());
        }
        ApplicationSchemaMember::OperationDecisionFactBudget {
            operation,
            maximum_fact_count,
        } => {
            basis.text(format!("{prefix}.kind"), "operation-decision-fact-budget");
            basis.text(format!("{prefix}.operation"), operation);
            basis.usize(format!("{prefix}.maximum-fact-count"), *maximum_fact_count);
        }
        ApplicationSchemaMember::OperationProjectionWorkBudget {
            operation,
            maximum_work_units,
        } => {
            basis.text(format!("{prefix}.kind"), "operation-projection-work-budget");
            basis.text(format!("{prefix}.operation"), operation);
            basis.usize(format!("{prefix}.maximum-work-units"), *maximum_work_units);
        }
        ApplicationSchemaMember::Policy { policy } => {
            basis.text(format!("{prefix}.kind"), "policy");
            basis.text(format!("{prefix}.policy"), policy);
        }
        ApplicationSchemaMember::Ability {
            ability,
            scope_entity,
        } => {
            basis.text(format!("{prefix}.kind"), "ability");
            basis.text(format!("{prefix}.ability"), ability);
            basis.text(format!("{prefix}.scope-entity"), scope_entity);
        }
        ApplicationSchemaMember::OperationAbility {
            operation,
            ability,
            scope_entity,
        } => {
            basis.text(format!("{prefix}.kind"), "operation-ability");
            basis.text(format!("{prefix}.operation"), operation);
            basis.text(format!("{prefix}.ability"), ability);
            basis.text(format!("{prefix}.scope-entity"), scope_entity);
        }
        ApplicationSchemaMember::AbilityPolicy {
            ability,
            scope_entity,
            policy,
            paths,
        } => {
            basis.text(format!("{prefix}.kind"), "ability-policy");
            basis.text(format!("{prefix}.ability"), ability);
            basis.text(format!("{prefix}.scope-entity"), scope_entity);
            basis.text(format!("{prefix}.policy"), policy);
            basis.usize(format!("{prefix}.path-count"), paths.len());
            for (path_index, path) in paths.iter().enumerate() {
                append_authorization_path(basis, &format!("{prefix}.path[{path_index}]"), path);
            }
        }
        ApplicationSchemaMember::Currency { currency } => {
            basis.text(format!("{prefix}.kind"), "currency");
            basis.text(format!("{prefix}.currency"), currency);
        }
        ApplicationSchemaMember::Effect {
            effect,
            payload_type,
        } => {
            basis.text(format!("{prefix}.kind"), "effect");
            basis.text(format!("{prefix}.effect"), effect);
            basis.text(format!("{prefix}.payload-type"), payload_type);
        }
    }
}

fn append_principal_binding(
    basis: &mut ApplicationSchemaCanonicalBasis,
    prefix: &str,
    member: &ApplicationSchemaMember,
) {
    let ApplicationSchemaMember::PrincipalBinding {
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
        principal_identity_scalar_family,
        principal_identity_value_type,
    } = member
    else {
        unreachable!("principal-binding lowering requires a principal-binding member")
    };
    basis.text(format!("{prefix}.kind"), "principal-binding");
    basis.text(format!("{prefix}.binding"), binding);
    basis.text(format!("{prefix}.mapping-entity"), mapping_entity);
    basis.text(format!("{prefix}.identity-aspect"), identity_aspect);
    basis.text(format!("{prefix}.identity-field"), identity_field);
    basis.text(format!("{prefix}.status-aspect"), status_aspect);
    basis.text(format!("{prefix}.status-field"), status_field);
    basis.text(format!("{prefix}.target-relation"), target_relation);
    basis.text(format!("{prefix}.principal-entity"), principal_entity);
    basis.text(
        format!("{prefix}.principal-identity-aspect"),
        principal_identity_aspect,
    );
    basis.text(
        format!("{prefix}.principal-identity-field"),
        principal_identity_field,
    );
    basis.text(
        format!("{prefix}.principal-identity-scalar-family"),
        principal_identity_scalar_family.canonical_name(),
    );
    basis.text(
        format!("{prefix}.principal-identity-value-type"),
        principal_identity_value_type,
    );
}

fn append_schema_field(
    basis: &mut ApplicationSchemaCanonicalBasis,
    prefix: &str,
    member: &ApplicationSchemaMember,
) {
    let ApplicationSchemaMember::Field {
        entity,
        aspect,
        field,
        scalar_family,
        value_type,
        currency,
        writable,
        equality_queryable,
    } = member
    else {
        unreachable!("field lowering requires a field member")
    };
    basis.text(format!("{prefix}.kind"), "field");
    basis.text(format!("{prefix}.entity"), entity);
    basis.text(format!("{prefix}.aspect"), aspect);
    basis.text(format!("{prefix}.field"), field);
    basis.text(
        format!("{prefix}.scalar-family"),
        scalar_family.canonical_name(),
    );
    basis.text(format!("{prefix}.value-type"), value_type);
    basis.optional_text(format!("{prefix}.currency"), currency.as_deref());
    basis.bool(format!("{prefix}.writable"), *writable);
    basis.bool(format!("{prefix}.equality-queryable"), *equality_queryable);
}

#[cfg(test)]
#[path = "canonical_identity_tests.rs"]
mod tests;
