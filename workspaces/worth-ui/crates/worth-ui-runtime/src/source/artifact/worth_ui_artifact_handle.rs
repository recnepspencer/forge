use worth_ui_dsl::WorthUiSourceModuleId;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum WorthUiArtifactNodeKind {
    Import,
    Component,
    Surface,
    Binding,
    Token,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct WorthUiArtifactImportHandle {
    module_id: WorthUiSourceModuleId,
    node_index: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct WorthUiArtifactComponentHandle {
    module_id: WorthUiSourceModuleId,
    node_index: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct WorthUiArtifactSurfaceHandle {
    module_id: WorthUiSourceModuleId,
    node_index: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct WorthUiArtifactBindingHandle {
    module_id: WorthUiSourceModuleId,
    node_index: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) struct WorthUiArtifactThemeTokenHandle {
    module_id: WorthUiSourceModuleId,
    node_index: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum WorthUiArtifactHandle {
    Import(WorthUiArtifactImportHandle),
    Component(WorthUiArtifactComponentHandle),
    Surface(WorthUiArtifactSurfaceHandle),
    Binding(WorthUiArtifactBindingHandle),
    Token(WorthUiArtifactThemeTokenHandle),
}

macro_rules! artifact_handle_impl {
    ($name:ident) => {
        impl $name {
            pub(crate) fn new(module_id: WorthUiSourceModuleId, node_index: usize) -> Self {
                Self {
                    module_id,
                    node_index,
                }
            }

            pub(crate) fn module_id(&self) -> &WorthUiSourceModuleId {
                &self.module_id
            }

            pub(crate) fn node_index(&self) -> usize {
                self.node_index
            }
        }
    };
}

artifact_handle_impl!(WorthUiArtifactImportHandle);
artifact_handle_impl!(WorthUiArtifactComponentHandle);
artifact_handle_impl!(WorthUiArtifactSurfaceHandle);
artifact_handle_impl!(WorthUiArtifactBindingHandle);
artifact_handle_impl!(WorthUiArtifactThemeTokenHandle);

impl WorthUiArtifactHandle {
    pub(crate) fn module_id(&self) -> &WorthUiSourceModuleId {
        match self {
            Self::Import(handle) => handle.module_id(),
            Self::Component(handle) => handle.module_id(),
            Self::Surface(handle) => handle.module_id(),
            Self::Binding(handle) => handle.module_id(),
            Self::Token(handle) => handle.module_id(),
        }
    }

    pub(crate) fn node_index(&self) -> usize {
        match self {
            Self::Import(handle) => handle.node_index(),
            Self::Component(handle) => handle.node_index(),
            Self::Surface(handle) => handle.node_index(),
            Self::Binding(handle) => handle.node_index(),
            Self::Token(handle) => handle.node_index(),
        }
    }

    pub(crate) fn kind(&self) -> WorthUiArtifactNodeKind {
        match self {
            Self::Import(_) => WorthUiArtifactNodeKind::Import,
            Self::Component(_) => WorthUiArtifactNodeKind::Component,
            Self::Surface(_) => WorthUiArtifactNodeKind::Surface,
            Self::Binding(_) => WorthUiArtifactNodeKind::Binding,
            Self::Token(_) => WorthUiArtifactNodeKind::Token,
        }
    }
}
