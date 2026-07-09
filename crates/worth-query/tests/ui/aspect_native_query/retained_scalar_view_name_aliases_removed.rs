use worth_query::facade::{
    WorthQueryRetainedScalarAlignment, WorthQueryRetainedScalarFactSet,
};

fn main() {
    let facts = fact_set_fixture();
    let alignment = alignment_fixture();

    let _ = facts.view_name();
    let _ = alignment.left_view_name();
    let _ = alignment.right_view_name();
}

fn fact_set_fixture() -> WorthQueryRetainedScalarFactSet {
    panic!("fixture only")
}

fn alignment_fixture() -> WorthQueryRetainedScalarAlignment {
    panic!("fixture only")
}
