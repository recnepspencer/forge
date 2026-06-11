use crate::domain_artifacts::core_artifact::require_non_empty;
use crate::domain_artifacts::HadwigerArtifactShapeError;

use super::{ScreeningRectangularRegion, ScreeningSolverTranscript};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundaryOwnedRegion {
    region: ScreeningRectangularRegion,
    color_id: String,
    boundary_owner: bool,
}

impl BoundaryOwnedRegion {
    pub fn new(
        region: ScreeningRectangularRegion,
        color_id: impl Into<String>,
        boundary_owner: bool,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            region,
            color_id: require_non_empty(color_id, "color_id")?,
            boundary_owner,
        })
    }

    pub(crate) fn region(&self) -> &ScreeningRectangularRegion {
        &self.region
    }

    pub(crate) fn color_id(&self) -> &str {
        &self.color_id
    }

    pub(crate) fn owns_boundary(&self) -> bool {
        self.boundary_owner
    }

    fn stable_token(&self) -> String {
        format!(
            "{}:{}:{}",
            self.region.stable_token(),
            self.color_id,
            self.boundary_owner
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundaryOwnershipCertificate {
    certificate_id: String,
    regions: Vec<BoundaryOwnedRegion>,
    solver_transcript: ScreeningSolverTranscript,
}

impl BoundaryOwnershipCertificate {
    pub fn new(
        certificate_id: impl Into<String>,
        mut regions: Vec<BoundaryOwnedRegion>,
        solver_transcript: ScreeningSolverTranscript,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        if regions.is_empty() {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "boundary_owned_regions",
            });
        }
        regions.sort_by_key(BoundaryOwnedRegion::stable_token);
        Ok(Self {
            certificate_id: require_non_empty(certificate_id, "certificate_id")?,
            regions,
            solver_transcript,
        })
    }

    pub(crate) fn regions(&self) -> &[BoundaryOwnedRegion] {
        &self.regions
    }

    pub fn stable_token(&self) -> String {
        let mut token = format!(
            "{}:{}",
            self.certificate_id,
            self.solver_transcript.stable_token()
        );
        for region in &self.regions {
            token.push_str(&format!(":{}", region.stable_token()));
        }
        token
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ColorPermutation {
    mapping: Vec<(String, String)>,
}

impl ColorPermutation {
    pub fn new(mut mapping: Vec<(String, String)>) -> Result<Self, HadwigerArtifactShapeError> {
        normalize_pairs(&mut mapping, "color_permutation")?;
        Ok(Self { mapping })
    }

    pub(crate) fn apply(&self, color: &str) -> String {
        self.mapping
            .iter()
            .find(|(left, _)| left == color)
            .map(|(_, right)| right.clone())
            .unwrap_or_else(|| color.to_string())
    }

    fn stable_token(&self) -> String {
        format!("{:?}", self.mapping)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MonodromyColorHolonomyCertificate {
    certificate_id: String,
    tracked_color: String,
    loop_permutations: Vec<ColorPermutation>,
    solver_transcript: ScreeningSolverTranscript,
}

impl MonodromyColorHolonomyCertificate {
    pub fn new(
        certificate_id: impl Into<String>,
        tracked_color: impl Into<String>,
        loop_permutations: Vec<ColorPermutation>,
        solver_transcript: ScreeningSolverTranscript,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        if loop_permutations.is_empty() {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "loop_permutations",
            });
        }
        Ok(Self {
            certificate_id: require_non_empty(certificate_id, "certificate_id")?,
            tracked_color: require_non_empty(tracked_color, "tracked_color")?,
            loop_permutations,
            solver_transcript,
        })
    }

    pub(crate) fn tracked_color(&self) -> &str {
        &self.tracked_color
    }

    pub(crate) fn loop_permutations(&self) -> &[ColorPermutation] {
        &self.loop_permutations
    }

    pub fn stable_token(&self) -> String {
        let mut token = format!("{}:{}", self.certificate_id, self.tracked_color);
        for permutation in &self.loop_permutations {
            token.push_str(&format!(":{}", permutation.stable_token()));
        }
        format!("{token}:{}", self.solver_transcript.stable_token())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TranslationRotationClosureCertificate {
    certificate_id: String,
    vertex_mapping: Vec<(String, String)>,
    same_color_pairs: Vec<(String, String)>,
    solver_transcript: ScreeningSolverTranscript,
}

impl TranslationRotationClosureCertificate {
    pub fn new(
        certificate_id: impl Into<String>,
        mut vertex_mapping: Vec<(String, String)>,
        mut same_color_pairs: Vec<(String, String)>,
        solver_transcript: ScreeningSolverTranscript,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        normalize_pairs(&mut vertex_mapping, "closure_vertex_mapping")?;
        normalize_pairs(&mut same_color_pairs, "same_color_pairs")?;
        Ok(Self {
            certificate_id: require_non_empty(certificate_id, "certificate_id")?,
            vertex_mapping,
            same_color_pairs,
            solver_transcript,
        })
    }

    pub(crate) fn vertex_mapping(&self) -> &[(String, String)] {
        &self.vertex_mapping
    }

    pub(crate) fn same_color_pairs(&self) -> &[(String, String)] {
        &self.same_color_pairs
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{:?}:{:?}:{}",
            self.certificate_id,
            self.vertex_mapping,
            self.same_color_pairs,
            self.solver_transcript.stable_token()
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubstitutionConsistencyFailureKind {
    Internal,
    Boundary,
    CrossLevel,
    ParentChildColor,
}

impl SubstitutionConsistencyFailureKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Internal => "internal",
            Self::Boundary => "boundary",
            Self::CrossLevel => "cross_level",
            Self::ParentChildColor => "parent_child_color",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubstitutionConsistencyCertificate {
    certificate_id: String,
    level: u32,
    failures: Vec<SubstitutionConsistencyFailureKind>,
    solver_transcript: ScreeningSolverTranscript,
}

impl SubstitutionConsistencyCertificate {
    pub fn new(
        certificate_id: impl Into<String>,
        level: u32,
        failures: Vec<SubstitutionConsistencyFailureKind>,
        solver_transcript: ScreeningSolverTranscript,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        Ok(Self {
            certificate_id: require_non_empty(certificate_id, "certificate_id")?,
            level,
            failures,
            solver_transcript,
        })
    }

    pub(crate) fn failures(&self) -> &[SubstitutionConsistencyFailureKind] {
        &self.failures
    }

    pub fn stable_token(&self) -> String {
        let failures = self
            .failures
            .iter()
            .map(|failure| failure.as_str())
            .collect::<Vec<_>>()
            .join(".");
        format!(
            "{}:{}:{}:{}",
            self.certificate_id,
            self.level,
            failures,
            self.solver_transcript.stable_token()
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FinitePatchBoundaryExtensionCertificate {
    certificate_id: String,
    boundary_colorings: Vec<String>,
    extendable_colorings: Vec<String>,
    solver_transcript: ScreeningSolverTranscript,
}

impl FinitePatchBoundaryExtensionCertificate {
    pub fn new(
        certificate_id: impl Into<String>,
        mut boundary_colorings: Vec<String>,
        mut extendable_colorings: Vec<String>,
        solver_transcript: ScreeningSolverTranscript,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        normalize_non_empty_strings(&mut boundary_colorings, "boundary_coloring")?;
        normalize_strings(&mut extendable_colorings, "extendable_coloring")?;
        if extendable_colorings.len() > boundary_colorings.len() {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "extension_coloring_subset",
            });
        }
        Ok(Self {
            certificate_id: require_non_empty(certificate_id, "certificate_id")?,
            boundary_colorings,
            extendable_colorings,
            solver_transcript,
        })
    }

    pub(crate) fn all_boundary_colorings_fail(&self) -> bool {
        self.extendable_colorings.is_empty()
    }

    pub fn stable_token(&self) -> String {
        format!(
            "{}:{:?}:{:?}:{}",
            self.certificate_id,
            self.boundary_colorings,
            self.extendable_colorings,
            self.solver_transcript.stable_token()
        )
    }
}

fn normalize_pairs(
    pairs: &mut Vec<(String, String)>,
    field: &'static str,
) -> Result<(), HadwigerArtifactShapeError> {
    if pairs.is_empty() {
        return Err(HadwigerArtifactShapeError::EmptyField { field });
    }
    for (left, right) in pairs.iter() {
        require_non_empty(left.clone(), field)?;
        require_non_empty(right.clone(), field)?;
    }
    pairs.sort();
    pairs.dedup();
    Ok(())
}

fn normalize_non_empty_strings(
    values: &mut Vec<String>,
    field: &'static str,
) -> Result<(), HadwigerArtifactShapeError> {
    if values.is_empty() {
        return Err(HadwigerArtifactShapeError::EmptyField { field });
    }
    normalize_strings(values, field)
}

fn normalize_strings(
    values: &mut Vec<String>,
    field: &'static str,
) -> Result<(), HadwigerArtifactShapeError> {
    for value in values.iter() {
        require_non_empty(value.clone(), field)?;
    }
    values.sort();
    values.dedup();
    Ok(())
}
