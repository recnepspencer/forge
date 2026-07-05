#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapIslandCandidateKind {
    BoundaryContact,
    AreaOverlap,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapIslandCandidateRow {
    candidate_identity: String,
    island_identity: String,
    cell_identity: String,
    neighborhood_identity: String,
    boundary_component_identities: Vec<String>,
    boundary_segment_identities: Vec<String>,
    source_loop_identities: Vec<String>,
    propagated_persistent_name_identities: Vec<String>,
    kind: PlanarBooleanOverlapIslandCandidateKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapIslandRow {
    island_identity: String,
    neighborhood_identity: String,
    candidate_identities: Vec<String>,
    cell_identities: Vec<String>,
    boundary_contact_component_identities: Vec<String>,
    area_overlap_component_identities: Vec<String>,
    propagated_persistent_name_identities: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanBoundaryContactComponentRow {
    component_identity: String,
    island_identity: String,
    neighborhood_identity: String,
    cell_identities: Vec<String>,
    boundary_component_identities: Vec<String>,
    boundary_segment_identities: Vec<String>,
    source_loop_identities: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanAreaOverlapComponentRow {
    component_identity: String,
    island_identity: String,
    neighborhood_identity: String,
    cell_identities: Vec<String>,
    boundary_component_identities: Vec<String>,
    boundary_segment_identities: Vec<String>,
    source_loop_identities: Vec<String>,
}

impl PlanarBooleanOverlapIslandCandidateRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        candidate_identity: String,
        island_identity: String,
        cell_identity: String,
        neighborhood_identity: String,
        boundary_component_identities: Vec<String>,
        boundary_segment_identities: Vec<String>,
        source_loop_identities: Vec<String>,
        propagated_persistent_name_identities: Vec<String>,
        kind: PlanarBooleanOverlapIslandCandidateKind,
    ) -> Self {
        Self {
            candidate_identity,
            island_identity,
            cell_identity,
            neighborhood_identity,
            boundary_component_identities,
            boundary_segment_identities,
            source_loop_identities,
            propagated_persistent_name_identities,
            kind,
        }
    }

    pub fn candidate_identity(&self) -> &str {
        &self.candidate_identity
    }

    pub fn island_identity(&self) -> &str {
        &self.island_identity
    }

    pub fn cell_identity(&self) -> &str {
        &self.cell_identity
    }

    pub fn neighborhood_identity(&self) -> &str {
        &self.neighborhood_identity
    }

    pub fn boundary_component_identities(&self) -> &[String] {
        &self.boundary_component_identities
    }

    pub fn boundary_segment_identities(&self) -> &[String] {
        &self.boundary_segment_identities
    }

    pub fn source_loop_identities(&self) -> &[String] {
        &self.source_loop_identities
    }

    pub fn propagated_persistent_name_identities(&self) -> &[String] {
        &self.propagated_persistent_name_identities
    }

    pub fn kind(&self) -> PlanarBooleanOverlapIslandCandidateKind {
        self.kind
    }
}

impl PlanarBooleanOverlapIslandRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        island_identity: String,
        neighborhood_identity: String,
        candidate_identities: Vec<String>,
        cell_identities: Vec<String>,
        boundary_contact_component_identities: Vec<String>,
        area_overlap_component_identities: Vec<String>,
        propagated_persistent_name_identities: Vec<String>,
    ) -> Self {
        Self {
            island_identity,
            neighborhood_identity,
            candidate_identities,
            cell_identities,
            boundary_contact_component_identities,
            area_overlap_component_identities,
            propagated_persistent_name_identities,
        }
    }

    pub fn island_identity(&self) -> &str {
        &self.island_identity
    }

    pub fn neighborhood_identity(&self) -> &str {
        &self.neighborhood_identity
    }

    pub fn candidate_identities(&self) -> &[String] {
        &self.candidate_identities
    }

    pub fn cell_identities(&self) -> &[String] {
        &self.cell_identities
    }

    pub fn boundary_contact_component_identities(&self) -> &[String] {
        &self.boundary_contact_component_identities
    }

    pub fn area_overlap_component_identities(&self) -> &[String] {
        &self.area_overlap_component_identities
    }
}

impl PlanarBooleanBoundaryContactComponentRow {
    pub(crate) fn new(
        component_identity: String,
        island_identity: String,
        neighborhood_identity: String,
        cell_identities: Vec<String>,
        boundary_component_identities: Vec<String>,
        boundary_segment_identities: Vec<String>,
        source_loop_identities: Vec<String>,
    ) -> Self {
        Self {
            component_identity,
            island_identity,
            neighborhood_identity,
            cell_identities,
            boundary_component_identities,
            boundary_segment_identities,
            source_loop_identities,
        }
    }

    pub fn component_identity(&self) -> &str {
        &self.component_identity
    }

    pub fn island_identity(&self) -> &str {
        &self.island_identity
    }

    pub fn neighborhood_identity(&self) -> &str {
        &self.neighborhood_identity
    }

    pub fn cell_identities(&self) -> &[String] {
        &self.cell_identities
    }

    pub fn boundary_component_identities(&self) -> &[String] {
        &self.boundary_component_identities
    }

    pub fn boundary_segment_identities(&self) -> &[String] {
        &self.boundary_segment_identities
    }

    pub fn source_loop_identities(&self) -> &[String] {
        &self.source_loop_identities
    }
}

impl PlanarBooleanAreaOverlapComponentRow {
    pub(crate) fn new(
        component_identity: String,
        island_identity: String,
        neighborhood_identity: String,
        cell_identities: Vec<String>,
        boundary_component_identities: Vec<String>,
        boundary_segment_identities: Vec<String>,
        source_loop_identities: Vec<String>,
    ) -> Self {
        Self {
            component_identity,
            island_identity,
            neighborhood_identity,
            cell_identities,
            boundary_component_identities,
            boundary_segment_identities,
            source_loop_identities,
        }
    }

    pub fn component_identity(&self) -> &str {
        &self.component_identity
    }

    pub fn island_identity(&self) -> &str {
        &self.island_identity
    }

    pub fn neighborhood_identity(&self) -> &str {
        &self.neighborhood_identity
    }

    pub fn cell_identities(&self) -> &[String] {
        &self.cell_identities
    }

    pub fn boundary_component_identities(&self) -> &[String] {
        &self.boundary_component_identities
    }

    pub fn boundary_segment_identities(&self) -> &[String] {
        &self.boundary_segment_identities
    }

    pub fn source_loop_identities(&self) -> &[String] {
        &self.source_loop_identities
    }
}
