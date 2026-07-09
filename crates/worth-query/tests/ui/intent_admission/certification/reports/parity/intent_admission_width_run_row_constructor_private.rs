use worth_query::facade::runtime::{
    WorthQueryIntentAdmissionSlopeLane, WorthQueryIntentAdmissionWidthRunRow,
    WorthQueryIntentAdmissionWidthRunScale,
};

fn main() {
    let _ = WorthQueryIntentAdmissionWidthRunRow {
        lane: WorthQueryIntentAdmissionSlopeLane::AdmissionClassification,
        scale: WorthQueryIntentAdmissionWidthRunScale::Small,
        width: 1,
        row_digest: String::new(),
    };
}
