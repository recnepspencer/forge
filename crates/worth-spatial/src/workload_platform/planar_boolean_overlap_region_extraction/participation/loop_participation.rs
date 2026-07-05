use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_events::PlanarBooleanLoopRole;
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopClassifiedProductKind, PlanarBooleanLoopIslandKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopOverlapParticipationRow {
    participation_identity: String,
    ledger_row_identity: String,
    canonical_loop_identity: String,
    tracked_loop_identity: String,
    loop_kind: PlanarBooleanLoopClassifiedProductKind,
    loop_role: PlanarBooleanLoopRole,
    role_outcome_identity: String,
    island_identity: String,
    island_origin_loop_identity: String,
    island_kind: PlanarBooleanLoopIslandKind,
    source_loop_identities: Vec<String>,
    source_loop_operand_sides: Vec<PlanarBooleanCommonPlaneOperandSide>,
    source_loop_winding_signs: Vec<i8>,
    propagated_persistent_name_identities: Vec<String>,
    overlap_chain_lineage_identities: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopOverlapParticipationMap {
    map_identity: String,
    request_identity: String,
    rows: Vec<PlanarBooleanLoopOverlapParticipationRow>,
}

impl PlanarBooleanLoopOverlapParticipationRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        participation_identity: String,
        ledger_row_identity: String,
        canonical_loop_identity: String,
        tracked_loop_identity: String,
        loop_kind: PlanarBooleanLoopClassifiedProductKind,
        loop_role: PlanarBooleanLoopRole,
        role_outcome_identity: String,
        island_identity: String,
        island_origin_loop_identity: String,
        island_kind: PlanarBooleanLoopIslandKind,
        source_loop_identities: Vec<String>,
        source_loop_operand_sides: Vec<PlanarBooleanCommonPlaneOperandSide>,
        source_loop_winding_signs: Vec<i8>,
        propagated_persistent_name_identities: Vec<String>,
        overlap_chain_lineage_identities: Vec<String>,
    ) -> Self {
        Self {
            participation_identity,
            ledger_row_identity,
            canonical_loop_identity,
            tracked_loop_identity,
            loop_kind,
            loop_role,
            role_outcome_identity,
            island_identity,
            island_origin_loop_identity,
            island_kind,
            source_loop_identities,
            source_loop_operand_sides,
            source_loop_winding_signs,
            propagated_persistent_name_identities,
            overlap_chain_lineage_identities,
        }
    }

    pub fn participation_identity(&self) -> &str {
        &self.participation_identity
    }

    pub fn ledger_row_identity(&self) -> &str {
        &self.ledger_row_identity
    }

    pub fn canonical_loop_identity(&self) -> &str {
        &self.canonical_loop_identity
    }

    pub fn tracked_loop_identity(&self) -> &str {
        &self.tracked_loop_identity
    }

    pub fn loop_kind(&self) -> PlanarBooleanLoopClassifiedProductKind {
        self.loop_kind
    }

    pub fn loop_role(&self) -> PlanarBooleanLoopRole {
        self.loop_role
    }

    pub fn role_outcome_identity(&self) -> &str {
        &self.role_outcome_identity
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

    pub fn source_loop_identities(&self) -> &[String] {
        &self.source_loop_identities
    }

    pub fn source_loop_operand_sides(&self) -> &[PlanarBooleanCommonPlaneOperandSide] {
        &self.source_loop_operand_sides
    }

    pub fn source_loop_winding_signs(&self) -> &[i8] {
        &self.source_loop_winding_signs
    }

    pub fn propagated_persistent_name_identities(&self) -> &[String] {
        &self.propagated_persistent_name_identities
    }

    pub fn overlap_chain_lineage_identities(&self) -> &[String] {
        &self.overlap_chain_lineage_identities
    }
}

impl PlanarBooleanLoopOverlapParticipationMap {
    pub(crate) fn new(
        map_identity: String,
        request_identity: String,
        rows: Vec<PlanarBooleanLoopOverlapParticipationRow>,
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

    pub fn rows(&self) -> &[PlanarBooleanLoopOverlapParticipationRow] {
        &self.rows
    }
}
