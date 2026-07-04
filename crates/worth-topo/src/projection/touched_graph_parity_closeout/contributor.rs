use schema::facade::platform::authority::touched_graph_parity_closeout::{
    TouchedGraphParityCoverageContributor as SharedCoverageContributor,
    TouchedGraphParityQuerySurfaceKind as SharedQuerySurfaceKind,
};

pub type TopologyTouchedGraphParityCoverageContributor = SharedCoverageContributor;
pub type TopologyTouchedGraphParityQuerySurfaceKind = SharedQuerySurfaceKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyTouchedGraphParityCoverageError {
    detail: String,
}

impl TopologyTouchedGraphParityCoverageError {
    pub(crate) fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
