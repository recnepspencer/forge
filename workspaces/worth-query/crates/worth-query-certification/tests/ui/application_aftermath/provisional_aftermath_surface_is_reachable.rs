//! Positive twin: the compiled experiment remains honestly reachable through
//! the host audience facade's explicitly provisional namespace.

use worth_query_host::facade::provisional_aftermath::{
    WorthQueryProvedUndo, WorthQueryRedoAdmission, WorthQueryRedoIntent,
    WorthQueryRedoRecovery, WorthQueryUndoAdmission,
};

type ProvisionalSurface = (
    WorthQueryUndoAdmission,
    WorthQueryProvedUndo,
    WorthQueryRedoIntent,
    WorthQueryRedoRecovery,
    WorthQueryRedoAdmission,
);

fn provisional_surface(surface: ProvisionalSurface) {
    let _ = surface;
}

fn main() {}
