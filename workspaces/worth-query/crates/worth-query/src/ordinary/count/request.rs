use crate::ordinary::read::{WorthQueryReadContextDeclaration, WorthQueryReadContextKind};

use super::{WorthQueryCountDeclaration, WorthQueryCountDeclarationIdentity};

#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryCountRequest {
    declaration: WorthQueryCountDeclaration,
    context: WorthQueryReadContextDeclaration,
}

impl WorthQueryCountRequest {
    pub fn declaration_identity(&self) -> &WorthQueryCountDeclarationIdentity {
        self.declaration.identity()
    }

    pub fn context_kind(&self) -> WorthQueryReadContextKind {
        self.context.kind()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (WorthQueryCountDeclaration, WorthQueryReadContextDeclaration) {
        (self.declaration, self.context)
    }
}

impl WorthQueryCountDeclaration {
    pub fn using(
        self,
        context: impl Into<WorthQueryReadContextDeclaration>,
    ) -> WorthQueryCountRequest {
        WorthQueryCountRequest {
            declaration: self,
            context: context.into(),
        }
    }
}
