#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalRecoveryPublicationDisposition {
    Readmitted,
    RejectedByAuthority,
    Abandoned,
    RetainedForForensics,
}

impl TerminalRecoveryPublicationDisposition {
    pub(crate) const fn from_tag(tag: u8) -> Option<Self> {
        match tag {
            1 => Some(Self::Readmitted),
            2 => Some(Self::RejectedByAuthority),
            3 => Some(Self::Abandoned),
            4 => Some(Self::RetainedForForensics),
            _ => None,
        }
    }

    pub(crate) const fn tag(self) -> u8 {
        match self {
            Self::Readmitted => 1,
            Self::RejectedByAuthority => 2,
            Self::Abandoned => 3,
            Self::RetainedForForensics => 4,
        }
    }

    const fn fence_disposition(self) -> worth_store_authority::RecoveryWriteFenceDisposition {
        match self {
            Self::Readmitted => worth_store_authority::RecoveryWriteFenceDisposition::Readmitted,
            Self::RejectedByAuthority => {
                worth_store_authority::RecoveryWriteFenceDisposition::RejectedByAuthority
            }
            Self::Abandoned => worth_store_authority::RecoveryWriteFenceDisposition::Abandoned,
            Self::RetainedForForensics => {
                worth_store_authority::RecoveryWriteFenceDisposition::RetainedForForensics
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TerminalRecoveryFenceReleaseHandle {
    operation_id: super::OperationalOperationId,
    authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
    binding: super::control_record::RecoveryPublicationControlBinding,
    disposition: TerminalRecoveryPublicationDisposition,
    disposition_basis: [u8; 32],
}

impl TerminalRecoveryFenceReleaseHandle {
    pub(crate) const fn new(
        operation_id: super::OperationalOperationId,
        authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
        binding: super::control_record::RecoveryPublicationControlBinding,
        disposition: TerminalRecoveryPublicationDisposition,
        disposition_basis: [u8; 32],
    ) -> Self {
        Self {
            operation_id,
            authority_identity,
            binding,
            disposition,
            disposition_basis,
        }
    }

    pub const fn operation_id(&self) -> &super::OperationalOperationId {
        &self.operation_id
    }
    pub const fn disposition(&self) -> TerminalRecoveryPublicationDisposition {
        self.disposition
    }
    pub const fn disposition_basis(&self) -> [u8; 32] {
        self.disposition_basis
    }
    pub fn reconcile(
        &self,
        control: &impl super::OperationalControlStorePort,
        port: &impl worth_store_authority::RecoveryWriteFencePort,
    ) -> Result<
        worth_store_authority::RecoveryWriteFenceReleaseReceipt,
        TerminalRecoveryFenceReleaseDenial,
    > {
        let receipt =
            worth_store_authority::RecoveryCutoverAuthorityOwner::release_terminal_write_fence(
                self.binding.fence_identity(),
                self.binding.fence_plan_fingerprint(),
                self.disposition.fence_disposition(),
                port,
            )
            .map_err(TerminalRecoveryFenceReleaseDenial::Fence)?;
        super::OperationalControlStorePort::append(
            control,
            &super::OperationalControlRecord::recovery_publication_fence_released(
                self.authority_identity,
                self.operation_id.clone(),
                self.binding.publication_identity(),
                self.binding.fence_identity(),
                self.binding.fence_plan_fingerprint(),
                self.disposition.tag(),
            ),
        )
        .map_err(TerminalRecoveryFenceReleaseDenial::Control)?;
        Ok(receipt)
    }
}

#[derive(Debug)]
pub enum TerminalRecoveryFenceReleaseDenial {
    Fence(worth_store_authority::RecoveryWriteFenceDenial),
    Control(super::OperationalControlAppendDenial),
}
