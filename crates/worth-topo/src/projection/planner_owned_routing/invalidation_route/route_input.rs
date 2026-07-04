use crate::derived_invalidation_selected_plan::{
    DerivedInvalidationExecutionAdmission, DerivedInvalidationSelectedPlan,
    DerivedInvalidationSelectedRow, DerivedInvalidationTouchedClosure,
};

use super::admission_error::TopologyInvalidationRouteInputAdmissionError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyInvalidationRouteInput {
    touched_closure: DerivedInvalidationTouchedClosure,
    selected_plan: DerivedInvalidationSelectedPlan,
}

pub fn admit_topology_invalidation_route_input(
    touched_closure: &DerivedInvalidationTouchedClosure,
    selected_plan: &DerivedInvalidationSelectedPlan,
) -> Result<TopologyInvalidationRouteInput, TopologyInvalidationRouteInputAdmissionError> {
    require_string_match(
        "touched closure digest",
        touched_closure.closure_digest(),
        selected_plan.touched_closure_digest(),
    )?;
    let observed_contract = touched_closure
        .conflict_routing_contract()
        .map_err(|error| {
            TopologyInvalidationRouteInputAdmissionError::new(format!(
            "invalidation route input could not lower touched-closure routing contract: {error:?}"
        ))
        })?;
    require_string_match(
        "routing contract digest",
        observed_contract.contract_digest(),
        selected_plan.routing_contract().contract_digest(),
    )?;
    Ok(TopologyInvalidationRouteInput {
        touched_closure: touched_closure.clone(),
        selected_plan: selected_plan.clone(),
    })
}

impl TopologyInvalidationRouteInput {
    pub fn touched_closure(&self) -> &DerivedInvalidationTouchedClosure {
        &self.touched_closure
    }

    pub fn selected_plan(&self) -> &DerivedInvalidationSelectedPlan {
        &self.selected_plan
    }

    pub fn touched_closure_digest(&self) -> &str {
        self.touched_closure.closure_digest()
    }

    pub fn selected_plan_digest(&self) -> &str {
        self.selected_plan.selected_plan_digest()
    }

    pub fn routing_contract_digest(&self) -> &str {
        self.selected_plan.routing_contract().contract_digest()
    }

    pub fn query_support_digest(&self) -> &str {
        self.selected_plan.query_support_digest()
    }

    pub fn legality_support_digest(&self) -> &str {
        self.selected_plan.legality_support_digest()
    }

    pub fn execution_admission(&self) -> DerivedInvalidationExecutionAdmission {
        self.selected_plan.execution_admission()
    }

    pub fn selected_rows(&self) -> &[DerivedInvalidationSelectedRow] {
        self.selected_plan.selected_rows()
    }
}

fn require_string_match(
    label: &str,
    observed: &str,
    expected: &str,
) -> Result<(), TopologyInvalidationRouteInputAdmissionError> {
    if observed != expected {
        return Err(TopologyInvalidationRouteInputAdmissionError::new(format!(
            "invalidation route input rejected mismatched {label}: expected {expected}, observed {observed}",
        )));
    }
    Ok(())
}
