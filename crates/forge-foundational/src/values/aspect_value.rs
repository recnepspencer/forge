use super::references::{ContentRefId, EntityId};
use super::scalar_kind::ScalarAspectType;
use super::scalar_wrappers::{
    CanonicalBigInt, CanonicalDate, CanonicalDecimal, CanonicalF32, CanonicalF64,
    CanonicalRational, CanonicalTime, CanonicalTimestamp, CanonicalTimestampTz, InternedString,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AspectValue {
    Null,
    Bool(bool),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Float32(CanonicalF32),
    Float64(CanonicalF64),
    Decimal(CanonicalDecimal),
    BigInt(CanonicalBigInt),
    Rational(CanonicalRational),
    String(InternedString),
    Bytes(ContentRefId),
    Uuid([u8; 16]),
    Date(CanonicalDate),
    Time(CanonicalTime),
    Timestamp(CanonicalTimestamp),
    TimestampTz(CanonicalTimestampTz),
    EntityRef(EntityId),
    ContentRef(ContentRefId),
}

impl AspectValue {
    pub fn value_family(&self) -> ScalarAspectType {
        match self {
            Self::Null => ScalarAspectType::Null,
            Self::Bool(_) => ScalarAspectType::Bool,
            Self::Int8(_) => ScalarAspectType::Int8,
            Self::Int16(_) => ScalarAspectType::Int16,
            Self::Int32(_) => ScalarAspectType::Int32,
            Self::Int64(_) => ScalarAspectType::Int64,
            Self::UInt8(_) => ScalarAspectType::UInt8,
            Self::UInt16(_) => ScalarAspectType::UInt16,
            Self::UInt32(_) => ScalarAspectType::UInt32,
            Self::UInt64(_) => ScalarAspectType::UInt64,
            Self::Float32(_) => ScalarAspectType::Float32,
            Self::Float64(_) => ScalarAspectType::Float64,
            Self::Decimal(_) => ScalarAspectType::Decimal,
            Self::BigInt(_) => ScalarAspectType::BigInt,
            Self::Rational(_) => ScalarAspectType::Rational,
            Self::String(_) => ScalarAspectType::String,
            Self::Bytes(_) => ScalarAspectType::Bytes,
            Self::Uuid(_) => ScalarAspectType::Uuid,
            Self::Date(_) => ScalarAspectType::Date,
            Self::Time(_) => ScalarAspectType::Time,
            Self::Timestamp(_) => ScalarAspectType::Timestamp,
            Self::TimestampTz(_) => ScalarAspectType::TimestampTz,
            Self::EntityRef(_) => ScalarAspectType::EntityRef,
            Self::ContentRef(_) => ScalarAspectType::ContentRef,
        }
    }
}
