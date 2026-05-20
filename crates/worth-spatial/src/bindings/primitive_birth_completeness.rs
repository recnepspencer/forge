use crate::facade::{
    primitive_birth_contract_matches_counts, primitive_birth_contract_matches_support_planes,
    PrimitiveConstructionBirthContractCounts, PrimitiveConstructionBirthScaffoldInput,
    SpatialConstructionBirthError, SpatialConstructionBirthPlan,
};

use super::primitive_birth_rejection::{
    reject_primitive_construction_birth_completeness, SpatialConstructionBirthRejectionKind,
    SpatialConstructionBirthRejectionRow,
};
use super::PrimitiveConstructionBirthFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialConstructionBirthCompletenessReport {
    family: PrimitiveConstructionBirthFamily,
    scaffold_digest: String,
    birth_digest: String,
    topology_birth_class: String,
    support_plane_count: usize,
    supported_vertex_count: usize,
    supported_edge_count: usize,
    supported_loop_count: usize,
    supported_wire_count: usize,
    supported_face_count: usize,
    supported_shell_count: usize,
    supported_body_count: usize,
    completeness_digest: String,
}

impl SpatialConstructionBirthCompletenessReport {
    fn new(
        input: &PrimitiveConstructionBirthScaffoldInput,
        plan: &SpatialConstructionBirthPlan,
    ) -> Self {
        let parts = [
            input.family().as_str().to_string(),
            input.scaffold_digest().to_string(),
            plan.birth_digest().to_string(),
            input.topology_birth_class().to_string(),
            input.support_planes().len().to_string(),
            plan.supported_vertex_count().to_string(),
            plan.supported_edge_count().to_string(),
            plan.supported_loop_count().to_string(),
            plan.supported_wire_count().to_string(),
            plan.supported_face_count().to_string(),
            plan.supported_shell_count().to_string(),
            plan.supported_body_count().to_string(),
        ];
        Self {
            family: input.family(),
            scaffold_digest: input.scaffold_digest().to_string(),
            birth_digest: plan.birth_digest().to_string(),
            topology_birth_class: input.topology_birth_class().to_string(),
            support_plane_count: input.support_planes().len(),
            supported_vertex_count: plan.supported_vertex_count(),
            supported_edge_count: plan.supported_edge_count(),
            supported_loop_count: plan.supported_loop_count(),
            supported_wire_count: plan.supported_wire_count(),
            supported_face_count: plan.supported_face_count(),
            supported_shell_count: plan.supported_shell_count(),
            supported_body_count: plan.supported_body_count(),
            completeness_digest: super::primitive_birth::digest_parts(&parts),
        }
    }

    pub fn family(&self) -> PrimitiveConstructionBirthFamily {
        self.family
    }

    pub fn scaffold_digest(&self) -> &str {
        &self.scaffold_digest
    }

    pub fn birth_digest(&self) -> &str {
        &self.birth_digest
    }

    pub fn topology_birth_class(&self) -> &str {
        &self.topology_birth_class
    }

    pub fn support_plane_count(&self) -> usize {
        self.support_plane_count
    }

    pub fn supported_vertex_count(&self) -> usize {
        self.supported_vertex_count
    }

    pub fn supported_edge_count(&self) -> usize {
        self.supported_edge_count
    }

    pub fn supported_loop_count(&self) -> usize {
        self.supported_loop_count
    }

    pub fn supported_wire_count(&self) -> usize {
        self.supported_wire_count
    }

    pub fn supported_face_count(&self) -> usize {
        self.supported_face_count
    }

    pub fn supported_shell_count(&self) -> usize {
        self.supported_shell_count
    }

    pub fn supported_body_count(&self) -> usize {
        self.supported_body_count
    }

    pub fn completeness_digest(&self) -> &str {
        &self.completeness_digest
    }
}

pub fn certify_primitive_construction_birth_completeness(
    input: &PrimitiveConstructionBirthScaffoldInput,
    plan: &SpatialConstructionBirthPlan,
) -> Result<SpatialConstructionBirthCompletenessReport, SpatialConstructionBirthError> {
    if let Some(row) = reject_primitive_construction_birth_completeness(input, plan) {
        return Err(SpatialConstructionBirthError::InvalidPrimitiveBirthScaffold(row.reason()));
    }
    let counts = PrimitiveConstructionBirthContractCounts::from_plan(plan);
    if !primitive_birth_contract_matches_counts(plan.family(), counts)
        || !primitive_birth_contract_matches_support_planes(
            plan.family(),
            input.support_planes().len(),
            counts,
        )
    {
        return Err(
            SpatialConstructionBirthError::InvalidPrimitiveBirthScaffold(
                "birth completeness requires admitted primitive family counts and support planes",
            ),
        );
    }
    Ok(SpatialConstructionBirthCompletenessReport::new(input, plan))
}

pub fn impossible_primitive_construction_birth_attachment(
    input: &PrimitiveConstructionBirthScaffoldInput,
    plan: &SpatialConstructionBirthPlan,
) -> Option<SpatialConstructionBirthRejectionRow> {
    if let Some(row) = reject_primitive_construction_birth_completeness(input, plan) {
        return Some(row);
    }
    let counts = PrimitiveConstructionBirthContractCounts::from_plan(plan);
    if !primitive_birth_contract_matches_counts(plan.family(), counts)
        || !primitive_birth_contract_matches_support_planes(
            plan.family(),
            input.support_planes().len(),
            counts,
        )
    {
        return Some(SpatialConstructionBirthRejectionRow::new(
            SpatialConstructionBirthRejectionKind::ContractCountsOrSupportMismatch,
            input,
            "birth completeness requires admitted primitive family counts and support planes",
        ));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::{
        certify_primitive_construction_birth_completeness,
        impossible_primitive_construction_birth_attachment,
    };
    use crate::facade::{
        plan_primitive_construction_birth, PrimitiveConstructionBirthFamily,
        PrimitiveConstructionBirthScaffoldInput,
    };
    use worth_geom::facade::Plane;

    #[test]
    fn birth_completeness_report_binds_wire_birth_truth() {
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
        let plan = plan_primitive_construction_birth(input.clone()).expect("birth plan");
        let report =
            certify_primitive_construction_birth_completeness(&input, &plan).expect("report");

        assert_eq!(report.family(), PrimitiveConstructionBirthFamily::WireBody);
        assert_eq!(report.topology_birth_class(), "planar_wire_body");
        assert_eq!(report.support_plane_count(), 1);
        assert_eq!(report.supported_wire_count(), 1);
        assert_eq!(report.birth_digest(), plan.birth_digest());
        assert!(!report.completeness_digest().is_empty());
    }

    #[test]
    fn impossible_birth_attachment_returns_typed_rejection_row() {
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
        let bad_input = PrimitiveConstructionBirthScaffoldInput::new(
            PrimitiveConstructionBirthFamily::WireBody,
            "wrong_class",
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
        let row = impossible_primitive_construction_birth_attachment(&bad_input, &plan)
            .expect("typed rejection row");

        assert_eq!(row.topology_birth_class(), "wrong_class");
        assert!(row.reason().contains("topology birth class"));
    }

    fn plane() -> Plane {
        Plane::from_point_normal([0.0, 0.0, 0.0], [0.0, 0.0, 1.0]).expect("plane")
    }
}
