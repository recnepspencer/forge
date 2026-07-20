use std::sync::Arc;

use crate::domain_capabilities::{
    WorthQueryAdmittedPlanBoundContributionTarget, WorthQueryDeclarationBoundContributionTarget,
    WorthQueryInstalledAdmittedPlanContributionTarget,
    WorthQueryInstalledDeclarationContributionTarget, WorthQueryInstalledDomainContributionTarget,
    WorthQueryInstalledLowerRuntimeContributionTarget,
    WorthQueryLowerRuntimeBoundaryBoundContributionTarget,
};
use crate::domain_installation::{
    WorthQueryDomainHandleDenial, WorthQueryDomainHandleDenialKind,
    WorthQueryInstalledDomainAuthority,
};
use crate::runtime::{
    WorthQueryAdmittedIntentPlan, WorthQueryIntentDeclaration,
    WorthQueryLowerRuntimeBoundaryEnvelope, WorthQueryLowerRuntimeBoundaryEnvelopeSource,
};

use super::admitted_plan::WorthQueryAdmittedPlanDomainContributionSurface;
use super::intent::WorthQueryIntentDomainContributionSurface;
use super::lower_runtime::WorthQueryLowerRuntimeDomainContributionSurface;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledDomainContributionSurface {
    authority: Arc<WorthQueryInstalledDomainAuthority>,
}

impl WorthQueryInstalledDomainContributionSurface {
    pub(crate) fn new(authority: Arc<WorthQueryInstalledDomainAuthority>) -> Self {
        Self { authority }
    }

    #[cfg(test)]
    pub(crate) fn authority_identity(
        &self,
    ) -> &crate::evidence_identity::WorthQueryEvidenceIdentity {
        self.authority.authority_identity()
    }

    pub fn intent_target(
        &self,
        declaration: &WorthQueryIntentDeclaration,
    ) -> Result<WorthQueryInstalledDeclarationContributionTarget, WorthQueryDomainHandleDenial>
    {
        self.validate_current_authority()?;
        Ok(WorthQueryInstalledDomainContributionTarget::bind(
            Arc::clone(&self.authority),
            WorthQueryDeclarationBoundContributionTarget::for_intent_declaration(declaration),
        ))
    }

    pub fn for_intent(
        &self,
        declaration: &WorthQueryIntentDeclaration,
    ) -> Result<WorthQueryIntentDomainContributionSurface, WorthQueryDomainHandleDenial> {
        Ok(WorthQueryIntentDomainContributionSurface {
            target: self.intent_target(declaration)?,
        })
    }

    pub fn for_intent_target(
        &self,
        target: WorthQueryInstalledDeclarationContributionTarget,
    ) -> Result<WorthQueryIntentDomainContributionSurface, WorthQueryDomainHandleDenial> {
        self.validate_current_authority()?;
        self.validate_target_authority(target.authority())?;
        Ok(WorthQueryIntentDomainContributionSurface { target })
    }

    pub fn admitted_plan_target(
        &self,
        plan: &WorthQueryAdmittedIntentPlan,
    ) -> Result<WorthQueryInstalledAdmittedPlanContributionTarget, WorthQueryDomainHandleDenial>
    {
        self.validate_current_authority()?;
        Ok(WorthQueryInstalledDomainContributionTarget::bind(
            Arc::clone(&self.authority),
            WorthQueryAdmittedPlanBoundContributionTarget::for_admitted_intent_plan(plan),
        ))
    }

    pub fn for_admitted_intent_plan(
        &self,
        plan: &WorthQueryAdmittedIntentPlan,
    ) -> Result<WorthQueryAdmittedPlanDomainContributionSurface, WorthQueryDomainHandleDenial> {
        Ok(WorthQueryAdmittedPlanDomainContributionSurface {
            target: self.admitted_plan_target(plan)?,
        })
    }

    pub fn for_admitted_plan_target(
        &self,
        target: WorthQueryInstalledAdmittedPlanContributionTarget,
    ) -> Result<WorthQueryAdmittedPlanDomainContributionSurface, WorthQueryDomainHandleDenial> {
        self.validate_current_authority()?;
        self.validate_target_authority(target.authority())?;
        Ok(WorthQueryAdmittedPlanDomainContributionSurface { target })
    }

    pub fn lower_runtime_target(
        &self,
        envelope: &WorthQueryLowerRuntimeBoundaryEnvelope,
    ) -> Result<WorthQueryInstalledLowerRuntimeContributionTarget, WorthQueryDomainHandleDenial>
    {
        self.validate_current_authority()?;
        Ok(WorthQueryInstalledDomainContributionTarget::bind(
            Arc::clone(&self.authority),
            WorthQueryLowerRuntimeBoundaryBoundContributionTarget::for_lower_runtime_boundary_envelope(
                envelope,
            ),
        ))
    }

    pub fn for_lower_runtime_boundary_envelope(
        &self,
        envelope: &WorthQueryLowerRuntimeBoundaryEnvelope,
    ) -> Result<WorthQueryLowerRuntimeDomainContributionSurface, WorthQueryDomainHandleDenial> {
        Ok(WorthQueryLowerRuntimeDomainContributionSurface {
            target: self.lower_runtime_target(envelope)?,
        })
    }

    pub fn for_lower_runtime_target(
        &self,
        target: WorthQueryInstalledLowerRuntimeContributionTarget,
    ) -> Result<WorthQueryLowerRuntimeDomainContributionSurface, WorthQueryDomainHandleDenial> {
        self.validate_current_authority()?;
        self.validate_target_authority(target.authority())?;
        Ok(WorthQueryLowerRuntimeDomainContributionSurface { target })
    }

    pub fn for_lower_runtime_boundary_source<S>(
        &self,
        source: &S,
    ) -> Result<WorthQueryLowerRuntimeDomainContributionSurface, WorthQueryDomainHandleDenial>
    where
        S: WorthQueryLowerRuntimeBoundaryEnvelopeSource + ?Sized,
    {
        self.for_lower_runtime_boundary_envelope(source.lower_runtime_boundary_envelope())
    }

    fn validate_current_authority(&self) -> Result<(), WorthQueryDomainHandleDenial> {
        if !self.authority.is_current_installation_generation() {
            return Err(WorthQueryDomainHandleDenial::new(
                WorthQueryDomainHandleDenialKind::StaleInstallationGeneration,
            ));
        }
        Ok(())
    }

    fn validate_target_authority(
        &self,
        target_authority: &WorthQueryInstalledDomainAuthority,
    ) -> Result<(), WorthQueryDomainHandleDenial> {
        if !target_authority.is_current_installation_generation() {
            return Err(WorthQueryDomainHandleDenial::new(
                WorthQueryDomainHandleDenialKind::StaleInstallationGeneration,
            ));
        }
        if self.authority.authority_identity() != target_authority.authority_identity() {
            return Err(WorthQueryDomainHandleDenial::new(
                WorthQueryDomainHandleDenialKind::ForeignRuntime,
            ));
        }
        Ok(())
    }
}
