use super::WorthQueryWorkspace;

impl WorthQueryWorkspace {
    pub(crate) fn operating_world<L: crate::basis_lifecycle::BasisOperationLane>(
        &self,
        entry: crate::domain_installation::WorthQueryOperatingWorldEntry<L>,
    ) -> crate::domain_installation::WorthQueryInstalledOperatingWorld<'_, L> {
        crate::domain_installation::WorthQueryInstalledOperatingWorld::new(
            &self.runtime,
            entry.into_capability(),
        )
    }

    pub fn observe_operating_world(
        &self,
    ) -> Result<
        crate::domain_installation::WorthQueryInstalledOperatingWorld<
            '_,
            crate::basis_lifecycle::ObservationLaneWitness,
        >,
        crate::domain_installation::WorthQueryOperatingWorldEntryDenial,
    > {
        crate::domain_installation::WorthQueryOperatingWorldEntry::observe_current()
            .map(|entry| self.operating_world(entry))
    }

    pub fn observe_branch_operating_world(
        &self,
        branch_identity: crate::domain_installation::WorthQueryBranchHeadIdentity,
    ) -> Result<
        crate::domain_installation::WorthQueryInstalledOperatingWorld<
            '_,
            crate::basis_lifecycle::ObservationLaneWitness,
        >,
        crate::domain_installation::WorthQueryOperatingWorldEntryDenial,
    > {
        crate::domain_installation::WorthQueryOperatingWorldEntry::observe_branch(&branch_identity)
            .map(|entry| self.operating_world(entry))
    }

    pub fn prepare_mutation_operating_world(
        &self,
    ) -> Result<
        crate::domain_installation::WorthQueryInstalledOperatingWorld<
            '_,
            crate::basis_lifecycle::MutationPreparationLaneWitness,
        >,
        crate::domain_installation::WorthQueryOperatingWorldEntryDenial,
    > {
        crate::domain_installation::WorthQueryOperatingWorldEntry::prepare_current_mutation()
            .map(|entry| self.operating_world(entry))
    }

    pub fn prepare_branch_mutation_operating_world(
        &self,
        branch_identity: crate::domain_installation::WorthQueryBranchHeadIdentity,
    ) -> Result<
        crate::domain_installation::WorthQueryInstalledOperatingWorld<
            '_,
            crate::basis_lifecycle::MutationPreparationLaneWitness,
        >,
        crate::domain_installation::WorthQueryOperatingWorldEntryDenial,
    > {
        crate::domain_installation::WorthQueryOperatingWorldEntry::prepare_branch_mutation(
            &branch_identity,
        )
        .map(|entry| self.operating_world(entry))
    }

    pub fn graph_participation<G: 'static>(
        &self,
        marker: G,
    ) -> Result<
        crate::domain_installation::WorthQueryInstalledGraphParticipation<G>,
        crate::domain_installation::WorthQueryGraphParticipationLookupDenial,
    > {
        self.runtime.graph_participation(marker)
    }

    pub fn domain<D: 'static>(
        &self,
        marker: D,
    ) -> Result<
        crate::domain_installation::WorthQueryInstalledDomainHandle<D>,
        crate::domain_installation::WorthQueryDomainHandleDenial,
    > {
        self.runtime.domain(marker)
    }

    pub fn domain_installation_receipt<D: 'static>(
        &self,
        marker: D,
    ) -> Option<&crate::domain_installation::WorthQueryDomainInstallationReceipt> {
        self.runtime.domain_installation_receipt(marker)
    }

    pub fn verify_domain_execution_index_rebuild(
        &self,
    ) -> crate::domain_installation::WorthQueryDomainExecutionIndexRebuildReport {
        self.runtime.verify_domain_execution_index_rebuild()
    }

    pub fn rebuild_conditional_execution_index(
        &mut self,
    ) -> crate::domain_installation::WorthQueryConditionalExecutionIndexRebuildReport {
        self.runtime.rebuild_conditional_execution_index()
    }

    /// Rebind a prior runtime-installed domain handle into this workspace.
    ///
    /// The workspace remains the owning runtime boundary; downstream
    /// consumers never reconstruct installation generation or runtime
    /// affinity from receipt fields.
    pub fn rebind_domain<D: 'static>(
        &self,
        request: crate::domain_installation::WorthQueryDomainRebindRequest<D>,
    ) -> Result<
        crate::domain_installation::WorthQueryReboundDomainHandle<D>,
        crate::domain_installation::WorthQueryDomainRebindDenial,
    > {
        self.runtime.rebind_domain(request)
    }

    pub(crate) fn validate_installed_domain_witness<D: 'static>(
        &self,
        witness: &crate::domain_installation::WorthQueryInstalledDomainAuthorityWitness,
    ) -> Result<(), crate::domain_installation::WorthQueryDomainHandleDenial> {
        self.runtime.validate_installed_domain_witness::<D>(witness)
    }

    pub(crate) fn replace_domain_installation_with_successor_generation(
        &mut self,
    ) -> Result<(), crate::runtime::WorthQueryRuntimeError> {
        self.runtime
            .replace_domain_installation_with_successor_generation()
    }
}
