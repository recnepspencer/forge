mod artifact_input;
mod canonical;
mod compile;
mod import_graph;
mod legality;
mod lexical;
mod lower;
mod module;
mod package;
mod parse;
#[cfg(test)]
mod tests;

#[cfg(feature = "certification-support")]
pub use compile::certification;

#[cfg(test)]
pub(crate) use artifact_input::WorthUiArtifactInputEquivalentShape;
pub(crate) use artifact_input::WorthUiArtifactInputNormalizer;
pub use artifact_input::{
    WorthUiArtifactInput, WorthUiArtifactInputBlockNode, WorthUiArtifactInputBodyAtom,
    WorthUiArtifactInputImportNode, WorthUiArtifactInputModule, WorthUiArtifactInputNode,
    WorthUiArtifactInputNodeKind, WorthUiArtifactInputProvenance, WorthUiArtifactInputReference,
    WorthUiArtifactInputSemanticArtifactNode, WorthUiArtifactInputTokenNode,
    WorthUiSemanticArtifactDeclaration,
};
pub(crate) use canonical::WorthUiCanonicalModuleOrder;
pub use compile::{
    WorthUiAuthoredMode, WorthUiAuthoredSourceInput, WorthUiDslCompileDiagnostic,
    WorthUiDslCompileDiagnosticCode, WorthUiDslCompileReport, WorthUiDslCompileStopClass,
    WorthUiDslCompiler, WorthUiDslDiagnosticIdentity, WorthUiDslProtocolIdentity,
    WorthUiDslSourceSpan, WorthUiSealedSemanticArtifact, WorthUiSealedSemanticPackage,
    WorthUiSemanticBlock, WorthUiSemanticDeclaration, WorthUiSemanticDeclarationView,
    WorthUiSemanticImport, WorthUiSemanticModule, WorthUiSemanticPackageIdentity,
    WorthUiSemanticProvenanceRef, WorthUiSemanticToken,
};
pub(crate) use import_graph::{WorthUiSourceImport, WorthUiSourceImportGraph};
pub use legality::{WorthUiAuthoredMount, WorthUiAuthoredRegion, WorthUiAuthoredStructuralBody};
pub(crate) use legality::{
    WorthUiStructuralBodyParser, WorthUiStructuralLanguageDiagnosticCode,
    WorthUiStructuralParseFailure,
};
pub(crate) use lexical::{tokenize_module_source, WorthUiSourceToken, WorthUiSourceTokenKind};
pub use lower::rust_authored::{
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
};
pub(crate) use lower::{
    WorthUiParsedSourceToArtifactInputLowerer, WorthUiRustAuthoredInputLoweringDenial,
    WorthUiRustAuthoredToArtifactInputLowerer,
};
pub use module::WorthUiSourceModuleId;
pub(crate) use module::WorthUiSourceModuleRecord;
pub(crate) use package::{
    WorthUiSourcePackage, WorthUiSourcePackageDiagnostic, WorthUiSourcePackageDiagnosticCode,
    WorthUiSourcePackageDigest, WorthUiSourcePackageLoader, WorthUiSourcePackageReport,
};
pub use parse::WorthUiSourceSpan;
pub(crate) use parse::{
    WorthUiParseDiagnostic, WorthUiParseDiagnosticCode, WorthUiParseReport, WorthUiParsedBlockBody,
    WorthUiParsedBlockDeclaration, WorthUiParsedImportDeclaration, WorthUiParsedSourceDeclaration,
    WorthUiParsedSourceModule, WorthUiParsedSourcePackage, WorthUiParsedTokenDeclaration,
    WorthUiSourceParser,
};
