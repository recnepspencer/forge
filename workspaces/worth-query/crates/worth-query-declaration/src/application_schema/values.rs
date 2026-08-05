use worth_foundational::facade::{AspectValue, InternedString, ScalarAspectType};

use super::ApplicationFieldPresence;

/// Exact conversion between an application value and Foundational-native
/// scalar meaning.
///
/// Implementing this trait grants no schema or runtime authority. Installation
/// checks the declared scalar family before a typed intent can be bound.
pub trait TypedApplicationValue: Sized {
    const SCALAR_FAMILY: ScalarAspectType;

    fn into_foundational_value(self) -> AspectValue;
}

/// A typed application value that can be recovered from authoritative
/// Foundational meaning without application-side parsing or unchecked casts.
pub trait TypedApplicationReadableValue: TypedApplicationValue + Clone + Eq + 'static {
    fn from_foundational_value(value: &AspectValue) -> Option<Self>;
}

/// A signed fixed-width application value that may participate in a checked
/// provider-derived sum.
///
/// This describes scalar conversion only. Installed field, relation, and
/// operation-read capabilities remain the authority for aggregate projection.
pub trait TypedApplicationSignedAggregateValue: TypedApplicationReadableValue {
    fn from_aggregate_i64(value: i64) -> Self;
}

pub trait DeclaredApplicationFieldValue {
    type Value: TypedApplicationValue;
    const PRESENCE: ApplicationFieldPresence;
}

/// Marker for a schema field whose value must exist on every live record.
pub trait RequiredApplicationFieldValue: DeclaredApplicationFieldValue {}

/// Marker for a schema field whose value may be lawfully absent.
pub trait OptionalApplicationFieldValue: DeclaredApplicationFieldValue {}

/// A typed application value that can be recovered from an authoritative
/// Foundational scalar without parsing application strings.
///
/// Principal bindings require this stronger contract for the target
/// principal's stable application identity.
pub trait TypedApplicationIdentityValue: TypedApplicationReadableValue {}

pub trait TypedCurrencyApplicationValue: TypedApplicationValue {
    type Currency: 'static;
}

impl TypedApplicationValue for bool {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::Bool;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::Bool(self)
    }
}

impl TypedApplicationReadableValue for bool {
    fn from_foundational_value(value: &AspectValue) -> Option<Self> {
        match value {
            AspectValue::Bool(value) => Some(*value),
            _ => None,
        }
    }
}

impl TypedApplicationValue for i64 {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::Int64;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::Int64(self)
    }
}

impl TypedApplicationReadableValue for i64 {
    fn from_foundational_value(value: &AspectValue) -> Option<Self> {
        match value {
            AspectValue::Int64(value) => Some(*value),
            _ => None,
        }
    }
}

impl TypedApplicationSignedAggregateValue for i64 {
    fn from_aggregate_i64(value: i64) -> Self {
        value
    }
}

impl TypedApplicationValue for u64 {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::UInt64;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::UInt64(self)
    }
}

impl TypedApplicationReadableValue for u64 {
    fn from_foundational_value(value: &AspectValue) -> Option<Self> {
        match value {
            AspectValue::UInt64(value) => Some(*value),
            _ => None,
        }
    }
}

impl TypedApplicationIdentityValue for u64 {}

impl TypedApplicationValue for String {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::String;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::String(InternedString::from(self))
    }
}

impl TypedApplicationReadableValue for String {
    fn from_foundational_value(value: &AspectValue) -> Option<Self> {
        match value {
            AspectValue::String(InternedString::Raw(value)) => Some(value.clone()),
            _ => None,
        }
    }
}

impl TypedApplicationIdentityValue for String {}

impl TypedApplicationValue for InternedString {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::String;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::String(self)
    }
}

impl TypedApplicationReadableValue for InternedString {
    fn from_foundational_value(value: &AspectValue) -> Option<Self> {
        match value {
            AspectValue::String(value) => Some(value.clone()),
            _ => None,
        }
    }
}

impl TypedApplicationIdentityValue for InternedString {}
