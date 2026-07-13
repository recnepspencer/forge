#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ContributionKind {
    Component,
    Material,
    System,
}

impl ContributionKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Component => "component",
            Self::Material => "material",
            Self::System => "system",
        }
    }
}
