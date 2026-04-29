use super::*;

impl ForgeQueryRuntime {
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

    pub fn public_support_matrix(&self) -> ForgeQueryRuntimePublicSupportMatrix {
        ForgeQueryRuntimePublicSupportMatrix::from_public_api_contract(&self.public_api_contract())
    }

    pub fn public_mutation_api_compatibility_report(
        &self,
    ) -> ForgeQueryMutationApiCompatibilityReport {
        ForgeQueryMutationApiCompatibilityReport::derive(
            self.public_api_contract().backend_posture(),
            &self.public_support_matrix(),
            &Self::public_api_naming_contract(),
        )
    }

    pub fn public_authoritative_mutation_evidence_support(
        &self,
    ) -> ForgeQueryAuthoritativeMutationEvidenceSupport {
        ForgeQueryAuthoritativeMutationEvidenceSupport::derive(
            self.public_api_contract().backend_posture(),
        )
    }

    pub fn public_aspect_api_finalization_closeout(
        &self,
    ) -> ForgeQueryAspectApiFinalizationCloseout {
        ForgeQueryAspectApiFinalizationCloseout::derive(
            self.public_api_contract().backend_posture(),
            &self.public_support_matrix(),
            &self.public_mutation_api_compatibility_report(),
            &Self::public_api_naming_contract(),
        )
    }

    pub fn public_authoritative_mutation_evidence_closeout(
        &self,
    ) -> ForgeQueryAuthoritativeMutationEvidenceCloseout {
        let bridge_support =
            forge_runtime_bridge::facade::RuntimeBridge::public_authoritative_mutation_evidence_support();
        let bridge_closeout =
            forge_runtime_bridge::facade::RuntimeBridge::public_authoritative_mutation_evidence_closeout();
        ForgeQueryAuthoritativeMutationEvidenceCloseout::derive(
            self.public_api_contract().backend_posture(),
            &self.public_support_matrix(),
            &self.public_mutation_api_compatibility_report(),
            &Self::public_api_naming_contract(),
            &self.public_authoritative_mutation_evidence_support(),
            &bridge_support,
            &bridge_closeout,
        )
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
