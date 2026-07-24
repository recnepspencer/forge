use super::WorthUiActiveApplicationSession;
use crate::facade::mounted::{
    UiHostSurfacePresentationMode, UiMountedFrameIdentity, UiMountedGraphNodeHandle,
    UiMountedIdentityDenial, UiMountedIdentityView, UiMountedInstanceIdentity,
    UiMountedNodeReceiptIdentity, UiMountedProjectionAudience, UiSemanticSurfaceIdentity,
    UiSurfaceBindingGeneration, UiSurfaceBindingIdentityView, UiSurfaceBindingProfile,
};

impl WorthUiActiveApplicationSession {
    pub fn create_semantic_surface(
        &mut self,
    ) -> Result<UiSemanticSurfaceIdentity, UiMountedIdentityDenial> {
        self.ensure_mounted_identity_mutation_available()?;
        self.mounted_identity.create_semantic_surface()
    }

    pub fn create_semantic_surface_for(
        &mut self,
        audience: UiMountedProjectionAudience,
    ) -> Result<UiSemanticSurfaceIdentity, UiMountedIdentityDenial> {
        self.ensure_mounted_identity_mutation_available()?;
        self.mounted_identity.create_semantic_surface_for(audience)
    }

    pub fn mounted_graph_node(
        &self,
        graph_node_identity: crate::graph::UiGraphNodeIdentity,
    ) -> Result<UiMountedGraphNodeHandle, UiMountedIdentityDenial> {
        self.mounted_identity
            .graph_node_handle(self.app.graph(), graph_node_identity)
    }

    pub fn register_host_surface(
        &mut self,
        semantic_surface: UiSemanticSurfaceIdentity,
        mode: UiHostSurfacePresentationMode,
        profile: UiSurfaceBindingProfile,
    ) -> Result<UiSurfaceBindingIdentityView, UiMountedIdentityDenial> {
        self.ensure_mounted_identity_mutation_available()?;
        let candidate = self.mounted_identity.prepare_surface_registration(
            self.host_session.protocol(),
            self.host_session.capability_report(),
            semantic_surface,
            mode,
            profile,
        )?;
        let baseline = self
            .mounted_presentation
            .host_truth_mut()
            .register_surface(self.host_session.effect_port(), candidate.request())?;
        Ok(self
            .mounted_identity
            .commit_surface_registration(candidate, baseline))
    }

    pub fn deregister_host_surface(
        &mut self,
        binding: UiSurfaceBindingGeneration,
    ) -> Result<UiSemanticSurfaceIdentity, UiMountedIdentityDenial> {
        self.ensure_mounted_identity_mutation_available()?;
        let requires_reconciliation = self
            .mounted_presentation
            .binding_requires_reconciliation(binding);
        let required_by_current = self.mounted_identity.current_requires_binding(binding);
        let has_published_predecessor = self.mounted_identity.publication_receipt().is_some();
        let preserve_published_frame =
            has_published_predecessor && (requires_reconciliation || !required_by_current);
        let candidate = self
            .mounted_identity
            .prepare_surface_deregistration(binding, preserve_published_frame)?;
        self.mounted_presentation
            .host_truth_mut()
            .deregister_surface(self.host_session.effect_port(), candidate.request())?;
        let semantic_surface = self
            .mounted_identity
            .commit_surface_deregistration(candidate);
        if has_published_predecessor && requires_reconciliation && !required_by_current {
            self.mounted_presentation
                .reconcile_candidate_only_deregistration(binding);
        }
        Ok(semantic_surface)
    }

    pub fn rebind_host_surface(
        &mut self,
        binding: UiSurfaceBindingGeneration,
        mode: UiHostSurfacePresentationMode,
        profile: UiSurfaceBindingProfile,
    ) -> Result<UiSurfaceBindingIdentityView, UiMountedIdentityDenial> {
        let semantic_surface = self.deregister_host_surface(binding)?;
        self.register_host_surface(semantic_surface, mode, profile)
    }

    pub fn recover_indeterminate_host_surface(
        &mut self,
        semantic_surface: UiSemanticSurfaceIdentity,
    ) -> Result<(), UiMountedIdentityDenial> {
        self.ensure_mounted_identity_mutation_available()?;
        self.mounted_presentation
            .host_truth_mut()
            .recover_surface_effect(self.host_session.effect_port(), semantic_surface)
    }

    pub fn mount_instance(
        &mut self,
        node: UiMountedGraphNodeHandle,
        surface: UiSemanticSurfaceIdentity,
    ) -> Result<UiMountedInstanceIdentity, UiMountedIdentityDenial> {
        self.ensure_mounted_identity_mutation_available()?;
        self.mounted_identity.mount(self.app.graph(), node, surface)
    }

    pub fn unmount_instance(
        &mut self,
        identity: UiMountedInstanceIdentity,
    ) -> Result<(), UiMountedIdentityDenial> {
        self.ensure_mounted_identity_mutation_available()?;
        self.mounted_identity.unmount(identity)
    }

    pub fn reorder_mounted_instances(
        &mut self,
        order: &[UiMountedInstanceIdentity],
    ) -> Result<(), UiMountedIdentityDenial> {
        self.ensure_mounted_identity_mutation_available()?;
        self.mounted_identity.reorder(order)
    }

    pub fn mounted_instances_for(
        &self,
        node: UiMountedGraphNodeHandle,
    ) -> Result<Box<[UiMountedInstanceIdentity]>, UiMountedIdentityDenial> {
        self.mounted_identity.instances_for(node)
    }

    pub fn advance_mounted_identity_frame(
        &mut self,
    ) -> Result<UiMountedFrameIdentity, UiMountedIdentityDenial> {
        self.ensure_mounted_identity_mutation_available()?;
        self.mounted_identity.advance_frame()
    }

    pub fn validate_current_surface_binding(
        &self,
        binding: UiSurfaceBindingGeneration,
    ) -> Result<(), UiMountedIdentityDenial> {
        self.mounted_identity.validate_binding(binding)
    }

    pub fn validate_current_mounted_frame(
        &self,
        frame: UiMountedFrameIdentity,
    ) -> Result<(), UiMountedIdentityDenial> {
        self.mounted_identity.validate_current_frame(frame)
    }

    pub fn validate_current_mounted_node_receipt(
        &self,
        instance: UiMountedInstanceIdentity,
        receipt: UiMountedNodeReceiptIdentity,
    ) -> Result<(), UiMountedIdentityDenial> {
        self.mounted_identity
            .validate_current_receipt(instance, receipt)
    }

    pub fn inspect_mounted_identity(&self) -> UiMountedIdentityView {
        self.mounted_identity.view()
    }

    fn ensure_mounted_identity_mutation_available(&self) -> Result<(), UiMountedIdentityDenial> {
        if self.mounted_presentation.has_active_attempt() {
            return Err(UiMountedIdentityDenial::PresentationInFlight);
        }
        Ok(())
    }
}
