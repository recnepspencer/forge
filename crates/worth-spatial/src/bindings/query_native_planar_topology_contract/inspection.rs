use crate::planar_contracts::topology_contract_completeness::PlanarTopologyContractCompletenessBasis;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarTopologyContractCompletenessInspectionKind {
    TopologyFact,
    DeclaredQuerySurface,
    PlanarNeighborhood,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarTopologyContractCompletenessInspectionRow {
    kind: PlanarTopologyContractCompletenessInspectionKind,
    locus: String,
    value: String,
}

impl PlanarTopologyContractCompletenessInspectionRow {
    pub(crate) fn from_basis(basis: &PlanarTopologyContractCompletenessBasis) -> Vec<Self> {
        let mut rows = basis
            .topology_query_receipt()
            .fact_rows()
            .iter()
            .map(|row| {
                inspection_row(
                    PlanarTopologyContractCompletenessInspectionKind::TopologyFact,
                    format!("topology.fact.{}", row.kind().as_str()),
                    row.row_digest(),
                )
            })
            .collect::<Vec<_>>();
        rows.push(inspection_row(
            PlanarTopologyContractCompletenessInspectionKind::DeclaredQuerySurface,
            "topology.declared_query_surface",
            basis.declared_query_surface_identity(),
        ));
        rows.push(inspection_row(
            PlanarTopologyContractCompletenessInspectionKind::PlanarNeighborhood,
            "topology.planar_neighborhood",
            basis.planar_neighborhood_identity(),
        ));
        rows
    }

    pub fn kind(&self) -> PlanarTopologyContractCompletenessInspectionKind {
        self.kind
    }

    pub fn locus(&self) -> &str {
        &self.locus
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

fn inspection_row(
    kind: PlanarTopologyContractCompletenessInspectionKind,
    locus: impl Into<String>,
    value: impl Into<String>,
) -> PlanarTopologyContractCompletenessInspectionRow {
    PlanarTopologyContractCompletenessInspectionRow {
        kind,
        locus: locus.into(),
        value: value.into(),
    }
}
