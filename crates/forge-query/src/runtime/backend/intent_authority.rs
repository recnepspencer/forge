use forge_relational::facade::runtime::RelationalRuntime;
use forge_runtime_bridge::facade::RuntimeBridge;

use crate::memory_workspace::ForgeQueryWorkspaceError;
use crate::runtime::{ForgeQueryIntentDeclaration, ForgeQueryIntentExecution};

pub trait ForgeQueryIntentAuthorityAdapter {
    fn execute_intent(
        &mut self,
        bridge: &RuntimeBridge,
        relational_runtime: Option<&mut RelationalRuntime>,
        declaration: &ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryIntentExecution, ForgeQueryWorkspaceError>;
}
