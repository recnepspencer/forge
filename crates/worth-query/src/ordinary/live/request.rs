use crate::ordinary::read::{WorthQueryReadContextDeclaration, WorthQueryReadContextKind};

use super::{WorthQueryLiveDeclaration, WorthQueryLiveDeclarationIdentity};

#[derive(Debug, Eq, PartialEq)]
pub struct WorthQueryLiveRequest {
    declaration: WorthQueryLiveDeclaration,
    context: WorthQueryReadContextDeclaration,
}

impl WorthQueryLiveRequest {
    pub fn declaration_identity(&self) -> &WorthQueryLiveDeclarationIdentity {
        self.declaration.identity()
    }

    pub fn context_kind(&self) -> WorthQueryReadContextKind {
        self.context.kind()
    }

    pub(crate) fn into_parts(
        self,
    ) -> (WorthQueryLiveDeclaration, WorthQueryReadContextDeclaration) {
        (self.declaration, self.context)
    }
}

impl WorthQueryLiveDeclaration {
    pub fn using(
        self,
        context: impl Into<WorthQueryReadContextDeclaration>,
    ) -> WorthQueryLiveRequest {
        WorthQueryLiveRequest {
            declaration: self,
            context: context.into(),
        }
    }
}
