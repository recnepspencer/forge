//! Irreversible next-action contracts expose no undo method (R8.21).

use worth_query_host::facade::domain::IrreversibleNextActionContract;

fn exploit(actions: IrreversibleNextActionContract) {
    let _ = actions.undo_via_recorded_inverse();
}

fn main() {}
