use super::{ForgeQueryDesiredAspectValue, ForgeQueryParsedAspectTarget};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryParsedDesiredAspect {
    target: ForgeQueryParsedAspectTarget,
    desired: ForgeQueryDesiredAspectValue,
}

impl ForgeQueryParsedDesiredAspect {
    pub(crate) fn new(
        target: ForgeQueryParsedAspectTarget,
        desired: ForgeQueryDesiredAspectValue,
    ) -> Self {
        Self { target, desired }
    }

    pub(crate) fn target(&self) -> &ForgeQueryParsedAspectTarget {
        &self.target
    }

    pub(crate) fn desired(&self) -> &ForgeQueryDesiredAspectValue {
        &self.desired
    }
}
