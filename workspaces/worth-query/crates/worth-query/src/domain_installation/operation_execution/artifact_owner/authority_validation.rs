use super::{
    WorthQueryArtifactDenial, WorthQueryArtifactDenialKind, WorthQueryArtifactHandleCore,
    WorthQueryArtifactProductionAdmission, WorthQueryArtifactTransferAdmission,
};

impl WorthQueryArtifactHandleCore {
    pub(super) fn validate_transfer(
        &self,
        admission: &WorthQueryArtifactTransferAdmission,
    ) -> Result<(), WorthQueryArtifactDenial> {
        self.validate_binding(admission)?;
        let binding = self.owner.binding();
        if !binding
            .contract
            .contract()
            .consumer_roles()
            .iter()
            .any(|role| role == &admission.consumer_stage)
        {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::ConsumerRoleNotInstalled,
                "consumer stage is not admitted by the installed artifact contract",
            ));
        }
        Ok(())
    }

    pub(super) fn validate_output(
        &self,
        admission: &WorthQueryArtifactTransferAdmission,
    ) -> Result<(), WorthQueryArtifactDenial> {
        self.validate_binding(admission)?;
        if !self
            .owner
            .binding()
            .contract
            .contract()
            .producer_roles()
            .iter()
            .any(|role| role == &admission.consumer_stage)
        {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::ProducerRoleNotInstalled,
                "output stage is not admitted by the installed artifact contract",
            ));
        }
        Ok(())
    }

    pub(super) fn validate_replacement_binding(
        &self,
        admission: &WorthQueryArtifactProductionAdmission,
    ) -> Result<(), WorthQueryArtifactDenial> {
        let binding = self.owner.binding();
        if binding.domain_authority.runtime_authority()
            != admission.domain_authority.runtime_authority()
        {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::ForeignRuntime,
                "replacement admission belongs to a different Query runtime",
            ));
        }
        if !binding
            .domain_authority
            .is_current_installation_generation()
            || binding.domain_authority.installation_generation()
                != admission.domain_authority.installation_generation()
        {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::StaleInstallationGeneration,
                "replacement admission belongs to a different installation generation",
            ));
        }
        if binding.operation_identity != admission.operation_identity
            || binding.binding_identity != admission.binding_identity
        {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::OperationMismatch,
                "replacement admission belongs to a different operation binding",
            ));
        }
        if binding.run_identity != admission.run_identity {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::RunMismatch,
                "replacement admission belongs to a different workflow run",
            ));
        }
        if binding.basis_identity != admission.basis_identity {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::BasisMismatch,
                "replacement admission belongs to a different admitted basis",
            ));
        }
        if self.holder_stage != admission.stage_identity {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::StageMismatch,
                "replacement admission does not belong to the current holder stage",
            ));
        }
        if binding.contract.contract().identity() != admission.contract.contract().identity()
            || binding.contract.owner() != admission.contract.owner()
        {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::ArtifactContractMismatch,
                "replacement admission names a different installed artifact contract",
            ));
        }
        Ok(())
    }

    fn validate_binding(
        &self,
        admission: &WorthQueryArtifactTransferAdmission,
    ) -> Result<(), WorthQueryArtifactDenial> {
        let binding = self.owner.binding();
        if binding.domain_authority.runtime_authority()
            != admission.domain_authority.runtime_authority()
        {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::ForeignRuntime,
                "artifact belongs to a different Query runtime",
            ));
        }
        if !binding
            .domain_authority
            .is_current_installation_generation()
            || binding.domain_authority.installation_generation()
                != admission.domain_authority.installation_generation()
        {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::StaleInstallationGeneration,
                "artifact belongs to a stale installation generation",
            ));
        }
        if binding.operation_identity != admission.operation_identity
            || binding.binding_identity != admission.binding_identity
        {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::OperationMismatch,
                "artifact belongs to a different installed operation binding",
            ));
        }
        if binding.run_identity != admission.run_identity {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::RunMismatch,
                "artifact belongs to a different workflow run",
            ));
        }
        if binding.basis_identity != admission.basis_identity {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::BasisMismatch,
                "artifact belongs to a different admitted basis",
            ));
        }
        if self.holder_stage != admission.predecessor_stage {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::StageMismatch,
                "artifact is not held by the declared predecessor stage",
            ));
        }
        if binding.contract.contract().identity()
            != admission.expected_contract.contract().identity()
            || binding.contract.owner() != admission.expected_contract.owner()
        {
            return Err(self.denial(
                WorthQueryArtifactDenialKind::ArtifactContractMismatch,
                "artifact contract does not match the installed consumer edge",
            ));
        }
        Ok(())
    }

    pub(super) fn contract_matches(
        &self,
        reference: &worth_query_installation::facade::WorthQueryArtifactContractReference,
    ) -> bool {
        let contract = self.owner.binding().contract.contract();
        contract.family() == reference.family()
            && contract.schema_version() == reference.schema_version()
            && contract.protocol_version() == reference.protocol_version()
    }

    fn denial(
        &self,
        kind: WorthQueryArtifactDenialKind,
        detail: &'static str,
    ) -> WorthQueryArtifactDenial {
        WorthQueryArtifactDenial::new(
            kind,
            Some(self.owner.binding().contract.contract().family().as_str()),
            detail,
        )
    }
}
