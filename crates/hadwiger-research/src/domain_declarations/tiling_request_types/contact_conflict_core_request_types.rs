use super::require_non_empty;
use crate::domain_declarations::HadwigerResearchDeclarationShapeError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TileContactWitnessDeclaration {
    contact_id: String,
    left_tile_ref: Option<String>,
    right_tile_ref: Option<String>,
    contact_signature: Option<String>,
}

impl TileContactWitnessDeclaration {
    pub fn new(contact_id: impl Into<String>) -> Self {
        Self::try_new(contact_id).expect("contact_id must be non-empty")
    }

    pub fn try_new(
        contact_id: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            contact_id: require_non_empty(contact_id, "contact_id")?,
            left_tile_ref: None,
            right_tile_ref: None,
            contact_signature: None,
        })
    }

    pub fn with_left_tile_ref(self, left_tile_ref: impl Into<String>) -> Self {
        self.try_with_left_tile_ref(left_tile_ref)
            .expect("left_tile_ref must be non-empty")
    }

    pub fn try_with_left_tile_ref(
        mut self,
        left_tile_ref: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        self.left_tile_ref = Some(require_non_empty(left_tile_ref, "left_tile_ref")?);
        Ok(self)
    }

    pub fn with_right_tile_ref(self, right_tile_ref: impl Into<String>) -> Self {
        self.try_with_right_tile_ref(right_tile_ref)
            .expect("right_tile_ref must be non-empty")
    }

    pub fn try_with_right_tile_ref(
        mut self,
        right_tile_ref: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        self.right_tile_ref = Some(require_non_empty(right_tile_ref, "right_tile_ref")?);
        Ok(self)
    }

    pub fn with_contact_signature(self, contact_signature: impl Into<String>) -> Self {
        self.try_with_contact_signature(contact_signature)
            .expect("contact_signature must be non-empty")
    }

    pub fn try_with_contact_signature(
        mut self,
        contact_signature: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        self.contact_signature = Some(require_non_empty(contact_signature, "contact_signature")?);
        Ok(self)
    }

    pub(crate) fn contact_id(&self) -> &str {
        &self.contact_id
    }

    pub(crate) fn left_tile_ref(&self) -> Option<&str> {
        self.left_tile_ref.as_deref()
    }

    pub(crate) fn right_tile_ref(&self) -> Option<&str> {
        self.right_tile_ref.as_deref()
    }

    pub(crate) fn contact_signature(&self) -> Option<&str> {
        self.contact_signature.as_deref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictGraphExtractionDeclaration {
    extraction_id: String,
    subject_ref: String,
    distance_certificate_family: Option<String>,
    required_color_count: Option<u32>,
}

impl ConflictGraphExtractionDeclaration {
    pub fn new(extraction_id: impl Into<String>, subject_ref: impl Into<String>) -> Self {
        Self::try_new(extraction_id, subject_ref)
            .expect("extraction_id and subject_ref must be non-empty")
    }

    pub fn try_new(
        extraction_id: impl Into<String>,
        subject_ref: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            extraction_id: require_non_empty(extraction_id, "extraction_id")?,
            subject_ref: require_non_empty(subject_ref, "subject_ref")?,
            distance_certificate_family: None,
            required_color_count: None,
        })
    }

    pub fn with_distance_certificate_family(
        self,
        distance_certificate_family: impl Into<String>,
    ) -> Self {
        self.try_with_distance_certificate_family(distance_certificate_family)
            .expect("distance_certificate_family must be non-empty")
    }

    pub fn try_with_distance_certificate_family(
        mut self,
        distance_certificate_family: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        self.distance_certificate_family = Some(require_non_empty(
            distance_certificate_family,
            "distance_certificate_family",
        )?);
        Ok(self)
    }

    pub fn with_required_color_count(self, required_color_count: u32) -> Self {
        self.try_with_required_color_count(required_color_count)
            .expect("required_color_count must be non-zero")
    }

    pub fn try_with_required_color_count(
        mut self,
        required_color_count: u32,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        if required_color_count == 0 {
            return Err(HadwigerResearchDeclarationShapeError::EmptyIdentityField {
                field: "required_color_count",
            });
        }
        self.required_color_count = Some(required_color_count);
        Ok(self)
    }

    pub(crate) fn extraction_id(&self) -> &str {
        &self.extraction_id
    }

    pub(crate) fn subject_ref(&self) -> &str {
        &self.subject_ref
    }

    pub(crate) fn distance_certificate_family(&self) -> Option<&str> {
        self.distance_certificate_family.as_deref()
    }

    pub(crate) fn required_color_count(&self) -> Option<u32> {
        self.required_color_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreExtractionDeclaration {
    core_extraction_id: String,
    conflict_graph_ref: String,
}

impl CoreExtractionDeclaration {
    pub fn new(
        core_extraction_id: impl Into<String>,
        conflict_graph_ref: impl Into<String>,
    ) -> Self {
        Self::try_new(core_extraction_id, conflict_graph_ref)
            .expect("core_extraction_id and conflict_graph_ref must be non-empty")
    }

    pub fn try_new(
        core_extraction_id: impl Into<String>,
        conflict_graph_ref: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            core_extraction_id: require_non_empty(core_extraction_id, "core_extraction_id")?,
            conflict_graph_ref: require_non_empty(conflict_graph_ref, "conflict_graph_ref")?,
        })
    }

    pub(crate) fn core_extraction_id(&self) -> &str {
        &self.core_extraction_id
    }

    pub(crate) fn conflict_graph_ref(&self) -> &str {
        &self.conflict_graph_ref
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TilingEquivalenceClassificationDeclaration {
    equivalence_id: String,
    scope: String,
    left_ref: String,
    right_ref: String,
}

impl TilingEquivalenceClassificationDeclaration {
    pub fn new(
        equivalence_id: impl Into<String>,
        scope: impl Into<String>,
        left_ref: impl Into<String>,
        right_ref: impl Into<String>,
    ) -> Self {
        Self::try_new(equivalence_id, scope, left_ref, right_ref)
            .expect("equivalence declaration identity fields must be non-empty")
    }

    pub fn try_new(
        equivalence_id: impl Into<String>,
        scope: impl Into<String>,
        left_ref: impl Into<String>,
        right_ref: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            equivalence_id: require_non_empty(equivalence_id, "equivalence_id")?,
            scope: require_non_empty(scope, "equivalence_scope")?,
            left_ref: require_non_empty(left_ref, "left_ref")?,
            right_ref: require_non_empty(right_ref, "right_ref")?,
        })
    }

    pub(crate) fn equivalence_id(&self) -> &str {
        &self.equivalence_id
    }

    pub(crate) fn scope(&self) -> &str {
        &self.scope
    }

    pub(crate) fn left_ref(&self) -> &str {
        &self.left_ref
    }

    pub(crate) fn right_ref(&self) -> &str {
        &self.right_ref
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TilingSuppressionDeclaration {
    suppression_id: String,
    equivalence_ref: String,
    suppression_ref: String,
}

impl TilingSuppressionDeclaration {
    pub fn new(
        suppression_id: impl Into<String>,
        equivalence_ref: impl Into<String>,
        suppression_ref: impl Into<String>,
    ) -> Self {
        Self::try_new(suppression_id, equivalence_ref, suppression_ref)
            .expect("suppression declaration identity fields must be non-empty")
    }

    pub fn try_new(
        suppression_id: impl Into<String>,
        equivalence_ref: impl Into<String>,
        suppression_ref: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            suppression_id: require_non_empty(suppression_id, "suppression_id")?,
            equivalence_ref: require_non_empty(equivalence_ref, "equivalence_ref")?,
            suppression_ref: require_non_empty(suppression_ref, "suppression_ref")?,
        })
    }

    pub(crate) fn suppression_id(&self) -> &str {
        &self.suppression_id
    }

    pub(crate) fn equivalence_ref(&self) -> &str {
        &self.equivalence_ref
    }

    pub(crate) fn suppression_ref(&self) -> &str {
        &self.suppression_ref
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TilingReactivationDeclaration {
    reactivation_id: String,
    suppression_ref: String,
    qualifying_evidence_ref: String,
}

impl TilingReactivationDeclaration {
    pub fn new(
        reactivation_id: impl Into<String>,
        suppression_ref: impl Into<String>,
        qualifying_evidence_ref: impl Into<String>,
    ) -> Self {
        Self::try_new(reactivation_id, suppression_ref, qualifying_evidence_ref)
            .expect("reactivation declaration identity fields must be non-empty")
    }

    pub fn try_new(
        reactivation_id: impl Into<String>,
        suppression_ref: impl Into<String>,
        qualifying_evidence_ref: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            reactivation_id: require_non_empty(reactivation_id, "reactivation_id")?,
            suppression_ref: require_non_empty(suppression_ref, "suppression_ref")?,
            qualifying_evidence_ref: require_non_empty(
                qualifying_evidence_ref,
                "qualifying_evidence_ref",
            )?,
        })
    }

    pub(crate) fn reactivation_id(&self) -> &str {
        &self.reactivation_id
    }

    pub(crate) fn suppression_ref(&self) -> &str {
        &self.suppression_ref
    }

    pub(crate) fn qualifying_evidence_ref(&self) -> &str {
        &self.qualifying_evidence_ref
    }
}
