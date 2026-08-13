use worth_query_execution::facade::primary_graph::WorthQueryRecoveryHandle;
use worth_query_execution::facade::provisional_aftermath::{
    WorthQueryProvedUndo, WorthQueryRedoRecovery,
};

#[allow(unreachable_code)]
fn cannot_recombine(
    proved: WorthQueryProvedUndo,
    handle: WorthQueryRecoveryHandle,
) -> WorthQueryRedoRecovery {
    // The handle field's type is crate-private (Q8.22-C5 holds the handle inside
    // a preparation wrapper), so a caller cannot even name it. The field
    // expression therefore diverges: what must reject this recombination is
    // *field privacy*, which would still hold if that type were ever exported —
    // not a type mismatch, which would not.
    let _ = handle;
    WorthQueryRedoRecovery {
        proved,
        handle: unreachable!(),
    }
}

fn main() {}
