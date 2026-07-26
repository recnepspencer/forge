use worth_ui::facade::diagnostics::CapabilitySnapshot;
use worth_ui::facade::source::{
    WorthUiSourceEventIngress, WorthUiSourceProvider, WorthUiWatchedCandidateSubmission,
    WorthUiWatcherEvent,
};
use worth_ui_dsl::{
    WorthUiArtifactInputBodyAtom, WorthUiRustAuthoredArtifactInput,
    WorthUiRustAuthoredArtifactInputModule,
};

use super::application_definition::{
    CANDIDATE_COMPONENT, CROSS_LANE_CANVAS, CROSS_LANE_REALTIME, CURRENT_COMPONENT,
    PREVIEW_COMPONENT, PREVIEW_REGION, PREVIEW_SIZING, PREVIEW_STATE_SLOT, PREVIEW_SURFACE,
    QUERY_BINDING, REGION, SIZING, STATE_SLOT, SURFACE, TOKEN,
};

pub(super) fn file_submission(
    component: &str,
    provider_id: &str,
    snapshot: &CapabilitySnapshot,
) -> WorthUiWatchedCandidateSubmission {
    lower(
        WorthUiSourceProvider::in_memory(provider_id)
            .with_file("app/main.wui", query_file_source(component)),
        provider_id,
        snapshot,
    )
}

pub(crate) fn rust_submission(
    component: &str,
    provider_id: &str,
    snapshot: &CapabilitySnapshot,
) -> WorthUiWatchedCandidateSubmission {
    rust_submission_with_query_binding(component, provider_id, snapshot, false)
}

pub(crate) fn query_rust_submission(
    component: &str,
    provider_id: &str,
    snapshot: &CapabilitySnapshot,
) -> WorthUiWatchedCandidateSubmission {
    rust_submission_with_query_binding(component, provider_id, snapshot, true)
}

pub(crate) fn preview_cross_lane_rust_submission(
    snapshot: &CapabilitySnapshot,
) -> WorthUiWatchedCandidateSubmission {
    let ordinary_body = vec![
        identifier("region"),
        identifier(REGION),
        WorthUiArtifactInputBodyAtom::LeftBrace,
        identifier("sizing"),
        identifier(SIZING),
        WorthUiArtifactInputBodyAtom::Semicolon,
        identifier("state"),
        identifier(STATE_SLOT),
        WorthUiArtifactInputBodyAtom::Semicolon,
        WorthUiArtifactInputBodyAtom::RightBrace,
    ];
    let preview_body = vec![
        identifier("region"),
        identifier(PREVIEW_REGION),
        WorthUiArtifactInputBodyAtom::LeftBrace,
        identifier("sizing"),
        identifier(PREVIEW_SIZING),
        WorthUiArtifactInputBodyAtom::Semicolon,
        identifier("state"),
        identifier(PREVIEW_STATE_SLOT),
        WorthUiArtifactInputBodyAtom::Semicolon,
        WorthUiArtifactInputBodyAtom::RightBrace,
    ];
    let module = WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
        .with_component_body_atoms(CURRENT_COMPONENT, ordinary_body)
        .with_surface(SURFACE)
        .with_token(TOKEN, "theme.text.authority_primary")
        .with_component(CROSS_LANE_CANVAS)
        .with_component(CROSS_LANE_REALTIME)
        .with_binding(QUERY_BINDING)
        .with_component(PREVIEW_COMPONENT)
        .with_surface_body_atoms(PREVIEW_SURFACE, preview_body);
    lower(
        WorthUiSourceProvider::rust_authored("phase7-equivalent-cross-lane-rust")
            .with_rust_authored_input(WorthUiRustAuthoredArtifactInput::from_modules([module])),
        "phase7-equivalent-cross-lane-rust",
        snapshot,
    )
}

fn rust_submission_with_query_binding(
    component: &str,
    provider_id: &str,
    snapshot: &CapabilitySnapshot,
    include_query_binding: bool,
) -> WorthUiWatchedCandidateSubmission {
    let body = vec![
        identifier("region"),
        identifier(REGION),
        WorthUiArtifactInputBodyAtom::LeftBrace,
        identifier("sizing"),
        identifier(SIZING),
        WorthUiArtifactInputBodyAtom::Semicolon,
        WorthUiArtifactInputBodyAtom::RightBrace,
    ];
    let mut module = WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
        .with_component_body_atoms(component, body);
    if include_query_binding {
        module = module.with_binding(QUERY_BINDING);
    }
    lower(
        WorthUiSourceProvider::rust_authored(provider_id)
            .with_rust_authored_input(WorthUiRustAuthoredArtifactInput::from_modules([module])),
        provider_id,
        snapshot,
    )
}

pub(crate) fn file_source(component: &str) -> String {
    format!("component {component} {{ region {REGION} {{ sizing {SIZING}; }} }}")
}

fn query_file_source(component: &str) -> String {
    format!("{}\nbinding {QUERY_BINDING} {{}}", file_source(component))
}

pub(super) fn current_file(snapshot: &CapabilitySnapshot) -> WorthUiWatchedCandidateSubmission {
    file_submission(CURRENT_COMPONENT, "authority-file", snapshot)
}

pub(super) fn current_rust(snapshot: &CapabilitySnapshot) -> WorthUiWatchedCandidateSubmission {
    query_rust_submission(CURRENT_COMPONENT, "authority-rust", snapshot)
}

pub(super) fn candidate_file(snapshot: &CapabilitySnapshot) -> WorthUiWatchedCandidateSubmission {
    file_submission(CANDIDATE_COMPONENT, "authority-candidate", snapshot)
}

fn lower(
    provider: WorthUiSourceProvider,
    provider_id: &str,
    snapshot: &CapabilitySnapshot,
) -> WorthUiWatchedCandidateSubmission {
    let mut ingress = WorthUiSourceEventIngress::new(provider).start();
    ingress
        .ingest([WorthUiWatcherEvent::provider_revision(provider_id)])
        .expect("scenario watcher input should debounce")
        .lower_to_candidate_submission(snapshot)
        .expect("scenario composition should lower through source ingress")
}

fn identifier(value: &str) -> WorthUiArtifactInputBodyAtom {
    WorthUiArtifactInputBodyAtom::Identifier(value.to_owned())
}
