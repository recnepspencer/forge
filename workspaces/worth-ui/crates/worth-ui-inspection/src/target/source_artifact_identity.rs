#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiSourceArtifactIdentity {
    family: UiSourceArtifactFamily,
    path: String,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum UiSourceArtifactFamily {
    DslModule,
}

impl UiSourceArtifactIdentity {
    pub fn dsl_module(module_path: impl Into<String>) -> Self {
        Self {
            family: UiSourceArtifactFamily::DslModule,
            path: module_path.into(),
        }
    }

    pub fn family(&self) -> UiSourceArtifactFamily {
        self.family
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct UiSourceArtifactGeneration(u64);

impl UiSourceArtifactGeneration {
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }
}
