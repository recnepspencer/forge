use crate::runtime::{
    WorthUiAdmittedProjectionPlan, WorthUiProjectionPlanAdmissionDenial,
    WorthUiProjectionPlanContract, WorthUiRuntimeHost, WorthUiRuntimeInstanceWitness,
};

impl WorthUiRuntimeHost {
    pub fn admit_projection_plan<P>(
        &self,
        plan: P,
    ) -> Result<WorthUiAdmittedProjectionPlan<P>, WorthUiProjectionPlanAdmissionDenial>
    where
        P: WorthUiProjectionPlanContract,
    {
        WorthUiAdmittedProjectionPlan::admit(
            plan,
            WorthUiRuntimeInstanceWitness::from_raw(self.instance_id().raw()),
        )
    }
}
