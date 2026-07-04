use worth_ui::facade::inspection::{UiEvidenceFamily, UiEvidenceIdentity};

fn main() {
    let _ = UiEvidenceIdentity::new(UiEvidenceFamily::Obligation, 7);
}
