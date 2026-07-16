#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepairRecoveryTopology {
    CurrentAuthorityPreserving,
    NonCurrentAuthorityAffecting,
}

impl RepairRecoveryTopology {
    pub(crate) const fn tag(self) -> u8 {
        match self {
            Self::CurrentAuthorityPreserving => 1,
            Self::NonCurrentAuthorityAffecting => 2,
        }
    }
    pub(crate) const fn from_tag(tag: u8) -> Option<Self> {
        match tag {
            1 => Some(Self::CurrentAuthorityPreserving),
            2 => Some(Self::NonCurrentAuthorityAffecting),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveredRepairOwnerReceipt {
    node_fingerprint: [u8; 32],
    receipt_fingerprint: [u8; 32],
    owner_tag: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecoveredRepairOwnerStart {
    node_fingerprint: [u8; 32],
    owner_tag: u8,
}

impl RecoveredRepairOwnerStart {
    pub(crate) const fn new(node_fingerprint: [u8; 32], owner_tag: u8) -> Self {
        Self {
            node_fingerprint,
            owner_tag,
        }
    }
    pub const fn node_fingerprint(self) -> [u8; 32] {
        self.node_fingerprint
    }
    pub const fn owner_tag(self) -> u8 {
        self.owner_tag
    }
}

impl RecoveredRepairOwnerReceipt {
    pub(crate) const fn new(
        node_fingerprint: [u8; 32],
        receipt_fingerprint: [u8; 32],
        owner_tag: u8,
    ) -> Self {
        Self {
            node_fingerprint,
            receipt_fingerprint,
            owner_tag,
        }
    }
    pub const fn node_fingerprint(self) -> [u8; 32] {
        self.node_fingerprint
    }
    pub const fn receipt_fingerprint(self) -> [u8; 32] {
        self.receipt_fingerprint
    }
    pub const fn owner_tag(self) -> u8 {
        self.owner_tag
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepairResumePreconditions {
    authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
    authorization_identity: [u8; 32],
    plan_fingerprint: [u8; 32],
}

impl RepairResumePreconditions {
    pub const fn authority_identity(self) -> worth_store_authority::StoreCurrentAuthorityIdentity {
        self.authority_identity
    }
    pub const fn authorization_identity(self) -> [u8; 32] {
        self.authorization_identity
    }
    pub const fn plan_fingerprint(self) -> [u8; 32] {
        self.plan_fingerprint
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepairRecoveryDisposition {
    SafeToAbandonBeforeMutation,
    NonCurrentResidueRemainsIsolated { durable_owner_effects: u64 },
    CurrentAuthorityResumeRequired { durable_owner_effects: u64 },
}

#[derive(Debug)]
pub enum RepairRecoveryDispositionDenial {
    StaleAuthority,
    InvalidBasis,
    MutationAlreadyRequiresResume,
    ResidueIsNotNonCurrent,
    Control(super::OperationalControlAppendDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepairRecoveryStopReceipt {
    disposition: RepairRecoveryDisposition,
    basis: [u8; 32],
}

impl RepairRecoveryStopReceipt {
    pub const fn disposition(self) -> RepairRecoveryDisposition {
        self.disposition
    }
    pub const fn basis(self) -> [u8; 32] {
        self.basis
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndeterminateRepairRecoveryHandle {
    operation_id: super::OperationalOperationId,
    authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
    authorization_identity: [u8; 32],
    plan_fingerprint: [u8; 32],
    expected_owner_nodes: u64,
    topology: RepairRecoveryTopology,
    started_owner_nodes: Vec<RecoveredRepairOwnerStart>,
    durable_owner_receipts: Vec<RecoveredRepairOwnerReceipt>,
}

impl IndeterminateRepairRecoveryHandle {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        operation_id: super::OperationalOperationId,
        authority_identity: worth_store_authority::StoreCurrentAuthorityIdentity,
        authorization_identity: [u8; 32],
        plan_fingerprint: [u8; 32],
        expected_owner_nodes: u64,
        topology: RepairRecoveryTopology,
        started_owner_nodes: Vec<RecoveredRepairOwnerStart>,
        durable_owner_receipts: Vec<RecoveredRepairOwnerReceipt>,
    ) -> Self {
        Self {
            operation_id,
            authority_identity,
            authorization_identity,
            plan_fingerprint,
            expected_owner_nodes,
            topology,
            started_owner_nodes,
            durable_owner_receipts,
        }
    }
    pub const fn operation_id(&self) -> &super::OperationalOperationId {
        &self.operation_id
    }
    pub const fn plan_fingerprint(&self) -> [u8; 32] {
        self.plan_fingerprint
    }
    pub const fn authorization_identity(&self) -> [u8; 32] {
        self.authorization_identity
    }
    pub const fn expected_owner_nodes(&self) -> u64 {
        self.expected_owner_nodes
    }
    pub const fn topology(&self) -> RepairRecoveryTopology {
        self.topology
    }
    pub const fn resume_preconditions(&self) -> RepairResumePreconditions {
        RepairResumePreconditions {
            authority_identity: self.authority_identity,
            authorization_identity: self.authorization_identity,
            plan_fingerprint: self.plan_fingerprint,
        }
    }
    pub fn durable_owner_receipts(&self) -> &[RecoveredRepairOwnerReceipt] {
        &self.durable_owner_receipts
    }
    pub fn started_owner_nodes(&self) -> &[RecoveredRepairOwnerStart] {
        &self.started_owner_nodes
    }
    pub fn unapplied_owner_nodes(&self) -> u64 {
        self.expected_owner_nodes
            .saturating_sub(self.durable_owner_receipts.len() as u64)
    }
    pub fn recovery_disposition(&self) -> RepairRecoveryDisposition {
        let durable_mutations = self
            .started_owner_nodes
            .iter()
            .filter(|started| started.owner_tag != 2)
            .count() as u64;
        if durable_mutations == 0 {
            RepairRecoveryDisposition::SafeToAbandonBeforeMutation
        } else if self.topology == RepairRecoveryTopology::NonCurrentAuthorityAffecting {
            RepairRecoveryDisposition::NonCurrentResidueRemainsIsolated {
                durable_owner_effects: durable_mutations,
            }
        } else {
            RepairRecoveryDisposition::CurrentAuthorityResumeRequired {
                durable_owner_effects: durable_mutations,
            }
        }
    }

    pub fn abandon_before_mutation(
        &self,
        control: &impl super::OperationalControlStorePort,
        current: &worth_store_authority::StoreCurrentAuthorityWitness,
        reason_identity: [u8; 32],
    ) -> Result<RepairRecoveryStopReceipt, RepairRecoveryDispositionDenial> {
        if current.authority_identity() != self.authority_identity {
            return Err(RepairRecoveryDispositionDenial::StaleAuthority);
        }
        if reason_identity == [0; 32] {
            return Err(RepairRecoveryDispositionDenial::InvalidBasis);
        }
        if self.recovery_disposition() != RepairRecoveryDisposition::SafeToAbandonBeforeMutation {
            return Err(RepairRecoveryDispositionDenial::MutationAlreadyRequiresResume);
        }
        self.persist_stop(
            control,
            super::OperationalTransitionId::repair_recovery_abandoned(),
            2,
            reason_identity,
        )?;
        Ok(RepairRecoveryStopReceipt {
            disposition: RepairRecoveryDisposition::SafeToAbandonBeforeMutation,
            basis: reason_identity,
        })
    }

    pub fn retain_isolated_non_current_residue(
        &self,
        control: &impl super::OperationalControlStorePort,
        current: &worth_store_authority::StoreCurrentAuthorityWitness,
        isolation_policy_identity: [u8; 32],
    ) -> Result<RepairRecoveryStopReceipt, RepairRecoveryDispositionDenial> {
        if current.authority_identity() != self.authority_identity {
            return Err(RepairRecoveryDispositionDenial::StaleAuthority);
        }
        if isolation_policy_identity == [0; 32] {
            return Err(RepairRecoveryDispositionDenial::InvalidBasis);
        }
        let disposition = self.recovery_disposition();
        if !matches!(
            disposition,
            RepairRecoveryDisposition::NonCurrentResidueRemainsIsolated { .. }
        ) {
            return Err(RepairRecoveryDispositionDenial::ResidueIsNotNonCurrent);
        }
        self.persist_stop(
            control,
            super::OperationalTransitionId::repair_recovery_isolated(),
            4,
            isolation_policy_identity,
        )?;
        Ok(RepairRecoveryStopReceipt {
            disposition,
            basis: isolation_policy_identity,
        })
    }

    fn persist_stop(
        &self,
        control: &impl super::OperationalControlStorePort,
        transition: super::OperationalTransitionId,
        disposition_tag: u8,
        disposition_basis: [u8; 32],
    ) -> Result<(), RepairRecoveryDispositionDenial> {
        super::OperationalControlStorePort::append(
            control,
            &super::OperationalControlRecord::repair_disposition_recorded(
                self.authority_identity,
                self.operation_id.clone(),
                transition,
                self.plan_fingerprint,
                disposition_tag,
                disposition_basis,
            ),
        )
        .map(|_| ())
        .map_err(RepairRecoveryDispositionDenial::Control)
    }
}
