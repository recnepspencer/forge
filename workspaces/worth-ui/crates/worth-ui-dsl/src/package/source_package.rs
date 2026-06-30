#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDslPackage {
    package_name: String,
}

impl WorthUiDslPackage {
    pub fn empty() -> Self {
        Self::named("worth-ui.dsl.empty")
    }

    pub fn named(package_name: impl Into<String>) -> Self {
        Self {
            package_name: package_name.into(),
        }
    }

    pub fn package_name(&self) -> &str {
        &self.package_name
    }
}
