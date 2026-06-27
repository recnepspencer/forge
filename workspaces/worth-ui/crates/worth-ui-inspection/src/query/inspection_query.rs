use crate::{UiInspectionScope, UiInspectionTarget};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiInspectionQuery {
    target: UiInspectionTarget,
    scope: UiInspectionScope,
}

impl UiInspectionQuery {
    pub fn new(target: UiInspectionTarget, scope: UiInspectionScope) -> Self {
        Self { target, scope }
    }

    pub fn target(&self) -> &UiInspectionTarget {
        &self.target
    }

    pub fn scope(&self) -> UiInspectionScope {
        self.scope
    }
}
