use forge_query::facade::runtime::{
    ForgeQueryIntentAdmissionSlopeLane, ForgeQueryIntentAdmissionWidthRunRow,
    ForgeQueryIntentAdmissionWidthRunScale,
};

fn main() {
    let _ = ForgeQueryIntentAdmissionWidthRunRow {
        lane: ForgeQueryIntentAdmissionSlopeLane::AdmissionClassification,
        scale: ForgeQueryIntentAdmissionWidthRunScale::Small,
        width: 1,
        row_digest: String::new(),
    };
}
