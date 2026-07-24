use worth_foundational::facade::{
    CanonicalBigInt, CanonicalDate, CanonicalDecimal, CanonicalF32, CanonicalF64,
    CanonicalRational, CanonicalTime, CanonicalTimestamp, CanonicalTimestampTz, ContentRefId,
    EntityId, InternedString, StructAspectValue,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WorthQueryArtifactNativeValueView<'a> {
    Null,
    Bool(&'a bool),
    Int8(&'a i8),
    Int16(&'a i16),
    Int32(&'a i32),
    Int64(&'a i64),
    UInt8(&'a u8),
    UInt16(&'a u16),
    UInt32(&'a u32),
    UInt64(&'a u64),
    Float32(&'a CanonicalF32),
    Float64(&'a CanonicalF64),
    Decimal(&'a CanonicalDecimal),
    BigInt(&'a CanonicalBigInt),
    Rational(&'a CanonicalRational),
    String(&'a InternedString),
    Bytes(&'a ContentRefId),
    Uuid(&'a [u8; 16]),
    Date(&'a CanonicalDate),
    Time(&'a CanonicalTime),
    Timestamp(&'a CanonicalTimestamp),
    TimestampTz(&'a CanonicalTimestampTz),
    EntityRef(&'a EntityId),
    ContentRef(&'a ContentRefId),
    Struct(&'a StructAspectValue),
}

impl WorthQueryArtifactNativeValueView<'_> {
    pub fn as_u64(self) -> Option<u64> {
        match self {
            Self::UInt64(value) => Some(*value),
            _ => None,
        }
    }

    pub fn as_f64(self) -> Option<f64> {
        match self {
            Self::Float64(value) => Some((*value).as_f64()),
            _ => None,
        }
    }
}
