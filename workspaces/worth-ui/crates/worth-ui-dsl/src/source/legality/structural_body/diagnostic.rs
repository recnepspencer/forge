#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum WorthUiStructuralLanguageDiagnosticCode {
    InvalidStructuralSyntax,
    DuplicateRegionSizingDeclaration,
    DuplicateRegionStateDeclaration,
    DuplicateMountPlacementDeclaration,
    DuplicateMountStateDeclaration,
    IllegalRootStructuralStatement,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiStructuralParseFailure {
    pub(crate) code: WorthUiStructuralLanguageDiagnosticCode,
    pub(crate) authored_text: String,
    pub(crate) structural_locus: String,
}
