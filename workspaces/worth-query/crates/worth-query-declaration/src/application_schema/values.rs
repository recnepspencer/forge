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

impl TypedApplicationValue for String {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::String;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::String(InternedString::from(self))
    }
}

impl TypedApplicationValue for InternedString {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::String;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::String(self)
    }
}
