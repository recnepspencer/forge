use super::primitive_birth::{
    PrimitiveConstructionBirthScaffoldInput, SpatialConstructionBirthError,
};
use worth_primitives::PrimitiveConstructionFamilyKey;

use super::primitive_birth::digest_parts;
use super::primitive_birth_contract::{
    primitive_birth_contract_matches_counts, primitive_birth_contract_matches_support_planes,
    PrimitiveConstructionBirthContractCounts,
};
use super::primitive_birth_validation::validate_primitive_construction_birth_input;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SpatialConstructionBirthMappingKind {
    Vertex,
    Edge,
    Loop,
    Wire,
    Face,
    Shell,
    Body,
}

impl SpatialConstructionBirthMappingKind {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Vertex => "vertex",
            Self::Edge => "edge",
            Self::Loop => "loop",
            Self::Wire => "wire",
            Self::Face => "face",
            Self::Shell => "shell",
            Self::Body => "body",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SpatialConstructionBirthRejectionKind {
    FamilyMismatch,
    ScaffoldDigestMismatch,
    TopologyBirthClassMismatch,
    ContractCountsOrSupportMismatch,
}

impl SpatialConstructionBirthRejectionKind {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::FamilyMismatch => "family-mismatch",
            Self::ScaffoldDigestMismatch => "scaffold-digest-mismatch",
            Self::TopologyBirthClassMismatch => "topology-birth-class-mismatch",
            Self::ContractCountsOrSupportMismatch => "contract-counts-or-support-mismatch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SpatialConstructionBirthMappingRow {
    kind: SpatialConstructionBirthMappingKind,
    mapped_count: usize,
    support_plane_count: usize,
    row_digest: String,
}

impl SpatialConstructionBirthMappingRow {
    fn new(
        kind: SpatialConstructionBirthMappingKind,
        mapped_count: usize,
        support_plane_count: usize,
        birth_digest: &str,
        topology_birth_class: &str,
    ) -> Self {
        let row_digest = digest_parts(&[
            kind.as_str().to_string(),
            mapped_count.to_string(),
            support_plane_count.to_string(),
            birth_digest.to_string(),
            topology_birth_class.to_string(),
        ]);
        Self {
            kind,
            mapped_count,
            support_plane_count,
            row_digest,
        }
    }

    #[cfg(test)]
    pub(crate) fn kind(&self) -> SpatialConstructionBirthMappingKind {
        self.kind
    }

    #[cfg(test)]
    pub(crate) fn mapped_count(&self) -> usize {
        self.mapped_count
    }

    #[cfg(test)]
    pub(crate) fn support_plane_count(&self) -> usize {
        self.support_plane_count
    }

    pub(crate) fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AdmittedPrimitiveConstructionBirthConsequence {
    rows: Vec<SpatialConstructionBirthMappingRow>,
    consequence_digest: String,
}

impl AdmittedPrimitiveConstructionBirthConsequence {
    pub(crate) fn from_scaffold_input(input: &PrimitiveConstructionBirthScaffoldInput) -> Self {
        let support_plane_count = input.support_planes().len();
        let birth_digest = primitive_construction_birth_digest(input);
        let rows = vec![
            SpatialConstructionBirthMappingRow::new(
                SpatialConstructionBirthMappingKind::Vertex,
                input.expected_vertex_count(),
                support_plane_count,
                &birth_digest,
                input.topology_birth_class(),
            ),
            SpatialConstructionBirthMappingRow::new(
                SpatialConstructionBirthMappingKind::Edge,
                input.expected_edge_count(),
                support_plane_count,
                &birth_digest,
                input.topology_birth_class(),
            ),
            SpatialConstructionBirthMappingRow::new(
                SpatialConstructionBirthMappingKind::Loop,
                input.expected_loop_count(),
                support_plane_count,
                &birth_digest,
                input.topology_birth_class(),
            ),
            SpatialConstructionBirthMappingRow::new(
                SpatialConstructionBirthMappingKind::Wire,
                input.expected_wire_count(),
                support_plane_count,
                &birth_digest,
                input.topology_birth_class(),
            ),
            SpatialConstructionBirthMappingRow::new(
                SpatialConstructionBirthMappingKind::Face,
                input.expected_face_count(),
                support_plane_count,
                &birth_digest,
                input.topology_birth_class(),
            ),
            SpatialConstructionBirthMappingRow::new(
                SpatialConstructionBirthMappingKind::Shell,
                input.expected_shell_count(),
                support_plane_count,
                &birth_digest,
                input.topology_birth_class(),
            ),
            SpatialConstructionBirthMappingRow::new(
                SpatialConstructionBirthMappingKind::Body,
                input.expected_body_count(),
                support_plane_count,
                &birth_digest,
                input.topology_birth_class(),
            ),
        ];
        let mut parts = vec![
            input.family().as_str().to_string(),
            input.topology_birth_class().to_string(),
            input.scaffold_digest().to_string(),
            birth_digest,
        ];
        parts.extend(rows.iter().map(|row| row.row_digest().to_string()));
        Self {
            rows,
            consequence_digest: digest_parts(&parts),
        }
    }

    pub(crate) fn rows(&self) -> &[SpatialConstructionBirthMappingRow] {
        &self.rows
    }

    #[cfg(test)]
    pub fn row_for(
        &self,
        kind: SpatialConstructionBirthMappingKind,
    ) -> Option<&SpatialConstructionBirthMappingRow> {
        self.rows.iter().find(|row| row.kind() == kind)
    }

    pub(crate) fn consequence_digest(&self) -> &str {
        &self.consequence_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RejectedPrimitiveConstructionBirthConsequence {
    kind: SpatialConstructionBirthRejectionKind,
    family: PrimitiveConstructionFamilyKey,
    topology_birth_class: String,
    scaffold_digest: String,
    reason: &'static str,
    consequence_digest: String,
}

impl RejectedPrimitiveConstructionBirthConsequence {
    fn new(
        kind: SpatialConstructionBirthRejectionKind,
        input: &PrimitiveConstructionBirthScaffoldInput,
        reason: &'static str,
    ) -> Self {
        let consequence_digest = digest_parts(&[
            kind.as_str().to_string(),
            input.family().as_str().to_string(),
            input.topology_birth_class().to_string(),
            input.scaffold_digest().to_string(),
            reason.to_string(),
        ]);
        Self {
            kind,
            family: input.family(),
            topology_birth_class: input.topology_birth_class().to_string(),
            scaffold_digest: input.scaffold_digest().to_string(),
            reason,
            consequence_digest,
        }
    }

    pub(crate) fn kind(&self) -> SpatialConstructionBirthRejectionKind {
        self.kind
    }

    #[cfg(test)]
    pub(crate) fn topology_birth_class(&self) -> &str {
        &self.topology_birth_class
    }

    pub(crate) fn reason(&self) -> &'static str {
        self.reason
    }

    pub(crate) fn consequence_digest(&self) -> &str {
        &self.consequence_digest
    }
}

pub(crate) fn primitive_construction_birth_digest(
    input: &PrimitiveConstructionBirthScaffoldInput,
) -> String {
    let parts = [
        input.family().as_str().to_string(),
        input.scaffold_digest().to_string(),
        input.topology_birth_class().to_string(),
        input.expected_vertex_count().to_string(),
        input.expected_edge_count().to_string(),
        input.expected_loop_count().to_string(),
        input.expected_wire_count().to_string(),
        input.expected_face_count().to_string(),
        input.expected_shell_count().to_string(),
        input.expected_body_count().to_string(),
        input.realization_strategy().as_str().to_string(),
        input
            .attempted_realization_strategies()
            .iter()
            .map(|strategy| strategy.as_str())
            .collect::<Vec<_>>()
            .join("->"),
        input.stability_class().as_str().to_string(),
        input.feature_conditioning_class().as_str().to_string(),
        input.support_normal_class().as_str().to_string(),
        input.normalization_disposition().as_str().to_string(),
        input.realization_geometry_digest().to_string(),
        input.realization_fact_digest().to_string(),
    ];
    digest_parts(&parts)
}

pub(crate) fn reject_mismatched_primitive_construction_birth_consequence(
    reference: &PrimitiveConstructionBirthScaffoldInput,
    mismatched: &PrimitiveConstructionBirthScaffoldInput,
) -> Option<RejectedPrimitiveConstructionBirthConsequence> {
    if mismatched.family() != reference.family() {
        return Some(RejectedPrimitiveConstructionBirthConsequence::new(
            SpatialConstructionBirthRejectionKind::FamilyMismatch,
            mismatched,
            "primitive birth consequence requires the same admitted family across scaffold and plan",
        ));
    }
    if mismatched.scaffold_digest() != reference.scaffold_digest() {
        return Some(RejectedPrimitiveConstructionBirthConsequence::new(
            SpatialConstructionBirthRejectionKind::ScaffoldDigestMismatch,
            mismatched,
            "primitive birth consequence requires the same scaffold digest across scaffold and plan",
        ));
    }
    if mismatched.topology_birth_class() != reference.topology_birth_class() {
        return Some(RejectedPrimitiveConstructionBirthConsequence::new(
            SpatialConstructionBirthRejectionKind::TopologyBirthClassMismatch,
            mismatched,
            "primitive birth consequence requires the same topology birth class across scaffold and plan",
        ));
    }
    if mismatched.birth_contract() != reference.birth_contract() {
        return Some(RejectedPrimitiveConstructionBirthConsequence::new(
            SpatialConstructionBirthRejectionKind::ContractCountsOrSupportMismatch,
            mismatched,
            "primitive birth consequence requires the same canonical primitive family contract across scaffold and plan",
        ));
    }
    None
}

pub(crate) fn admit_primitive_construction_birth_consequence(
    input: &PrimitiveConstructionBirthScaffoldInput,
) -> Result<AdmittedPrimitiveConstructionBirthConsequence, SpatialConstructionBirthError> {
    validate_primitive_construction_birth_input(input)?;
    let counts = PrimitiveConstructionBirthContractCounts::from_input(input);
    if !primitive_birth_contract_matches_counts(input.birth_contract(), counts)
        || !primitive_birth_contract_matches_support_planes(
            input.birth_contract(),
            input.support_planes().len(),
        )
    {
        return Err(SpatialConstructionBirthError::InvalidPrimitiveBirthScaffold(
            "primitive birth consequence requires admitted primitive family counts and support planes",
        ));
    }
    Ok(AdmittedPrimitiveConstructionBirthConsequence::from_scaffold_input(input))
}

#[cfg(test)]
#[path = "primitive_birth_consequence_tests.rs"]
mod tests;
