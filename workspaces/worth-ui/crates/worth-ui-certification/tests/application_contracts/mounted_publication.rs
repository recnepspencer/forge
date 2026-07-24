use worth_ui::facade::app::{
    WorthUiMountedApplicationReplacementOutcome, WorthUiMountedReplacementPreparationOutcome,
};
use worth_ui::facade::graph::UiAdmittedAllocationCatalogDelta;
use worth_ui::facade::mounted::{
    UiHostSurfaceCancellationOutcome, UiMountedFrameOutcome, UiMountedFrameRequest,
    UiMountedFrameReuse, UiMountedPresentationCompletionDenial, UiPresentationDeadline,
};
use worth_ui::facade::runtime::WorthUiFrameBoundary;
use worth_ui::facade::source::WorthUiFilesystemSourceProvider;
use worth_ui_certification::scenario::application_authority_closure::candidate_catalog::admit_candidate_catalog;
use worth_ui_certification::scenario::filesystem_application_lifecycle::FilesystemApplicationLifecycleScenario;

use super::filesystem_contract_workspace::FilesystemContractWorkspace;
use super::mounted_application_lifecycle::in_flight_presentation_world::{
    mounted_session, prepared,
};
use super::mounted_application_lifecycle::known_empty_surface_world::profile;
use super::mounted_host_protocol::scripted_host::{
    presented_completion, ScriptedPresentationHost,
    ScriptedSurfaceCompletion as UiHostSurfaceInFlightCompletion,
};
use super::mounted_protocol_model::ModelPublicationWorld;

#[path = "mounted_publication/exact_reuse.rs"]
mod exact_reuse;

#[test]
fn accepted_frame_owns_the_successor_slot_until_terminal_publication() {
    let host = ScriptedPresentationHost::default();
    let (mut session, _) = mounted_session(host.clone(), "mounted-publication-lease", 1);
    let candidate = prepared(&mut session);
    let candidate_frame = candidate.canonical_core().frame();
    let workspace = replacement_workspace("mounted-publication-lease");
    let (pending, catalog, boundary) = stage_replacement(&workspace, &mut session);
    host.push_in_flight(
        vec![presented_completion()],
        UiHostSurfaceCancellationOutcome::EffectsMayHaveBegun,
    );
    let handle = in_flight(session.present_prepared_mounted_frame(
        candidate,
        UiPresentationDeadline::at_tick(20),
        0,
    ));
    let mut model = ModelPublicationWorld::default();
    model.begin_presentation();
    let observed_handle = handle.clone();
    drop(handle);

    assert!(!model.successor_mutation_allowed());
    assert!(matches!(
        session.execute_framework_turn(|_| {}),
        Err(worth_ui::facade::mounted::UiMountedPublicationLeaseDenial::PresentationInFlight)
    ));
    let binding = session.inspect_mounted_identity().surface_bindings()[0];
    assert_eq!(
        session
            .rebind_host_surface(
                binding.binding_generation(),
                binding.presentation_mode(),
                profile(2),
            )
            .unwrap_err(),
        worth_ui::facade::mounted::UiMountedIdentityDenial::PresentationInFlight
    );
    let replacement_denial = match session.prepare_mounted_replacement(
        pending,
        catalog,
        boundary,
        None,
        UiMountedFrameRequest::all_bound_surfaces(),
    ) {
        Err(denial) => denial,
        Ok(_) => panic!("a staged successor cannot cut over while A owns the publication slot"),
    };
    assert!(matches!(
        replacement_denial,
        worth_ui::facade::app::WorthUiApplicationCutoverDenial::MountedPresentationInFlight
    ));
    assert!(session.current_mounted_publication().is_none());

    let first = published(session.complete_mounted_presentation(observed_handle, 1));
    model.complete_presentation(1);
    assert_eq!(first.frame(), candidate_frame);
    assert_eq!(model.current_frame_ordinal(), Some(1));

    let (pending, catalog, boundary) = stage_replacement(&workspace, &mut session);
    let replacement = match session
        .prepare_mounted_replacement(
            pending,
            catalog,
            boundary,
            None,
            UiMountedFrameRequest::all_bound_surfaces(),
        )
        .unwrap()
    {
        WorthUiMountedReplacementPreparationOutcome::Prepared(replacement) => replacement,
        WorthUiMountedReplacementPreparationOutcome::SemanticNoOp(_) => {
            panic!("the filesystem successor remains a real semantic change")
        }
    };
    host.push_presented();
    let second = match replacement.present(UiPresentationDeadline::at_tick(30), 2) {
        WorthUiMountedApplicationReplacementOutcome::Published { mounted, .. } => mounted,
        _ => panic!("B publishes after A reaches a terminal state"),
    };
    assert_eq!(second.predecessor(), Some(first.frame()));
    workspace.close();
}

#[test]
fn session_publication_rejects_a_foreign_completion_without_losing_its_attempt() {
    let left_host = ScriptedPresentationHost::default();
    let right_host = ScriptedPresentationHost::default();
    let (mut left, _) = mounted_session(left_host.clone(), "publication-left", 1);
    let (mut right, _) = mounted_session(right_host.clone(), "publication-right", 1);
    left_host.push_in_flight(
        vec![presented_completion()],
        UiHostSurfaceCancellationOutcome::EffectsMayHaveBegun,
    );
    right_host.push_in_flight(
        vec![UiHostSurfaceInFlightCompletion::Pending],
        UiHostSurfaceCancellationOutcome::EffectsMayHaveBegun,
    );
    let left_frame = prepared(&mut left);
    let left_pending = in_flight(left.present_prepared_mounted_frame(
        left_frame,
        UiPresentationDeadline::at_tick(20),
        0,
    ));
    let right_frame = prepared(&mut right);
    let right_pending = in_flight(right.present_prepared_mounted_frame(
        right_frame,
        UiPresentationDeadline::at_tick(20),
        0,
    ));

    assert!(matches!(
        left.complete_mounted_presentation(right_pending.clone(), 1),
        UiMountedFrameOutcome::CompletionDenied(
            UiMountedPresentationCompletionDenial::UnknownAttempt
        )
    ));
    assert!(left.current_mounted_publication().is_none());
    let receipt = published(left.complete_mounted_presentation(left_pending, 1));
    assert_eq!(left.current_mounted_publication(), Some(&receipt));
}

#[test]
fn filesystem_replacement_publishes_real_candidate_lane_output_with_application() {
    let host = ScriptedPresentationHost::default();
    let (mut session, bindings) =
        mounted_session(host.clone(), "mounted-publication-replacement", 1);
    host.push_presented();
    let predecessor_frame = prepared(&mut session);
    let predecessor = published(session.present_prepared_mounted_frame(
        predecessor_frame,
        UiPresentationDeadline::at_tick(10),
        0,
    ));
    let workspace = replacement_workspace("mounted-publication-replacement");
    let (pending, catalog, boundary) = stage_replacement(&workspace, &mut session);
    let prepared = match session
        .prepare_mounted_replacement(
            pending,
            catalog,
            boundary,
            None,
            UiMountedFrameRequest::all_bound_surfaces(),
        )
        .unwrap()
    {
        WorthUiMountedReplacementPreparationOutcome::Prepared(prepared) => prepared,
        WorthUiMountedReplacementPreparationOutcome::SemanticNoOp(_) => {
            panic!("changed filesystem meaning requires activation")
        }
    };
    assert_eq!(prepared.frame().manifest().surfaces().len(), 1);
    assert!(prepared
        .frame()
        .manifest()
        .lane_contributions()
        .iter()
        .any(|cell| {
            cell.status() == worth_ui::facade::mounted::UiRequiredLaneContributionStatus::Admitted
        }));
    let candidate_projection = prepared.frame().surfaces()[0].projection();
    assert!(
        !candidate_projection.nodes().is_empty(),
        "replacement assembly must project nodes from the staged candidate plan"
    );
    assert!(
        !candidate_projection.paint_batches().rows().is_empty(),
        "replacement assembly must retain actual candidate lane output"
    );
    host.push_presented();
    let (application, mounted) = match prepared.present(UiPresentationDeadline::at_tick(20), 1) {
        WorthUiMountedApplicationReplacementOutcome::Published {
            application,
            mounted,
        } => (application, mounted),
        _ => panic!("complete replacement presentation publishes one successor tuple"),
    };

    assert_eq!(
        application.active_generation(),
        session.generation_identity()
    );
    assert_eq!(mounted.generation(), session.generation_identity());
    assert_eq!(mounted.predecessor(), Some(predecessor.frame()));
    assert_eq!(mounted.bindings(), bindings);
    assert_eq!(
        session.inspect_mounted_identity().current_frame(),
        Some(mounted.frame())
    );
    assert_eq!(
        session.inspect_mounted_identity().mounted_instances().len(),
        1,
        "the candidate frame and published successor retain the uninterrupted semantic mount"
    );
    assert_eq!(session.current_mounted_publication(), Some(&mounted));
    workspace.close();
}

#[test]
fn prepared_mounted_replacement_publication_performs_no_allocations() {
    let host = ScriptedPresentationHost::default();
    let (mut session, _) = mounted_session(host.clone(), "mounted-publication-allocation", 1);
    host.push_presented();
    let predecessor_frame = prepared(&mut session);
    let _predecessor = published(session.present_prepared_mounted_frame(
        predecessor_frame,
        UiPresentationDeadline::at_tick(10),
        0,
    ));
    let workspace = replacement_workspace("mounted-publication-allocation");
    let (pending, catalog, boundary) = stage_replacement(&workspace, &mut session);
    let replacement = match session
        .prepare_mounted_replacement(
            pending,
            catalog,
            boundary,
            None,
            UiMountedFrameRequest::all_bound_surfaces(),
        )
        .unwrap()
    {
        WorthUiMountedReplacementPreparationOutcome::Prepared(replacement) => replacement,
        WorthUiMountedReplacementPreparationOutcome::SemanticNoOp(_) => {
            panic!("changed filesystem meaning requires activation")
        }
    };
    host.push_presented();
    let mut allocations = None;
    let outcome = replacement.present_observing_publication_tail_for_certification(
        UiPresentationDeadline::at_tick(20),
        1,
        |commit| {
            allocations = Some(allocation_counter::measure(commit));
        },
    );

    let (application, mounted) = match outcome {
        WorthUiMountedApplicationReplacementOutcome::Published {
            application,
            mounted,
        } => (application, mounted),
        _ => panic!("prepared complete presentation must publish"),
    };
    assert_eq!(
        allocations
            .expect("the complete presentation reaches the observed tail")
            .count_total,
        0,
        "prepared post-presentation publication must not allocate"
    );
    assert!(application.publication().generation_is_coherent());
    assert_eq!(session.current_mounted_publication(), Some(&mounted));
    workspace.close();
}

fn published(
    outcome: UiMountedFrameOutcome,
) -> worth_ui::facade::mounted::UiMountedFramePublicationReceipt {
    match outcome {
        UiMountedFrameOutcome::Published(receipt) => receipt,
        _ => panic!("scripted complete presentation publishes"),
    }
}

fn in_flight(
    outcome: UiMountedFrameOutcome,
) -> worth_ui::facade::mounted::UiMountedPresentationInFlight {
    match outcome {
        UiMountedFrameOutcome::InFlight(handle) => handle,
        _ => panic!("scripted pending presentation remains in flight"),
    }
}

pub(super) fn replacement_workspace(label: &str) -> FilesystemContractWorkspace {
    let workspace = FilesystemContractWorkspace::new(label);
    let source = format!(
        "{}\n{}",
        FilesystemApplicationLifecycleScenario::ordinary_execution_source_text(),
        FilesystemApplicationLifecycleScenario::candidate_source_text()
    );
    workspace.write("app/main.wui", &source);
    workspace
}

pub(super) fn stage_replacement(
    workspace: &FilesystemContractWorkspace,
    session: &mut worth_ui::facade::app::WorthUiActiveApplicationSession,
) -> (
    worth_ui::facade::app::WorthUiPendingApplicationCutover,
    UiAdmittedAllocationCatalogDelta,
    WorthUiFrameBoundary,
) {
    let snapshot = WorthUiFilesystemSourceProvider::new(workspace.root())
        .read()
        .expect("replacement source settles through production filesystem acquisition");
    let submission = snapshot
        .lower_to_candidate_submission(session.capabilities())
        .expect("replacement bytes lower through production semantics");
    let mut prepared = session.prepare_replacement(submission).unwrap();
    let catalog = admit_candidate_catalog(session, &mut prepared);
    let lowered = session.lower_prepared_replacement(*prepared).unwrap();
    let pending = session.stage_prepared_replacement(lowered).unwrap();
    let boundary = session
        .execute_framework_turn(|_| {})
        .expect("no mounted presentation lease is active")
        .into_execution()
        .unwrap_or_else(|_| panic!("replacement boundary turn completes"))
        .into_activation_boundary();
    (pending, catalog, boundary)
}
