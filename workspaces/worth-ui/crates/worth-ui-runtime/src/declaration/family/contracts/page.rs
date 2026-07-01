#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiDeclarationQueryBindingProjectionRole {
    Absent,
    Attached,
}

impl UiDeclarationQueryBindingProjectionRole {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiDeclarationIntentProjectionRole {
    Absent,
    Attached,
}

impl UiDeclarationIntentProjectionRole {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiPageStructure;

impl UiPageStructure {
    pub const fn is_root_page(self) -> bool {
        true
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiPageDeclarationFamily {
    query_binding_role: UiDeclarationQueryBindingProjectionRole,
    intent_role: UiDeclarationIntentProjectionRole,
}

impl UiPageDeclarationFamily {
    pub(crate) const fn new(
        query_binding_role: UiDeclarationQueryBindingProjectionRole,
        intent_role: UiDeclarationIntentProjectionRole,
    ) -> Self {
        Self {
            query_binding_role,
            intent_role,
        }
    }

    pub const fn structure(&self) -> UiPageStructure {
        UiPageStructure
    }
}
