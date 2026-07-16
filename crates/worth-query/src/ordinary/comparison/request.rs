use super::{
    WorthQueryComparisonContext, WorthQueryComparisonIntent, WorthQueryComparisonRefinement,
};

#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryComparisonRequest {
    pub(crate) declaration: WorthQueryComparisonRefinement,
    pub(crate) context: WorthQueryComparisonContext,
}

impl WorthQueryComparisonRequest {
    pub fn intent(&self) -> WorthQueryComparisonIntent {
        self.declaration.intent()
    }

    pub fn context(&self) -> &WorthQueryComparisonContext {
        &self.context
    }

    pub(crate) fn new(
        declaration: WorthQueryComparisonRefinement,
        context: WorthQueryComparisonContext,
    ) -> Self {
        Self {
            declaration,
            context,
        }
    }
}
