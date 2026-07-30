use worth_ui::facade::query_binding::{
    UiCollectionProjectionFactReceipt, UiCollectionSchemaRequirement, UiPresentProjection,
    UiProjectionAvailability, UiProjectionFieldRequirement, UiProjectionLifecycleRequirement,
    UiScalarProjectionFactReceipt, UiScalarSchemaRequirement,
};

fn observe_scalar(receipt: &UiScalarProjectionFactReceipt) {
    match receipt.availability() {
        UiProjectionAvailability::Unavailable(_) | UiProjectionAvailability::Stopped(_) => {}
        UiProjectionAvailability::Present(UiPresentProjection::Current(value))
        | UiProjectionAvailability::Present(UiPresentProjection::RetainedStale {
            value, ..
        }) => {
            let _ = value.as_str();
        }
    }
}

fn observe_collection(receipt: &UiCollectionProjectionFactReceipt) {
    if let UiProjectionAvailability::Present(present) = receipt.availability() {
        let value = match present {
            UiPresentProjection::Current(value)
            | UiPresentProjection::RetainedStale { value, .. } => value,
        };
        let _ = (value.completeness(), value.continuation());
    }
}

fn declare_requirements() {
    let status = UiProjectionFieldRequirement::declared("status").unwrap();
    let scalar =
        UiScalarSchemaRequirement::text(status.clone(), UiProjectionLifecycleRequirement::Live);
    let collection = UiCollectionSchemaRequirement::text(
        UiProjectionFieldRequirement::declared("identity").unwrap(),
        [status],
        UiProjectionLifecycleRequirement::Live,
        true,
        true,
    )
    .expect("collection requirement is canonical");
    let _ = (scalar, collection);
}

fn main() {
    let _ = (observe_scalar, observe_collection, declare_requirements);
}
