use worth_ui_dsl::{
    WorthUiAuthoredSourceInput, WorthUiDslCompiler, WorthUiRustAuthoredArtifactInput,
    WorthUiSealedSemanticPackage, WorthUiSemanticDeclaration,
};

use crate::source::WorthUiRuntimeSemanticImport;

pub(crate) fn compile_rust_authored(
    input: &WorthUiRustAuthoredArtifactInput,
) -> WorthUiSealedSemanticPackage {
    WorthUiDslCompiler::compile_rust_authored(input)
        .expect("runtime test authoring should compile into a sealed DSL package")
}

pub(crate) fn compile_source<I, P, S>(modules: I) -> WorthUiSealedSemanticPackage
where
    I: IntoIterator<Item = (P, S)>,
    P: Into<String>,
    S: Into<String>,
{
    let mut input = WorthUiAuthoredSourceInput::rooted_at(r"C:\workspace");
    for (relative_path, source_text) in modules {
        input = input.with_module(relative_path, source_text);
    }
    WorthUiDslCompiler::compile_source(input)
        .expect("runtime test source should compile into a sealed DSL package")
}

pub(crate) fn semantic_import(target: &str) -> WorthUiRuntimeSemanticImport {
    semantic_import_at(target, 0)
}

pub(crate) fn semantic_import_at(
    target: &str,
    declaration_index: usize,
) -> WorthUiRuntimeSemanticImport {
    let preceding_declarations = (0..declaration_index)
        .map(|index| format!("token fixture_{index} = \"fixture\";"))
        .collect::<Vec<_>>()
        .join("\n");
    let source = format!("{preceding_declarations}\nimport \"{target}\";");
    let package = compile_source([("app/main.wui", source), (target, String::new())]);
    let (target, provenance) = package
        .module_ids()
        .iter()
        .filter_map(|module_id| package.declaration_views(module_id))
        .flatten()
        .find_map(|view| match view.declaration() {
            WorthUiSemanticDeclaration::Import(import) => {
                Some((import.target().clone(), view.provenance().clone()))
            }
            _ => None,
        })
        .expect("test source should contain one semantic import");
    assert_eq!(
        provenance.declaration_index(),
        declaration_index,
        "compiler-derived test import should retain its requested declaration position"
    );
    WorthUiRuntimeSemanticImport::new(target, provenance)
}
