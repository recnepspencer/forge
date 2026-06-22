use worth_ui::facade::{
    RuntimeOutcomeFamily, RuntimeOutcomeProjectionDescriptor, RuntimeOutcomeProjectionId,
};

fn main() {
    let _descriptor = RuntimeOutcomeProjectionDescriptor {
        id: RuntimeOutcomeProjectionId::new("workspace.outcome.denied").unwrap(),
        family: RuntimeOutcomeFamily::denied(),
        source: None,
        presentation: None,
        denial_posture: None,
        recovery_posture: None,
        local_status_enum_claim: None,
    };
}
