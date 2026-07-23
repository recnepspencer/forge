use super::AbsenceLaw;
use crate::values::ScalarAspectType;

/// Foundational-owned posture of a native aspect value at a typed boundary.
///
/// This is descriptive vocabulary only. It does not validate a value against
/// a contract; owner-issued contract validation remains authoritative.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AspectValuePosture {
    Scalar(ScalarAspectType),
    Struct,
    Absent(AbsenceLaw),
}
