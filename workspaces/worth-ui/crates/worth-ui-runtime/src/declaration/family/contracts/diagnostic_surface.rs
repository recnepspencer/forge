#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiDiagnosticSurfaceDeclarationFamily {
    _sealed: (),
}

impl UiDiagnosticSurfaceDeclarationFamily {
    pub(crate) const fn new() -> Self {
        Self { _sealed: () }
    }
}
