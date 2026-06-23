use crate::facade::{WorthUi, WorthUiApp, WorthUiRuntimeSourceModule};
use crate::runtime::{
    WorthUiObservedAuthoredEdit, WorthUiRuntimeHost, WorthUiValidationReloadRequest,
};

pub(crate) fn validation_reload_request_for_modules(
    modules: Vec<(String, String)>,
) -> WorthUiValidationReloadRequest {
    WorthUiValidationReloadRequest::from_source_modules(modules)
}

pub(crate) fn observed_authored_edit_for_modules(
    modules: Vec<(String, String)>,
) -> WorthUiObservedAuthoredEdit {
    let provider = modules.into_iter().fold(
        crate::runtime::WorthUiSourceProvider::in_memory("validation-app-reload"),
        |provider, (relative_path, source_text)| provider.with_file(relative_path, source_text),
    );
    WorthUiObservedAuthoredEdit::from_source_provider(provider)
        .expect("validation-app source package should lower into a real observed edit")
}

pub(crate) fn runtime_for_modules(
    app: &WorthUiApp,
    modules: Vec<(String, String)>,
) -> WorthUiRuntimeHost {
    let prepared = WorthUi::runtime_launch()
        .from_source_modules(modules.into_iter().map(|(relative_path, source_text)| {
            WorthUiRuntimeSourceModule::new(relative_path, source_text)
        }))
        .prepare_authoring_for(app)
        .expect("source-authored runtime prepares");
    app.launch_runtime(prepared.into_runtime_launch())
        .expect("runtime launches")
}

pub(crate) fn packaged_source_modules(collection_surface: &str) -> Vec<(String, String)> {
    vec![
        (
            "app/main.wui".to_owned(),
            r#"import "app/products_page.wui";"#.to_owned(),
        ),
        (
            "app/products_page.wui".to_owned(),
            super::source_ingress_authored_delta_test_support::source_text(collection_surface),
        ),
    ]
}
