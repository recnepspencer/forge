#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryGraphTouchDescriptorDenialKind {
    ProgramComponentCountMismatch,
    BreadthProgramComponentCountMismatch,
    SymbolicEntityDeclarationCountMismatch,
    SymbolicRelationDeclarationCountMismatch,
    ProgramStepIndexOutOfBounds,
    DuplicateProgramStepIndex,
    EmptyReadCollection,
    EmptyDeclaredMutationCollection,
    ProgramCommandCollectionMismatch,
    ProgramCommandSymbolMismatch,
    ProgramCommandMutationFamilyMismatch,
}

impl ForgeQueryGraphTouchDescriptorDenialKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProgramComponentCountMismatch => "program-component-count-mismatch",
            Self::BreadthProgramComponentCountMismatch => {
                "breadth-program-component-count-mismatch"
            }
            Self::SymbolicEntityDeclarationCountMismatch => {
                "symbolic-entity-declaration-count-mismatch"
            }
            Self::SymbolicRelationDeclarationCountMismatch => {
                "symbolic-relation-declaration-count-mismatch"
            }
            Self::ProgramStepIndexOutOfBounds => "program-step-index-out-of-bounds",
            Self::DuplicateProgramStepIndex => "duplicate-program-step-index",
            Self::EmptyReadCollection => "empty-read-collection",
            Self::EmptyDeclaredMutationCollection => "empty-declared-mutation-collection",
            Self::ProgramCommandCollectionMismatch => "program-command-collection-mismatch",
            Self::ProgramCommandSymbolMismatch => "program-command-symbol-mismatch",
            Self::ProgramCommandMutationFamilyMismatch => {
                "program-command-mutation-family-mismatch"
            }
        }
    }
}

impl std::fmt::Display for ForgeQueryGraphTouchDescriptorDenialKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphTouchDescriptorDenial {
    kind: ForgeQueryGraphTouchDescriptorDenialKind,
    message: String,
}

impl ForgeQueryGraphTouchDescriptorDenial {
    pub(crate) fn new(
        kind: ForgeQueryGraphTouchDescriptorDenialKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub fn kind(&self) -> ForgeQueryGraphTouchDescriptorDenialKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for ForgeQueryGraphTouchDescriptorDenial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "graph touch descriptor denied for {}: {}",
            self.kind, self.message
        )
    }
}

impl std::error::Error for ForgeQueryGraphTouchDescriptorDenial {}
