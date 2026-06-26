use forge_query::facade::runtime::ForgeQueryGraphReadAccessRequirementCounters;

use super::row::WorthGraphReadQueryRequirementRowEvidence;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadQueryRequirementSetEvidence {
    query_requirement_set_digest: String,
    read_graph_digest: String,
    access_shape_digest: String,
    selectivity_shape_digest: String,
    query_requirement_rows: Vec<WorthGraphReadQueryRequirementRowEvidence>,
    query_requirement_counters: ForgeQueryGraphReadAccessRequirementCounters,
}

impl WorthGraphReadQueryRequirementSetEvidence {
    pub fn query_requirement_set_digest(&self) -> &str {
        &self.query_requirement_set_digest
    }

    pub fn read_graph_digest(&self) -> &str {
        &self.read_graph_digest
    }

    pub fn access_shape_digest(&self) -> &str {
        &self.access_shape_digest
    }

    pub fn selectivity_shape_digest(&self) -> &str {
        &self.selectivity_shape_digest
    }

    pub fn query_requirement_rows(&self) -> &[WorthGraphReadQueryRequirementRowEvidence] {
        &self.query_requirement_rows
    }

    pub fn query_requirement_counters(&self) -> &ForgeQueryGraphReadAccessRequirementCounters {
        &self.query_requirement_counters
    }
}
