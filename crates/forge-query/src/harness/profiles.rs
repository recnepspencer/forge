#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CertificationProfile {
    DirectConstruction,
    BuilderReordering,
    BindingVariation,
    ReplayParity,
}
