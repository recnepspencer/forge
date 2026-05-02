use crate::data::async_node::{
    AsyncCapableNode, AsyncNodeCapabilityAliasLoweringProof, AsyncNodeCapabilityDeclaration,
    FrozenAsyncNodeCapabilityDescriptor, LoweredAsyncNodeCapabilityBundle,
    ValidatedAsyncNodeCapabilityDeclaration,
};
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::resource::{ResourceNodeId, ResourcePayloadContractDigest};

use super::SignalRuntime;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn validate_async_node_capability_declaration(
        &mut self,
        declaration: &AsyncNodeCapabilityDeclaration,
    ) -> Result<ValidatedAsyncNodeCapabilityDeclaration, SignalError> {
        self.ensure_live_async_node_owner(declaration.node(), "validate async node capability")?;
        let validated = self.resource.validate_async_capability_declaration(
            declaration.as_resource_declaration(),
            &mut self.telemetry.resource,
        )?;
        Ok(ValidatedAsyncNodeCapabilityDeclaration::new(
            declaration.clone(),
            validated,
        ))
    }

    pub fn freeze_async_node_capability_descriptor(
        &mut self,
        validated: &ValidatedAsyncNodeCapabilityDeclaration,
    ) -> Result<FrozenAsyncNodeCapabilityDescriptor, SignalError> {
        let frozen = self.resource.freeze_async_capability_declaration(
            validated.validated(),
            &mut self.telemetry.resource,
        )?;
        let declaration = validated.declaration();
        let payload_contract = declaration.payload_contract();
        Ok(FrozenAsyncNodeCapabilityDescriptor::new(
            declaration.node(),
            ResourcePayloadContractDigest::from_contract(
                payload_contract.id().into_resource(),
                payload_contract.max_payload_bytes(),
            ),
            frozen,
        ))
    }

    pub fn lower_async_node_capability_bundle(
        &mut self,
        frozen: &FrozenAsyncNodeCapabilityDescriptor,
    ) -> LoweredAsyncNodeCapabilityBundle {
        let lowered = self
            .resource
            .lower_async_capability_bundle(frozen.frozen(), &mut self.telemetry.resource);
        LoweredAsyncNodeCapabilityBundle::new(
            frozen.node(),
            frozen.payload_contract_digest().clone(),
            lowered,
        )
    }

    pub fn async_node_capability_bundle_for_node(
        &mut self,
        node: NodeId,
    ) -> Option<LoweredAsyncNodeCapabilityBundle> {
        self.resource
            .descriptor_for_node(ResourceNodeId::from_node(node))
            .map(|descriptor| {
                LoweredAsyncNodeCapabilityBundle::new(
                    node,
                    descriptor.payload_contract_digest().clone(),
                    descriptor.lowered_policy_bundle().clone(),
                )
            })
    }

    pub fn prove_async_node_capability_alias_lowering(
        &mut self,
        declaration: &AsyncNodeCapabilityDeclaration,
    ) -> Result<AsyncNodeCapabilityAliasLoweringProof, SignalError> {
        let validated = self.validate_async_node_capability_declaration(declaration)?;
        let frozen = self.freeze_async_node_capability_descriptor(&validated)?;
        let lowered = self.lower_async_node_capability_bundle(&frozen);

        let legacy = declaration.clone().into_legacy_resource_declaration();
        let legacy_validated = self
            .resource
            .validate_resource_policy_declaration_without_async_accounting(
                &legacy,
                &mut self.telemetry.resource,
            )?;
        let legacy_frozen = self
            .resource
            .freeze_resource_policy_declaration_without_async_accounting(
                &legacy_validated,
                &mut self.telemetry.resource,
            )?;
        let legacy_lowered = self
            .resource
            .lower_resource_policy_bundle_without_async_accounting(&legacy_frozen);
        let legacy_payload_digest = ResourcePayloadContractDigest::from_contract(
            legacy.payload_contract().id(),
            legacy.payload_contract().max_payload_bytes(),
        );
        if lowered.registry_digest() != legacy_frozen.registry_digest()
            || lowered.bundle_digest() != legacy_lowered.bundle_digest()
            || lowered.payload_contract_digest() != &legacy_payload_digest
        {
            return Err(SignalError::invalid_input(format!(
                "async capability declaration for node {} did not lower identically through legacy resource compatibility",
                declaration.node()
            )));
        }
        self.telemetry
            .resource
            .async_node_capability_alias_lowering_count += 1;
        Ok(AsyncNodeCapabilityAliasLoweringProof::new(
            declaration.node(),
            lowered.registry_digest().clone(),
            legacy_frozen.registry_digest().clone(),
            lowered.bundle_digest().clone(),
            legacy_lowered.bundle_digest().clone(),
            lowered.payload_contract_digest().clone(),
            legacy_payload_digest,
            3,
        ))
    }

    pub fn declare_async_node_capability(
        &mut self,
        declaration: AsyncNodeCapabilityDeclaration,
    ) -> Result<crate::data::resource::ResourceDeclarationReport, SignalError> {
        self.ensure_live_async_node_owner(declaration.node(), "declare async node capability")?;
        self.telemetry
            .resource
            .async_node_capability_attachment_count += 1;
        self.resource.declare_resource_node(
            declaration.into_legacy_resource_declaration(),
            &mut self.telemetry.resource,
        )
    }

    pub fn attach_async_capability(
        &mut self,
        declaration: AsyncNodeCapabilityDeclaration,
    ) -> Result<AsyncCapableNode, SignalError> {
        let node = declaration.node();
        self.declare_async_node_capability(declaration)?;
        self.async_capable_node(node).ok_or_else(|| {
            SignalError::invalid_input(format!(
                "async capability attachment for node {node} did not leave an attachable capability handle"
            ))
        })
    }

    pub fn async_capable_node(&mut self, node: NodeId) -> Option<AsyncCapableNode> {
        self.async_node_capability_bundle_for_node(node)
            .map(|bundle| {
                AsyncCapableNode::new(
                    node,
                    bundle.registry_digest().clone(),
                    bundle.bundle_digest().clone(),
                    bundle.payload_contract_digest().clone(),
                )
            })
    }
}
