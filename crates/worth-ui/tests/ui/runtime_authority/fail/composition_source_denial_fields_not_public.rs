use worth_ui::facade::{
    WorthUiCompositionSourceAdmissionDenial, WorthUiCompositionSourceDenialCode,
};

fn main() {
    let _forged = WorthUiCompositionSourceAdmissionDenial {
        code: WorthUiCompositionSourceDenialCode::StaleControlReference,
        subject: "live_view.control.fake".to_owned(),
        message: "fake",
        expected_syntax: "fake",
        source_span: None,
        denial_digest: 1,
    };
}
