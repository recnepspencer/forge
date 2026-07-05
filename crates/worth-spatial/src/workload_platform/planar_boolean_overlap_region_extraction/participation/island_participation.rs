use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopIslandKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopIslandOverlapParticipationRow {
    participation_identity: String,
    island_identity: String,
    island_origin_loop_identity: String,
    island_kind: PlanarBooleanLoopIslandKind,
    member_loop_identities: Vec<String>,
    member_source_loop_identities: Vec<String>,
    member_source_loop_operand_sides: Vec<PlanarBooleanCommonPlaneOperandSide>,
    member_source_loop_winding_signs: Vec<i8>,
    member_role_outcome_identities: Vec<String>,
    propagated_persistent_name_identities: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopIslandOverlapParticipationMap {
    map_identity: String,
    request_identity: String,
    rows: Vec<PlanarBooleanLoopIslandOverlapParticipationRow>,
}

impl PlanarBooleanLoopIslandOverlapParticipationRow {
    pub(crate) fn new(
        participation_identity: String,
        island_identity: String,
        island_origin_loop_identity: String,
        island_kind: PlanarBooleanLoopIslandKind,
        member_loop_identities: Vec<String>,
        member_source_loop_identities: Vec<String>,
        member_source_loop_operand_sides: Vec<PlanarBooleanCommonPlaneOperandSide>,
        member_source_loop_winding_signs: Vec<i8>,
        member_role_outcome_identities: Vec<String>,
        propagated_persistent_name_identities: Vec<String>,
    ) -> Self {
        Self {
            participation_identity,
            island_identity,
            island_origin_loop_identity,
            island_kind,
            member_loop_identities,
            member_source_loop_identities,
            member_source_loop_operand_sides,
            member_source_loop_winding_signs,
            member_role_outcome_identities,
            propagated_persistent_name_identities,
        }
    }

    pub fn participation_identity(&self) -> &str {
        &self.participation_identity
    }

    pub fn island_identity(&self) -> &str {
        &self.island_identity
    }

    pub fn island_origin_loop_identity(&self) -> &str {
        &self.island_origin_loop_identity
    }

    pub fn island_kind(&self) -> PlanarBooleanLoopIslandKind {
        self.island_kind
    }

    pub fn member_loop_identities(&self) -> &[String] {
        &self.member_loop_identities
    }

    pub fn member_source_loop_identities(&self) -> &[String] {
        &self.member_source_loop_identities
    }

    pub fn member_source_loop_operand_sides(&self) -> &[PlanarBooleanCommonPlaneOperandSide] {
        &self.member_source_loop_operand_sides
    }

    pub fn member_source_loop_winding_signs(&self) -> &[i8] {
        &self.member_source_loop_winding_signs
    }

    pub fn member_role_outcome_identities(&self) -> &[String] {
        &self.member_role_outcome_identities
    }

    pub fn propagated_persistent_name_identities(&self) -> &[String] {
        &self.propagated_persistent_name_identities
    }
}

impl PlanarBooleanLoopIslandOverlapParticipationMap {
    pub(crate) fn new(
        map_identity: String,
        request_identity: String,
        rows: Vec<PlanarBooleanLoopIslandOverlapParticipationRow>,
    ) -> Self {
        Self {
            map_identity,
            request_identity,
            rows,
        }
    }

    pub fn map_identity(&self) -> &str {
        &self.map_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanLoopIslandOverlapParticipationRow] {
        &self.rows
    }
}
