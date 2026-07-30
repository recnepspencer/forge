use worth_ui::facade::query_binding::{
    UiCollectionProjectionFactReceipt, UiCollectionProjectionRegistration,
    UiCollectionSchemaRequirement, UiInstalledProjectionView, UiPresentProjection,
    UiProjectionAvailability, UiProjectionFieldRequirement, UiProjectionLifecycleRequirement,
    UiScalarProjectionFactReceipt, UiScalarProjectionRegistration, UiScalarSchemaRequirement,
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

fn register_scalar(view: UiInstalledProjectionView) {
    let registration = UiScalarProjectionRegistration::text(
        view,
        UiProjectionFieldRequirement::declared("status").expect("valid selected field"),
    );
    let _app = worth_ui::facade::app::WorthUi::app()
        .with_change_profile(
            worth_ui::facade::rebind::UiChangeProfile::platform_pulse(),
        )
        .register_scalar_projection(registration)
        .expect("installed scalar projection registration")
        .freeze()
        .expect("application preparation should succeed");
}

fn register_collection(view: UiInstalledProjectionView) {
    let registration = UiCollectionProjectionRegistration::text(
        view,
        UiProjectionFieldRequirement::declared("identity.id").expect("valid row identity"),
        [UiProjectionFieldRequirement::declared("status").expect("valid selected field")],
        true,
        false,
    )
    .expect("valid collection projection requirement");
    let _app = worth_ui::facade::app::WorthUi::app()
        .with_change_profile(
            worth_ui::facade::rebind::UiChangeProfile::platform_pulse(),
        )
        .register_collection_projection(registration)
        .expect("installed collection projection registration")
        .freeze()
        .expect("application preparation should succeed");
}

fn main() {
    let _ = (
        observe_scalar,
        observe_collection,
        declare_requirements,
        register_scalar,
        register_collection,
    );
}
