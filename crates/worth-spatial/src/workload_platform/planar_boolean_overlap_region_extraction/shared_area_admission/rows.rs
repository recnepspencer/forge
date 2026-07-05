#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSharedAreaAdmissionOutcomeRow {
    outcome_identity: String,
    island_identity: String,
    neighborhood_identity: String,
    area_overlap_component_identity: String,
    cell_identities: Vec<String>,
    boundary_component_identities: Vec<String>,
    boundary_segment_identities: Vec<String>,
    source_loop_identities: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanMixedBoundaryAreaOutcomeRow {
    outcome_identity: String,
    island_identity: String,
    neighborhood_identity: String,
    area_overlap_component_identities: Vec<String>,
    boundary_contact_component_identities: Vec<String>,
    cell_identities: Vec<String>,
}

impl PlanarBooleanSharedAreaAdmissionOutcomeRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        outcome_identity: String,
        island_identity: String,
        neighborhood_identity: String,
        area_overlap_component_identity: String,
        cell_identities: Vec<String>,
        boundary_component_identities: Vec<String>,
        boundary_segment_identities: Vec<String>,
        source_loop_identities: Vec<String>,
    ) -> Self {
        Self {
            outcome_identity,
            island_identity,
            neighborhood_identity,
            area_overlap_component_identity,
            cell_identities,
            boundary_component_identities,
            boundary_segment_identities,
            source_loop_identities,
        }
    }

    pub fn outcome_identity(&self) -> &str {
        &self.outcome_identity
    }

    pub fn island_identity(&self) -> &str {
        &self.island_identity
    }

    pub fn neighborhood_identity(&self) -> &str {
        &self.neighborhood_identity
    }

    pub fn area_overlap_component_identity(&self) -> &str {
        &self.area_overlap_component_identity
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

impl PlanarBooleanMixedBoundaryAreaOutcomeRow {
    pub(crate) fn new(
        outcome_identity: String,
        island_identity: String,
        neighborhood_identity: String,
        area_overlap_component_identities: Vec<String>,
        boundary_contact_component_identities: Vec<String>,
        cell_identities: Vec<String>,
    ) -> Self {
        Self {
            outcome_identity,
            island_identity,
            neighborhood_identity,
            area_overlap_component_identities,
            boundary_contact_component_identities,
            cell_identities,
        }
    }

    pub fn outcome_identity(&self) -> &str {
        &self.outcome_identity
    }

    pub fn island_identity(&self) -> &str {
        &self.island_identity
    }

    pub fn neighborhood_identity(&self) -> &str {
        &self.neighborhood_identity
    }

    pub fn area_overlap_component_identities(&self) -> &[String] {
        &self.area_overlap_component_identities
    }

    pub fn boundary_contact_component_identities(&self) -> &[String] {
        &self.boundary_contact_component_identities
    }

    pub fn cell_identities(&self) -> &[String] {
        &self.cell_identities
    }
}
