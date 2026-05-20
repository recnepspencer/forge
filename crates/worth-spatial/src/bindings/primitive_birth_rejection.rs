use crate::facade::{PrimitiveConstructionBirthScaffoldInput, SpatialConstructionBirthPlan};

use super::primitive_birth::digest_parts;
use super::PrimitiveConstructionBirthFamily;

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
pub struct SpatialConstructionBirthRejectionRow {
    kind: SpatialConstructionBirthRejectionKind,
    family: PrimitiveConstructionBirthFamily,
    topology_birth_class: String,
    scaffold_digest: String,
    reason: &'static str,
    row_digest: String,
}

impl SpatialConstructionBirthRejectionRow {
    pub(crate) fn new(
        kind: SpatialConstructionBirthRejectionKind,
        input: &PrimitiveConstructionBirthScaffoldInput,
        reason: &'static str,
    ) -> Self {
        let row_digest = digest_parts(&[
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
            row_digest,
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

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

pub fn reject_primitive_construction_birth_completeness(
    input: &PrimitiveConstructionBirthScaffoldInput,
    plan: &SpatialConstructionBirthPlan,
) -> Option<SpatialConstructionBirthRejectionRow> {
    if input.family() != plan.family() {
        return Some(SpatialConstructionBirthRejectionRow::new(
            SpatialConstructionBirthRejectionKind::FamilyMismatch,
            input,
            "birth completeness requires the same admitted family across scaffold and plan",
        ));
    }
    if input.scaffold_digest() != plan.scaffold_digest() {
        return Some(SpatialConstructionBirthRejectionRow::new(
            SpatialConstructionBirthRejectionKind::ScaffoldDigestMismatch,
            input,
            "birth completeness requires the same scaffold digest across scaffold and plan",
        ));
    }
    if input.topology_birth_class() != plan.topology_birth_class() {
        return Some(SpatialConstructionBirthRejectionRow::new(
            SpatialConstructionBirthRejectionKind::TopologyBirthClassMismatch,
            input,
            "birth completeness requires the same topology birth class across scaffold and plan",
        ));
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::facade::{
        plan_primitive_construction_birth, PrimitiveConstructionBirthFamily,
        PrimitiveConstructionBirthScaffoldInput,
    };
    use worth_geom::facade::Plane;

    use super::{
        reject_primitive_construction_birth_completeness, SpatialConstructionBirthRejectionKind,
    };

    #[test]
    fn birth_rejection_row_tracks_topology_birth_class_mismatch() {
        let input = PrimitiveConstructionBirthScaffoldInput::new(
            PrimitiveConstructionBirthFamily::WireBody,
            "planar_wire_body",
            "wire-scaffold".to_string(),
            vec![plane()],
            vec![
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [-1.0, 0.0, 0.0],
                [0.0, -1.0, 0.0],
            ],
            4,
            4,
            1,
            1,
            0,
            0,
            1,
        );
        let mismatched = PrimitiveConstructionBirthScaffoldInput::new(
            PrimitiveConstructionBirthFamily::WireBody,
            "bad_birth_class",
            "wire-scaffold".to_string(),
            vec![plane()],
            vec![
                [1.0, 0.0, 0.0],
                [0.0, 1.0, 0.0],
                [-1.0, 0.0, 0.0],
                [0.0, -1.0, 0.0],
            ],
            4,
            4,
            1,
            1,
            0,
            0,
            1,
        );
        let plan = plan_primitive_construction_birth(input).expect("birth plan");
        let row =
            reject_primitive_construction_birth_completeness(&mismatched, &plan).expect("row");

        assert_eq!(
            row.kind(),
            SpatialConstructionBirthRejectionKind::TopologyBirthClassMismatch
        );
        assert_eq!(row.topology_birth_class(), "bad_birth_class");
        assert!(!row.row_digest().is_empty());
    }

    fn plane() -> Plane {
        Plane::from_point_normal([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]).expect("plane")
    }
}
