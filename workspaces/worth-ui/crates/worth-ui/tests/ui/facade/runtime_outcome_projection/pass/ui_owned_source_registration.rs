use worth_ui::facade::app::WorthUi;
use worth_ui::facade::registry::{
    RuntimeOutcomeFamily, RuntimeOutcomeProjectionDescriptor, RuntimeOutcomeProjectionId,
    RuntimeOutcomeSourceReference,
};

fn main() {
    let family = RuntimeOutcomeFamily::denied();
    let _app = WorthUi::app()
        .register_runtime_outcome_projection(RuntimeOutcomeProjectionDescriptor::new(
            RuntimeOutcomeProjectionId::new("workspace.outcome.denied").unwrap(),
            family.clone(),
            RuntimeOutcomeSourceReference::new(family),
        ))
        .freeze();
}
