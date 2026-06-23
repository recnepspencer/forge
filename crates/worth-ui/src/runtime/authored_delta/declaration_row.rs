#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthUiAuthoredDeclarationKind {
    Workspace,
    Page,
    Layout,
    Content,
    Surface,
    Appearance,
    RuntimeBinding,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthUiAuthoredDeltaChangePosture {
    Added,
    Removed,
    Changed,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthUiTouchedAuthoredDeclarationRow {
    kind: WorthUiAuthoredDeclarationKind,
    declaration_name: String,
    change_posture: WorthUiAuthoredDeltaChangePosture,
}

impl WorthUiTouchedAuthoredDeclarationRow {
    pub(crate) fn new(
        kind: WorthUiAuthoredDeclarationKind,
        declaration_name: impl Into<String>,
        change_posture: WorthUiAuthoredDeltaChangePosture,
    ) -> Self {
        Self {
            kind,
            declaration_name: declaration_name.into(),
            change_posture,
        }
    }

    pub fn kind(&self) -> WorthUiAuthoredDeclarationKind {
        self.kind
    }

    pub fn declaration_name(&self) -> &str {
        &self.declaration_name
    }

    pub fn change_posture(&self) -> WorthUiAuthoredDeltaChangePosture {
        self.change_posture
    }
}
