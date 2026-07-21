use worth_ui::facade::app::{
    WorthUiApplicationReplacementPreparationDenial, WorthUiHostMeasurementSessionInput,
    WorthUiPreparedApplicationGenerationIdentity,
};
use worth_ui::facade::host::{
    UiHostMeasurementAssumptionProfile, UiHostMeasurementNeed,
    UiHostMeasurementNormalizationContext,
};
use worth_ui::facade::inspection::{
    UiEvidenceAuthorityGeneration, UiInspectionDeclarationIdentity, UiInspectionQuery,
    UiInspectionScope, UiInspectionTarget,
};
use worth_ui::facade::runtime::WorthUiTransientInteractionState;
use worth_ui_host_contract::{
    UiMeasurementEvidenceFamily, UiMeasurementRequestIdentity, UiViewportExtentRequest,
};
use worth_ui_query_binding::certification::WorthUiInstalledQueryTestFixture;

use super::application_definition::{
    application_builder, application_builder_with_capability_drift, application_builder_with_host,
};
use super::authored_composition::{candidate_file, current_file, current_rust};
use super::candidate_catalog::admit_candidate_catalog;
use super::foreign_graph_authority::equal_visible_graph_evidence_cannot_cross_candidate_authority;
use super::operational_host::AuthorityClosureHost;
use super::report::{ApplicationAuthorityClosureReport, ApplicationPlanningObservation};

pub fn certify_application_authority_closure() -> ApplicationAuthorityClosureReport {
    let mut file_query = WorthUiInstalledQueryTestFixture::new("authority-closure-file");
    let file_snapshot = application_builder(&file_query)
        .freeze()
        .expect("file snapshot should prepare");
    let rust_snapshot = application_builder(&file_query)
        .freeze()
        .expect("Rust snapshot should prepare");
    let file_app = application_builder(&file_query)
        .with_candidate_submission(current_file(file_snapshot.capabilities()))
        .freeze()
        .expect("file-authored application should prepare");
    let rust_app = application_builder(&file_query)
        .with_candidate_submission(current_rust(rust_snapshot.capabilities()))
        .freeze()
        .expect("Rust-authored application should prepare");
    let file_rust_converged = file_app.generation_identity() == rust_app.generation_identity()
        && file_app.graph().node_count() == rust_app.graph().node_count()
        && file_app.capabilities().digest() == rust_app.capabilities().digest();
    assert!(
        file_rust_converged,
        "equivalent authoring lanes must converge"
    );

    let mut rust_session = rust_app.launch().expect("Rust-authored app should launch");
    assert_generation_boundaries(&mut rust_session);
    let _ = rust_session.shutdown();

    let operational_file_app = application_builder_with_host(&file_query, AuthorityClosureHost)
        .with_candidate_submission(current_file(file_snapshot.capabilities()))
        .freeze()
        .expect("operational file-authored application should prepare");
    let mut session = operational_file_app
        .launch()
        .expect("file-authored app should launch");
    let host_session = session.host_session_identity();
    assert_eq!(session.session_identity().as_u64(), host_session.as_u64());
    assert_operational_host_evidence(&mut session);
    let initial_generation = session.generation_identity().clone();
    assert_generation_boundaries(&mut session);

    let equivalent = session
        .prepare_replacement(current_file(session.capabilities()))
        .expect("equivalent replacement should prepare");
    let equivalent = session
        .lower_prepared_replacement(*equivalent)
        .expect("equivalent replacement must not stop at digest comparison");
    let equivalent = session
        .stage_prepared_replacement(equivalent)
        .expect("equivalent replacement must reach staged candidate authority");
    drop(equivalent);
    assert_eq!(session.generation_identity(), &initial_generation);
    let drifted_snapshot = application_builder_with_capability_drift(&file_query)
        .freeze()
        .expect("drifted capability snapshot should prepare");
    let invalid = session.prepare_replacement(candidate_file(drifted_snapshot.capabilities()));
    let Err(WorthUiApplicationReplacementPreparationDenial::Preparation(_)) = invalid else {
        panic!("snapshot-drifted replacement must deny at active preparation");
    };
    assert_eq!(session.generation_identity(), &initial_generation);
    let foreign_graph_denied =
        equal_visible_graph_evidence_cannot_cross_candidate_authority(&mut session);
    assert!(foreign_graph_denied);

    let prepared = session
        .prepare_replacement(candidate_file(session.capabilities()))
        .expect("structural replacement should prepare");
    let mut prepared = prepared;
    let candidate_inspection = prepared.inspect_candidate(UiInspectionQuery::new(
        UiInspectionTarget::product_root(),
        UiInspectionScope::graph(),
    ));
    assert_ne!(
        candidate_inspection.generation_identity(),
        &initial_generation
    );
    let catalog = admit_candidate_catalog(&session, &mut prepared);
    let lowered = session
        .lower_prepared_replacement(*prepared)
        .expect("prepared replacement should lower");
    let pending = session
        .stage_prepared_replacement(lowered)
        .expect("lowered replacement should stage");
    let boundary_turn = session.execute_framework_turn(|_| {});
    assert_eq!(boundary_turn.generation_identity(), &initial_generation);
    let boundary_completion = boundary_turn.into_completion();
    let boundary_counters = boundary_completion
        .planning_counters()
        .expect("activation boundary should pass through transition planning");
    assert_eq!(boundary_counters.policy_family_count(), 0);
    assert_eq!(boundary_counters.policy_classification_count(), 0);
    let boundary = boundary_completion
        .into_execution()
        .expect("empty turn should publish an activation boundary")
        .into_activation_boundary();
    let cutover = session
        .activate_prepared_replacement(pending, catalog, boundary, None)
        .expect("candidate-owned catalog should cut over atomically");
    let cutover = cutover
        .into_activation()
        .expect("changed candidate publishes a successor");
    assert_eq!(cutover.prior_generation(), &initial_generation);
    assert_eq!(cutover.active_generation(), session.generation_identity());
    assert!(cutover.publication().generation_is_coherent());
    assert!(cutover.publication().host_is_coherent());
    assert_eq!(
        cutover.publication().application_generation(),
        session.generation_identity()
    );
    assert_eq!(
        cutover.publication().runtime().active_plan_digest(),
        cutover.plan_swap().next_active_plan_digest()
    );
    assert!(matches!(
        cutover.publication().scheduler(),
        worth_ui::facade::runtime::UiAllocationFrameDispatcherState::Open(_)
    ));
    assert_ne!(session.generation_identity(), &initial_generation);
    assert_eq!(session.host_session_identity(), host_session);

    let active_generation = session.generation_identity().clone();
    let interaction_target = session
        .graph()
        .allocation_planning_node_identities()
        .next()
        .expect("active catalog should retain one planning root");
    let active_graph_generation = session.graph().generation();
    let mut interaction_admitted = false;
    let interaction = session.execute_framework_turn(|turn| {
        turn.interaction(|source| {
            interaction_admitted = source
                .admit_and_submit(
                    interaction_target,
                    WorthUiTransientInteractionState::TextInput,
                )
                .is_ok();
        });
    });
    assert!(
        interaction_admitted,
        "active graph interaction should admit"
    );
    assert_eq!(interaction.generation_identity(), &active_generation);
    let interaction = interaction.into_completion();
    let counters = interaction
        .planning_counters()
        .expect("ordinary interaction should publish transition planning counters");
    let selection = interaction
        .replan_selection()
        .expect("ordinary interaction should select the active planning root");
    assert_eq!(
        selection.primary().locality().graph_generation(),
        active_graph_generation
    );
    let transaction = interaction
        .replan_transaction()
        .expect("ordinary interaction should retain its allocation transaction");
    let receipts = match transaction {
        worth_ui::facade::runtime::UiAllocationReplanTransactionOutcome::Committed(committed)
        | worth_ui::facade::runtime::UiAllocationReplanTransactionOutcome::Replayed(committed) => {
            committed.receipts()
        }
        worth_ui::facade::runtime::UiAllocationReplanTransactionOutcome::Denied(denial) => {
            panic!("active allocation transaction should commit: {denial:?}")
        }
    };
    assert!(!receipts.is_empty());
    assert!(receipts.iter().all(|receipt| {
        receipt.generation().neighborhood_generation() == active_graph_generation
    }));
    drop(interaction);

    let mut projection_admitted = false;
    let completion = session.execute_framework_turn(|turn| {
        turn.query_projection(|source| {
            projection_admitted = source.admit_and_submit(file_query.project()).is_ok();
        });
    });
    assert!(
        projection_admitted,
        "registered Query authority should submit"
    );
    assert_eq!(completion.generation_identity(), &active_generation);
    drop(completion.into_completion());

    assert_generation_boundaries(&mut session);
    let graph = session.graph();
    let node = graph
        .node_identities()
        .next()
        .expect("active graph should retain a node");
    let declaration = graph
        .lookup()
        .graph_node(node)
        .expect("active graph lookup should resolve its node")
        .value()
        .declaration_identity()
        .clone();
    let inspection = session.inspect(UiInspectionQuery::new(
        UiInspectionTarget::declaration_identity(UiInspectionDeclarationIdentity::new(
            declaration.digest().raw(),
        )),
        UiInspectionScope::graph(),
    ));
    assert_eq!(inspection.generation_identity(), &active_generation);
    assert_eq!(
        session.inspect_runtime().generation_identity(),
        &active_generation
    );
    assert!(graph.generation().as_u64() > 0);

    ApplicationAuthorityClosureReport::new(
        cutover.prior_generation() != cutover.active_generation(),
        file_rust_converged,
        graph.node_count(),
        session.capabilities().view_bindings().len(),
        session.host_session_identity() == host_session,
        ApplicationPlanningObservation::new(
            counters.policy_family_count(),
            counters.policy_classification_count(),
        ),
        foreign_graph_denied,
    )
}

fn assert_operational_host_evidence(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
) {
    let capability = session.host_measurement_capability();
    let profile = UiHostMeasurementAssumptionProfile::from_capability_report(
        capability.capability_report(),
        1,
        2,
        3,
        4,
    );
    let input = WorthUiHostMeasurementSessionInput::new(
        UiMeasurementRequestIdentity::new(1),
        UiMeasurementEvidenceFamily::ViewportExtent,
        UiHostMeasurementNeed::ViewportExtent(UiViewportExtentRequest),
        UiEvidenceAuthorityGeneration::new(1),
        UiHostMeasurementNormalizationContext::viewport_logical_exact(profile),
    );
    let generation = session.generation_identity().clone();
    let mut admitted = false;
    let completion = session.execute_framework_turn(|turn| {
        turn.host_measurement(|source| {
            admitted = source
                .collect_and_submit_capability(&capability, input)
                .is_ok();
        });
    });
    assert!(
        admitted,
        "operational host evidence must enter the active turn"
    );
    assert_eq!(completion.generation_identity(), &generation);
    drop(completion.into_completion());
}

fn assert_generation_boundaries(
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
) {
    let generation: WorthUiPreparedApplicationGenerationIdentity =
        session.generation_identity().clone();
    let inspection = session.inspect(UiInspectionQuery::new(
        UiInspectionTarget::product_root(),
        UiInspectionScope::graph(),
    ));
    let completion = session.execute_framework_turn(|_| {});
    assert_eq!(completion.generation_identity(), &generation);
    drop(completion);
    assert_eq!(inspection.generation_identity(), &generation);
    assert_eq!(session.inspect_runtime().generation_identity(), &generation);
    assert_eq!(
        session.host_measurement_capability().session_identity(),
        session.host_session_identity()
    );
}
