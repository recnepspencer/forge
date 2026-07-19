#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthQueryPortableDefinitionKind {
    Invariant,
    GraphObligation,
    GraphReadOperation,
    DeclarationFamily,
}

impl WorthQueryPortableDefinitionKind {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Invariant => "invariant",
            Self::GraphObligation => "graph-obligation",
            Self::GraphReadOperation => "graph-read-operation",
            Self::DeclarationFamily => "declaration-family",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct WorthQueryPortableDefinitionBody {
    slot: String,
    semantics: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct WorthQueryPortableDefinition {
    kind: WorthQueryPortableDefinitionKind,
    body: WorthQueryPortableDefinitionBody,
}

impl WorthQueryPortableDefinition {
    pub fn invariant(slot: impl Into<String>, semantics: impl Into<String>) -> Self {
        Self::new(WorthQueryPortableDefinitionKind::Invariant, slot, semantics)
    }

    pub fn graph_obligation(slot: impl Into<String>, semantics: impl Into<String>) -> Self {
        Self::new(
            WorthQueryPortableDefinitionKind::GraphObligation,
            slot,
            semantics,
        )
    }

    pub fn graph_read_operation(slot: impl Into<String>, semantics: impl Into<String>) -> Self {
        Self::new(
            WorthQueryPortableDefinitionKind::GraphReadOperation,
            slot,
            semantics,
        )
    }

    pub fn declaration_family(slot: impl Into<String>, semantics: impl Into<String>) -> Self {
        Self::new(
            WorthQueryPortableDefinitionKind::DeclarationFamily,
            slot,
            semantics,
        )
    }

    pub fn kind(&self) -> WorthQueryPortableDefinitionKind {
        self.kind
    }

    pub fn slot(&self) -> &str {
        &self.body.slot
    }

    pub fn semantics(&self) -> &str {
        &self.body.semantics
    }

    fn new(
        kind: WorthQueryPortableDefinitionKind,
        slot: impl Into<String>,
        semantics: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            body: WorthQueryPortableDefinitionBody {
                slot: slot.into(),
                semantics: semantics.into(),
            },
        }
    }
}
