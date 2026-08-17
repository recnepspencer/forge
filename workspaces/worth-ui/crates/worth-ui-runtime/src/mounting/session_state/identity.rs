use super::WorthUiMountedSessionState;
use crate::mounting::{
    UiHostSurfacePresentationMode, UiMountedFrameIdentity, UiMountedGraphNodeHandle,
    UiMountedIdentityDenial, UiMountedIdentityView, UiMountedInstanceIdentity,
    UiMountedNodeReceiptIdentity, UiMountedProjectionAudience, UiSemanticSurfaceIdentity,
    UiSurfaceBindingGeneration, UiSurfaceBindingIdentityView, UiSurfaceBindingProfile,
};

impl WorthUiMountedSessionState {
    pub(crate) fn create_semantic_surface(
        &mut self,
    ) -> Result<UiSemanticSurfaceIdentity, UiMountedIdentityDenial> {
        self.ensure_identity_mutation_available()?;
        self.identity.create_semantic_surface()
    }

    pub(crate) fn create_semantic_surface_for(
        &mut self,
        audience: UiMountedProjectionAudience,
    ) -> Result<UiSemanticSurfaceIdentity, UiMountedIdentityDenial> {
        self.ensure_identity_mutation_available()?;
        self.identity.create_semantic_surface_for(audience)
    }

    pub(crate) fn graph_node_handle(
        &self,
        graph: crate::graph::UiGraphAuthority<'_>,
        graph_node_identity: crate::graph::UiGraphNodeIdentity,
    ) -> Result<UiMountedGraphNodeHandle, UiMountedIdentityDenial> {
        self.identity.graph_node_handle(graph, graph_node_identity)
    }

    pub(crate) fn register_host_surface(
        &mut self,
        host: &crate::facade::WorthUiHostSessionAuthority,
        semantic_surface: UiSemanticSurfaceIdentity,
        mode: UiHostSurfacePresentationMode,
        profile: UiSurfaceBindingProfile,
    ) -> Result<UiSurfaceBindingIdentityView, UiMountedIdentityDenial> {
        self.ensure_identity_mutation_available()?;
        let candidate = self.identity.prepare_surface_registration(
            host.protocol(),
            host.capability_report(),
            semantic_surface,
            mode,
            profile,
        )?;
        let baseline = self
            .presentation
            .host_truth_mut()
            .register_surface(host.effect_port(), candidate.request())?;
        Ok(self
            .identity
            .commit_surface_registration(candidate, baseline))
    }

    pub(crate) fn deregister_host_surface(
        &mut self,
        host: &crate::facade::WorthUiHostSessionAuthority,
        binding: UiSurfaceBindingGeneration,
    ) -> Result<UiSemanticSurfaceIdentity, UiMountedIdentityDenial> {
        self.ensure_identity_mutation_available()?;
        let requires_reconciliation = self.presentation.binding_requires_reconciliation(binding);
        let required_by_current = self.identity.current_requires_binding(binding);
        let has_published_predecessor = self.identity.publication_receipt().is_some();
        let preserve_published_frame =
            has_published_predecessor && (requires_reconciliation || !required_by_current);
        let candidate = self
            .identity
            .prepare_surface_deregistration(binding, preserve_published_frame)?;
        let text_pin_candidate = self.presentation.prepare_text_pin_deregistration(binding);
        self.presentation
            .host_truth_mut()
            .deregister_surface(host.effect_port(), candidate.request())?;
        self.presentation
            .commit_text_pin_deregistration(text_pin_candidate);
        let semantic_surface = self.identity.commit_surface_deregistration(candidate);
        if has_published_predecessor && requires_reconciliation && !required_by_current {
            self.presentation
                .reconcile_candidate_only_deregistration(binding);
        }
        Ok(semantic_surface)
    }

    pub(crate) fn recover_indeterminate_host_surface(
        &mut self,
        host: &crate::facade::WorthUiHostSessionAuthority,
        semantic_surface: UiSemanticSurfaceIdentity,
    ) -> Result<(), UiMountedIdentityDenial> {
        self.ensure_identity_mutation_available()?;
        self.presentation
            .host_truth_mut()
            .recover_surface_effect(host.effect_port(), semantic_surface)
    }

    pub(crate) fn mount_instance(
        &mut self,
        graph: crate::graph::UiGraphAuthority<'_>,
        node: UiMountedGraphNodeHandle,
        surface: UiSemanticSurfaceIdentity,
    ) -> Result<UiMountedInstanceIdentity, UiMountedIdentityDenial> {
        self.ensure_identity_mutation_available()?;
        self.identity.mount(graph, node, surface)
    }

    pub(crate) fn unmount_instance(
        &mut self,
        identity: UiMountedInstanceIdentity,
    ) -> Result<(), UiMountedIdentityDenial> {
        self.ensure_identity_mutation_available()?;
        self.identity.unmount(identity)
    }

    pub(crate) fn reorder_mounted_instances(
        &mut self,
        order: &[UiMountedInstanceIdentity],
    ) -> Result<(), UiMountedIdentityDenial> {
        self.ensure_identity_mutation_available()?;
        self.identity.reorder(order)
    }

    pub(crate) fn mounted_instances_for(
        &self,
        node: UiMountedGraphNodeHandle,
    ) -> Result<Box<[UiMountedInstanceIdentity]>, UiMountedIdentityDenial> {
        self.identity.instances_for(node)
    }

    pub(crate) fn advance_frame(
        &mut self,
    ) -> Result<UiMountedFrameIdentity, UiMountedIdentityDenial> {
        self.ensure_identity_mutation_available()?;
        self.identity.advance_frame()
    }

    pub(crate) fn validate_binding(
        &self,
        binding: UiSurfaceBindingGeneration,
    ) -> Result<(), UiMountedIdentityDenial> {
        self.identity.validate_binding(binding)
    }

    pub(crate) fn validate_current_frame(
        &self,
        frame: UiMountedFrameIdentity,
    ) -> Result<(), UiMountedIdentityDenial> {
        self.identity.validate_current_frame(frame)
    }

    pub(crate) fn validate_current_receipt(
        &self,
        instance: UiMountedInstanceIdentity,
        receipt: UiMountedNodeReceiptIdentity,
    ) -> Result<(), UiMountedIdentityDenial> {
        self.identity.validate_current_receipt(instance, receipt)
    }

    pub(crate) fn view(&self) -> UiMountedIdentityView {
        self.identity.view()
    }

    fn ensure_identity_mutation_available(&self) -> Result<(), UiMountedIdentityDenial> {
        if self.has_active_presentation_attempt() {
            return Err(UiMountedIdentityDenial::PresentationInFlight);
        }
        Ok(())
    }
}
