use worth_ui::facade::app::WorthUiActiveApplicationSession;
use worth_ui::facade::observation::{
    UiChangeClassificationDenial, UiChangeClassificationOutcome, UiClassifiedChange,
    UiObservationProfile,
};
use worth_ui::facade::rebind::{
    UiAffectedScopeDenial, UiChangeProfile, UiRebindExecutionPolicy, UiRebindLimit,
    UiRebindPlanningDenial, UiRebindProfile, UiResolvedIdentityLifecycle,
};
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;
use worth_ui_test_support::{
    UiRebindPlanningBasisMutation, UiResolvedIdentityLifecycleCertificationExt,
};

use super::filesystem_contract_workspace::FilesystemContractWorkspace;

#[test]
fn rebind_planning_denies_exhaustion_and_stale_basis() {
    prove_rebind_planning_denies_exhaustion_and_stale_basis();
}

pub(crate) fn prove_rebind_planning_denies_exhaustion_and_stale_basis() {
    classification_denies_before_changed_fact_retention();
    scope_denies_before_widening();
    planning_denies_before_plan_retention();
    planning_rejects_each_stale_identity_basis();
    successful_plan_retains_the_exact_configured_budget();
}

fn classification_denies_before_changed_fact_retention() {
    let mut world = RebindWorld::new(
        "phase-312-classification-budget",
        profile_with_limit(UiRebindLimit::ChangedFacts, 1),
    );
    let predecessor = world.session.generation_identity().clone();
    assert!(matches!(
        world.classify_candidate(),
        Err(UiChangeClassificationDenial::ChangedFactCapacityExceeded {
            limit: 1,
            observed: 2
        })
    ));
    assert_eq!(world.session.generation_identity(), &predecessor);
    world.close();
}

fn scope_denies_before_widening() {
    let mut world = RebindWorld::new(
        "phase-312-scope-budget",
        profile_with_limit(UiRebindLimit::DistinctConsumers, 3),
    );
    let predecessor = world.session.generation_identity().clone();
    let changed = world.changed_candidate();
    assert!(matches!(
        world.session.resolve_affected_scope(changed),
        Err(UiAffectedScopeDenial::BudgetExceeded {
            limit: UiRebindLimit::DistinctConsumers,
            configured: 3,
            observed: 4
        })
    ));
    assert_eq!(world.session.generation_identity(), &predecessor);
    world.close();
}

fn planning_denies_before_plan_retention() {
    let mut world = RebindWorld::new(
        "phase-312-plan-budget",
        profile_with_limit(UiRebindLimit::TerminalDecisionRecords, 3),
    );
    let predecessor = world.session.generation_identity().clone();
    let lifecycle = world.lifecycle();
    assert!(matches!(
        world
            .session
            .compile_rebind_plan(lifecycle, UiRebindExecutionPolicy::ordinary()),
        Err(UiRebindPlanningDenial::BudgetExceeded {
            limit: UiRebindLimit::TerminalDecisionRecords,
            configured: 3,
            observed: 4
        })
    ));
    assert_eq!(world.session.generation_identity(), &predecessor);
    world.close();
}

fn planning_rejects_each_stale_identity_basis() {
    stale_session_is_rejected();
    stale_predecessor_is_rejected();
    stale_candidate_is_rejected();
}

fn stale_session_is_rejected() {
    let mut world = RebindWorld::new("phase-312-stale-session", UiChangeProfile::platform_pulse());
    let foreign = RebindWorld::new(
        "phase-312-foreign-session",
        UiChangeProfile::platform_pulse(),
    );
    let lifecycle = world
        .lifecycle()
        .with_planning_basis_mutation_for_certification(UiRebindPlanningBasisMutation::Session(
            foreign.session.session_identity(),
        ));
    assert!(matches!(
        world
            .session
            .compile_rebind_plan(lifecycle, UiRebindExecutionPolicy::ordinary()),
        Err(UiRebindPlanningDenial::ForeignSession)
    ));
    foreign.close();
    world.close();
}

fn stale_predecessor_is_rejected() {
    let mut world = RebindWorld::new(
        "phase-312-stale-predecessor",
        UiChangeProfile::platform_pulse(),
    );
    let lifecycle = world.lifecycle();
    let candidate = lifecycle.scope().basis().candidate_generation().clone();
    let lifecycle = lifecycle.with_planning_basis_mutation_for_certification(
        UiRebindPlanningBasisMutation::PredecessorGeneration(candidate),
    );
    assert!(matches!(
        world
            .session
            .compile_rebind_plan(lifecycle, UiRebindExecutionPolicy::ordinary()),
        Err(UiRebindPlanningDenial::StalePredecessorGeneration)
    ));
    world.close();
}

fn stale_candidate_is_rejected() {
    let mut world = RebindWorld::new(
        "phase-312-stale-candidate",
        UiChangeProfile::platform_pulse(),
    );
    let predecessor = world.session.generation_identity().clone();
    let lifecycle = world
        .lifecycle()
        .with_planning_basis_mutation_for_certification(
            UiRebindPlanningBasisMutation::CandidateGeneration(predecessor),
        );
    assert!(matches!(
        world
            .session
            .compile_rebind_plan(lifecycle, UiRebindExecutionPolicy::ordinary()),
        Err(UiRebindPlanningDenial::StaleCandidateGeneration)
    ));
    world.close();
}

fn successful_plan_retains_the_exact_configured_budget() {
    let profile = UiChangeProfile::platform_pulse();
    let expected = profile.rebind().budget();
    let mut world = RebindWorld::new("phase-312-plan-budget-retention", profile);
    let lifecycle = world.lifecycle();
    let plan = world
        .session
        .compile_rebind_plan(lifecycle, UiRebindExecutionPolicy::ordinary())
        .expect("a within-budget real filesystem plan compiles");
    assert_eq!(plan.budget(), expected);
    drop(plan);
    world.close();
}

struct RebindWorld {
    scenario: FilesystemApplicationLifecycleScenario,
    workspace: FilesystemContractWorkspace,
    session: WorthUiActiveApplicationSession,
}

impl RebindWorld {
    fn new(label: &str, profile: UiChangeProfile) -> Self {
        let scenario = FilesystemApplicationLifecycleScenario::new(label);
        let workspace = FilesystemContractWorkspace::new(label);
        workspace.write(
            "app/main.wui",
            &FilesystemApplicationLifecycleScenario::dual_generation_scope_initial_source_text(),
        );
        let provider = WorthUiFilesystemSourceProvider::new(workspace.root());
        let capabilities = scenario.capability_application();
        let initial = FilesystemApplicationLifecycleScenario::lower_snapshot(
            provider.read().expect("initial filesystem world reads"),
            capabilities.capabilities(),
        );
        let session = scenario
            .prepare_application_with_change_profile(initial, profile)
            .launch()
            .expect("small-profile filesystem world launches");
        Self {
            scenario,
            workspace,
            session,
        }
    }

    fn classify_candidate(
        &mut self,
    ) -> Result<UiChangeClassificationOutcome, UiChangeClassificationDenial> {
        self.workspace.write(
            "app/main.wui",
            &FilesystemApplicationLifecycleScenario::dual_generation_scope_candidate_source_text(),
        );
        let provider = WorthUiFilesystemSourceProvider::new(self.workspace.root());
        let candidate = FilesystemApplicationLifecycleScenario::lower_snapshot(
            provider.read().expect("candidate filesystem world reads"),
            self.session.capabilities(),
        );
        let mut turn = self.session.begin_observation_turn().unwrap();
        turn.admit_source(candidate).unwrap();
        let admitted = turn.seal().unwrap();
        self.session.classify_observations(admitted)
    }

    fn changed_candidate(&mut self) -> UiClassifiedChange {
        match self.classify_candidate().expect("candidate classifies") {
            UiChangeClassificationOutcome::Changed(changed) => changed,
            _ => panic!("the dual-generation edit must remain changed"),
        }
    }

    fn lifecycle(&mut self) -> UiResolvedIdentityLifecycle {
        let changed = self.changed_candidate();
        self.session
            .resolve_affected_scope(changed)
            .expect("within-budget scope resolves")
            .resolve_identity_lifecycle()
            .expect("identity lifecycle resolves")
    }

    fn close(self) {
        let _ = self.session.shutdown();
        self.workspace.close();
        drop(self.scenario);
    }
}

fn profile_with_limit(limit: UiRebindLimit, value: usize) -> UiChangeProfile {
    let standard = UiRebindProfile::platform_pulse();
    let mut budget = standard.budget();
    match limit {
        UiRebindLimit::ChangedFacts => budget.changed_facts = value,
        UiRebindLimit::DistinctConsumers => budget.distinct_consumers = value,
        UiRebindLimit::TerminalDecisionRecords => budget.terminal_decision_records = value,
        _ => panic!("the P3-12 fixture supports only exercised limits"),
    }
    let rebind = UiRebindProfile::bounded(budget, standard.concurrency())
        .expect("small nonzero certification limit is valid");
    UiChangeProfile::new(UiObservationProfile::platform_pulse(), rebind)
}
