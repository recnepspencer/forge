use worth_ui::facade::app::WorthUi;
use worth_ui::facade::query_binding::{
    UiCollectionProjectionRegistration, UiInstalledProjectionView,
    UiProjectionFieldRequirement, UiScalarProjectionRegistration,
};

fn register_scalar(view: UiInstalledProjectionView) {
    let registration = UiScalarProjectionRegistration::text(
        view,
        UiProjectionFieldRequirement::declared("status").expect("valid selected field"),
    );
    let _app = WorthUi::app()
        .with_change_profile(
            worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse(),
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
    let _app = WorthUi::app()
        .with_change_profile(
            worth_ui_runtime::facade::rebind::UiChangeProfile::platform_pulse(),
        )
        .register_collection_projection(registration)
        .expect("installed collection projection registration")
        .freeze()
        .expect("application preparation should succeed");
}

fn main() {}
