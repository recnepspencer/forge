use serde::Serialize;

use crate::compiled_product_family::TopologyPriorProofPosture;
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationSelectedPlan, DerivedInvalidationTouchedClosure,
};

use super::denial::{
    TopologyCompiledProductAdmissionError, TopologyCompiledProductAdmissionErrorKind,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum TopologyCompiledProductPriorProofBasis {
    NotRequired,
    SelectedPlan {
        selected_plan_digest: String,
        touched_closure_digest: String,
    },
}

impl TopologyCompiledProductPriorProofBasis {
    pub fn admit(
        posture: TopologyPriorProofPosture,
        selected_plan: Option<&DerivedInvalidationSelectedPlan>,
        touched_closure: Option<&DerivedInvalidationTouchedClosure>,
    ) -> Result<Self, TopologyCompiledProductAdmissionError> {
        match posture {
            TopologyPriorProofPosture::NotRequired => {
                match (selected_plan, touched_closure) {
                    (None, None) => Ok(Self::NotRequired),
                    (Some(selected_plan), Some(touched_closure)) => {
                        admit_selected_plan_basis(selected_plan, touched_closure)
                    }
                    _ => Err(TopologyCompiledProductAdmissionError::new(
                        TopologyCompiledProductAdmissionErrorKind::SelectedPlanRequired,
                        "topology compiled-product admission required both selected plan and touched closure when prior proof was supplied",
                    )),
                }
            }
            TopologyPriorProofPosture::DerivedInvalidationSelectedPlan => {
                let selected_plan = selected_plan.ok_or_else(|| {
                    TopologyCompiledProductAdmissionError::new(
                        TopologyCompiledProductAdmissionErrorKind::SelectedPlanRequired,
                        "topology compiled-product admission required a selected invalidation plan",
                    )
                })?;
                let touched_closure = touched_closure.ok_or_else(|| {
                    TopologyCompiledProductAdmissionError::new(
                        TopologyCompiledProductAdmissionErrorKind::SelectedPlanRequired,
                        "topology compiled-product admission required a touched closure",
                    )
                })?;
                admit_selected_plan_basis(selected_plan, touched_closure)
            }
        }
    }
}

fn admit_selected_plan_basis(
    selected_plan: &DerivedInvalidationSelectedPlan,
    touched_closure: &DerivedInvalidationTouchedClosure,
) -> Result<TopologyCompiledProductPriorProofBasis, TopologyCompiledProductAdmissionError> {
    if selected_plan.touched_closure_digest() != touched_closure.closure_digest() {
        return Err(TopologyCompiledProductAdmissionError::new(
            TopologyCompiledProductAdmissionErrorKind::TouchedClosureNotBoundToSelectedPlan,
            "selected invalidation plan was not bound to the provided touched closure",
        ));
    }
    Ok(TopologyCompiledProductPriorProofBasis::SelectedPlan {
        selected_plan_digest: selected_plan.selected_plan_digest().to_string(),
        touched_closure_digest: touched_closure.closure_digest().to_string(),
    })
}
