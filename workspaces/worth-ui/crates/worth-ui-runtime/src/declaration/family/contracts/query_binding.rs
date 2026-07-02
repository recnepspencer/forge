#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiQueryBindingDeclarationFamily {
    _sealed: (),
}

impl UiQueryBindingDeclarationFamily {
    pub(crate) const fn new() -> Self {
        Self { _sealed: () }
    }

    pub const fn is_standalone_family(&self) -> bool {
        true
    }
}
