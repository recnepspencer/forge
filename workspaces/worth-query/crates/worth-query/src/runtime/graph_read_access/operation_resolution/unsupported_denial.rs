#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum WorthQueryGraphReadOperationUnsupportedDenialKind {
    DeniedUnsupportedShape,
}

impl WorthQueryGraphReadOperationUnsupportedDenialKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DeniedUnsupportedShape => "denied_unsupported_shape",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthQueryGraphReadOperationUnsupportedDenial {
    kind: WorthQueryGraphReadOperationUnsupportedDenialKind,
    shape_name: String,
    explanation: String,
    read_graph_digest: String,
    matched_relations: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WorthQueryGraphReadOperationUnsupportedShapeDeclaration {
    kind: WorthQueryGraphReadOperationUnsupportedDenialKind,
    shape_name: String,
    explanation: String,
}

impl WorthQueryGraphReadOperationUnsupportedShapeDeclaration {
    pub fn kind(&self) -> &WorthQueryGraphReadOperationUnsupportedDenialKind {
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
            kind: WorthQueryGraphReadOperationUnsupportedDenialKind::DeniedUnsupportedShape,
            shape_name: shape_name.into(),
            explanation: explanation.into(),
        }
    }

    pub(crate) fn resolve_for_read_graph(
        &self,
        read_graph_digest: impl Into<String>,
        mut matched_relations: Vec<String>,
    ) -> WorthQueryGraphReadOperationUnsupportedDenial {
        matched_relations.sort();
        matched_relations.dedup();
        WorthQueryGraphReadOperationUnsupportedDenial {
            kind: self.kind.clone(),
            shape_name: self.shape_name.clone(),
            explanation: self.explanation.clone(),
            read_graph_digest: read_graph_digest.into(),
            matched_relations,
        }
    }
}

impl WorthQueryGraphReadOperationUnsupportedDenial {
    pub fn kind(&self) -> &WorthQueryGraphReadOperationUnsupportedDenialKind {
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
