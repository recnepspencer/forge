use worth_ui::facade::WorthUiCompositionSourceAdmissionReport;

fn main() {
    let _forged = WorthUiCompositionSourceAdmissionReport {
        denials: Vec::new(),
        counters: Default::default(),
        denial_set_digest: 1,
    };
}
