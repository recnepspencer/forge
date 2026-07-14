use crate::data::async_node::{
    AsyncCapableNode, AsyncKeyedNodeCapabilityBinding, AsyncNodeCapabilityDeclaration,
    AsyncNodePayloadContract, AsyncNodeRequestIntent, AsyncNodeRevalidationIntent,
};
use crate::data::error::SignalError;

use super::{DefinedKeyedComputation, SignalRuntime};

impl<'a, T, F> DefinedKeyedComputation<'a, T, F>
where
    T: Copy + Ord,
{
    pub fn async_capability_declaration<D, I, E, Ctx>(
        &self,
        runtime: &mut SignalRuntime<D, I, E, Ctx, T>,
        payload_contract: AsyncNodePayloadContract,
    ) -> AsyncNodeCapabilityDeclaration
    where
        D: Copy + Ord + std::fmt::Debug + 'static,
        I: Copy + Ord,
    {
        AsyncNodeCapabilityDeclaration::new(self.node(runtime), payload_contract)
    }

    pub fn declare_async_capability<D, I, E, Ctx>(
        &self,
        runtime: &mut SignalRuntime<D, I, E, Ctx, T>,
        payload_contract: AsyncNodePayloadContract,
    ) -> Result<AsyncKeyedNodeCapabilityBinding, SignalError>
    where
        D: Copy + Ord + std::fmt::Debug + 'static,
        I: Copy + Ord,
    {
        let node = self.node(runtime);
        let declaration = AsyncNodeCapabilityDeclaration::new(node, payload_contract);
        runtime.declare_async_node_capability(declaration)?;
        let bundle = runtime
            .async_node_capability_bundle_for_node(node)
            .expect("declared async capability should lower bundle for keyed node");
        runtime.telemetry.resource.async_node_family_binding_count += 1;
        Ok(AsyncKeyedNodeCapabilityBinding::new(
            self.family().clone(),
            self.key().clone(),
            node,
            bundle.registry_digest().clone(),
            bundle.bundle_digest().clone(),
            bundle.payload_contract_digest().clone(),
        ))
    }

    pub fn attach_async_capability<D, I, E, Ctx>(
        &self,
        runtime: &mut SignalRuntime<D, I, E, Ctx, T>,
        payload_contract: AsyncNodePayloadContract,
    ) -> Result<AsyncCapableNode, SignalError>
    where
        D: Copy + Ord + std::fmt::Debug + 'static,
        I: Copy + Ord,
    {
        let node = self.node(runtime);
        runtime.attach_async_capability(AsyncNodeCapabilityDeclaration::new(node, payload_contract))
    }

    pub fn async_capable_node<D, I, E, Ctx>(
        &self,
        runtime: &mut SignalRuntime<D, I, E, Ctx, T>,
    ) -> Option<AsyncCapableNode>
    where
        D: Copy + Ord + std::fmt::Debug + 'static,
        I: Copy + Ord,
    {
        let node = self.node(runtime);
        runtime.async_capable_node(node)
    }

    pub fn async_request_intent<D, I, E, Ctx>(
        &self,
        runtime: &mut SignalRuntime<D, I, E, Ctx, T>,
    ) -> AsyncNodeRequestIntent
    where
        D: Copy + Ord + std::fmt::Debug + 'static,
        I: Copy + Ord,
    {
        AsyncNodeRequestIntent::new(self.node(runtime))
    }

    pub fn async_revalidation_intent<D, I, E, Ctx>(
        &self,
        runtime: &mut SignalRuntime<D, I, E, Ctx, T>,
    ) -> AsyncNodeRevalidationIntent
    where
        D: Copy + Ord + std::fmt::Debug + 'static,
        I: Copy + Ord,
    {
        AsyncNodeRevalidationIntent::new(self.node(runtime))
    }
}
