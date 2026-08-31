use std::collections::BTreeMap;

#[cfg(test)]
use worth_ui_dsl::{UiDslLoweringReceipt, WorthUiDslCompiler};
use worth_ui_dsl::{
    UiDslSemanticArtifactSpec, WorthUiRustAuthoredArtifactInput,
    WorthUiRustAuthoredArtifactInputModule, WorthUiSemanticArtifactDeclaration,
};

#[derive(Clone)]
pub(crate) struct WorthUiRustAuthoredDeclarationFixture {
    specs: Vec<UiDslSemanticArtifactSpec>,
}

impl WorthUiRustAuthoredDeclarationFixture {
    pub(crate) fn empty() -> Self {
        Self { specs: Vec::new() }
    }

    pub(crate) fn named(_diagnostic_name: impl Into<String>) -> Self {
        Self::empty()
    }

    pub(crate) fn with_semantic_artifact_spec(mut self, spec: UiDslSemanticArtifactSpec) -> Self {
        self.specs.push(spec);
        self
    }

    #[cfg(test)]
    pub(crate) fn admit_semantic_artifact(
        &self,
        spec: UiDslSemanticArtifactSpec,
    ) -> UiDslLoweringReceipt {
        let package = WorthUiDslCompiler::compile_rust_authored(
            &self.clone().with_semantic_artifact_spec(spec).into_input(),
        )
        .expect("semantic declaration fixture should compile");
        package
            .declaration_lowering_receipts()
            .into_iter()
            .last()
            .expect("incoming declaration should mint one lowering receipt")
    }

    pub(crate) fn into_input(self) -> WorthUiRustAuthoredArtifactInput {
        rust_authored_input_from_semantic_specs(self.specs)
    }
}

fn rust_authored_input_from_semantic_specs(
    specs: impl IntoIterator<Item = UiDslSemanticArtifactSpec>,
) -> WorthUiRustAuthoredArtifactInput {
    let mut modules = BTreeMap::<String, WorthUiRustAuthoredArtifactInputModule>::new();
    for spec in specs {
        let artifact = spec.into_semantic_artifact();
        push_artifact(&mut modules, &artifact);
    }
    WorthUiRustAuthoredArtifactInput::from_modules(modules.into_values())
}

fn push_artifact(
    modules: &mut BTreeMap<String, WorthUiRustAuthoredArtifactInputModule>,
    artifact: &worth_ui_dsl::UiDslSemanticArtifact,
) {
    let module_path = artifact.provenance().module_path().to_owned();
    let declaration = semantic_declaration(artifact);
    let module = modules
        .remove(&module_path)
        .unwrap_or_else(|| WorthUiRustAuthoredArtifactInputModule::new(&module_path))
        .with_semantic_declaration(declaration);
    modules.insert(module_path, module);
}

fn semantic_declaration(
    artifact: &worth_ui_dsl::UiDslSemanticArtifact,
) -> WorthUiSemanticArtifactDeclaration {
    let declaration =
        WorthUiSemanticArtifactDeclaration::new(artifact.key().clone(), artifact.family());
    let declaration = artifact.published_aspects().iter().cloned().fold(
        declaration,
        WorthUiSemanticArtifactDeclaration::with_published_aspect,
    );
    let declaration = artifact.consumed_aspects().iter().cloned().fold(
        declaration,
        WorthUiSemanticArtifactDeclaration::with_consumed_aspect,
    );
    let declaration = artifact.structural_tokens().iter().cloned().fold(
        declaration,
        WorthUiSemanticArtifactDeclaration::with_structural_token,
    );
    let declaration = artifact.posture_tokens().iter().cloned().fold(
        declaration,
        WorthUiSemanticArtifactDeclaration::with_posture_token,
    );
    let mut declaration = artifact.support_tokens().iter().cloned().fold(
        declaration,
        WorthUiSemanticArtifactDeclaration::with_support_token,
    );
    if let Some(component) = artifact.component_reference() {
        declaration = declaration
            .with_component_reference(component.clone())
            .expect("one semantic artifact carries at most one component reference");
    }
    match artifact.appearance_role_attachment() {
        Some(attachment) => declaration
            .with_appearance_role_attachment(attachment.clone())
            .expect("one semantic artifact carries at most one appearance attachment"),
        None => declaration,
    }
}
