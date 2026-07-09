use super::{WorthQueryDesiredAspectValue, WorthQueryParsedAspectTarget};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryParsedDesiredAspect {
    target: WorthQueryParsedAspectTarget,
    desired: WorthQueryDesiredAspectValue,
}

impl WorthQueryParsedDesiredAspect {
    pub(crate) fn new(
        target: WorthQueryParsedAspectTarget,
        desired: WorthQueryDesiredAspectValue,
    ) -> Self {
        Self { target, desired }
    }

    pub(crate) fn target(&self) -> &WorthQueryParsedAspectTarget {
        &self.target
    }

    pub(crate) fn desired(&self) -> &WorthQueryDesiredAspectValue {
        &self.desired
    }
}
