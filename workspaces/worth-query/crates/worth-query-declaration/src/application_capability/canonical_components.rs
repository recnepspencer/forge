use worth_foundational::facade::{
    canonical_basis_value_for_aspect_value, CanonicalBasisValue, CanonicalIntegerWidth,
};

use super::{
    canonical_composition_components::append_composition, ApplicationCapabilityFieldBinding,
    ApplicationCapabilityFieldDimension, ApplicationCapabilityRelationBinding,
    ApplicationCapabilityRelationDimension, ApplicationCapabilityValueBinding,
    ErasedApplicationCapabilityContract,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApplicationCapabilityCanonicalComponent {
    locus: String,
    value: CanonicalBasisValue,
}

impl ApplicationCapabilityCanonicalComponent {
    pub(super) fn new(locus: impl Into<String>, value: CanonicalBasisValue) -> Self {
        Self {
            locus: locus.into(),
            value,
        }
    }

    pub fn locus(&self) -> &str {
        &self.locus
    }

    pub const fn value(&self) -> &CanonicalBasisValue {
        &self.value
    }
}

pub fn application_capability_canonical_components(
    contract: &ErasedApplicationCapabilityContract,
) -> Vec<ApplicationCapabilityCanonicalComponent> {
    let mut components = Vec::with_capacity(80);
    text(&mut components, "name", contract.name());
    text(
        &mut components,
        "capability-type",
        contract.capability_type(),
    );
    text(&mut components, "operation", contract.operation());
    text(&mut components, "operation-type", contract.operation_type());
    text(&mut components, "input-type", contract.input_type());
    text(&mut components, "grant-entity", contract.grant_entity());
    append_target(&mut components, contract);
    append_constraints(&mut components, contract);
    append_delegation(&mut components, contract);
    append_composition(&mut components, contract);
    components
}

fn append_target(
    components: &mut Vec<ApplicationCapabilityCanonicalComponent>,
    contract: &ErasedApplicationCapabilityContract,
) {
    let target = contract.target();
    append_value_binding(components, "target.action", target.action());
    append_relation(components, "target.resource", target.resource());
    append_relation_dimension(components, "target.relation", target.relation());
    append_field_dimension(components, "target.field", target.field());
    append_value_binding(components, "target.purpose", target.purpose());
}

fn append_constraints(
    components: &mut Vec<ApplicationCapabilityCanonicalComponent>,
    contract: &ErasedApplicationCapabilityContract,
) {
    let constraints = contract.constraints();
    append_field_dimension(components, "constraints.amount", constraints.amount());
    match constraints.cardinality() {
        super::ApplicationCapabilityCardinalityDimension::One => {
            text(components, "constraints.cardinality", "one");
        }
        super::ApplicationCapabilityCardinalityDimension::Many => {
            text(components, "constraints.cardinality", "many");
        }
        super::ApplicationCapabilityCardinalityDimension::Bounded(limit) => {
            text(components, "constraints.cardinality", "bounded");
            unsigned(components, "constraints.cardinality-limit", limit);
        }
    }
    append_field(
        components,
        "constraints.workflow-stage",
        constraints.workflow_stage(),
    );
    append_field(
        components,
        "constraints.validity.not-before",
        constraints.validity().not_before(),
    );
    append_field(
        components,
        "constraints.validity.not-after",
        constraints.validity().not_after(),
    );
    text(components, "constraints.context", constraints.context());
}

fn append_delegation(
    components: &mut Vec<ApplicationCapabilityCanonicalComponent>,
    contract: &ErasedApplicationCapabilityContract,
) {
    let delegation = contract.delegation();
    append_relation(components, "delegation.parent", delegation.parent());
    append_relation(components, "delegation.grantor", delegation.grantor());
    append_relation(components, "delegation.grantee", delegation.grantee());
    append_field(components, "delegation.limit", delegation.limit());
    text(components, "delegation.provenance", delegation.provenance());
}

pub(super) fn append_field(
    components: &mut Vec<ApplicationCapabilityCanonicalComponent>,
    prefix: &str,
    field: &ApplicationCapabilityFieldBinding,
) {
    text(components, format!("{prefix}.entity"), field.entity());
    text(components, format!("{prefix}.aspect"), field.aspect());
    text(components, format!("{prefix}.field"), field.field());
    text(
        components,
        format!("{prefix}.value-type"),
        field.value_type(),
    );
}

pub(super) fn append_value_binding(
    components: &mut Vec<ApplicationCapabilityCanonicalComponent>,
    prefix: &str,
    binding: &ApplicationCapabilityValueBinding,
) {
    append_field(
        components,
        &format!("{prefix}.field-binding"),
        binding.field(),
    );
    components.push(component(
        format!("{prefix}.value"),
        canonical_basis_value_for_aspect_value(binding.value()),
    ));
}

fn append_relation(
    components: &mut Vec<ApplicationCapabilityCanonicalComponent>,
    prefix: &str,
    relation: &ApplicationCapabilityRelationBinding,
) {
    text(
        components,
        format!("{prefix}.relation"),
        relation.relation(),
    );
    text(components, format!("{prefix}.from"), relation.from());
    text(components, format!("{prefix}.to"), relation.to());
}

fn append_field_dimension(
    components: &mut Vec<ApplicationCapabilityCanonicalComponent>,
    prefix: &str,
    dimension: &ApplicationCapabilityFieldDimension,
) {
    match dimension {
        ApplicationCapabilityFieldDimension::NotApplicable => {
            text(components, format!("{prefix}.posture"), "not-applicable");
        }
        ApplicationCapabilityFieldDimension::Bound(field) => {
            text(components, format!("{prefix}.posture"), "bound");
            append_field(components, prefix, field);
        }
    }
}

fn append_relation_dimension(
    components: &mut Vec<ApplicationCapabilityCanonicalComponent>,
    prefix: &str,
    dimension: &ApplicationCapabilityRelationDimension,
) {
    match dimension {
        ApplicationCapabilityRelationDimension::NotApplicable => {
            text(components, format!("{prefix}.posture"), "not-applicable");
        }
        ApplicationCapabilityRelationDimension::Bound(relation) => {
            text(components, format!("{prefix}.posture"), "bound");
            append_relation(components, prefix, relation);
        }
    }
}

pub(super) fn text(
    components: &mut Vec<ApplicationCapabilityCanonicalComponent>,
    locus: impl Into<String>,
    value: impl AsRef<str>,
) {
    components.push(component(
        locus,
        CanonicalBasisValue::ExactText(value.as_ref().to_owned().into()),
    ));
}

pub(super) fn unsigned(
    components: &mut Vec<ApplicationCapabilityCanonicalComponent>,
    locus: impl Into<String>,
    value: u32,
) {
    components.push(component(
        locus,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits32,
            value: value.into(),
        },
    ));
}

pub(super) fn structural_count(
    components: &mut Vec<ApplicationCapabilityCanonicalComponent>,
    locus: impl Into<String>,
    value: usize,
) {
    components.push(component(
        locus,
        CanonicalBasisValue::UnsignedInteger {
            width: CanonicalIntegerWidth::Bits64,
            value: u64::try_from(value)
                .expect("capability structural counts fit in u64")
                .into(),
        },
    ));
}

fn component(
    locus: impl Into<String>,
    value: CanonicalBasisValue,
) -> ApplicationCapabilityCanonicalComponent {
    ApplicationCapabilityCanonicalComponent::new(locus, value)
}
