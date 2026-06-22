use forge_query::facade::{
    ForgeQueryRetainedScalarAlignment, ForgeQueryRetainedScalarFactSet,
};

fn main() {
    let facts = fact_set_fixture();
    let alignment = alignment_fixture();

    let _ = facts.view_name();
    let _ = alignment.left_view_name();
    let _ = alignment.right_view_name();
}

fn fact_set_fixture() -> ForgeQueryRetainedScalarFactSet {
    panic!("fixture only")
}

fn alignment_fixture() -> ForgeQueryRetainedScalarAlignment {
    panic!("fixture only")
}
