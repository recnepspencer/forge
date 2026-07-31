use worth_query_declaration::{
    facade::{
        application_query::ApplicationQueryLiveCauseBinding,
        application_schema::{ApplicationEffectPayload, ApplicationEffectRef},
    },
    worth_query_effect,
};

use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct PlanningLiveEvent {
    account: u64,
    activity: u64,
}

impl ApplicationEffectPayload for PlanningLiveEvent {
    fn retained_bytes(&self) -> u64 {
        u64::try_from(std::mem::size_of::<Self>()).unwrap_or(u64::MAX)
    }
}

worth_query_effect!(
    pub(super) PlanningLiveEffect(PlanningLiveEvent) in PlanningTestSchema
);

pub(super) struct PlanningLiveCause;

impl ApplicationQueryLiveCauseBinding<PlanningTestSchema, ActivityQuery, Account, Activity>
    for PlanningLiveCause
{
    type Effect = PlanningLiveEffect;
    type Payload = PlanningLiveEvent;
    type ScopeIdentity = u64;
    type TargetIdentity = u64;

    fn effect() -> ApplicationEffectRef<PlanningTestSchema, Self::Effect, Self::Payload> {
        PlanningLiveEffect::reference()
    }

    fn scope_identity(payload: &Self::Payload) -> Self::ScopeIdentity {
        payload.account
    }

    fn target_identity(payload: &Self::Payload) -> Self::TargetIdentity {
        payload.activity
    }
}

#[test]
fn live_lane_adds_only_its_declared_maintenance_requirement() {
    let query = installed_query();
    let parameters = admit_application_query_parameters(
        &query,
        ApplicationQueryParameterSet::new().bind(account_parameter(), 7_u64),
    )
    .unwrap();
    let one_shot = admitted_requirements(
        query.read_graph(),
        WorthQueryApplicationQueryLane::OneShot,
        32,
        parameters.identity(),
    );
    let live = admitted_requirements(
        query.read_graph(),
        WorthQueryApplicationQueryLane::Live,
        32,
        parameters.identity(),
    );

    assert!(
        !one_shot.contains_kind(&WorthQueryGraphReadAccessRequirementKind::LiveMaintenanceSupport)
    );
    assert!(live.contains_kind(&WorthQueryGraphReadAccessRequirementKind::LiveMaintenanceSupport));
    assert_eq!(live.rows().len(), one_shot.rows().len() + 1);
}
