use crate::runtime::{
    WorthUiAdmittedProjectionPlan, WorthUiAdmittedRuntimeChangeEvidence,
    WorthUiProjectionPlanContract,
};

use super::{WorthUiProjectionRebindPlan, WorthUiProjectionRebindPlanDenial};

pub(crate) struct WorthUiProjectionRebindCoordinator;

impl WorthUiProjectionRebindCoordinator {
    pub(crate) fn prepare<P>(
        evidence: &WorthUiAdmittedRuntimeChangeEvidence,
        projection: WorthUiAdmittedProjectionPlan<P>,
    ) -> Result<WorthUiProjectionRebindPlan<P>, WorthUiProjectionRebindPlanDenial>
    where
        P: WorthUiProjectionPlanContract,
    {
        WorthUiProjectionRebindPlan::prepare(evidence, projection)
    }
}
