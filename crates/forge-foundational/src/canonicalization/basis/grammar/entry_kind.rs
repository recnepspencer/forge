#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CanonicalBasisEntryKind {
    Header,
    Shape,
    Value,
    Field,
    Mask,
    StateAspect,
    PatchOperation,
    Identity,
    Locator,
    Profile,
    CompatibilityOrigin,
    Cost,
    Future(&'static str),
}
