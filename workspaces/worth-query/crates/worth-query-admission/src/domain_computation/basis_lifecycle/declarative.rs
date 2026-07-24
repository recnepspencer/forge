use super::{
    admit_basis_capability, evaluate_basis_certification_eligibility,
    evaluate_basis_inspection_eligibility, evaluate_basis_materialization_eligibility,
    evaluate_basis_preview_closeout_eligibility, evaluate_basis_replay_eligibility,
    evaluate_basis_subscription_activation_eligibility,
    evaluate_basis_subscription_declaration_eligibility, normalize_raw_basis_intent,
    scope_basis_for_certification, scope_basis_for_inspection, scope_basis_for_materialization,
    scope_basis_for_mutation_preparation, scope_basis_for_preview_closeout, scope_basis_for_replay,
    scope_basis_for_subscription_activation, scope_basis_for_subscription_declaration,
    AdmittedBasisCapability, BasisIntentDenial, BasisLifecycleIntentBuilder,
    BasisLifecycleIntentDraft, BasisLifecyclePolicyIntentDraft, BasisOperationLane,
    CertificationLaneWitness, DeniedBasisCapability, InspectionLaneWitness,
    MaterializationLaneWitness, PreviewCloseoutLaneWitness, RawBasisIntent, ReplayLaneWitness,
    ScopedCertificationBasis, ScopedInspectionBasis, ScopedMaterializationBasis,
    ScopedMutationPreparationBasis, ScopedObservationBasis, ScopedPreviewCloseoutBasis,
    ScopedReplayBasis, ScopedSubscriptionActivationBasis, ScopedSubscriptionDeclarationBasis,
    SubscriptionActivationLaneWitness, SubscriptionDeclarationLaneWitness,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BasisLifecycleDeclarationError {
    Intent(BasisIntentDenial),
    Eligibility(DeniedBasisCapability),
}

impl BasisLifecycleDeclarationError {
    pub fn intent_denial(&self) -> Option<&BasisIntentDenial> {
        match self {
            Self::Intent(denial) => Some(denial),
            Self::Eligibility(_) => None,
        }
    }

    pub fn eligibility_denial(&self) -> Option<&DeniedBasisCapability> {
        match self {
            Self::Intent(_) => None,
            Self::Eligibility(denial) => Some(denial),
        }
    }
}

macro_rules! phase_path {
    (
        $admission:ident,
        $use_path:ident,
        $witness:ty,
        $evaluate:ident,
        $scope:ident,
        $scoped:ty
    ) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $admission {
            normalized: super::NormalizedBasisIntent,
        }

        impl $admission {
            fn new(raw: RawBasisIntent) -> Result<Self, BasisIntentDenial> {
                let normalized =
                    normalize_raw_basis_intent(raw, <$witness as BasisOperationLane>::lane_name())?;
                Ok(Self { normalized })
            }

            pub fn admit(self) -> Result<$use_path, DeniedBasisCapability> {
                let eligibility = $evaluate(self.normalized)?;
                Ok($use_path {
                    capability: admit_basis_capability(eligibility),
                })
            }
        }

        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $use_path {
            capability: AdmittedBasisCapability<$witness>,
        }

        impl $use_path {
            pub fn capability(&self) -> &AdmittedBasisCapability<$witness> {
                &self.capability
            }

            pub fn scope(self) -> $scoped {
                $scope(self.capability)
            }
        }
    };
}

phase_path!(
    ReplayBasisAdmissionPath,
    ReplayBasisUsePath,
    ReplayLaneWitness,
    evaluate_basis_replay_eligibility,
    scope_basis_for_replay,
    ScopedReplayBasis
);
phase_path!(
    InspectionBasisAdmissionPath,
    InspectionBasisUsePath,
    InspectionLaneWitness,
    evaluate_basis_inspection_eligibility,
    scope_basis_for_inspection,
    ScopedInspectionBasis
);
phase_path!(
    MaterializationBasisAdmissionPath,
    MaterializationBasisUsePath,
    MaterializationLaneWitness,
    evaluate_basis_materialization_eligibility,
    scope_basis_for_materialization,
    ScopedMaterializationBasis
);
phase_path!(
    SubscriptionDeclarationBasisAdmissionPath,
    SubscriptionDeclarationBasisUsePath,
    SubscriptionDeclarationLaneWitness,
    evaluate_basis_subscription_declaration_eligibility,
    scope_basis_for_subscription_declaration,
    ScopedSubscriptionDeclarationBasis
);
phase_path!(
    SubscriptionActivationBasisAdmissionPath,
    SubscriptionActivationBasisUsePath,
    SubscriptionActivationLaneWitness,
    evaluate_basis_subscription_activation_eligibility,
    scope_basis_for_subscription_activation,
    ScopedSubscriptionActivationBasis
);
phase_path!(
    PreviewCloseoutBasisAdmissionPath,
    PreviewCloseoutBasisUsePath,
    PreviewCloseoutLaneWitness,
    evaluate_basis_preview_closeout_eligibility,
    scope_basis_for_preview_closeout,
    ScopedPreviewCloseoutBasis
);
phase_path!(
    CertificationBasisAdmissionPath,
    CertificationBasisUsePath,
    CertificationLaneWitness,
    evaluate_basis_certification_eligibility,
    scope_basis_for_certification,
    ScopedCertificationBasis
);

impl BasisLifecycleIntentBuilder {
    pub fn branch_snapshot(
        &self,
        branch_identity: impl Into<String>,
        snapshot_identity: impl Into<String>,
    ) -> BasisLifecycleIntentDraft {
        BasisLifecycleIntentDraft::new(RawBasisIntent::BranchSnapshot {
            branch_identity: branch_identity.into(),
            snapshot_identity: snapshot_identity.into(),
        })
    }

    pub fn preview(&self, preview_identity: impl Into<String>) -> BasisLifecycleIntentDraft {
        self.preview_with_staleness(preview_identity, false)
    }

    pub fn stale_preview(&self, preview_identity: impl Into<String>) -> BasisLifecycleIntentDraft {
        self.preview_with_staleness(preview_identity, true)
    }

    fn preview_with_staleness(
        &self,
        preview_identity: impl Into<String>,
        stale: bool,
    ) -> BasisLifecycleIntentDraft {
        BasisLifecycleIntentDraft::new(RawBasisIntent::Preview {
            preview_identity: preview_identity.into(),
            stale,
        })
    }

    pub fn runtime_snapshot(
        &self,
        snapshot_identity: impl Into<String>,
        lower_runtime_binding_digest: impl Into<String>,
    ) -> BasisLifecycleIntentDraft {
        BasisLifecycleIntentDraft::new(RawBasisIntent::RuntimeSnapshot {
            snapshot_identity: snapshot_identity.into(),
            lower_runtime_binding_digest: Some(lower_runtime_binding_digest.into()),
        })
    }

    pub fn historical_snapshot(
        &self,
        snapshot_identity: impl Into<String>,
        replay_supported: bool,
    ) -> BasisLifecycleIntentDraft {
        BasisLifecycleIntentDraft::new(RawBasisIntent::HistoricalSnapshot {
            snapshot_identity: snapshot_identity.into(),
            replay_supported,
        })
    }

    pub fn historical_commit(
        &self,
        commit_identity: impl Into<String>,
        replay_supported: bool,
    ) -> BasisLifecycleIntentDraft {
        BasisLifecycleIntentDraft::new(RawBasisIntent::HistoricalCommit {
            commit_identity: commit_identity.into(),
            replay_supported,
        })
    }

    pub fn tenant_scoped(
        &self,
        tenant_identity: impl Into<String>,
        branch_identity: impl Into<String>,
        schema_identity: impl Into<String>,
        tenant_schema_matches: bool,
    ) -> BasisLifecycleIntentDraft {
        BasisLifecycleIntentDraft::new(RawBasisIntent::TenantScoped {
            tenant_identity: tenant_identity.into(),
            branch_identity: branch_identity.into(),
            schema_identity: schema_identity.into(),
            tenant_schema_matches,
        })
    }
}

impl BasisLifecycleIntentDraft {
    pub fn for_replay(self) -> Result<ReplayBasisAdmissionPath, BasisIntentDenial> {
        ReplayBasisAdmissionPath::new(self.into_raw())
    }

    pub fn for_inspection(self) -> Result<InspectionBasisAdmissionPath, BasisIntentDenial> {
        InspectionBasisAdmissionPath::new(self.into_raw())
    }

    pub fn for_materialization(
        self,
    ) -> Result<MaterializationBasisAdmissionPath, BasisIntentDenial> {
        MaterializationBasisAdmissionPath::new(self.into_raw())
    }

    pub fn for_subscription_declaration(
        self,
    ) -> Result<SubscriptionDeclarationBasisAdmissionPath, BasisIntentDenial> {
        SubscriptionDeclarationBasisAdmissionPath::new(self.into_raw())
    }

    pub fn for_subscription_activation(
        self,
    ) -> Result<SubscriptionActivationBasisAdmissionPath, BasisIntentDenial> {
        SubscriptionActivationBasisAdmissionPath::new(self.into_raw())
    }

    pub fn for_preview_closeout(
        self,
    ) -> Result<PreviewCloseoutBasisAdmissionPath, BasisIntentDenial> {
        PreviewCloseoutBasisAdmissionPath::new(self.into_raw())
    }

    pub fn for_certification(self) -> Result<CertificationBasisAdmissionPath, BasisIntentDenial> {
        CertificationBasisAdmissionPath::new(self.into_raw())
    }

    pub fn observe(self) -> Result<ScopedObservationBasis, BasisLifecycleDeclarationError> {
        self.for_observation()
            .map_err(BasisLifecycleDeclarationError::Intent)?
            .admit()
            .map_err(BasisLifecycleDeclarationError::Eligibility)
            .map(|path| path.scope())
    }

    pub fn prepare_mutation(
        self,
    ) -> Result<ScopedMutationPreparationBasis, BasisLifecycleDeclarationError> {
        self.for_mutation_preparation()
            .map_err(BasisLifecycleDeclarationError::Intent)?
            .admit()
            .map_err(BasisLifecycleDeclarationError::Eligibility)
            .map(scope_basis_for_mutation_preparation)
    }

    pub fn replay(self) -> Result<ScopedReplayBasis, BasisLifecycleDeclarationError> {
        self.for_replay()
            .map_err(BasisLifecycleDeclarationError::Intent)?
            .admit()
            .map_err(BasisLifecycleDeclarationError::Eligibility)
            .map(|path| path.scope())
    }

    pub fn inspect(self) -> Result<ScopedInspectionBasis, BasisLifecycleDeclarationError> {
        self.for_inspection()
            .map_err(BasisLifecycleDeclarationError::Intent)?
            .admit()
            .map_err(BasisLifecycleDeclarationError::Eligibility)
            .map(|path| path.scope())
    }

    pub fn materialize(self) -> Result<ScopedMaterializationBasis, BasisLifecycleDeclarationError> {
        self.for_materialization()
            .map_err(BasisLifecycleDeclarationError::Intent)?
            .admit()
            .map_err(BasisLifecycleDeclarationError::Eligibility)
            .map(|path| path.scope())
    }

    pub fn declare_subscription(
        self,
    ) -> Result<ScopedSubscriptionDeclarationBasis, BasisLifecycleDeclarationError> {
        self.for_subscription_declaration()
            .map_err(BasisLifecycleDeclarationError::Intent)?
            .admit()
            .map_err(BasisLifecycleDeclarationError::Eligibility)
            .map(|path| path.scope())
    }

    pub fn activate_subscription(
        self,
    ) -> Result<ScopedSubscriptionActivationBasis, BasisLifecycleDeclarationError> {
        self.for_subscription_activation()
            .map_err(BasisLifecycleDeclarationError::Intent)?
            .admit()
            .map_err(BasisLifecycleDeclarationError::Eligibility)
            .map(|path| path.scope())
    }

    pub fn close_preview(
        self,
    ) -> Result<ScopedPreviewCloseoutBasis, BasisLifecycleDeclarationError> {
        self.for_preview_closeout()
            .map_err(BasisLifecycleDeclarationError::Intent)?
            .admit()
            .map_err(BasisLifecycleDeclarationError::Eligibility)
            .map(|path| path.scope())
    }
}

impl BasisLifecyclePolicyIntentDraft {
    pub fn observe(self) -> Result<ScopedObservationBasis, BasisLifecycleDeclarationError> {
        self.into_draft().observe()
    }

    pub fn prepare_mutation(
        self,
    ) -> Result<ScopedMutationPreparationBasis, BasisLifecycleDeclarationError> {
        self.into_draft().prepare_mutation()
    }

    pub fn inspect(self) -> Result<ScopedInspectionBasis, BasisLifecycleDeclarationError> {
        self.into_draft().inspect()
    }

    pub fn declare_subscription(
        self,
    ) -> Result<ScopedSubscriptionDeclarationBasis, BasisLifecycleDeclarationError> {
        self.into_draft().declare_subscription()
    }
}

#[cfg(test)]
mod tests;
