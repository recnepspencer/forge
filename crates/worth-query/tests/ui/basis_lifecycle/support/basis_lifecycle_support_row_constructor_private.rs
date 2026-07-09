use worth_query::facade::{
    BasisFamily, BasisLifecycleSupportRow, BasisSupportPosture,
};

fn main() {
    let _ = BasisLifecycleSupportRow {
        family: BasisFamily::CurrentHead,
        operation_lane: "observation",
        posture: BasisSupportPosture::Admitted,
        row_digest: String::new(),
    };
}
