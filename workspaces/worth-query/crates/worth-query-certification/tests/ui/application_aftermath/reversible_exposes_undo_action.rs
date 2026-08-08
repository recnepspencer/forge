//! Positive twin for irreversible_has_no_undo_method: reversible exposes undo.

use worth_query_host::facade::domain::ReversibleNextActionContract;

fn exploit(actions: ReversibleNextActionContract) {
    let _ = actions.undo_via_recorded_inverse();
}

fn main() {}
