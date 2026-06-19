#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeServerOperationalRouteKind {
    Health,
    Metrics,
    Preflight,
    DocsExport,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeServerOperationalRoute {
    kind: ForgeServerOperationalRouteKind,
    method: String,
    path: String,
}

impl ForgeServerOperationalRoute {
    pub(crate) fn new(
        kind: ForgeServerOperationalRouteKind,
        method: impl Into<String>,
        path: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            method: method.into(),
            path: path.into(),
        }
    }

    pub fn kind(&self) -> ForgeServerOperationalRouteKind {
        self.kind
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}
