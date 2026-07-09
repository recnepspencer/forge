use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
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
