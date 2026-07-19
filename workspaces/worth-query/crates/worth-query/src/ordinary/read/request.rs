use super::{
    WorthQueryReadContextDeclaration, WorthQueryReadContextKind, WorthQueryReadDeclaration,
    WorthQueryReadDeclarationIdentity,
};

#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryReadRequest {
    declaration: WorthQueryReadDeclaration,
    context: WorthQueryReadContextDeclaration,
}

impl WorthQueryReadRequest {
    pub fn declaration_identity(&self) -> &WorthQueryReadDeclarationIdentity {
        self.declaration.identity()
    }

    pub fn context_kind(&self) -> WorthQueryReadContextKind {
        self.context.kind()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (WorthQueryReadDeclaration, WorthQueryReadContextDeclaration) {
        (self.declaration, self.context)
    }

    pub(crate) fn new(
        declaration: WorthQueryReadDeclaration,
        context: WorthQueryReadContextDeclaration,
    ) -> Self {
        Self {
            declaration,
            context,
        }
    }
}

impl WorthQueryReadDeclaration {
    pub fn using(
        self,
        context: impl Into<WorthQueryReadContextDeclaration>,
    ) -> WorthQueryReadRequest {
        WorthQueryReadRequest::new(self, context.into())
    }
}
