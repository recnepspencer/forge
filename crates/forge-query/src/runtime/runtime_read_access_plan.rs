use super::*;
use crate::intent_admission::ForgeQueryReadExecutionHandoff;

impl ForgeQueryRuntime {
    pub(crate) fn admit_graph_read_access_plan_for_handoff(
        &self,
        handoff: &ForgeQueryReadExecutionHandoff,
        authority: Option<&ForgeQueryGraphReadAccessAuthorityContext>,
    ) -> Result<ForgeQueryAdmittedGraphReadAccessPlan, ForgeQueryRuntimeError> {
        let admission = match authority {
            Some(authority) => self
                .admit_graph_read_access_for_family_in_authority(handoff.read_family(), authority),
            None => self.admit_graph_read_access_for_family(handoff.read_family()),
        }
        .map_err(|error| {
            ForgeQueryRuntimeError::ReadCompositionDenied(ForgeQueryReadDenial::new(
                ForgeQueryReadDenialKind::AuthoringDenied,
                error.as_str(),
            ))
        })?;
        ForgeQueryAdmittedGraphReadAccessPlan::from_admission(admission.clone()).ok_or_else(|| {
            let detail = admission
                .denial()
                .map(|denial| denial.kind().as_str())
                .unwrap_or("graph_read_access_not_admitted");
            ForgeQueryRuntimeError::ReadCompositionDenied(
                ForgeQueryReadDenial::new(ForgeQueryReadDenialKind::BasisPreflightDenied, detail)
                    .with_graph_read_persistent_artifact_audit_for_admission(&admission)
                    .with_graph_read_access_admission(admission)
                    .with_graph_read_access_execution_counters(
                        ForgeQueryGraphReadAccessExecutionCounters::pre_execution_denial(),
                    ),
            )
        })
    }
}
