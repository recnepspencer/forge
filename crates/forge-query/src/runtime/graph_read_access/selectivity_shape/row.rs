use super::super::{
    ForgeQueryGraphReadAdmittedSchemaFieldKind, ForgeQueryPredicateOperandOperator,
    ForgeQueryPredicateSelectivityClass,
};
use forge_foundational::facade::{AspectKey, FieldKey};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBooleanPredicateSelectivityRow {
    aspect: AspectKey,
    field: FieldKey,
    family: String,
    operand_identity: String,
    operator: ForgeQueryPredicateOperandOperator,
    normalized_operand_values: Vec<String>,
    field_kind: ForgeQueryGraphReadAdmittedSchemaFieldKind,
    selectivity_class: ForgeQueryPredicateSelectivityClass,
    pre_traversal_eligible: bool,
}

impl ForgeQueryBooleanPredicateSelectivityRow {
    pub fn native_aspect_key(&self) -> &AspectKey {
        &self.aspect
    }

    pub fn native_field_key(&self) -> &FieldKey {
        &self.field
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn operand_identity(&self) -> &str {
        &self.operand_identity
    }

    pub fn operator(&self) -> &ForgeQueryPredicateOperandOperator {
        &self.operator
    }

    pub fn normalized_operand_values(&self) -> &[String] {
        &self.normalized_operand_values
    }

    pub fn field_kind(&self) -> &ForgeQueryGraphReadAdmittedSchemaFieldKind {
        &self.field_kind
    }

    pub fn selectivity_class(&self) -> &ForgeQueryPredicateSelectivityClass {
        &self.selectivity_class
    }

    pub fn is_pre_traversal_eligible(&self) -> bool {
        self.pre_traversal_eligible
    }

    pub(crate) fn new(
        aspect: AspectKey,
        field: FieldKey,
        family: impl Into<String>,
        operator: ForgeQueryPredicateOperandOperator,
        normalized_operand_values: Vec<String>,
        field_kind: ForgeQueryGraphReadAdmittedSchemaFieldKind,
        selectivity_class: ForgeQueryPredicateSelectivityClass,
    ) -> Self {
        let pre_traversal_eligible = selectivity_class.is_pre_traversal_eligible();
        let operand_identity = format!(
            "{}:{}",
            operator.as_str(),
            normalized_operand_values.join("|")
        );
        Self {
            aspect,
            field,
            family: family.into(),
            operand_identity,
            operator,
            normalized_operand_values,
            field_kind,
            selectivity_class,
            pre_traversal_eligible,
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "predicate_selectivity:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            self.aspect.as_str(),
            self.field.as_str(),
            self.family,
            self.operand_identity,
            self.operator.as_str(),
            self.normalized_operand_values.join("|"),
            self.field_kind.as_str(),
            self.selectivity_class.as_str(),
            self.pre_traversal_eligible
        )
    }
}
