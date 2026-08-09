use std::collections::BTreeMap;

use worth_ui::facade::app::WorthUiApplicationBuilder;
use worth_ui_dsl::{
    UiDslLoweringReceipt, UiDslSemanticArtifactSpec, UiDslSourceProvenance, WorthUiDslCompiler,
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
    WorthUiSemanticArtifactDeclaration,
};

#[derive(Clone)]
pub struct WorthUiRustAuthoredDeclarationFixture {
    specs: Vec<UiDslSemanticArtifactSpec>,
}

impl WorthUiRustAuthoredDeclarationFixture {
    pub fn empty() -> Self {
        Self { specs: Vec::new() }
    }

    pub fn named(_diagnostic_name: impl Into<String>) -> Self {
        Self::empty()
    }

    pub fn with_semantic_artifact_spec(mut self, spec: UiDslSemanticArtifactSpec) -> Self {
        self.specs.push(spec);
        self
    }

    pub fn admitted_declarations(&self) -> Vec<UiDslLoweringReceipt> {
        WorthUiDslCompiler::compile_rust_authored(&self.clone().into_input())
            .expect("certification declaration fixture should compile")
            .declaration_lowering_receipts()
    }

    pub fn admit_semantic_artifact(&self, spec: UiDslSemanticArtifactSpec) -> UiDslLoweringReceipt {
        self.clone()
            .with_semantic_artifact_spec(spec)
            .admitted_declarations()
            .into_iter()
            .last()
            .expect("incoming declaration should mint one lowering receipt")
    }

    pub fn admitted_provenance_for(&self, semantic_key: &str) -> UiDslSourceProvenance {
        let mut matching = self
            .admitted_declarations()
            .into_iter()
            .filter(|receipt| receipt.semantic_artifact().key().as_str() == semantic_key);
        let provenance = matching
            .next()
            .unwrap_or_else(|| panic!("fixture should admit semantic key `{semantic_key}`"))
            .source_provenance()
            .clone();
        assert!(
            matching.next().is_none(),
            "fixture semantic key `{semantic_key}` should be unique"
        );
        provenance
    }

    pub fn into_input(self) -> WorthUiRustAuthoredArtifactInput {
        let mut modules = BTreeMap::<String, WorthUiRustAuthoredArtifactInputModule>::new();
        for spec in self.specs {
            let artifact = spec.into_semantic_artifact();
            let module_path = artifact.provenance().module_path().to_owned();
            let declaration = semantic_declaration(&artifact);
            let module = modules
                .remove(&module_path)
                .unwrap_or_else(|| WorthUiRustAuthoredArtifactInputModule::new(&module_path))
                .with_semantic_declaration(declaration);
            modules.insert(module_path, module);
        }
        WorthUiRustAuthoredArtifactInput::from_modules(modules.into_values())
    }
}

pub trait WorthUiCertificationBuilderExt {
    fn with_rust_authored_declaration_fixture(
        self,
        fixture: WorthUiRustAuthoredDeclarationFixture,
    ) -> Self;
}

impl<ChangeProfileState, IntentWiringState, HostBindingState> WorthUiCertificationBuilderExt
    for WorthUiApplicationBuilder<ChangeProfileState, IntentWiringState, HostBindingState>
{
    fn with_rust_authored_declaration_fixture(
        self,
        fixture: WorthUiRustAuthoredDeclarationFixture,
    ) -> Self {
        self.with_rust_authored_input(fixture.into_input())
    }
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
    artifact.support_tokens().iter().cloned().fold(
        declaration,
        WorthUiSemanticArtifactDeclaration::with_support_token,
    )
}
