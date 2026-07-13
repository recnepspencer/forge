use super::{WorthQueryReadDeclaration, WorthQueryReadOutcome, WorthQueryReadStop};
use crate::runtime::WorthQueryWorkspace;

impl WorthQueryReadDeclaration {
    /// Execute this declaration through Query-owned admission, planning,
    /// routing, and receipt assembly.
    ///
    /// Consuming the declaration makes post-execution refinement or replay by
    /// accidental value reuse mechanically unavailable. A fresh declaration is
    /// required for a distinct execution request.
    pub fn run(self, workspace: &mut WorthQueryWorkspace) -> WorthQueryReadOutcome {
        match workspace.execute_declared_read_graph(self.into_read_graph()) {
            Ok(result) => WorthQueryReadOutcome::Completed(result),
            Err(error) => WorthQueryReadOutcome::Stopped(WorthQueryReadStop::new(error)),
        }
    }
}
