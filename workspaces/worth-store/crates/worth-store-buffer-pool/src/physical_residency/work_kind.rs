#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpeculativePhysicalWorkKind {
    ReadAhead,
    Prefetch,
    WriteBehind,
}
