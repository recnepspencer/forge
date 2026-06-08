use super::*;

impl ForgeQueryRuntime {
    pub fn public_authoritative_mutation_evidence_support_for_posture(
        posture: ForgeQueryRuntimeBackendPosture,
    ) -> ForgeQueryAuthoritativeMutationEvidenceSupport {
        ForgeQueryAuthoritativeMutationEvidenceSupport::derive(
            &ForgeQueryRuntimeSupportProfile::scaffold_backend_profile().with_posture(posture),
        )
    }

    pub fn public_authoritative_mutation_evidence_support_for_support_profile(
        support_profile: &ForgeQueryRuntimeSupportProfile,
    ) -> ForgeQueryAuthoritativeMutationEvidenceSupport {
        ForgeQueryAuthoritativeMutationEvidenceSupport::derive(support_profile)
    }

    pub fn public_authoritative_mutation_evidence_closeout_for_support_profile(
        support_profile: &ForgeQueryRuntimeSupportProfile,
    ) -> ForgeQueryAuthoritativeMutationEvidenceCloseout {
        let public_api_contract =
            ForgeQueryRuntimePublicApiContract::from_support_profile(support_profile);
        let support_matrix =
            ForgeQueryRuntimePublicSupportMatrix::from_public_api_contract(&public_api_contract);
        let naming_contract = Self::public_api_naming_contract();
        let mutation_surface = ForgeQueryMutationSurfaceReport::derive(
            public_api_contract.backend_posture(),
            &support_matrix,
            &naming_contract,
        );
        let query_support =
            Self::public_authoritative_mutation_evidence_support_for_support_profile(
                support_profile,
            );
        let bridge_support =
            forge_runtime_bridge::facade::RuntimeBridge::public_authoritative_mutation_evidence_support();
        let bridge_closeout =
            forge_runtime_bridge::facade::RuntimeBridge::public_authoritative_mutation_evidence_closeout();
        ForgeQueryAuthoritativeMutationEvidenceCloseout::derive(
            public_api_contract.backend_posture(),
            &support_matrix,
            &mutation_surface,
            &naming_contract,
            &query_support,
            &bridge_support,
            &bridge_closeout,
        )
    }

    pub fn builder() -> ForgeQueryRuntimeBuilder {
        ForgeQueryRuntimeBuilder::new()
    }

    pub fn workspace(
        self,
        name: impl Into<String>,
    ) -> Result<ForgeQueryWorkspace, ForgeQueryRuntimeError> {
        ForgeQueryWorkspace::new(name, self)
    }

    pub fn public_api_naming_contract() -> ForgeQueryRuntimePublicApiNamingContract {
        ForgeQueryRuntimePublicApiNamingContract::standard()
    }

    pub fn public_api_contract(&self) -> ForgeQueryRuntimePublicApiContract {
        ForgeQueryRuntimePublicApiContract::from_support_profile(&self.backend.support_profile())
    }

    pub fn public_handle_contract(&self) -> ForgeQueryHandleContract {
        ForgeQueryHandleContract::from_public_api_contract(&self.public_api_contract())
    }

    pub fn public_downstream_delivery_contract(
        &self,
    ) -> ForgeQueryRuntimeDownstreamDeliveryContract {
        ForgeQueryRuntimeDownstreamDeliveryContract::from_support_profile(
            &self.backend.support_profile(),
        )
    }

    pub fn public_support_matrix(&self) -> ForgeQueryRuntimePublicSupportMatrix {
        ForgeQueryRuntimePublicSupportMatrix::from_public_api_contract(&self.public_api_contract())
    }

    pub fn public_mutation_surface_report(&self) -> ForgeQueryMutationSurfaceReport {
        ForgeQueryMutationSurfaceReport::derive(
            self.public_api_contract().backend_posture(),
            &self.public_support_matrix(),
            &Self::public_api_naming_contract(),
        )
    }

    pub fn public_authoritative_mutation_evidence_support(
        &self,
    ) -> ForgeQueryAuthoritativeMutationEvidenceSupport {
        Self::public_authoritative_mutation_evidence_support_for_support_profile(
            &self.backend.support_profile(),
        )
    }

    pub fn public_aspect_api_finalization_closeout(
        &self,
    ) -> ForgeQueryAspectApiFinalizationCloseout {
        ForgeQueryAspectApiFinalizationCloseout::derive(
            self.public_api_contract().backend_posture(),
            &self.public_support_matrix(),
            &self.public_mutation_surface_report(),
            &Self::public_api_naming_contract(),
        )
    }

    pub fn public_authoritative_mutation_evidence_closeout(
        &self,
    ) -> ForgeQueryAuthoritativeMutationEvidenceCloseout {
        Self::public_authoritative_mutation_evidence_closeout_for_support_profile(
            &self.backend.support_profile(),
        )
    }

    pub fn downstream_delivery<T>(
        &self,
        view: &ForgeQueryLiveView<T>,
    ) -> Result<Option<ForgeQueryRuntimeDownstreamDelivery>, ForgeQueryRuntimeError> {
        let state = self.live_subscriptions.get(view.name()).ok_or_else(|| {
            ForgeQueryRuntimeError::MissingLiveSubscription(view.name().to_string())
        })?;
        Ok(project_downstream_delivery(
            &self.public_downstream_delivery_contract(),
            state,
        ))
    }

    pub fn admit_public_api_family(
        &self,
        family: ForgeQueryRuntimeFacadeFamily,
    ) -> Result<ForgeQueryRuntimePublicApiFamilyContract, ForgeQueryRuntimeError> {
        let contract = self.public_api_contract();
        let row = contract.family(family).cloned().ok_or_else(|| {
            ForgeQueryRuntimeError::UnsupportedFacadeFamily(ForgeQueryRuntimeSupportDenial::new(
                family,
                "runtime support matrix does not declare this public API family",
            ))
        })?;
        self.admit_facade_family(family)?;
        Ok(row)
    }
}
