use super::primitive_birth::{
    PrimitiveConstructionBirthFamily, PrimitiveConstructionBirthScaffoldInput,
    SpatialConstructionBirthPlan,
};

use super::primitive_birth::digest_parts;
use super::primitive_birth_contract::{
    primitive_birth_contract_matches_counts, primitive_birth_contract_matches_support_planes,
    PrimitiveConstructionBirthContractCounts,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialConstructionBirthMappingKind {
    Vertex,
    Edge,
    Loop,
    Wire,
    Face,
    Shell,
    Body,
}

impl SpatialConstructionBirthMappingKind {
    pub fn as_str(self) -> &'static str {
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
pub enum SpatialConstructionBirthRejectionKind {
    FamilyMismatch,
    ScaffoldDigestMismatch,
    TopologyBirthClassMismatch,
    ContractCountsOrSupportMismatch,
}

impl SpatialConstructionBirthRejectionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FamilyMismatch => "family-mismatch",
            Self::ScaffoldDigestMismatch => "scaffold-digest-mismatch",
            Self::TopologyBirthClassMismatch => "topology-birth-class-mismatch",
            Self::ContractCountsOrSupportMismatch => "contract-counts-or-support-mismatch",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialConstructionBirthMappingRow {
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

    pub fn kind(&self) -> SpatialConstructionBirthMappingKind {
        self.kind
    }

    pub fn mapped_count(&self) -> usize {
        self.mapped_count
    }

    pub fn support_plane_count(&self) -> usize {
        self.support_plane_count
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedPrimitiveConstructionBirthConsequence {
    family: PrimitiveConstructionBirthFamily,
    topology_birth_class: String,
    scaffold_digest: String,
    birth_digest: String,
    rows: Vec<SpatialConstructionBirthMappingRow>,
    consequence_digest: String,
}

impl AdmittedPrimitiveConstructionBirthConsequence {
    fn new(
        input: &PrimitiveConstructionBirthScaffoldInput,
        plan: &SpatialConstructionBirthPlan,
    ) -> Self {
        let support_plane_count = input.support_planes().len();
        let rows = vec![
            SpatialConstructionBirthMappingRow::new(
                SpatialConstructionBirthMappingKind::Vertex,
                plan.supported_vertex_count(),
                support_plane_count,
                plan.birth_digest(),
                input.topology_birth_class(),
            ),
            SpatialConstructionBirthMappingRow::new(
                SpatialConstructionBirthMappingKind::Edge,
                plan.supported_edge_count(),
                support_plane_count,
                plan.birth_digest(),
                input.topology_birth_class(),
            ),
            SpatialConstructionBirthMappingRow::new(
                SpatialConstructionBirthMappingKind::Loop,
                plan.supported_loop_count(),
                support_plane_count,
                plan.birth_digest(),
                input.topology_birth_class(),
            ),
            SpatialConstructionBirthMappingRow::new(
                SpatialConstructionBirthMappingKind::Wire,
                plan.supported_wire_count(),
                support_plane_count,
                plan.birth_digest(),
                input.topology_birth_class(),
            ),
            SpatialConstructionBirthMappingRow::new(
                SpatialConstructionBirthMappingKind::Face,
                plan.supported_face_count(),
                support_plane_count,
                plan.birth_digest(),
                input.topology_birth_class(),
            ),
            SpatialConstructionBirthMappingRow::new(
                SpatialConstructionBirthMappingKind::Shell,
                plan.supported_shell_count(),
                support_plane_count,
                plan.birth_digest(),
                input.topology_birth_class(),
            ),
            SpatialConstructionBirthMappingRow::new(
                SpatialConstructionBirthMappingKind::Body,
                plan.supported_body_count(),
                support_plane_count,
                plan.birth_digest(),
                input.topology_birth_class(),
            ),
        ];
        let mut parts = vec![
            input.family().as_str().to_string(),
            input.topology_birth_class().to_string(),
            input.scaffold_digest().to_string(),
            plan.birth_digest().to_string(),
        ];
        parts.extend(rows.iter().map(|row| row.row_digest().to_string()));
        Self {
            family: input.family(),
            topology_birth_class: input.topology_birth_class().to_string(),
            scaffold_digest: input.scaffold_digest().to_string(),
            birth_digest: plan.birth_digest().to_string(),
            rows,
            consequence_digest: digest_parts(&parts),
        }
    }

    pub fn family(&self) -> PrimitiveConstructionBirthFamily {
        self.family
    }

    pub fn topology_birth_class(&self) -> &str {
        &self.topology_birth_class
    }

    pub fn scaffold_digest(&self) -> &str {
        &self.scaffold_digest
    }

    pub fn birth_digest(&self) -> &str {
        &self.birth_digest
    }

    pub fn rows(&self) -> &[SpatialConstructionBirthMappingRow] {
        &self.rows
    }

    pub fn row_for(
        &self,
        kind: SpatialConstructionBirthMappingKind,
    ) -> Option<&SpatialConstructionBirthMappingRow> {
        self.rows.iter().find(|row| row.kind() == kind)
    }

    pub fn consequence_digest(&self) -> &str {
        &self.consequence_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RejectedPrimitiveConstructionBirthConsequence {
    kind: SpatialConstructionBirthRejectionKind,
    family: PrimitiveConstructionBirthFamily,
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

    pub fn kind(&self) -> SpatialConstructionBirthRejectionKind {
        self.kind
    }

    pub fn family(&self) -> PrimitiveConstructionBirthFamily {
        self.family
    }

    pub fn topology_birth_class(&self) -> &str {
        &self.topology_birth_class
    }

    pub fn scaffold_digest(&self) -> &str {
        &self.scaffold_digest
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }

    pub fn consequence_digest(&self) -> &str {
        &self.consequence_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SpatialConstructionBirthConsequence {
    Admitted(AdmittedPrimitiveConstructionBirthConsequence),
    Rejected(RejectedPrimitiveConstructionBirthConsequence),
}

fn reject_mismatched_birth_consequence(
    input: &PrimitiveConstructionBirthScaffoldInput,
    plan: &SpatialConstructionBirthPlan,
) -> Option<RejectedPrimitiveConstructionBirthConsequence> {
    if input.family() != plan.family() {
        return Some(RejectedPrimitiveConstructionBirthConsequence::new(
            SpatialConstructionBirthRejectionKind::FamilyMismatch,
            input,
            "primitive birth consequence requires the same admitted family across scaffold and plan",
        ));
    }
    if input.scaffold_digest() != plan.scaffold_digest() {
        return Some(RejectedPrimitiveConstructionBirthConsequence::new(
            SpatialConstructionBirthRejectionKind::ScaffoldDigestMismatch,
            input,
            "primitive birth consequence requires the same scaffold digest across scaffold and plan",
        ));
    }
    if input.topology_birth_class() != plan.topology_birth_class() {
        return Some(RejectedPrimitiveConstructionBirthConsequence::new(
            SpatialConstructionBirthRejectionKind::TopologyBirthClassMismatch,
            input,
            "primitive birth consequence requires the same topology birth class across scaffold and plan",
        ));
    }
    None
}

pub fn evaluate_primitive_construction_birth_consequence(
    input: &PrimitiveConstructionBirthScaffoldInput,
    plan: &SpatialConstructionBirthPlan,
) -> SpatialConstructionBirthConsequence {
    if let Some(rejected) = reject_mismatched_birth_consequence(input, plan) {
        return SpatialConstructionBirthConsequence::Rejected(rejected);
    }
    let counts = PrimitiveConstructionBirthContractCounts::from_plan(plan);
    if !primitive_birth_contract_matches_counts(plan.family(), counts)
        || !primitive_birth_contract_matches_support_planes(
            plan.family(),
            input.support_planes().len(),
            counts,
        )
    {
        return SpatialConstructionBirthConsequence::Rejected(
            RejectedPrimitiveConstructionBirthConsequence::new(
                SpatialConstructionBirthRejectionKind::ContractCountsOrSupportMismatch,
                input,
                "primitive birth consequence requires admitted primitive family counts and support planes",
            ),
        );
    }
    SpatialConstructionBirthConsequence::Admitted(
        AdmittedPrimitiveConstructionBirthConsequence::new(input, plan),
    )
}

#[cfg(test)]
#[path = "primitive_birth_consequence_tests.rs"]
mod tests;
