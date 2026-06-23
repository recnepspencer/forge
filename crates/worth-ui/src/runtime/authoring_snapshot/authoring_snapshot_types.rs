use crate::runtime::authoring_snapshot::{
    WorthUiAppearanceRecipeCatalog, WorthUiAuthoredSurfaceCatalog,
    WorthUiAuthoredSurfacePropsCatalog, WorthUiAuthoringSnapshotDigest, WorthUiPageInstanceCatalog,
    WorthUiPageTemplateCatalog, WorthUiRuntimeBindingCatalog, WorthUiWorkspaceShellCatalog,
};
use crate::source::{WorthUiContentSlotCatalog, WorthUiLayoutTopologyCatalog};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiActiveAuthoringSnapshotWitness {
    _private: (),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCandidateRuntimeAuthoringSnapshot {
    workspace_shell: WorthUiWorkspaceShellCatalog,
    page_templates: WorthUiPageTemplateCatalog,
    page_instances: WorthUiPageInstanceCatalog,
    layout_topology: WorthUiLayoutTopologyCatalog,
    content_slots: WorthUiContentSlotCatalog,
    authored_surfaces: WorthUiAuthoredSurfaceCatalog,
    authored_surface_props: WorthUiAuthoredSurfacePropsCatalog,
    appearance_recipes: WorthUiAppearanceRecipeCatalog,
    runtime_bindings: WorthUiRuntimeBindingCatalog,
    digest: WorthUiAuthoringSnapshotDigest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeAuthoringSnapshot {
    workspace_shell: WorthUiWorkspaceShellCatalog,
    page_templates: WorthUiPageTemplateCatalog,
    page_instances: WorthUiPageInstanceCatalog,
    layout_topology: WorthUiLayoutTopologyCatalog,
    content_slots: WorthUiContentSlotCatalog,
    authored_surfaces: WorthUiAuthoredSurfaceCatalog,
    authored_surface_props: WorthUiAuthoredSurfacePropsCatalog,
    appearance_recipes: WorthUiAppearanceRecipeCatalog,
    runtime_bindings: WorthUiRuntimeBindingCatalog,
    digest: WorthUiAuthoringSnapshotDigest,
    witness: WorthUiActiveAuthoringSnapshotWitness,
}

impl WorthUiCandidateRuntimeAuthoringSnapshot {
    pub(crate) fn new(
        workspace_shell: WorthUiWorkspaceShellCatalog,
        page_templates: WorthUiPageTemplateCatalog,
        page_instances: WorthUiPageInstanceCatalog,
        layout_topology: WorthUiLayoutTopologyCatalog,
        content_slots: WorthUiContentSlotCatalog,
        authored_surfaces: WorthUiAuthoredSurfaceCatalog,
        authored_surface_props: WorthUiAuthoredSurfacePropsCatalog,
        appearance_recipes: WorthUiAppearanceRecipeCatalog,
        runtime_bindings: WorthUiRuntimeBindingCatalog,
        digest: WorthUiAuthoringSnapshotDigest,
    ) -> Self {
        Self {
            workspace_shell,
            page_templates,
            page_instances,
            layout_topology,
            content_slots,
            authored_surfaces,
            authored_surface_props,
            appearance_recipes,
            runtime_bindings,
            digest,
        }
    }

    pub(crate) fn activate(self) -> WorthUiRuntimeAuthoringSnapshot {
        WorthUiRuntimeAuthoringSnapshot {
            workspace_shell: self.workspace_shell,
            page_templates: self.page_templates,
            page_instances: self.page_instances,
            layout_topology: self.layout_topology,
            content_slots: self.content_slots,
            authored_surfaces: self.authored_surfaces,
            authored_surface_props: self.authored_surface_props,
            appearance_recipes: self.appearance_recipes,
            runtime_bindings: self.runtime_bindings,
            digest: self.digest,
            witness: WorthUiActiveAuthoringSnapshotWitness { _private: () },
        }
    }

    pub fn digest(&self) -> WorthUiAuthoringSnapshotDigest {
        self.digest
    }

    pub(crate) fn workspace_shell(&self) -> &WorthUiWorkspaceShellCatalog {
        &self.workspace_shell
    }

    pub(crate) fn page_templates(&self) -> &WorthUiPageTemplateCatalog {
        &self.page_templates
    }

    pub fn layout_topology(&self) -> &WorthUiLayoutTopologyCatalog {
        &self.layout_topology
    }

    pub fn content_slots(&self) -> &WorthUiContentSlotCatalog {
        &self.content_slots
    }

    pub fn authored_surfaces(&self) -> &WorthUiAuthoredSurfaceCatalog {
        &self.authored_surfaces
    }

    pub fn authored_surface_props(&self) -> &WorthUiAuthoredSurfacePropsCatalog {
        &self.authored_surface_props
    }

    pub(crate) fn appearance_recipes(&self) -> &WorthUiAppearanceRecipeCatalog {
        &self.appearance_recipes
    }

    pub(crate) fn runtime_bindings(&self) -> &WorthUiRuntimeBindingCatalog {
        &self.runtime_bindings
    }
}

impl WorthUiRuntimeAuthoringSnapshot {
    pub fn workspace_shell(&self) -> &WorthUiWorkspaceShellCatalog {
        &self.workspace_shell
    }

    pub fn page_templates(&self) -> &WorthUiPageTemplateCatalog {
        &self.page_templates
    }

    pub fn page_instances(&self) -> &WorthUiPageInstanceCatalog {
        &self.page_instances
    }

    pub fn layout_topology(&self) -> &WorthUiLayoutTopologyCatalog {
        &self.layout_topology
    }

    pub fn content_slots(&self) -> &WorthUiContentSlotCatalog {
        &self.content_slots
    }

    pub fn authored_surfaces(&self) -> &WorthUiAuthoredSurfaceCatalog {
        &self.authored_surfaces
    }

    pub fn authored_surface_props(&self) -> &WorthUiAuthoredSurfacePropsCatalog {
        &self.authored_surface_props
    }

    pub fn appearance_recipes(&self) -> &WorthUiAppearanceRecipeCatalog {
        &self.appearance_recipes
    }

    pub fn runtime_bindings(&self) -> &WorthUiRuntimeBindingCatalog {
        &self.runtime_bindings
    }

    pub fn digest(&self) -> WorthUiAuthoringSnapshotDigest {
        self.digest
    }

    pub fn witness(&self) -> &WorthUiActiveAuthoringSnapshotWitness {
        &self.witness
    }
}
