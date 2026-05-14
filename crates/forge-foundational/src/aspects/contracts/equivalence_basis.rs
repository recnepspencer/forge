#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AspectEquivalenceBasis {
    ExactCanonicalValue,
    DeclaredStructFields,
    OpaqueIdentity,
    ReferenceIdentity,
    ContentIdentity,
}
