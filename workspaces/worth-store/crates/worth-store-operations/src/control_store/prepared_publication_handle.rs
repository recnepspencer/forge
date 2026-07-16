use super::{
    OperationalControlRecord, OperationalControlStore, OperationalControlStorePort,
    OperationalOperationId, OperationalTransitionId, RecoveryPublicationControlBinding,
    RecoveryPublicationOperationKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AbandonedPreparedRecoveryPublication {
    publication_identity: [u8; 32],
    reason_identity: [u8; 32],
    fence_release: worth_store_authority::RecoveryWriteFenceReleaseReceipt,
}

impl AbandonedPreparedRecoveryPublication {
    pub const fn publication_identity(self) -> [u8; 32] {
        self.publication_identity
    }
    pub const fn reason_identity(self) -> [u8; 32] {
        self.reason_identity
    }
    pub const fn fence_release(self) -> worth_store_authority::RecoveryWriteFenceReleaseReceipt {
        self.fence_release
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedRecoveryPublicationHandle {
    operation_id: OperationalOperationId,
    authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
    operation_kind: RecoveryPublicationOperationKind,
    binding: RecoveryPublicationControlBinding,
}

impl PreparedRecoveryPublicationHandle {
    pub(crate) const fn new(
        operation_id: OperationalOperationId,
        authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
        operation_kind: RecoveryPublicationOperationKind,
        binding: RecoveryPublicationControlBinding,
    ) -> Self {
        Self {
            operation_id,
            authority_identity,
            operation_kind,
            binding,
        }
    }

    pub const fn operation_id(&self) -> &OperationalOperationId {
        &self.operation_id
    }
    pub const fn operation_kind(&self) -> RecoveryPublicationOperationKind {
        self.operation_kind
    }
    pub const fn publication_identity(&self) -> [u8; 32] {
        self.binding.publication_identity()
    }
    pub const fn publication_plan_fingerprint(&self) -> [u8; 32] {
        self.binding.publication_plan_fingerprint()
    }
    pub const fn candidate_media_identity(&self) -> [u8; 32] {
        self.binding.candidate_media_identity()
    }

    pub fn resume(
        self,
        exact_plan: worth_store_physical_isolation::RecoveryPublicationLoweredPlan,
        control: &OperationalControlStore,
        current: &worth_store_authority::StoreCurrentAuthorityWitness,
        fence_port: &impl worth_store_authority::RecoveryWriteFencePort,
    ) -> Result<
        worth_store_physical_isolation::AtomicRecoveryPublicationReceipt,
        crate::workflow::RecoveryCutoverExecutionDenial,
    > {
        self.validate_rebind(&exact_plan, current)?;
        let fence =
            worth_store_authority::RecoveryCutoverAuthorityOwner::recover_active_write_fence(
                current,
                self.binding.fence_identity(),
                self.binding.fence_plan_fingerprint(),
                self.binding.cutover_plan_fingerprint(),
                self.binding.candidate_media_identity(),
                fence_port,
            )
            .map_err(crate::workflow::RecoveryCutoverExecutionDenial::Fence)?;
        let publication =
            worth_store_physical_isolation::RecoveryPublicationOwner::publish(exact_plan, fence)
                .map_err(crate::workflow::RecoveryCutoverExecutionDenial::Publication)?;
        control
            .append(&OperationalControlRecord::recovery_publication_pending(
                self.authority_identity,
                self.operation_id,
                OperationalTransitionId::recovery_publication_published(),
                self.binding,
            ))
            .map_err(crate::workflow::RecoveryCutoverExecutionDenial::Control)?;
        Ok(publication)
    }

    pub fn complete_already_published(
        self,
        publication_directory: &std::path::Path,
        control: &OperationalControlStore,
        current: &worth_store_authority::StoreCurrentAuthorityWitness,
        fence_port: &impl worth_store_authority::RecoveryWriteFencePort,
    ) -> Result<
        worth_store_physical_isolation::AtomicRecoveryPublicationReceipt,
        crate::workflow::RecoveryCutoverExecutionDenial,
    > {
        if self.authority_identity != current.authority_identity() {
            return Err(crate::workflow::RecoveryCutoverExecutionDenial::StaleAuthority);
        }
        worth_store_authority::RecoveryCutoverAuthorityOwner::recover_active_write_fence(
            current,
            self.binding.fence_identity(),
            self.binding.fence_plan_fingerprint(),
            self.binding.cutover_plan_fingerprint(),
            self.binding.candidate_media_identity(),
            fence_port,
        )
        .map_err(crate::workflow::RecoveryCutoverExecutionDenial::Fence)?;
        let publication =
            worth_store_physical_isolation::RecoveryPublicationOwner::reopen_published_by_identity(
                worth_store_physical_isolation::ReopenRecoveryPublicationByIdentityRequest::new(
                    publication_directory,
                    self.binding.publication_identity(),
                    self.binding.publication_plan_fingerprint(),
                    self.binding.candidate_media_identity(),
                ),
            )
            .map_err(crate::workflow::RecoveryCutoverExecutionDenial::Publication)?;
        self.record_published(control)?;
        Ok(publication)
    }

    pub fn abandon_before_publication(
        self,
        publication_directory: &std::path::Path,
        reason_identity: [u8; 32],
        control: &OperationalControlStore,
        transition: OperationalTransitionId,
        current: &worth_store_authority::StoreCurrentAuthorityWitness,
        fence_port: &impl worth_store_authority::RecoveryWriteFencePort,
    ) -> Result<AbandonedPreparedRecoveryPublication, crate::workflow::RecoveryCutoverExecutionDenial>
    {
        if reason_identity == [0; 32] {
            return Err(crate::workflow::RecoveryCutoverExecutionDenial::InvalidDispositionBasis);
        }
        if self.authority_identity != current.authority_identity() {
            return Err(crate::workflow::RecoveryCutoverExecutionDenial::StaleAuthority);
        }
        let posture =
            worth_store_physical_isolation::RecoveryPublicationOwner::classify_publication_start(
                publication_directory,
                self.binding.publication_identity(),
                self.binding.publication_plan_fingerprint(),
                self.binding.candidate_media_identity(),
            )
            .map_err(crate::workflow::RecoveryCutoverExecutionDenial::Publication)?;
        if posture
            == worth_store_physical_isolation::RecoveryPublicationStartPosture::DurableLocatorPresent
        {
            return Err(crate::workflow::RecoveryCutoverExecutionDenial::PublicationMayHaveStarted);
        }
        let fence_release =
            worth_store_authority::RecoveryCutoverAuthorityOwner::release_recovered_write_fence(
                current,
                self.binding.fence_identity(),
                self.binding.fence_plan_fingerprint(),
                worth_store_authority::RecoveryWriteFenceDisposition::Abandoned,
                fence_port,
            )
            .map_err(crate::workflow::RecoveryCutoverExecutionDenial::Fence)?;
        control
            .append(&OperationalControlRecord::recovery_publication_disposition(
                self.authority_identity,
                self.operation_id,
                transition,
                self.binding.publication_identity(),
                3,
                reason_identity,
                current.authority_identity(),
            ))
            .map_err(crate::workflow::RecoveryCutoverExecutionDenial::Control)?;
        Ok(AbandonedPreparedRecoveryPublication {
            publication_identity: self.binding.publication_identity(),
            reason_identity,
            fence_release,
        })
    }

    fn validate_rebind(
        &self,
        exact_plan: &worth_store_physical_isolation::RecoveryPublicationLoweredPlan,
        current: &worth_store_authority::StoreCurrentAuthorityWitness,
    ) -> Result<(), crate::workflow::RecoveryCutoverExecutionDenial> {
        if self.authority_identity != current.authority_identity() {
            return Err(crate::workflow::RecoveryCutoverExecutionDenial::StaleAuthority);
        }
        if exact_plan.fingerprint() != self.binding.publication_plan_fingerprint()
            || exact_plan.publication_identity() != self.binding.publication_identity()
            || exact_plan.candidate_media_identity() != self.binding.candidate_media_identity()
        {
            return Err(
                crate::workflow::RecoveryCutoverExecutionDenial::Publication(
                    worth_store_physical_isolation::RecoveryPublicationDenial::InvalidBinding,
                ),
            );
        }
        Ok(())
    }

    fn record_published(
        self,
        control: &OperationalControlStore,
    ) -> Result<(), crate::workflow::RecoveryCutoverExecutionDenial> {
        control
            .append(&OperationalControlRecord::recovery_publication_pending(
                self.authority_identity,
                self.operation_id,
                OperationalTransitionId::recovery_publication_published(),
                self.binding,
            ))
            .map(|_| ())
            .map_err(crate::workflow::RecoveryCutoverExecutionDenial::Control)
    }
}
