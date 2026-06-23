use worth_ui::facade::{WorthUiSourceAuthoredCandidateSubmission, WorthUiValidationReloadRequest};

fn needs_source_authored_submission(_submission: WorthUiSourceAuthoredCandidateSubmission) {}

fn main() {
    let request = WorthUiValidationReloadRequest::from_source_module(
        "app/main.wui",
        "app Demo { theme DemoTheme workspace DemoWorkspace }",
    );

    needs_source_authored_submission(request);
}
