use super::{WorthQueryMutationContext, WorthQueryMutationDeclaration};

pub struct WorthQueryMutationRequest {
    pub(crate) declaration: WorthQueryMutationDeclaration,
    pub(crate) context: WorthQueryMutationContext,
}

impl WorthQueryMutationDeclaration {
    pub fn using(self, context: WorthQueryMutationContext) -> WorthQueryMutationRequest {
        WorthQueryMutationRequest {
            declaration: self,
            context,
        }
    }
}
