use crate::frontier_planning::FrontierSurfaceDigest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SignalFrontierSurfaceEvidence {
    surface_digest: FrontierSurfaceDigest,
    predicted_breadth: usize,
    realized_breadth: Option<usize>,
}

impl SignalFrontierSurfaceEvidence {
    pub(super) fn from_materialized_surface(
        surface_digest: FrontierSurfaceDigest,
        predicted_breadth: usize,
        realized_breadth: Option<usize>,
    ) -> Self {
        Self {
            surface_digest,
            predicted_breadth,
            realized_breadth,
        }
    }

    pub fn surface_digest(&self) -> &FrontierSurfaceDigest {
        &self.surface_digest
    }

    pub fn predicted_breadth(&self) -> usize {
        self.predicted_breadth
    }

    pub fn realized_breadth(&self) -> Option<usize> {
        self.realized_breadth
    }
}
