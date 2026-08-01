use worth_ui::facade::rebind::UiRebindPlan;
use worth_ui_query_binding::{
    UiCollectionProjectionBindingAdmission, UiCollectionProjectionBudget,
    UiCollectionProjectionOpenOutcome, WorthUiQueryWorkspaceExt,
};

use super::{
    application,
    interaction_evidence::{native_host, OrderingInteractionWorld},
};
use crate::{
    intent::{
        execution::lifecycle::{
            AttemptStep, ExecutionScript, ScriptedProvider, ScriptedProviderObservation,
        },
        operability::OperabilityFacts,
    },
    projection_presentation::collection_query::{collection_plan, collection_registration},
};

pub(super) struct ReadyOrderingScenario {
    pub(super) interaction: OrderingInteractionWorld,
    pub(super) workspace: worth_query::facade::runtime::WorthQueryWorkspace,
    pub(super) entities: Vec<worth_query::facade::foundation::WorthQueryEntityIdentity>,
    pub(super) live: worth_ui_query_binding::UiLiveCollectionProjection,
    pub(super) facts: OperabilityFacts,
    pub(super) provider_observation: ScriptedProviderObservation,
    pub(super) predecessor_plan: UiRebindPlan,
}

pub(super) struct SettledOrderingScenario {
    pub(super) interaction: OrderingInteractionWorld,
    pub(super) workspace: worth_query::facade::runtime::WorthQueryWorkspace,
    pub(super) live: worth_ui_query_binding::UiLiveCollectionProjection,
    pub(super) provider_observation: ScriptedProviderObservation,
}

impl ReadyOrderingScenario {
    pub(super) fn launch() -> Self {
        let (provider, provider_observation) = ScriptedProvider::new([ExecutionScript::running([
            AttemptStep::PendingEffectMayHaveBegun,
        ])
        .with_cancellations([AttemptStep::Completed])]);
        let (mut workspace, entities) = seeded_workspace();
        let domain = workspace
            .worth_ui()
            .expect("the IA-09 Query domain is installed");
        let registration = collection_registration(&domain);
        let host = native_host();
        let (app, facts) = application::build(registration.clone(), host.clone(), provider);
        let mut interaction = OrderingInteractionWorld::launch(app, host);
        let binding = match registration.admit(&workspace) {
            UiCollectionProjectionBindingAdmission::Ready(binding) => binding,
            UiCollectionProjectionBindingAdmission::Stopped(stop) => {
                panic!("the IA-09 collection binding admits: {stop:?}")
            }
        };
        let opened = match binding.open(
            UiCollectionProjectionBudget::new(2, 4, 0, 2_048).unwrap(),
            &mut workspace,
        ) {
            UiCollectionProjectionOpenOutcome::Opened(opened) => opened,
            UiCollectionProjectionOpenOutcome::Stopped(stop) => {
                panic!("the IA-09 collection projection opens: {stop:?}")
            }
        };
        let (live, snapshot) = opened.into_parts();
        let predecessor_plan =
            collection_plan(&mut interaction.session, snapshot.into_observation());
        Self {
            interaction,
            workspace,
            entities,
            live,
            facts,
            provider_observation,
            predecessor_plan,
        }
    }
}

impl SettledOrderingScenario {
    pub(super) fn finish(mut self) -> [usize; 7] {
        match self.live.close(&mut self.workspace) {
            worth_ui_query_binding::UiLiveCollectionProjectionCloseOutcome::Closed(_) => {}
            worth_ui_query_binding::UiLiveCollectionProjectionCloseOutcome::Stopped(stop) => {
                panic!("the IA-09 live Query closes: {:?}", stop.query_error())
            }
        }
        let shutdown = self.interaction.session.shutdown();
        assert_eq!(shutdown.intent_execution().active_after(), 0);
        assert_eq!(shutdown.intent_admission().active_after(), 0);
        assert!(shutdown.rebind().is_empty());
        assert!(shutdown.mounted_presentation().is_empty());
        let counts = self.provider_observation.counts();
        assert_eq!(counts, [1, 1, 1, 0, 1, 0, 1]);
        counts
    }
}

fn seeded_workspace() -> (
    worth_query::facade::runtime::WorthQueryWorkspace,
    Vec<worth_query::facade::foundation::WorthQueryEntityIdentity>,
) {
    worth_ui_query_binding::certification::seeded_collection_projection_workspace(
        vec![
            ("pulse.alpha".to_owned(), "Alpha".to_owned()),
            ("pulse.bravo".to_owned(), "Bravo".to_owned()),
        ],
        worth_ui_query_binding::certification::WorthUiCollectionProjectionSeedPosture::Complete,
    )
}
