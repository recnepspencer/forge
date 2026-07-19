use crate::authoring::WorthQueryPredicateOperand;

pub trait TypedSchemaRoot {
    const ROOT_ENTITY: &'static str;
}

pub trait TypedSchemaField {
    type Schema: TypedSchemaRoot;

    const ASPECT: &'static str;
    const FIELD: &'static str;

    fn default_delivered_name() -> &'static str {
        Self::FIELD
    }
}

pub trait TypedProjectableField: TypedSchemaField {}

pub trait TypedEqualityField: TypedSchemaField {
    type Value;

    fn into_scalar(value: Self::Value) -> WorthQueryPredicateOperand;
}

pub trait TypedNativeComparableField: TypedEqualityField {}
pub trait TypedStringContainsField: TypedSchemaField {}
pub trait TypedMembershipField: TypedEqualityField {}
pub trait TypedPresenceField: TypedSchemaField {}
pub trait TypedOrderableField: TypedSchemaField {}

pub trait TypedTraversalRelation {
    type Schema: TypedSchemaRoot;

    const RELATION: &'static str;
}
