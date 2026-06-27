#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayUndoDeclaredSourceKind {
    PublicFunction,
    PublicType,
    CompositionWrapper,
}
