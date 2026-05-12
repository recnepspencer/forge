use crate::facade::ResponsibilityArea;

pub fn responsibility() -> ResponsibilityArea {
    ResponsibilityArea::new(
        "canonical_values",
        "canonical value carriers and representation-normalized scalar wrappers",
        "aspect contracts, mutation execution, or runtime storage layout",
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ContentRefId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntityId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalF32(pub u32);

impl CanonicalF32 {
    pub fn from_f32(value: f32) -> Self {
        Self::from_bits(value.to_bits())
    }

    pub fn from_bits(bits: u32) -> Self {
        if f32::from_bits(bits).is_nan() {
            Self(f32::NAN.to_bits())
        } else {
            Self(bits)
        }
    }

    pub fn bits(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalF64(pub u64);

impl CanonicalF64 {
    pub fn from_f64(value: f64) -> Self {
        Self::from_bits(value.to_bits())
    }

    pub fn from_bits(bits: u64) -> Self {
        if f64::from_bits(bits).is_nan() {
            Self(f64::NAN.to_bits())
        } else {
            Self(bits)
        }
    }

    pub fn bits(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalDecimal(pub String);

impl CanonicalDecimal {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalBigInt(pub String);

impl CanonicalBigInt {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalRational {
    pub numerator: CanonicalBigInt,
    pub denominator: CanonicalBigInt,
}

impl CanonicalRational {
    pub fn new(numerator: CanonicalBigInt, denominator: CanonicalBigInt) -> Option<Self> {
        if denominator.as_str() == "0" {
            None
        } else {
            Some(Self {
                numerator,
                denominator,
            })
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalDate {
    pub days_from_unix_epoch: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalTime {
    pub nanos_since_midnight: u64,
}

impl CanonicalTime {
    pub const NANOS_PER_DAY: u64 = 86_400_000_000_000;

    pub fn new(nanos_since_midnight: u64) -> Option<Self> {
        if nanos_since_midnight < Self::NANOS_PER_DAY {
            Some(Self {
                nanos_since_midnight,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalTimestamp {
    pub micros_since_unix_epoch: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalTimestampTz {
    pub utc_micros_since_unix_epoch: i64,
    pub offset_minutes: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalString(String);

impl CanonicalString {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for CanonicalString {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

impl From<String> for CanonicalString {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

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
    String(CanonicalString),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ScalarAspectType {
    Null,
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float32,
    Float64,
    Decimal,
    BigInt,
    Rational,
    String,
    Bytes,
    Uuid,
    Date,
    Time,
    Timestamp,
    TimestampTz,
    EntityRef,
    ContentRef,
}
