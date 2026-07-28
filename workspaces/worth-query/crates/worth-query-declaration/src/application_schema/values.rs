use worth_foundational::facade::{AspectValue, InternedString, ScalarAspectType};

/// Exact conversion between an application value and Foundational-native
/// scalar meaning.
///
/// Implementing this trait grants no schema or runtime authority. Installation
/// checks the declared scalar family before a typed intent can be bound.
pub trait TypedApplicationValue: Sized {
    const SCALAR_FAMILY: ScalarAspectType;

    fn into_foundational_value(self) -> AspectValue;
}

pub trait DeclaredApplicationFieldValue {
    type Value: TypedApplicationValue;
}

/// A typed application value that can be recovered from an authoritative
/// Foundational scalar without parsing application strings.
///
/// Principal bindings require this stronger contract for the target
/// principal's stable application identity.
pub trait TypedApplicationIdentityValue: TypedApplicationValue + Clone + Eq + 'static {
    fn from_foundational_value(value: &AspectValue) -> Option<Self>;
}

pub trait TypedCurrencyApplicationValue: TypedApplicationValue {
    type Currency: 'static;
}

impl TypedApplicationValue for bool {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::Bool;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::Bool(self)
    }
}

impl TypedApplicationValue for i64 {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::Int64;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::Int64(self)
    }
}

impl TypedApplicationValue for u64 {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::UInt64;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::UInt64(self)
    }
}

impl TypedApplicationIdentityValue for u64 {
    fn from_foundational_value(value: &AspectValue) -> Option<Self> {
        match value {
            AspectValue::UInt64(value) => Some(*value),
            _ => None,
        }
    }
}

impl TypedApplicationValue for String {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::String;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::String(InternedString::from(self))
    }
}

impl TypedApplicationIdentityValue for String {
    fn from_foundational_value(value: &AspectValue) -> Option<Self> {
        match value {
            AspectValue::String(InternedString::Raw(value)) => Some(value.clone()),
            _ => None,
        }
    }
}

impl TypedApplicationValue for InternedString {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::String;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::String(self)
    }
}

impl TypedApplicationIdentityValue for InternedString {
    fn from_foundational_value(value: &AspectValue) -> Option<Self> {
        match value {
            AspectValue::String(value) => Some(value.clone()),
            _ => None,
        }
    }
}
