use super::request_types::HadwigerResearchDeclarationShapeError;

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
pub struct PlaneLowerBoundClaimDeclaration {
    claim_id: String,
    graph_version_id: String,
    forbidden_color_count: u32,
}

impl PlaneLowerBoundClaimDeclaration {
    pub fn new(
        claim_id: impl Into<String>,
        graph_version_id: impl Into<String>,
        forbidden_color_count: u32,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            claim_id: require_non_empty(claim_id, "claim_id")?,
            graph_version_id: require_non_empty(graph_version_id, "graph_version_id")?,
            forbidden_color_count: require_color_count(
                forbidden_color_count,
                "forbidden_color_count",
            )?,
        })
    }

    pub fn claim_id(&self) -> &str {
        &self.claim_id
    }

    pub fn graph_version_id(&self) -> &str {
        &self.graph_version_id
    }

    pub fn forbidden_color_count(&self) -> u32 {
        self.forbidden_color_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlaneUpperBoundClaimDeclaration {
    claim_id: String,
    color_count: u32,
    upper_bound_source: String,
}

impl PlaneUpperBoundClaimDeclaration {
    pub fn new(
        claim_id: impl Into<String>,
        color_count: u32,
        upper_bound_source: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            claim_id: require_non_empty(claim_id, "claim_id")?,
            color_count: require_color_count(color_count, "color_count")?,
            upper_bound_source: require_non_empty(upper_bound_source, "upper_bound_source")?,
        })
    }

    pub fn claim_id(&self) -> &str {
        &self.claim_id
    }

    pub fn color_count(&self) -> u32 {
        self.color_count
    }

    pub fn upper_bound_source(&self) -> &str {
        &self.upper_bound_source
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlaneExactValueClaimDeclaration {
    claim_id: String,
    color_count: u32,
    lower_bound_claim_digest: String,
    upper_bound_source: String,
}

impl PlaneExactValueClaimDeclaration {
    pub fn new(
        claim_id: impl Into<String>,
        color_count: u32,
        lower_bound_claim_digest: impl Into<String>,
        upper_bound_source: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            claim_id: require_non_empty(claim_id, "claim_id")?,
            color_count: require_color_count(color_count, "color_count")?,
            lower_bound_claim_digest: require_non_empty(
                lower_bound_claim_digest,
                "lower_bound_claim_digest",
            )?,
            upper_bound_source: require_non_empty(upper_bound_source, "upper_bound_source")?,
        })
    }

    pub fn claim_id(&self) -> &str {
        &self.claim_id
    }

    pub fn color_count(&self) -> u32 {
        self.color_count
    }

    pub fn lower_bound_claim_digest(&self) -> &str {
        &self.lower_bound_claim_digest
    }

    pub fn upper_bound_source(&self) -> &str {
        &self.upper_bound_source
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackgroundTheoremDeclaration {
    theorem_id: String,
    theorem_statement: String,
    source: String,
    provenance_digest: String,
}

impl BackgroundTheoremDeclaration {
    pub fn plane_seven_upper_bound(
        theorem_id: impl Into<String>,
        source: impl Into<String>,
        provenance_digest: impl Into<String>,
    ) -> Result<Self, HadwigerResearchDeclarationShapeError> {
        Ok(Self {
            theorem_id: require_non_empty(theorem_id, "theorem_id")?,
            theorem_statement: "chi(plane) <= 7".to_string(),
            source: require_non_empty(source, "source")?,
            provenance_digest: require_non_empty(provenance_digest, "provenance_digest")?,
        })
    }

    pub fn theorem_id(&self) -> &str {
        &self.theorem_id
    }

    pub fn theorem_statement(&self) -> &str {
        &self.theorem_statement
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn provenance_digest(&self) -> &str {
        &self.provenance_digest
    }
}
