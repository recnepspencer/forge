use super::{ExtensionFamilyPosture, FutureLayoutTarget, FutureLayoutTargetDeclaration};
use worth_proof::TransitionOutcome;
use worth_store_layout_indexes::{
    customization::{
        layout_customization_boundary,
        FutureLayoutCustomizationAdmission as StoreLayoutCustomizationAdmission,
        FutureLayoutCustomizationDeferred as StoreLayoutCustomizationDeferred,
        FutureLayoutCustomizationDenial as StoreLayoutCustomizationDenial,
        FutureLayoutCustomizationRequest,
    },
    AdmittedPhysicalArtifactFamily, AdmittedPhysicalKeyDomain,
};

pub type FutureLayoutCustomizationOutcome = TransitionOutcome<
    FutureLayoutCustomizationAdmission,
    FutureLayoutCustomizationDenial,
    FutureLayoutCustomizationDeferred,
>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FutureLayoutCustomizationAdmissionRequest {
    declaration: FutureLayoutTargetDeclaration,
    authority_source: AdmittedPhysicalArtifactFamily,
}

impl FutureLayoutCustomizationAdmissionRequest {
    pub const fn new(
        declaration: FutureLayoutTargetDeclaration,
        authority_source: AdmittedPhysicalArtifactFamily,
    ) -> Self {
        Self {
            declaration,
            authority_source,
        }
    }

    pub const fn declaration(self) -> FutureLayoutTargetDeclaration {
        self.declaration
    }

    pub const fn authority_source(self) -> AdmittedPhysicalArtifactFamily {
        self.authority_source
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FutureLayoutCustomizationAdmission {
    target: FutureLayoutTargetDeclaration,
    store_admission: StoreLayoutCustomizationAdmission,
}

impl FutureLayoutCustomizationAdmission {
    const fn new(
        target: FutureLayoutTargetDeclaration,
        store_admission: StoreLayoutCustomizationAdmission,
    ) -> Self {
        Self {
            target,
            store_admission,
        }
    }

    pub const fn target(&self) -> FutureLayoutTargetDeclaration {
        self.target
    }

    pub const fn store_admission(&self) -> &StoreLayoutCustomizationAdmission {
        &self.store_admission
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FutureLayoutCustomizationDenial {
    TargetRejected { target: FutureLayoutTarget },
    StoreDenied(StoreLayoutCustomizationDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FutureLayoutCustomizationDeferred {
    TargetRebuildRequired { target: FutureLayoutTarget },
    StoreDeferred(StoreLayoutCustomizationDeferred),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FutureLayoutCustomizationCatalogFacade;

impl FutureLayoutCustomizationCatalogFacade {
    pub const fn declare_stable_basis_read(
        &self,
        posture: ExtensionFamilyPosture,
        declared_domain: AdmittedPhysicalKeyDomain,
    ) -> FutureLayoutTargetDeclaration {
        FutureLayoutTargetDeclaration::new(
            FutureLayoutTarget::StableBasisRead,
            posture,
            declared_domain,
        )
    }

    pub const fn declare_aspect_projection(
        &self,
        posture: ExtensionFamilyPosture,
        declared_domain: AdmittedPhysicalKeyDomain,
    ) -> FutureLayoutTargetDeclaration {
        FutureLayoutTargetDeclaration::new(
            FutureLayoutTarget::AspectProjection,
            posture,
            declared_domain,
        )
    }

    pub const fn declare_subscription_support(
        &self,
        posture: ExtensionFamilyPosture,
        declared_domain: AdmittedPhysicalKeyDomain,
    ) -> FutureLayoutTargetDeclaration {
        FutureLayoutTargetDeclaration::new(
            FutureLayoutTarget::SubscriptionSupport,
            posture,
            declared_domain,
        )
    }

    pub const fn declare_support_trust(
        &self,
        posture: ExtensionFamilyPosture,
        declared_domain: AdmittedPhysicalKeyDomain,
    ) -> FutureLayoutTargetDeclaration {
        FutureLayoutTargetDeclaration::new(
            FutureLayoutTarget::SupportTrust,
            posture,
            declared_domain,
        )
    }

    pub fn admit(
        &self,
        request: FutureLayoutCustomizationAdmissionRequest,
    ) -> FutureLayoutCustomizationOutcome {
        match request.declaration().posture() {
            ExtensionFamilyPosture::Registered => {}
            ExtensionFamilyPosture::RebuildRequired => {
                return TransitionOutcome::deferred(
                    FutureLayoutCustomizationDeferred::TargetRebuildRequired {
                        target: request.declaration().target(),
                    },
                );
            }
            ExtensionFamilyPosture::Rejected => {
                return TransitionOutcome::denied(
                    FutureLayoutCustomizationDenial::TargetRejected {
                        target: request.declaration().target(),
                    },
                );
            }
        }

        let store_request = FutureLayoutCustomizationRequest::new(
            request.authority_source(),
            request.declaration().capability_request(),
            request.declaration().workload_envelope(),
        );

        match layout_customization_boundary().admit(store_request) {
            TransitionOutcome::Success(store_admission) => TransitionOutcome::success(
                FutureLayoutCustomizationAdmission::new(request.declaration(), store_admission),
            ),
            TransitionOutcome::Denied(denial) => {
                TransitionOutcome::denied(FutureLayoutCustomizationDenial::StoreDenied(denial))
            }
            TransitionOutcome::Deferred(deferred) => TransitionOutcome::deferred(
                FutureLayoutCustomizationDeferred::StoreDeferred(deferred),
            ),
            TransitionOutcome::Stale(stale) => match stale {},
            TransitionOutcome::RebindRequired(rebind) => match rebind {},
            TransitionOutcome::Failed(failed) => match failed {},
        }
    }
}

pub const fn layout_customization_catalog() -> FutureLayoutCustomizationCatalogFacade {
    FutureLayoutCustomizationCatalogFacade
}
