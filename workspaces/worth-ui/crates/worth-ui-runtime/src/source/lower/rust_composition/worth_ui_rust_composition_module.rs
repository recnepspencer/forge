use crate::source::{WorthUiArtifactInputBodyAtom, WorthUiRustAuthoredArtifactInputModule};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiRustCompositionModule {
    authored_module: WorthUiRustAuthoredArtifactInputModule,
}

impl WorthUiRustCompositionModule {
    pub(crate) fn new(relative_module_path: impl Into<String>) -> Self {
        Self {
            authored_module: WorthUiRustAuthoredArtifactInputModule::new(relative_module_path),
        }
    }

    pub(crate) fn import(mut self, target_module_path: impl Into<String>) -> Self {
        self.authored_module = self.authored_module.with_import(target_module_path);
        self
    }

    pub(crate) fn component(mut self, name_text: impl Into<String>) -> Self {
        self.authored_module = self.authored_module.with_component(name_text);
        self
    }

    pub(crate) fn component_authored_identity(
        mut self,
        name_text: impl Into<String>,
        authored_identity: impl Into<String>,
    ) -> Self {
        self.authored_module = self
            .authored_module
            .with_component_authored_identity(name_text, authored_identity);
        self
    }

    pub(crate) fn component_body_atoms(
        mut self,
        name_text: impl Into<String>,
        body_atoms: impl IntoIterator<Item = WorthUiArtifactInputBodyAtom>,
    ) -> Self {
        self.authored_module = self
            .authored_module
            .with_component_body_atoms(name_text, body_atoms);
        self
    }

    pub(crate) fn component_body_atoms_and_authored_identity(
        mut self,
        name_text: impl Into<String>,
        authored_identity: impl Into<String>,
        body_atoms: impl IntoIterator<Item = WorthUiArtifactInputBodyAtom>,
    ) -> Self {
        self.authored_module = self
            .authored_module
            .with_component_body_atoms_and_authored_identity(
                name_text,
                authored_identity,
                body_atoms,
            );
        self
    }

    pub(crate) fn surface(mut self, name_text: impl Into<String>) -> Self {
        self.authored_module = self.authored_module.with_surface(name_text);
        self
    }

    pub(crate) fn surface_authored_identity(
        mut self,
        name_text: impl Into<String>,
        authored_identity: impl Into<String>,
    ) -> Self {
        self.authored_module = self
            .authored_module
            .with_surface_authored_identity(name_text, authored_identity);
        self
    }

    pub(crate) fn surface_body_atoms(
        mut self,
        name_text: impl Into<String>,
        body_atoms: impl IntoIterator<Item = WorthUiArtifactInputBodyAtom>,
    ) -> Self {
        self.authored_module = self
            .authored_module
            .with_surface_body_atoms(name_text, body_atoms);
        self
    }

    pub(crate) fn binding(mut self, name_text: impl Into<String>) -> Self {
        self.authored_module = self.authored_module.with_binding(name_text);
        self
    }

    pub(crate) fn binding_authored_identity(
        mut self,
        name_text: impl Into<String>,
        authored_identity: impl Into<String>,
    ) -> Self {
        self.authored_module = self
            .authored_module
            .with_binding_authored_identity(name_text, authored_identity);
        self
    }

    pub(crate) fn binding_body_atoms(
        mut self,
        name_text: impl Into<String>,
        body_atoms: impl IntoIterator<Item = WorthUiArtifactInputBodyAtom>,
    ) -> Self {
        self.authored_module = self
            .authored_module
            .with_binding_body_atoms(name_text, body_atoms);
        self
    }

    pub(crate) fn token(
        mut self,
        name_text: impl Into<String>,
        value_text: impl Into<String>,
    ) -> Self {
        self.authored_module = self.authored_module.with_token(name_text, value_text);
        self
    }

    pub(crate) fn token_authored_identity(
        mut self,
        name_text: impl Into<String>,
        authored_identity: impl Into<String>,
        value_text: impl Into<String>,
    ) -> Self {
        self.authored_module = self.authored_module.with_token_authored_identity(
            name_text,
            authored_identity,
            value_text,
        );
        self
    }

    pub(super) fn declaration_count(&self) -> usize {
        self.authored_module.declarations().len()
    }

    pub(super) fn authored_module(&self) -> &WorthUiRustAuthoredArtifactInputModule {
        &self.authored_module
    }
}
