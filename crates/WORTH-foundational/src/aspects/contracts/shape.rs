use crate::aspects::structs::StructAspectShape;
use crate::values::ScalarAspectType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OpaqueAspectType {
    Token,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReferenceAspectType {
    Entity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AspectShape {
    Scalar(ScalarAspectType),
    Struct(StructAspectShape),
    Opaque(OpaqueAspectType),
    Reference(ReferenceAspectType),
    Content,
}
