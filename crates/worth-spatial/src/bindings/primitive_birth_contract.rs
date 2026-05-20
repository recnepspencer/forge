use crate::facade::{PrimitiveConstructionBirthFamily, SpatialConstructionBirthPlan};

use super::PrimitiveConstructionBirthScaffoldInput;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PrimitiveConstructionBirthContractCounts {
    vertex_count: usize,
    edge_count: usize,
    loop_count: usize,
    wire_count: usize,
    face_count: usize,
    shell_count: usize,
    body_count: usize,
}

impl PrimitiveConstructionBirthContractCounts {
    pub fn from_input(input: &PrimitiveConstructionBirthScaffoldInput) -> Self {
        Self {
            vertex_count: input.expected_vertex_count(),
            edge_count: input.expected_edge_count(),
            loop_count: input.expected_loop_count(),
            wire_count: input.expected_wire_count(),
            face_count: input.expected_face_count(),
            shell_count: input.expected_shell_count(),
            body_count: input.expected_body_count(),
        }
    }

    pub fn from_plan(plan: &SpatialConstructionBirthPlan) -> Self {
        Self {
            vertex_count: plan.supported_vertex_count(),
            edge_count: plan.supported_edge_count(),
            loop_count: plan.supported_loop_count(),
            wire_count: plan.supported_wire_count(),
            face_count: plan.supported_face_count(),
            shell_count: plan.supported_shell_count(),
            body_count: plan.supported_body_count(),
        }
    }

    pub fn vertex_count(self) -> usize {
        self.vertex_count
    }

    pub fn edge_count(self) -> usize {
        self.edge_count
    }

    pub fn loop_count(self) -> usize {
        self.loop_count
    }

    pub fn wire_count(self) -> usize {
        self.wire_count
    }

    pub fn face_count(self) -> usize {
        self.face_count
    }

    pub fn shell_count(self) -> usize {
        self.shell_count
    }

    pub fn body_count(self) -> usize {
        self.body_count
    }
}

pub fn primitive_birth_contract_matches_counts(
    family: PrimitiveConstructionBirthFamily,
    counts: PrimitiveConstructionBirthContractCounts,
) -> bool {
    match family {
        PrimitiveConstructionBirthFamily::SimplexSolid => {
            counts.vertex_count() == 4
                && counts.edge_count() == 6
                && counts.loop_count() == 4
                && counts.wire_count() == 0
                && counts.face_count() == 4
                && counts.shell_count() == 1
                && counts.body_count() == 1
        }
        PrimitiveConstructionBirthFamily::Orthotope => {
            counts.vertex_count() == 8
                && counts.edge_count() == 12
                && counts.loop_count() == 6
                && counts.wire_count() == 0
                && counts.face_count() == 6
                && counts.shell_count() == 1
                && counts.body_count() == 1
        }
        PrimitiveConstructionBirthFamily::RegularPrism => {
            counts.vertex_count() >= 6
                && counts.vertex_count() % 2 == 0
                && counts.edge_count() == counts.vertex_count() * 3 / 2
                && counts.face_count() == counts.vertex_count() / 2 + 2
                && counts.loop_count() == counts.face_count()
                && counts.wire_count() == 0
                && counts.shell_count() == 1
                && counts.body_count() == 1
        }
        PrimitiveConstructionBirthFamily::RegularPyramid => {
            counts.vertex_count() >= 4
                && counts.edge_count() == (counts.vertex_count() - 1) * 2
                && counts.face_count() == counts.vertex_count()
                && counts.loop_count() == counts.face_count()
                && counts.wire_count() == 0
                && counts.shell_count() == 1
                && counts.body_count() == 1
        }
        PrimitiveConstructionBirthFamily::WireBody => {
            counts.vertex_count() >= 3
                && counts.edge_count() == counts.vertex_count()
                && counts.loop_count() == 1
                && counts.wire_count() == 1
                && counts.face_count() == 0
                && counts.shell_count() == 0
                && counts.body_count() == 1
        }
        PrimitiveConstructionBirthFamily::ShellWithHole => {
            counts.vertex_count() >= 6
                && counts.edge_count() == counts.vertex_count()
                && counts.loop_count() >= 2
                && counts.wire_count() == 0
                && counts.face_count() == 1
                && counts.shell_count() == 1
                && counts.body_count() == 1
        }
    }
}

pub fn primitive_birth_contract_matches_support_planes(
    family: PrimitiveConstructionBirthFamily,
    support_plane_count: usize,
    counts: PrimitiveConstructionBirthContractCounts,
) -> bool {
    match family {
        PrimitiveConstructionBirthFamily::SimplexSolid => support_plane_count == 4,
        PrimitiveConstructionBirthFamily::Orthotope => support_plane_count == 6,
        PrimitiveConstructionBirthFamily::RegularPrism
        | PrimitiveConstructionBirthFamily::RegularPyramid => {
            support_plane_count == counts.face_count()
        }
        PrimitiveConstructionBirthFamily::WireBody
        | PrimitiveConstructionBirthFamily::ShellWithHole => support_plane_count == 1,
    }
}
