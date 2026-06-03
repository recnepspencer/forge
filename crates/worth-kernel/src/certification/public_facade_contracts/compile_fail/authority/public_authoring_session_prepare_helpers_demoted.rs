use worth_kernel::facade::authoring::construction::{
    PrimitiveConstructionAuthoringSession, PrimitiveConstructionIntent, WireBodySpec,
};

fn call(session: &mut PrimitiveConstructionAuthoringSession<'_>) {
    let _ = session.prepare_result(PrimitiveConstructionIntent::wire_body(WireBodySpec {
        edge_count: 8,
    }));
}

fn main() {}
