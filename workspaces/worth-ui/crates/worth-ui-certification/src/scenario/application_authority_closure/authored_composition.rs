use worth_ui::facade::diagnostics::CapabilitySnapshot;
use worth_ui::facade::source::WorthUiArtifactInputBodyAtom;
use worth_ui::facade::source::{
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
    WorthUiSourceEventIngress, WorthUiSourceProvider, WorthUiWatchedCandidateSubmission,
    WorthUiWatcherEvent,
};

use super::application_definition::{
    CANDIDATE_COMPONENT, CURRENT_COMPONENT, QUERY_BINDING, REGION, SIZING,
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
