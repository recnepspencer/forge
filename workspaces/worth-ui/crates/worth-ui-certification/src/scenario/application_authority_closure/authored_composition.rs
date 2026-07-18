use worth_ui::facade::diagnostics::CapabilitySnapshot;
use worth_ui::facade::source::WorthUiArtifactInputBodyAtom;
use worth_ui::facade::source::{
    WorthUiRustAuthoredArtifactInput, WorthUiRustAuthoredArtifactInputModule,
    WorthUiSourceProvider, WorthUiSourceWatcher, WorthUiWatchedCandidateSubmission,
    WorthUiWatcherEvent,
};

use super::application_definition::{CANDIDATE_COMPONENT, CURRENT_COMPONENT, REGION, SIZING};

pub(super) fn file_submission(
    component: &str,
    provider_id: &str,
    snapshot: &CapabilitySnapshot,
) -> WorthUiWatchedCandidateSubmission {
    lower(
        WorthUiSourceProvider::in_memory(provider_id).with_file(
            "app/main.wui",
            format!("component {component} {{ region {REGION} {{ sizing {SIZING}; }} }}"),
        ),
        provider_id,
        snapshot,
    )
}

pub(super) fn rust_submission(
    component: &str,
    provider_id: &str,
    snapshot: &CapabilitySnapshot,
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
    lower(
        WorthUiSourceProvider::rust_authored(provider_id).with_rust_authored_input(
            WorthUiRustAuthoredArtifactInput::from_modules([
                WorthUiRustAuthoredArtifactInputModule::new("app/main.wui")
                    .with_component_body_atoms(component, body),
            ]),
        ),
        provider_id,
        snapshot,
    )
}

pub(super) fn current_file(snapshot: &CapabilitySnapshot) -> WorthUiWatchedCandidateSubmission {
    file_submission(CURRENT_COMPONENT, "authority-file", snapshot)
}

pub(super) fn current_rust(snapshot: &CapabilitySnapshot) -> WorthUiWatchedCandidateSubmission {
    rust_submission(CURRENT_COMPONENT, "authority-rust", snapshot)
}

pub(super) fn candidate_file(snapshot: &CapabilitySnapshot) -> WorthUiWatchedCandidateSubmission {
    file_submission(CANDIDATE_COMPONENT, "authority-candidate", snapshot)
}

fn lower(
    provider: WorthUiSourceProvider,
    provider_id: &str,
    snapshot: &CapabilitySnapshot,
) -> WorthUiWatchedCandidateSubmission {
    let mut ingress = WorthUiSourceWatcher::new(provider).start();
    ingress
        .ingest([WorthUiWatcherEvent::provider_revision(provider_id)])
        .expect("scenario watcher input should debounce")
        .lower_to_candidate_submission(snapshot)
        .expect("scenario composition should lower through source ingress")
}

fn identifier(value: &str) -> WorthUiArtifactInputBodyAtom {
    WorthUiArtifactInputBodyAtom::Identifier(value.to_owned())
}
