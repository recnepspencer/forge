use crate::data::resource::{ResourceDeclarationReport, ResourceNodeDeclaration};

use super::super::SignalRuntime;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn declare_resource_node(
        &mut self,
        declaration: ResourceNodeDeclaration,
    ) -> Result<ResourceDeclarationReport, crate::data::error::SignalError> {
        if !self.graph.is_alive(declaration.node().node()) {
            self.telemetry.resource.resource_non_live_owner_denial_count += 1;
            return Err(crate::data::error::SignalError::invalid_input(format!(
                "cannot declare resource node for non-live owner {}",
                declaration.node().node()
            )));
        }

        self.resource
            .declare_resource_node(declaration, &mut self.telemetry.resource)
    }
}
