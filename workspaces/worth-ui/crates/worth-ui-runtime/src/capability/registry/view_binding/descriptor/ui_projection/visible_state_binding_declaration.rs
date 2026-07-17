#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VisibleStateBindingDeclaration {
    name: String,
}

impl VisibleStateBindingDeclaration {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn digest_basis(&self) -> String {
        self.name.clone()
    }
}
