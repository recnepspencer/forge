#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectedOverlapFaceDenial {
    human_reason: String,
}

impl ProjectedOverlapFaceDenial {
    pub(crate) fn new(human_reason: impl Into<String>) -> Self {
        Self {
            human_reason: human_reason.into(),
        }
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
