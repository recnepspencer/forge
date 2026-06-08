#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HadwigerResearchDeclarationShapeError {
    EmptyIdentityField { field: &'static str },
    ZeroColorCount { field: &'static str },
}

fn require_non_empty(
    value: impl Into<String>,
    field: &'static str,
) -> Result<String, HadwigerResearchDeclarationShapeError> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(HadwigerResearchDeclarationShapeError::EmptyIdentityField { field });
    }
    Ok(value)
}

fn require_color_count(
    color_count: u32,
    field: &'static str,
) -> Result<u32, HadwigerResearchDeclarationShapeError> {
    if color_count == 0 {
        return Err(HadwigerResearchDeclarationShapeError::ZeroColorCount { field });
    }
    Ok(color_count)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CandidateGraphDeclaration {
    graph_id: String,
    graph_version: Option<String>,
    source_note: Option<String>,
}

impl CandidateGraphDeclaration {
    pub fn new(graph_id: impl Into<String>) -> Self {
        Self::try_new(graph_id).expect("graph_id must be non-empty")
    }

    pub fn try_new(
        graph_id: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            graph_id: require_non_empty(graph_id, "graph_id")?,
            graph_version: None,
            source_note: None,
        })
    }

    pub fn with_graph_version(self, graph_version: impl Into<String>) -> Self {
        self.try_with_graph_version(graph_version)
            .expect("graph_version must be non-empty")
    }

    pub fn try_with_graph_version(
        mut self,
        graph_version: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        self.graph_version = Some(require_non_empty(graph_version, "graph_version")?);
        Ok(self)
    }

    pub fn with_source_note(self, source_note: impl Into<String>) -> Self {
        self.try_with_source_note(source_note)
            .expect("source_note must be non-empty")
    }

    pub fn try_with_source_note(
        mut self,
        source_note: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        self.source_note = Some(require_non_empty(source_note, "source_note")?);
        Ok(self)
    }

    pub(crate) fn graph_id(&self) -> &str {
        &self.graph_id
    }

    pub(crate) fn graph_version(&self) -> Option<&str> {
        self.graph_version.as_deref()
    }

    pub(crate) fn source_note(&self) -> Option<&str> {
        self.source_note.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EmbeddingDeclaration {
    graph_id: String,
    embedding_id: String,
}

impl EmbeddingDeclaration {
    pub fn new(graph_id: impl Into<String>, embedding_id: impl Into<String>) -> Self {
        Self::try_new(graph_id, embedding_id).expect("graph_id and embedding_id must be non-empty")
    }

    pub fn try_new(
        graph_id: impl Into<String>,
        embedding_id: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            graph_id: require_non_empty(graph_id, "graph_id")?,
            embedding_id: require_non_empty(embedding_id, "embedding_id")?,
        })
    }

    pub(crate) fn graph_id(&self) -> &str {
        &self.graph_id
    }

    pub(crate) fn embedding_id(&self) -> &str {
        &self.embedding_id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColorabilityDeclaration {
    graph_id: String,
    color_count: u32,
}

impl ColorabilityDeclaration {
    pub fn new(graph_id: impl Into<String>, color_count: u32) -> Self {
        Self::try_new(graph_id, color_count)
            .expect("graph_id must be non-empty and color_count must be greater than zero")
    }

    pub fn try_new(
        graph_id: impl Into<String>,
        color_count: u32,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            graph_id: require_non_empty(graph_id, "graph_id")?,
            color_count: require_color_count(color_count, "color_count")?,
        })
    }

    pub(crate) fn graph_id(&self) -> &str {
        &self.graph_id
    }

    pub(crate) fn color_count(&self) -> u32 {
        self.color_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LowerBoundWitnessDeclaration {
    graph_id: String,
    embedding_id: String,
    color_count: u32,
}

impl LowerBoundWitnessDeclaration {
    pub fn new(
        graph_id: impl Into<String>,
        embedding_id: impl Into<String>,
        color_count: u32,
    ) -> Self {
        Self::try_new(graph_id, embedding_id, color_count).expect(
            "graph_id and embedding_id must be non-empty and color_count must be greater than zero",
        )
    }

    pub fn try_new(
        graph_id: impl Into<String>,
        embedding_id: impl Into<String>,
        color_count: u32,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            graph_id: require_non_empty(graph_id, "graph_id")?,
            embedding_id: require_non_empty(embedding_id, "embedding_id")?,
            color_count: require_color_count(color_count, "color_count")?,
        })
    }

    pub(crate) fn graph_id(&self) -> &str {
        &self.graph_id
    }

    pub(crate) fn embedding_id(&self) -> &str {
        &self.embedding_id
    }

    pub(crate) fn color_count(&self) -> u32 {
        self.color_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdvisoryNoteDeclaration {
    graph_id: String,
    note: String,
}

impl AdvisoryNoteDeclaration {
    pub fn new(graph_id: impl Into<String>, note: impl Into<String>) -> Self {
        Self::try_new(graph_id, note).expect("graph_id and note must be non-empty")
    }

    pub fn try_new(
        graph_id: impl Into<String>,
        note: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            graph_id: require_non_empty(graph_id, "graph_id")?,
            note: require_non_empty(note, "note")?,
        })
    }

    pub(crate) fn graph_id(&self) -> &str {
        &self.graph_id
    }

    pub(crate) fn note(&self) -> &str {
        &self.note
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RejectionExplanationDeclaration {
    graph_id: String,
    rejection_basis: String,
}

impl RejectionExplanationDeclaration {
    pub fn new(graph_id: impl Into<String>, rejection_basis: impl Into<String>) -> Self {
        Self::try_new(graph_id, rejection_basis)
            .expect("graph_id and rejection_basis must be non-empty")
    }

    pub fn try_new(
        graph_id: impl Into<String>,
        rejection_basis: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            graph_id: require_non_empty(graph_id, "graph_id")?,
            rejection_basis: require_non_empty(rejection_basis, "rejection_basis")?,
        })
    }

    pub(crate) fn graph_id(&self) -> &str {
        &self.graph_id
    }

    pub(crate) fn rejection_basis(&self) -> &str {
        &self.rejection_basis
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PartialAdmissionExplanationDeclaration {
    graph_id: String,
}

impl PartialAdmissionExplanationDeclaration {
    pub fn new(graph_id: impl Into<String>) -> Self {
        Self::try_new(graph_id).expect("graph_id must be non-empty")
    }

    pub fn try_new(
        graph_id: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            graph_id: require_non_empty(graph_id, "graph_id")?,
        })
    }

    pub(crate) fn graph_id(&self) -> &str {
        &self.graph_id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UnitDistanceVerificationDeclaration {
    graph_id: String,
    embedding_id: String,
}

impl UnitDistanceVerificationDeclaration {
    pub fn new(graph_id: impl Into<String>, embedding_id: impl Into<String>) -> Self {
        Self::try_new(graph_id, embedding_id).expect("graph_id and embedding_id must be non-empty")
    }

    pub fn try_new(
        graph_id: impl Into<String>,
        embedding_id: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            graph_id: require_non_empty(graph_id, "graph_id")?,
            embedding_id: require_non_empty(embedding_id, "embedding_id")?,
        })
    }

    pub(crate) fn graph_id(&self) -> &str {
        &self.graph_id
    }

    pub(crate) fn embedding_id(&self) -> &str {
        &self.embedding_id
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WholePlaneColoringConstructionDeclaration {
    construction_id: String,
    color_count: u32,
}

impl WholePlaneColoringConstructionDeclaration {
    pub fn new(construction_id: impl Into<String>, color_count: u32) -> Self {
        Self::try_new(construction_id, color_count)
            .expect("construction_id must be non-empty and color_count must be greater than zero")
    }

    pub fn try_new(
        construction_id: impl Into<String>,
        color_count: u32,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            construction_id: require_non_empty(construction_id, "construction_id")?,
            color_count: require_color_count(color_count, "color_count")?,
        })
    }

    pub(crate) fn construction_id(&self) -> &str {
        &self.construction_id
    }

    pub(crate) fn color_count(&self) -> u32 {
        self.color_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FractionalChromaticScreeningDeclaration {
    graph_version_reference: String,
    color_limit: u32,
    screening_basis: String,
}

impl FractionalChromaticScreeningDeclaration {
    pub fn new(
        graph_version_reference: impl Into<String>,
        color_limit: u32,
        screening_basis: impl Into<String>,
    ) -> Self {
        Self::try_new(graph_version_reference, color_limit, screening_basis).expect(
            "graph_version_reference and screening_basis must be non-empty and color_limit must be greater than zero",
        )
    }

    pub fn try_new(
        graph_version_reference: impl Into<String>,
        color_limit: u32,
        screening_basis: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            graph_version_reference: require_non_empty(
                graph_version_reference,
                "graph_version_reference",
            )?,
            color_limit: require_color_count(color_limit, "color_limit")?,
            screening_basis: require_non_empty(screening_basis, "screening_basis")?,
        })
    }

    pub(crate) fn graph_version_reference(&self) -> &str {
        &self.graph_version_reference
    }

    pub(crate) fn color_limit(&self) -> u32 {
        self.color_limit
    }

    pub(crate) fn screening_basis(&self) -> &str {
        &self.screening_basis
    }
}
