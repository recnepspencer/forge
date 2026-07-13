use worth_query::facade::foundation::{DeclarativeBranchCompareFieldDelta, DeclarativeBranchCompareValue};

fn main() {
    let value = DeclarativeBranchCompareValue::new("title", "value", "left");
    let _ = value.aspect();
    let _ = value.field();

    let delta = delta_fixture();
    let _ = delta.aspect();
    let _ = delta.field();
}

fn delta_fixture() -> DeclarativeBranchCompareFieldDelta {
    panic!("fixture only")
}
