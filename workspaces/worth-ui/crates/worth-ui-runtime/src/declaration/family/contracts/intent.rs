#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiIntentDeclarationFamily {
    _sealed: (),
}

impl UiIntentDeclarationFamily {
    pub(crate) const fn new() -> Self {
        Self { _sealed: () }
    }

    pub const fn is_standalone_family(&self) -> bool {
        true
    }
}
