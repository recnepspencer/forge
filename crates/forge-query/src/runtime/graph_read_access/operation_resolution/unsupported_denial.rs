#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ForgeQueryGraphReadOperationUnsupportedDenialKind {
    DeniedUnsupportedShape,
}

impl ForgeQueryGraphReadOperationUnsupportedDenialKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DeniedUnsupportedShape => "denied_unsupported_shape",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ForgeQueryGraphReadOperationUnsupportedDenial {
    kind: ForgeQueryGraphReadOperationUnsupportedDenialKind,
    shape_name: String,
    explanation: String,
    read_graph_digest: String,
    matched_relations: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ForgeQueryGraphReadOperationUnsupportedShapeDeclaration {
    kind: ForgeQueryGraphReadOperationUnsupportedDenialKind,
    shape_name: String,
    explanation: String,
}

impl ForgeQueryGraphReadOperationUnsupportedShapeDeclaration {
    pub fn kind(&self) -> &ForgeQueryGraphReadOperationUnsupportedDenialKind {
        &self.kind
    }

    pub fn shape_name(&self) -> &str {
        &self.shape_name
    }

    pub fn explanation(&self) -> &str {
        &self.explanation
    }

    pub fn unsupported_shape(
        shape_name: impl Into<String>,
        explanation: impl Into<String>,
    ) -> Self {
        Self {
            kind: ForgeQueryGraphReadOperationUnsupportedDenialKind::DeniedUnsupportedShape,
            shape_name: shape_name.into(),
            explanation: explanation.into(),
        }
    }

    pub(crate) fn resolve_for_read_graph(
        &self,
        read_graph_digest: impl Into<String>,
        mut matched_relations: Vec<String>,
    ) -> ForgeQueryGraphReadOperationUnsupportedDenial {
        matched_relations.sort();
        matched_relations.dedup();
        ForgeQueryGraphReadOperationUnsupportedDenial {
            kind: self.kind.clone(),
            shape_name: self.shape_name.clone(),
            explanation: self.explanation.clone(),
            read_graph_digest: read_graph_digest.into(),
            matched_relations,
        }
    }
}

impl ForgeQueryGraphReadOperationUnsupportedDenial {
    pub fn kind(&self) -> &ForgeQueryGraphReadOperationUnsupportedDenialKind {
        &self.kind
    }

    pub fn shape_name(&self) -> &str {
        &self.shape_name
    }

    pub fn explanation(&self) -> &str {
        &self.explanation
    }

    pub fn read_graph_digest(&self) -> &str {
        &self.read_graph_digest
    }

    pub fn matched_relations(&self) -> &[String] {
        &self.matched_relations
    }
}
