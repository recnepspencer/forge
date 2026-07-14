use forge_store_lsm_authority::{
    select_lsm_compaction_membership, LsmCompactionMembership, LsmMembershipKey,
    LsmMembershipSession,
};
use forge_store_wal::{CheckpointDurablePublicationScope, StoreCheckpointRecordIdentity};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaselineLsmCompactionPlan {
    pub(super) membership: LsmCompactionMembership,
    membership_observation: BaselineLsmMembershipObservation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BaselineLsmMembershipObservation {
    partition_probes: u16,
    component_probes: u16,
}

impl BaselineLsmMembershipObservation {
    pub const fn partition_probes(self) -> u16 {
        self.partition_probes
    }

    pub const fn component_probes(self) -> u16 {
        self.component_probes
    }
}

impl BaselineLsmCompactionPlan {
    pub(crate) fn replay_membership(&self) -> LsmCompactionMembership {
        self.membership.clone()
    }

    pub(crate) fn lower_from_persisted(
        session: &LsmMembershipSession,
        key: LsmMembershipKey,
        admission: super::BaselineLsmCompactionAdmission,
    ) -> Result<Self, super::BaselineLsmExecutionAdmissionDenial> {
        if key.canonical() != admission.selected().request_identity().canonical_key() {
            return Err(super::BaselineLsmExecutionAdmissionDenial::SelectedOperationKeyMismatch);
        }
        let membership = select_lsm_compaction_membership(session, key)
            .into_result()
            .map_err(super::map_membership_denial)?;
        let membership_observation = BaselineLsmMembershipObservation {
            partition_probes: membership.partition_probes(),
            component_probes: membership.component_probes(),
        };
        Ok(Self {
            membership,
            membership_observation,
        })
    }

    pub const fn membership_observation(&self) -> BaselineLsmMembershipObservation {
        self.membership_observation
    }

    pub fn manifest_scope(
        &self,
        checkpoint: StoreCheckpointRecordIdentity,
        covered_lsn_start: u64,
        covered_lsn_end: u64,
    ) -> Option<CheckpointDurablePublicationScope> {
        self.membership
            .manifest_scope(checkpoint, covered_lsn_start, covered_lsn_end)
    }

    pub fn output_frame_digest(
        &self,
        physical: &forge_store_lsm_authority::LsmPhysicalCompactionIntent,
    ) -> String {
        self.membership.compaction_output_digest(
            physical.root_scope(),
            physical.target_epoch(),
            physical.manifest_epoch(),
        )
    }
}
