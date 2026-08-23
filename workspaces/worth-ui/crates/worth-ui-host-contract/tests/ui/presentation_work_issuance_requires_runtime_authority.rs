use worth_ui_host_contract::{
    UiMountedPresentationLeaseGate, UiMountedPresentationRuntimeAuthority,
    UiMountedPresentationWork,
};

fn issue_from_retired_host_authority() {
    let _ = (
        UiMountedPresentationLeaseGate::default(),
        UiMountedPresentationRuntimeAuthority::for_runtime(),
        None::<UiMountedPresentationWork>,
    );
}

fn reach_private_runtime_authority() {
    let _ = worth_ui_runtime::mounting::presentation::UiMountedPresentationLeaseGate::default();
}

fn main() {
    let _ = (
        issue_from_retired_host_authority,
        reach_private_runtime_authority,
    );
}
