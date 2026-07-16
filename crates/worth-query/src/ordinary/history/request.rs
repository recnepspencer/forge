use super::{
    WorthQueryHistoricalContext, WorthQueryHistoricalPathDeclaration, WorthQueryHistoricalPathKind,
};

#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryHistoricalRequest {
    pub(crate) declaration: WorthQueryHistoricalPathDeclaration,
    pub(crate) context: WorthQueryHistoricalContext,
}

impl WorthQueryHistoricalRequest {
    pub fn path_kind(&self) -> WorthQueryHistoricalPathKind {
        self.declaration.path_kind()
    }

    pub fn context(&self) -> &WorthQueryHistoricalContext {
        &self.context
    }

    pub(crate) fn new(
        declaration: WorthQueryHistoricalPathDeclaration,
        context: WorthQueryHistoricalContext,
    ) -> Self {
        Self {
            declaration,
            context,
        }
    }
}
