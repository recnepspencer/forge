use std::path::PathBuf;

use worth_ui_dsl::{
    WorthUiAuthoredSourceInput, WorthUiDslCompiler, WorthUiProjectionRequirementIdentity,
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
    WorthUiSealedSemanticPackage,
};

use super::model::RequirementModel;

#[derive(Debug, Eq, PartialEq)]
pub(super) struct CompiledRequirements {
    pub(super) models: Vec<RequirementModel>,
    pub(super) identities: Vec<WorthUiProjectionRequirementIdentity>,
}

pub(super) fn compile_file(
    main: &str,
    support_modules: &[(&str, &str)],
) -> WorthUiSealedSemanticPackage {
    let input = support_modules.iter().fold(
        WorthUiAuthoredSourceInput::rooted_at(PathBuf::from("workspace"))
            .with_module("main.wui", main),
        |input, (path, source)| input.with_module(*path, *source),
    );
    WorthUiDslCompiler::compile_source(input).expect("QP10 file-authored pair must compile")
}

pub(super) fn compile_rust(
    modules: impl IntoIterator<Item = WorthUiRustAuthoredArtifactInputModule>,
) -> WorthUiSealedSemanticPackage {
    let input = WorthUiRustAuthoredArtifactInput::from_modules(modules);
    WorthUiDslCompiler::compile_rust_authored(&input)
        .expect("QP10 Rust-authored pair must compile")
}

pub(super) fn capture(package: &WorthUiSealedSemanticPackage) -> CompiledRequirements {
    let mut requirements = package
        .projection_requirements()
        .map(|requirement| (RequirementModel::capture(requirement), requirement.identity()))
        .collect::<Vec<_>>();
    requirements.sort_by(|left, right| left.0.cmp(&right.0));
    CompiledRequirements {
        models: requirements
            .iter()
            .map(|(model, _)| model.clone())
            .collect(),
        identities: requirements
            .into_iter()
            .map(|(_, identity)| identity)
            .collect(),
    }
}
