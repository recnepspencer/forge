#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum UiInspectionScope {
    Graph,
    Measurement,
    Planning,
    Mounting,
    Rebind,
}

impl UiInspectionScope {
    pub fn graph() -> Self {
        Self::Graph
    }

    pub fn measurement() -> Self {
        Self::Measurement
    }

    pub fn planning() -> Self {
        Self::Planning
    }

    pub fn mounting() -> Self {
        Self::Mounting
    }

    pub fn rebind() -> Self {
        Self::Rebind
    }
}
