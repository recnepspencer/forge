use crate::application_capability::{
    ApplicationCapabilityFieldBinding, ApplicationCapabilityFieldDimension,
    ApplicationCapabilityRelationBinding, ApplicationCapabilityRelationDimension,
    ApplicationCapabilityRule, ApplicationCapabilityValueBinding,
    ErasedApplicationCapabilityContract,
};

use super::canonical_basis::ApplicationSchemaCanonicalBasis;

pub(super) fn append_capability_contract(
    basis: &mut ApplicationSchemaCanonicalBasis,
    prefix: &str,
    contract: &ErasedApplicationCapabilityContract,
) {
    basis.text(format!("{prefix}.name"), contract.name());
    basis.text(format!("{prefix}.operation"), contract.operation());
    basis.text(format!("{prefix}.input-type"), contract.input_type());
    basis.text(format!("{prefix}.grant-entity"), contract.grant_entity());
    append_target(basis, &format!("{prefix}.target"), contract);
    append_constraints(basis, &format!("{prefix}.constraints"), contract);
    append_delegation(basis, &format!("{prefix}.delegation"), contract);
    append_composition(basis, &format!("{prefix}.composition"), contract);
}

fn append_target(
    basis: &mut ApplicationSchemaCanonicalBasis,
    prefix: &str,
    contract: &ErasedApplicationCapabilityContract,
) {
    let target = contract.target();
    append_value_binding(basis, &format!("{prefix}.action"), target.action());
    append_relation(basis, &format!("{prefix}.resource"), target.resource());
    append_relation_dimension(basis, &format!("{prefix}.relation"), target.relation());
    append_field_dimension(basis, &format!("{prefix}.field"), target.field());
    append_value_binding(basis, &format!("{prefix}.purpose"), target.purpose());
}

fn append_constraints(
    basis: &mut ApplicationSchemaCanonicalBasis,
    prefix: &str,
    contract: &ErasedApplicationCapabilityContract,
) {
    let constraints = contract.constraints();
    append_field_dimension(basis, &format!("{prefix}.amount"), constraints.amount());
    match constraints.cardinality() {
        crate::application_capability::ApplicationCapabilityCardinalityDimension::One => {
            basis.text(format!("{prefix}.cardinality"), "one");
        }
        crate::application_capability::ApplicationCapabilityCardinalityDimension::Many => {
            basis.text(format!("{prefix}.cardinality"), "many");
        }
        crate::application_capability::ApplicationCapabilityCardinalityDimension::Bounded(
            limit,
        ) => {
            basis.text(format!("{prefix}.cardinality"), "bounded");
            basis.u32(format!("{prefix}.cardinality-limit"), limit);
        }
    }
    append_field(
        basis,
        &format!("{prefix}.workflow-stage"),
        constraints.workflow_stage(),
    );
    append_field(
        basis,
        &format!("{prefix}.validity.not-before"),
        constraints.validity().not_before(),
    );
    append_field(
        basis,
        &format!("{prefix}.validity.not-after"),
        constraints.validity().not_after(),
    );
    basis.text(format!("{prefix}.context"), constraints.context());
}

fn append_delegation(
    basis: &mut ApplicationSchemaCanonicalBasis,
    prefix: &str,
    contract: &ErasedApplicationCapabilityContract,
) {
    let delegation = contract.delegation();
    append_relation(basis, &format!("{prefix}.parent"), delegation.parent());
    append_relation(basis, &format!("{prefix}.grantor"), delegation.grantor());
    append_relation(basis, &format!("{prefix}.grantee"), delegation.grantee());
    append_field(basis, &format!("{prefix}.limit"), delegation.limit());
    basis.text(format!("{prefix}.provenance"), delegation.provenance());
}

fn append_composition(
    basis: &mut ApplicationSchemaCanonicalBasis,
    prefix: &str,
    contract: &ErasedApplicationCapabilityContract,
) {
    let composition = contract.composition();
    append_rule(
        basis,
        &format!("{prefix}.allow"),
        composition.decision().allow(),
    );
    append_rule(
        basis,
        &format!("{prefix}.deny"),
        composition.decision().deny(),
    );
    append_rule(
        basis,
        &format!("{prefix}.conflict"),
        composition.decision().conflict(),
    );
    append_rule(
        basis,
        &format!("{prefix}.separation-of-duty"),
        composition.actors().separation_of_duty(),
    );
    append_rule(
        basis,
        &format!("{prefix}.distinct-actor"),
        composition.actors().distinct_actor(),
    );
    append_rule(
        basis,
        &format!("{prefix}.delegation"),
        composition.propagation().delegation(),
    );
    append_rule(
        basis,
        &format!("{prefix}.disclosure"),
        composition.propagation().disclosure(),
    );
}

fn append_field(
    basis: &mut ApplicationSchemaCanonicalBasis,
    prefix: &str,
    field: &ApplicationCapabilityFieldBinding,
) {
    basis.text(format!("{prefix}.entity"), field.entity());
    basis.text(format!("{prefix}.aspect"), field.aspect());
    basis.text(format!("{prefix}.field"), field.field());
    basis.text(format!("{prefix}.value-type"), field.value_type());
}

fn append_value_binding(
    basis: &mut ApplicationSchemaCanonicalBasis,
    prefix: &str,
    binding: &ApplicationCapabilityValueBinding,
) {
    append_field(basis, &format!("{prefix}.field-binding"), binding.field());
    basis.aspect_value(format!("{prefix}.value"), binding.value());
}

fn append_relation(
    basis: &mut ApplicationSchemaCanonicalBasis,
    prefix: &str,
    relation: &ApplicationCapabilityRelationBinding,
) {
    basis.text(format!("{prefix}.relation"), relation.relation());
    basis.text(format!("{prefix}.from"), relation.from());
    basis.text(format!("{prefix}.to"), relation.to());
}

fn append_field_dimension(
    basis: &mut ApplicationSchemaCanonicalBasis,
    prefix: &str,
    dimension: &ApplicationCapabilityFieldDimension,
) {
    match dimension {
        ApplicationCapabilityFieldDimension::NotApplicable => {
            basis.text(format!("{prefix}.posture"), "not-applicable");
        }
        ApplicationCapabilityFieldDimension::Bound(field) => {
            basis.text(format!("{prefix}.posture"), "bound");
            append_field(basis, prefix, field);
        }
    }
}

fn append_relation_dimension(
    basis: &mut ApplicationSchemaCanonicalBasis,
    prefix: &str,
    dimension: &ApplicationCapabilityRelationDimension,
) {
    match dimension {
        ApplicationCapabilityRelationDimension::NotApplicable => {
            basis.text(format!("{prefix}.posture"), "not-applicable");
        }
        ApplicationCapabilityRelationDimension::Bound(relation) => {
            basis.text(format!("{prefix}.posture"), "bound");
            append_relation(basis, prefix, relation);
        }
    }
}

fn append_rule(
    basis: &mut ApplicationSchemaCanonicalBasis,
    prefix: &str,
    rule: &ApplicationCapabilityRule,
) {
    match rule {
        ApplicationCapabilityRule::NotApplicable => {
            basis.text(format!("{prefix}.posture"), "not-applicable");
        }
        ApplicationCapabilityRule::Policy(policy) => {
            basis.text(format!("{prefix}.posture"), "policy");
            basis.text(format!("{prefix}.policy"), policy);
        }
    }
}
