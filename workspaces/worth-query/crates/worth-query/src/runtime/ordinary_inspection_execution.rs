use super::{
    CausalInspectionPlan, QueryCausalInspectionArtifact, WorthQueryBackendInspectionError,
    WorthQueryRuntime,
};

impl WorthQueryRuntime {
    pub(crate) fn materialize_ordinary_inspection(
        &self,
        plan: &CausalInspectionPlan,
    ) -> Result<QueryCausalInspectionArtifact, WorthQueryBackendInspectionError> {
        self.backend.execute_query_causal_inspection(plan)
    }
}
