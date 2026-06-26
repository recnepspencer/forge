use worth_ui::facade::{
    RuntimeOutcomeFamily, RuntimeOutcomeProjectionDescriptor, RuntimeOutcomeProjectionId,
};

enum LocalStatus {
    Loading,
    Success,
    Error,
}

fn main() {
    let local_status = LocalStatus::Loading;

    let _descriptor = RuntimeOutcomeProjectionDescriptor::new(
        RuntimeOutcomeProjectionId::new("workspace.outcome.local").unwrap(),
        RuntimeOutcomeFamily::loading(),
        local_status,
    );
}
