use worth_ui::facade::mounted::{
    UiMountedPresentationAttemptIdentity, UiMountedPresentationWitness,
};
use worth_ui::facade::host::{
    UiHostAdapterSessionAuthority, WorthUiOperationalHostAdapter,
};
use worth_ui_host_contract::{
    UiHostPresentationCompletionToken, UiMountedFrameConsumptionView,
};

fn mint_removed_runtime_authority() {
    let _ = worth_ui_host_contract::UiMountedPresentationAuthority::mint_for_runtime();
}

fn construct_consumption_view() {
    let _ = UiMountedFrameConsumptionView {};
}

fn clone_completion_token(token: UiHostPresentationCompletionToken) {
    let _ = token.clone();
}

fn forge_publication_witness(attempt: UiMountedPresentationAttemptIdentity) {
    let _ = UiMountedPresentationWitness::new(attempt);
}

fn invoke_host_effect_without_runtime_session(
    adapter: &dyn WorthUiOperationalHostAdapter,
    request: worth_ui_host_contract::UiHostSurfaceRegistrationRequest,
) {
    let forged = UiHostAdapterSessionAuthority::activate(request.host_session_identity());
    let _ = adapter.register_surface(&forged, request);
}

fn main() {
    let _ = (
        mint_removed_runtime_authority,
        construct_consumption_view,
        clone_completion_token,
        forge_publication_witness,
        invoke_host_effect_without_runtime_session,
    );
}
