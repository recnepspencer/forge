use super::LayoutOwnerObservationLedger;
use crate::courtroom::layout::executed_evidence::LayoutExecutedEvidenceKind as Evidence;

impl LayoutOwnerObservationLedger {
    pub fn record_restored_layout_materialization(
        &mut self,
        observed: worth_store_operations::RestoredLayoutMaterializationObservation,
    ) {
        self.record(
            super::LayoutOwnerFamily::RestoredLayoutMaterialization,
            observed.case_id().as_str(),
        );
    }

    pub fn record_lsm_membership(
        &mut self,
        observed: worth_store_lsm_authority::LsmMembershipOwnerCaseObservation,
    ) {
        use worth_store_lsm_authority::LsmMembershipOperation;
        let id = observed.id();
        let family = match id.operation() {
            LsmMembershipOperation::Open => super::LayoutOwnerFamily::LsmMembershipOpen,
            LsmMembershipOperation::PersistRecord => {
                super::LayoutOwnerFamily::LsmMembershipPersistRecord
            }
            LsmMembershipOperation::SelectCompaction => {
                super::LayoutOwnerFamily::LsmMembershipSelectCompaction
            }
            LsmMembershipOperation::ReplaceMembership => {
                super::LayoutOwnerFamily::LsmMembershipReplace
            }
            LsmMembershipOperation::LookupPublishedReplacement => {
                super::LayoutOwnerFamily::LsmMembershipPublishedLookup
            }
        };
        if id.operation() == LsmMembershipOperation::SelectCompaction
            && id.disposition()
                == worth_store_lsm_authority::LsmMembershipDisposition::Denied(
                    worth_store_lsm_authority::LsmMembershipDenial::TombstoneRecordRequired,
                )
        {
            self.record_executed_evidence(Evidence::LsmTombstoneRequired);
        }
        if id.operation() == LsmMembershipOperation::Open
            && id.disposition()
                == worth_store_lsm_authority::LsmMembershipDisposition::Denied(
                    worth_store_lsm_authority::LsmMembershipDenial::PersistedMembershipArtifactInvalid,
                )
        {
            self.record_executed_evidence(Evidence::LsmCacheArtifactInvalid);
        }
        self.record(family, id.disposition().as_str());
    }

    pub fn record_lsm_execution(
        &mut self,
        observed: worth_store_layout_indexes::LsmExecutionOwnerCaseObservation,
    ) {
        use worth_store_layout_indexes::LsmExecutionOperation;
        let id = observed.id();
        let family = match id.operation() {
            LsmExecutionOperation::PrepareCompaction => {
                super::LayoutOwnerFamily::LsmCompactionPreparation
            }
            LsmExecutionOperation::BindPhysicalCompaction => {
                super::LayoutOwnerFamily::LsmPhysicalCompactionBinding
            }
            LsmExecutionOperation::PrepareMembershipActivation => {
                super::LayoutOwnerFamily::LsmMembershipActivation
            }
            LsmExecutionOperation::PublishCompaction => {
                super::LayoutOwnerFamily::LsmCompactionPublication
            }
            LsmExecutionOperation::ExecuteReplay => super::LayoutOwnerFamily::LsmReplayExecution,
        };
        self.record(family, id.disposition().as_str());
    }

    pub fn record_physical_compaction(
        &mut self,
        observed: worth_store_physical_isolation::CompactionOwnerCaseObservation,
    ) {
        self.record(
            super::LayoutOwnerFamily::PhysicalCompaction,
            observed.id().name(),
        );
    }
}
