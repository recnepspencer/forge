use worth_query::facade::{
    AuthorizedProjectionArtifact, MaskedProjectionArtifact, PolicyAwareDeliveryShape,
    PolicyAwareOptimizerInput, ProjectionConsumptionBindingContext,
};

fn main() {
    let authorized = authorized_fixture();
    let _ = authorized.visible_fields();
    let _ = authorized.masked_projection().masked_fields();
    let _ = authorized.masked_projection().non_disclosing_fields();

    let delivery = delivery_fixture();
    let _ = delivery.delivered_fields();

    let optimizer = optimizer_fixture();
    let _ = optimizer.visible_fields();

    let binding = binding_fixture();
    let _ = binding.authorized_visible_fields();
}

fn authorized_fixture() -> AuthorizedProjectionArtifact {
    panic!("fixture only")
}

fn delivery_fixture() -> PolicyAwareDeliveryShape {
    panic!("fixture only")
}

fn optimizer_fixture() -> PolicyAwareOptimizerInput {
    panic!("fixture only")
}

fn binding_fixture() -> ProjectionConsumptionBindingContext {
    panic!("fixture only")
}

fn _masked_fixture() -> MaskedProjectionArtifact {
    panic!("fixture only")
}
