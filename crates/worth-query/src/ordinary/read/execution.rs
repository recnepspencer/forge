use super::{
    admit_read_context_declaration, WorthQueryReadCompletion, WorthQueryReadOutcome,
    WorthQueryReadRequest, WorthQueryReadStop,
};
use crate::runtime::WorthQueryWorkspace;

impl WorthQueryReadRequest {
    /// Admit the declared authority context and execute the read through
    /// Query-owned planning, routing, and receipt assembly.
    pub fn run(self, workspace: &mut WorthQueryWorkspace) -> WorthQueryReadOutcome {
        let (declaration, context) = self.into_parts();
        let read_graph = declaration.into_read_graph();
        let admitted_context = match admit_read_context_declaration(&read_graph, context) {
            Ok(context) => context,
            Err(denial) => {
                return WorthQueryReadOutcome::Stopped(WorthQueryReadStop::context(denial));
            }
        };
        let runtime_result = workspace
            .execute_declared_read_graph_in_authority(read_graph, admitted_context.authority());
        let context_receipt = admitted_context.into_receipt();
        match runtime_result {
            Ok(result) => WorthQueryReadOutcome::Completed(WorthQueryReadCompletion::new(
                result,
                context_receipt,
            )),
            Err(error) => {
                WorthQueryReadOutcome::Stopped(WorthQueryReadStop::runtime(error, context_receipt))
            }
        }
    }
}
