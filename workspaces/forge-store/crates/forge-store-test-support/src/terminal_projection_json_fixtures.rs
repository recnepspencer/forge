use forge_proof::TransitionOutcome;
use forge_store_aspect_native::{
    project_store_boundary_fact_to_terminal_json, StoreAspectBoundaryFact,
    StoreTerminalJsonProjection, StoreTerminalProjectionDenial,
};

use crate::{
    json_fixture_boundary::require_terminal_projection_boundary,
    StoreTerminalProjectionJsonFixtureBoundaryOutcome,
    StoreTerminalProjectionJsonFixtureBoundaryWitness,
};

#[derive(Debug, Clone)]
pub struct StoreTerminalProjectionJsonFixture {
    projection: StoreTerminalJsonProjection,
}

impl StoreTerminalProjectionJsonFixture {
    pub fn from_boundary_fact(
        fact: &StoreAspectBoundaryFact,
    ) -> Result<Self, StoreTerminalProjectionDenial> {
        project_store_boundary_fact_to_terminal_json(fact).map(|projection| Self { projection })
    }

    pub const fn projection(
        &self,
        _boundary: StoreTerminalProjectionJsonFixtureBoundaryWitness,
    ) -> &StoreTerminalJsonProjection {
        &self.projection
    }

    #[track_caller]
    pub fn allow_in_terminal_projection_suite(
        &self,
    ) -> StoreTerminalProjectionJsonFixtureBoundaryOutcome {
        require_terminal_projection_boundary()
    }

    pub fn deny_non_terminal_fixture_use(
        &self,
    ) -> StoreTerminalProjectionJsonFixtureBoundaryOutcome {
        TransitionOutcome::denied(
            crate::StoreJsonFixtureBoundaryDenial::TerminalProjectionJsonRequiresTerminalProjectionSuite,
        )
    }
}
