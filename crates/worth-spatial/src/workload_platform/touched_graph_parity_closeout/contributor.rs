use schema::facade::platform::authority::touched_graph_parity_closeout::{
    TouchedGraphParityCoverageContributor as SharedCoverageContributor,
    TouchedGraphParityQuerySurfaceKind as SharedQuerySurfaceKind,
};

pub type SpatialTouchedGraphParityCoverageContributor = SharedCoverageContributor;
pub type SpatialTouchedGraphParityQuerySurfaceKind = SharedQuerySurfaceKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpatialTouchedGraphParityCoverageError {
    detail: String,
}

impl SpatialTouchedGraphParityCoverageError {
    pub(crate) fn new(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
