use forge_query::facade::ForgeQueryRuntimeFacadeFamily;

use super::birth_synopsis::TopologyPrimitiveConstructionQueryBirthSynopsis;

use super::surface_vocab::{
    TopologyConstructionQueryFactKind, TopologyConstructionQueryFactProvenance,
    TopologyConstructionQueryFactRow, TopologyConstructionQueryInspectionSurface,
    TopologyConstructionQueryMutationSurface, TopologyConstructionQueryReadSurface,
};
use super::REQUIRED_QUERY_FAMILIES;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyPrimitiveConstructionQueryReceipt {
    receipt_name: &'static str,
    source_birth_digest: String,
    topology_birth_class: String,
    mutation_surface: TopologyConstructionQueryMutationSurface,
    required_query_families: Vec<ForgeQueryRuntimeFacadeFamily>,
    read_surface: TopologyConstructionQueryReadSurface,
    inspection_surface: TopologyConstructionQueryInspectionSurface,
    fact_provenance: TopologyConstructionQueryFactProvenance,
    fact_rows: Vec<TopologyConstructionQueryFactRow>,
    fact_digest: String,
    receipt_digest: String,
}

impl TopologyPrimitiveConstructionQueryReceipt {
    pub(crate) fn new(synopsis: &TopologyPrimitiveConstructionQueryBirthSynopsis) -> Self {
        Self::new_from_counts(
            synopsis.source_birth_digest().to_string(),
            synopsis.topology_birth_class().to_string(),
            synopsis.supported_vertex_count(),
            synopsis.supported_edge_count(),
            synopsis.supported_loop_count(),
            synopsis.supported_wire_count(),
            synopsis.supported_face_count(),
            synopsis.supported_shell_count(),
            synopsis.supported_body_count(),
        )
    }

    fn new_from_counts(
        source_birth_digest: String,
        topology_birth_class: String,
        vertex_births: usize,
        edge_births: usize,
        loop_memberships: usize,
        wire_memberships: usize,
        face_memberships: usize,
        shell_memberships: usize,
        body_memberships: usize,
    ) -> Self {
        let receipt_name = "worth-topo.query-native-construction-receipt";
        let mutation_surface = TopologyConstructionQueryMutationSurface::ComposeGraph;
        let read_surface =
            TopologyConstructionQueryReadSurface::ProjectionConsumptionFromInspectionReceipt;
        let inspection_surface = TopologyConstructionQueryInspectionSurface::InspectReceipt;
        let fact_provenance =
            TopologyConstructionQueryFactProvenance::InspectionBackedProjectionConsumption;
        let required_query_families = REQUIRED_QUERY_FAMILIES.to_vec();
        let fact_rows = vec![
            TopologyConstructionQueryFactRow::new(
                TopologyConstructionQueryFactKind::VertexBirth,
                vertex_births,
            ),
            TopologyConstructionQueryFactRow::new(
                TopologyConstructionQueryFactKind::EdgeBirth,
                edge_births,
            ),
            TopologyConstructionQueryFactRow::new(
                TopologyConstructionQueryFactKind::LoopMembership,
                loop_memberships,
            ),
            TopologyConstructionQueryFactRow::new(
                TopologyConstructionQueryFactKind::WireMembership,
                wire_memberships,
            ),
            TopologyConstructionQueryFactRow::new(
                TopologyConstructionQueryFactKind::FaceMembership,
                face_memberships,
            ),
            TopologyConstructionQueryFactRow::new(
                TopologyConstructionQueryFactKind::ShellMembership,
                shell_memberships,
            ),
            TopologyConstructionQueryFactRow::new(
                TopologyConstructionQueryFactKind::BodyMembership,
                body_memberships,
            ),
        ];
        let fact_digest = super::digest_parts(
            &fact_rows
                .iter()
                .map(|row| row.row_digest().to_string())
                .collect::<Vec<_>>(),
        );
        let mut parts = vec![
            receipt_name.to_string(),
            source_birth_digest.clone(),
            topology_birth_class.clone(),
            mutation_surface.as_str().to_string(),
            read_surface.as_str().to_string(),
            inspection_surface.as_str().to_string(),
            fact_provenance.as_str().to_string(),
            fact_digest.clone(),
        ];
        parts.extend(
            required_query_families
                .iter()
                .map(|family| format!("required-query-family:{family:?}")),
        );
        parts.extend(fact_rows.iter().map(|row| row.row_digest().to_string()));
        Self {
            receipt_name,
            source_birth_digest,
            topology_birth_class,
            mutation_surface,
            required_query_families,
            read_surface,
            inspection_surface,
            fact_provenance,
            fact_rows,
            fact_digest,
            receipt_digest: super::digest_parts(&parts),
        }
    }

    #[cfg(test)]
    pub(crate) fn new_for_tests(
        source_birth_digest: &str,
        topology_birth_class: &str,
        vertex_births: usize,
        edge_births: usize,
        loop_memberships: usize,
        wire_memberships: usize,
        face_memberships: usize,
        shell_memberships: usize,
        body_memberships: usize,
    ) -> Self {
        Self::new_from_counts(
            source_birth_digest.to_string(),
            topology_birth_class.to_string(),
            vertex_births,
            edge_births,
            loop_memberships,
            wire_memberships,
            face_memberships,
            shell_memberships,
            body_memberships,
        )
    }

    pub fn receipt_name(&self) -> &str {
        self.receipt_name
    }

    pub fn source_birth_digest(&self) -> &str {
        &self.source_birth_digest
    }

    pub fn topology_birth_class(&self) -> &str {
        &self.topology_birth_class
    }

    pub fn mutation_surface(&self) -> TopologyConstructionQueryMutationSurface {
        self.mutation_surface
    }

    pub fn required_query_families(&self) -> &[ForgeQueryRuntimeFacadeFamily] {
        &self.required_query_families
    }

    pub fn read_surface(&self) -> TopologyConstructionQueryReadSurface {
        self.read_surface
    }

    pub fn inspection_surface(&self) -> TopologyConstructionQueryInspectionSurface {
        self.inspection_surface
    }

    pub fn fact_provenance(&self) -> TopologyConstructionQueryFactProvenance {
        self.fact_provenance
    }

    pub fn fact_rows(&self) -> &[TopologyConstructionQueryFactRow] {
        &self.fact_rows
    }

    pub fn fact_digest(&self) -> &str {
        &self.fact_digest
    }

    pub fn row_for(
        &self,
        kind: TopologyConstructionQueryFactKind,
    ) -> Option<&TopologyConstructionQueryFactRow> {
        self.fact_rows.iter().find(|row| row.kind() == kind)
    }

    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }
}
