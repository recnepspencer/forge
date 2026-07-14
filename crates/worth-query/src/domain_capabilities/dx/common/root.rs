use std::any::TypeId;
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
    WorthQueryDomainInstallationGeneration, WorthQueryDomainPackageIdentity,
    WorthQueryInstalledDomainAuthority,
};
use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::runtime::{
    WorthQueryAdmittedIntentPlan, WorthQueryIntentDeclaration,
    WorthQueryLowerRuntimeBoundaryEnvelope, WorthQueryLowerRuntimeBoundaryEnvelopeSource,
};

use super::admitted_plan::WorthQueryAdmittedPlanDomainContributionSurface;
use super::intent::WorthQueryIntentDomainContributionSurface;
use super::lower_runtime::WorthQueryLowerRuntimeDomainContributionSurface;

struct WorthQueryCertificationDomain;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryInstalledDomainContributionSurface {
    authority: Arc<WorthQueryInstalledDomainAuthority>,
}

impl WorthQueryInstalledDomainContributionSurface {
    pub(crate) fn new(authority: Arc<WorthQueryInstalledDomainAuthority>) -> Self {
        Self { authority }
    }

    pub fn intent_target(
        &self,
        declaration: &WorthQueryIntentDeclaration,
    ) -> WorthQueryInstalledDeclarationContributionTarget {
        WorthQueryInstalledDomainContributionTarget::bind(
            Arc::clone(&self.authority),
            WorthQueryDeclarationBoundContributionTarget::for_intent_declaration(declaration),
        )
    }

    pub fn for_intent(
        &self,
        declaration: &WorthQueryIntentDeclaration,
    ) -> WorthQueryIntentDomainContributionSurface {
        WorthQueryIntentDomainContributionSurface {
            target: self.intent_target(declaration),
        }
    }

    pub fn for_intent_target(
        &self,
        target: WorthQueryInstalledDeclarationContributionTarget,
    ) -> Result<WorthQueryIntentDomainContributionSurface, WorthQueryDomainHandleDenial> {
        self.validate_target_authority(target.authority())?;
        Ok(WorthQueryIntentDomainContributionSurface { target })
    }

    pub fn admitted_plan_target(
        &self,
        plan: &WorthQueryAdmittedIntentPlan,
    ) -> WorthQueryInstalledAdmittedPlanContributionTarget {
        WorthQueryInstalledDomainContributionTarget::bind(
            Arc::clone(&self.authority),
            WorthQueryAdmittedPlanBoundContributionTarget::for_admitted_intent_plan(plan),
        )
    }

    pub fn for_admitted_intent_plan(
        &self,
        plan: &WorthQueryAdmittedIntentPlan,
    ) -> WorthQueryAdmittedPlanDomainContributionSurface {
        WorthQueryAdmittedPlanDomainContributionSurface {
            target: self.admitted_plan_target(plan),
        }
    }

    pub fn for_admitted_plan_target(
        &self,
        target: WorthQueryInstalledAdmittedPlanContributionTarget,
    ) -> Result<WorthQueryAdmittedPlanDomainContributionSurface, WorthQueryDomainHandleDenial> {
        self.validate_target_authority(target.authority())?;
        Ok(WorthQueryAdmittedPlanDomainContributionSurface { target })
    }

    pub fn lower_runtime_target(
        &self,
        envelope: &WorthQueryLowerRuntimeBoundaryEnvelope,
    ) -> WorthQueryInstalledLowerRuntimeContributionTarget {
        WorthQueryInstalledDomainContributionTarget::bind(
            Arc::clone(&self.authority),
            WorthQueryLowerRuntimeBoundaryBoundContributionTarget::for_lower_runtime_boundary_envelope(
                envelope,
            ),
        )
    }

    pub fn for_lower_runtime_boundary_envelope(
        &self,
        envelope: &WorthQueryLowerRuntimeBoundaryEnvelope,
    ) -> WorthQueryLowerRuntimeDomainContributionSurface {
        WorthQueryLowerRuntimeDomainContributionSurface {
            target: self.lower_runtime_target(envelope),
        }
    }

    pub fn for_lower_runtime_target(
        &self,
        target: WorthQueryInstalledLowerRuntimeContributionTarget,
    ) -> Result<WorthQueryLowerRuntimeDomainContributionSurface, WorthQueryDomainHandleDenial> {
        self.validate_target_authority(target.authority())?;
        Ok(WorthQueryLowerRuntimeDomainContributionSurface { target })
    }

    pub fn for_lower_runtime_boundary_source<S>(
        &self,
        source: &S,
    ) -> WorthQueryLowerRuntimeDomainContributionSurface
    where
        S: WorthQueryLowerRuntimeBoundaryEnvelopeSource + ?Sized,
    {
        self.for_lower_runtime_boundary_envelope(source.lower_runtime_boundary_envelope())
    }

    fn validate_target_authority(
        &self,
        target_authority: &WorthQueryInstalledDomainAuthority,
    ) -> Result<(), WorthQueryDomainHandleDenial> {
        if self.authority.authority_identity() != target_authority.authority_identity() {
            return Err(WorthQueryDomainHandleDenial::new(
                WorthQueryDomainHandleDenialKind::ForeignRuntime,
            ));
        }
        Ok(())
    }
}

/// Internal oracle root used by certification while the product facade is
/// exercised through runtime-installed handles.
pub(crate) fn worth_query_certification_domain(
    domain: impl Into<String>,
) -> WorthQueryInstalledDomainContributionSurface {
    let domain = domain.into();
    let package_identity = WorthQueryDomainPackageIdentity::new(
        worth_query_evidence_identity(WorthQueryEvidenceScope::DomainPackageIdentity)
            .field_shape(
                WorthQueryEvidenceTag::new("authority_family"),
                "domain-capability-certification-oracle",
            )
            .field_value(WorthQueryEvidenceTag::new("domain"), &domain)
            .seal(),
    );
    let installation_identity =
        worth_query_evidence_identity(WorthQueryEvidenceScope::DomainInstallation)
            .field_shape(
                WorthQueryEvidenceTag::new("authority_family"),
                "domain-capability-certification-oracle",
            )
            .field_value(WorthQueryEvidenceTag::new("domain"), &domain)
            .seal();
    let policy = vec![
        crate::application::WorthQueryDeclarationEntryContributionCategoryFamily::Admission,
        crate::application::WorthQueryDeclarationEntryContributionCategoryFamily::SupportTraceability,
        crate::application::WorthQueryDeclarationEntryContributionCategoryFamily::InvariantCapability,
        crate::application::WorthQueryDeclarationEntryContributionCategoryFamily::WorkflowPreview,
        crate::application::WorthQueryDeclarationEntryContributionCategoryFamily::ContinuityLineage,
        crate::application::WorthQueryDeclarationEntryContributionCategoryFamily::ConsequenceAftermath,
        crate::application::WorthQueryDeclarationEntryContributionCategoryFamily::ExplanationInspection,
    ];
    WorthQueryInstalledDomainContributionSurface::new(Arc::new(
        WorthQueryInstalledDomainAuthority::new(
            crate::runtime::WorthQueryRuntimeAuthorityIdentity::mint(),
            WorthQueryDomainInstallationGeneration::initial(),
            TypeId::of::<WorthQueryCertificationDomain>(),
            "WORTH.certification.domain",
            "WorthQueryCertificationDomain",
            domain,
            package_identity,
            installation_identity,
            crate::application::WorthQueryDomainEntrySupportSnapshot::from_support_report(
                crate::application::WorthQueryApplicationFacade::runtime_backed_default()
                    .support_report(),
            ),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            policy,
        ),
    ))
}

#[allow(unused_imports)]
pub(crate) use worth_query_certification_domain as worth_query_domain;
