use crate::declaration::family::contracts::{
    UiDeclarationIntentProjectionRole, UiDeclarationQueryBindingProjectionRole,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiPageSetDeclarationFamily {
    query_binding_role: UiDeclarationQueryBindingProjectionRole,
    intent_role: UiDeclarationIntentProjectionRole,
}

impl UiPageSetDeclarationFamily {
    pub(crate) const fn new(
        query_binding_role: UiDeclarationQueryBindingProjectionRole,
        intent_role: UiDeclarationIntentProjectionRole,
    ) -> Self {
        Self {
            query_binding_role,
            intent_role,
        }
    }
}
