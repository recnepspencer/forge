use crate::runtime::{
    WorthUiAdmittedProjectionPlan, WorthUiAdmittedRuntimeChangeEvidence,
    WorthUiProjectionPlanContract, WorthUiProjectionRebindPlan, WorthUiProjectionRebindPlanDenial,
    WorthUiRuntimeHost,
};

use super::projection_rebind::WorthUiProjectionRebindCoordinator;

impl WorthUiRuntimeHost {
    pub(crate) fn prepare_projection_rebind<P>(
        &self,
        evidence: &WorthUiAdmittedRuntimeChangeEvidence,
        projection: WorthUiAdmittedProjectionPlan<P>,
    ) -> Result<WorthUiProjectionRebindPlan<P>, WorthUiProjectionRebindPlanDenial>
    where
        P: WorthUiProjectionPlanContract,
    {
        WorthUiProjectionRebindCoordinator::prepare(evidence, projection)
    }
}
