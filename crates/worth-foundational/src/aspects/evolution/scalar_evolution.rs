use crate::values::ScalarAspectType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AspectEvolutionPolicy {
    Frozen,
    AdditiveFieldsAllowed,
    WideningAllowed,
    ExplicitBreakRequired,
}

pub fn scalar_widens(left: ScalarAspectType, right: ScalarAspectType) -> bool {
    matches!(
        (left, right),
        (ScalarAspectType::Int8, ScalarAspectType::Int16)
            | (ScalarAspectType::Int8, ScalarAspectType::Int32)
            | (ScalarAspectType::Int8, ScalarAspectType::Int64)
            | (ScalarAspectType::Int16, ScalarAspectType::Int32)
            | (ScalarAspectType::Int16, ScalarAspectType::Int64)
            | (ScalarAspectType::Int32, ScalarAspectType::Int64)
            | (ScalarAspectType::UInt8, ScalarAspectType::UInt16)
            | (ScalarAspectType::UInt8, ScalarAspectType::UInt32)
            | (ScalarAspectType::UInt8, ScalarAspectType::UInt64)
            | (ScalarAspectType::UInt16, ScalarAspectType::UInt32)
            | (ScalarAspectType::UInt16, ScalarAspectType::UInt64)
            | (ScalarAspectType::UInt32, ScalarAspectType::UInt64)
            | (ScalarAspectType::Float32, ScalarAspectType::Float64)
    )
}
