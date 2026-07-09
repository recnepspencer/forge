#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthServerOperationalRouteKind {
    Health,
    Metrics,
    Preflight,
    DocsExport,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthServerOperationalRoute {
    kind: WorthServerOperationalRouteKind,
    method: String,
    path: String,
}

impl WorthServerOperationalRoute {
    pub(crate) fn new(
        kind: WorthServerOperationalRouteKind,
        method: impl Into<String>,
        path: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            method: method.into(),
            path: path.into(),
        }
    }

    pub fn kind(&self) -> WorthServerOperationalRouteKind {
        self.kind
    }

    pub fn method(&self) -> &str {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}
