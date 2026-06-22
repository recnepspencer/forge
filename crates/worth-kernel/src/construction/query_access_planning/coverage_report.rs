use super::covered_surface::PrimitiveConstructionQueryAccessSurface;
use crate::construction::request::{PrimitiveConstructionFamily, PRIMITIVE_CONSTRUCTION_FAMILIES};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionQueryAccessCoverage {
    rows: Vec<PrimitiveConstructionQueryAccessCoverageRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveConstructionQueryAccessCoverageRow {
    family: PrimitiveConstructionFamily,
    surfaces: Vec<PrimitiveConstructionQueryAccessSurface>,
}

pub(crate) fn primitive_construction_query_access_coverage(
) -> PrimitiveConstructionQueryAccessCoverage {
    PrimitiveConstructionQueryAccessCoverage {
        rows: PRIMITIVE_CONSTRUCTION_FAMILIES
            .iter()
            .copied()
            .map(|family| PrimitiveConstructionQueryAccessCoverageRow {
                family,
                surfaces: vec![
                    PrimitiveConstructionQueryAccessSurface::TopologyBirth,
                    PrimitiveConstructionQueryAccessSurface::PhaseChainTopologyCheck,
                ],
            })
            .collect(),
    }
}

impl PrimitiveConstructionQueryAccessCoverage {
    pub(crate) fn rows(&self) -> &[PrimitiveConstructionQueryAccessCoverageRow] {
        &self.rows
    }
}

impl PrimitiveConstructionQueryAccessCoverageRow {
    pub(crate) fn family(&self) -> PrimitiveConstructionFamily {
        self.family
    }

    pub(crate) fn surfaces(&self) -> &[PrimitiveConstructionQueryAccessSurface] {
        &self.surfaces
    }
}
