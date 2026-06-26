use crate::runtime::host::WorthUiRuntimeHost;
use crate::runtime::{
    WorthUiLaneAndFrameCostCertification, WorthUiLaneFrameCostCertificationDenial,
    WorthUiLaneFrameCostCertificationScenario,
};

impl WorthUiRuntimeHost {
    pub fn certify_lane_and_frame_costs_against_active_plan(
        &self,
        scenario: WorthUiLaneFrameCostCertificationScenario,
    ) -> Result<WorthUiLaneAndFrameCostCertification, WorthUiLaneFrameCostCertificationDenial> {
        WorthUiLaneAndFrameCostCertification::certify(
            scenario,
            self.inspect_active().active_plan_digest(),
        )
    }
}
