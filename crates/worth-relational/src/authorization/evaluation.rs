use crate::logic::runtime::RelationalRuntime;

use super::field_observation::entity_is_live_kind;
use super::observation_identity::mint_observation_identity;
use super::path_evaluation::evaluate_path;
use super::{
    RelationalAuthorizationObservationCounters, RelationalAuthorizationObservationDenial,
    RelationalAuthorizationObservationEvidence, RelationalAuthorizationObservationPlan,
    RelationalAuthorizationPathObservation,
};

impl RelationalRuntime {
    pub fn observe_authorization(
        &self,
        plan: RelationalAuthorizationObservationPlan,
    ) -> Result<RelationalAuthorizationObservationEvidence, RelationalAuthorizationObservationDenial>
    {
        let evaluation = self.evaluate_authorization_plan(&plan)?;
        let observation_identity = mint_observation_identity(&plan)
            .ok_or(RelationalAuthorizationObservationDenial::ObservationIdentityExhausted)?;
        Ok(RelationalAuthorizationObservationEvidence::mint(
            plan,
            observation_identity,
            evaluation.paths,
            evaluation.counters,
        ))
    }

    pub(super) fn evaluate_authorization_plan(
        &self,
        plan: &RelationalAuthorizationObservationPlan,
    ) -> Result<RelationalAuthorizationEvaluation, RelationalAuthorizationObservationDenial> {
        if plan.snapshot().runtime_instance_id != self.runtime_instance_id() {
            return Err(RelationalAuthorizationObservationDenial::ForeignRuntime {
                expected_runtime_instance_id: self.runtime_instance_id(),
                actual_runtime_instance_id: plan.snapshot().runtime_instance_id,
            });
        }
        let mut counters = RelationalAuthorizationObservationCounters::default();
        let view = self
            .read_truth()
            .project_snapshot(plan.snapshot())
            .ok_or(RelationalAuthorizationObservationDenial::SnapshotUnavailable)?;
        if !entity_is_live_kind(
            &view,
            plan.principal(),
            plan.principal_kind(),
            &mut counters,
        ) {
            return Err(RelationalAuthorizationObservationDenial::PrincipalUnavailableOrWrongKind);
        }
        if plan.scope() != plan.principal()
            && !entity_is_live_kind(&view, plan.scope(), plan.scope_kind(), &mut counters)
        {
            return Err(RelationalAuthorizationObservationDenial::ScopeUnavailableOrWrongKind);
        }
        let paths = plan
            .paths()
            .iter()
            .map(|path| evaluate_path(self, &view, plan, path, &mut counters))
            .collect();
        Ok(RelationalAuthorizationEvaluation { paths, counters })
    }
}

pub(super) struct RelationalAuthorizationEvaluation {
    pub(super) paths: Vec<RelationalAuthorizationPathObservation>,
    pub(super) counters: RelationalAuthorizationObservationCounters,
}
