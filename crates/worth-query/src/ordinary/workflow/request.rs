use super::{WorthQueryWorkflowContext, WorthQueryWorkflowDeclaration};

pub struct WorthQueryWorkflowRequest {
    pub(crate) declaration: WorthQueryWorkflowDeclaration,
    pub(crate) context: WorthQueryWorkflowContext,
}

impl WorthQueryWorkflowDeclaration {
    pub fn using(self, context: WorthQueryWorkflowContext) -> WorthQueryWorkflowRequest {
        WorthQueryWorkflowRequest {
            declaration: self,
            context,
        }
    }
}
