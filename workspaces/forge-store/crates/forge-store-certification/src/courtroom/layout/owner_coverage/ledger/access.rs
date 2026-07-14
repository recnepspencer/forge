use super::{record_layout_observation, LayoutOwnerObservationLedger};
use crate::courtroom::layout::executed_evidence::LayoutExecutedEvidenceKind as Evidence;

impl LayoutOwnerObservationLedger {
    record_layout_observation!(
        record_artifact_family_admission,
        ArtifactFamilyAdmission,
        forge_store_layout_indexes::ArtifactFamilyAdmissionCaseId,
        as_str
    );
    pub fn record_physical_key_domain_admission(
        &mut self,
        observed: forge_store_layout_indexes::OwnerCaseObservation<
            forge_store_layout_indexes::PhysicalKeyDomainAdmissionCaseId,
        >,
    ) {
        let case = observed.case_id();
        if case.as_str() == "layout.key_domain.admission.denied.tenant_scope" {
            self.record_executed_evidence(Evidence::CrossTenantScopeDenied);
        }
        self.record(
            super::LayoutOwnerFamily::PhysicalKeyDomainAdmission,
            case.as_str(),
        );
    }
    record_layout_observation!(
        record_bootstrap_catalog_read,
        BootstrapCatalogRead,
        forge_store_layout_indexes::BootstrapCatalogReadCaseId,
        as_str
    );
    record_layout_observation!(
        record_access_plan_selection,
        AccessPlanSelection,
        forge_store_layout_indexes::AccessPlanSelectionCaseId,
        as_str
    );
    pub fn record_full_declared_scan(
        &mut self,
        observed: forge_store_layout_indexes::OwnerCaseObservation<
            forge_store_layout_indexes::FullDeclaredScanCaseId,
        >,
    ) {
        let case = observed.case_id();
        if case == forge_store_layout_indexes::FullDeclaredScanCaseId::HiddenBroadScanDenied {
            self.record_executed_evidence(Evidence::HiddenBroadScanDenied);
        }
        self.record(super::LayoutOwnerFamily::FullDeclaredScan, case.as_str());
    }

    pub fn record_btree_lookup_readiness(
        &mut self,
        observed: forge_store_layout_indexes::OwnerCaseObservation<
            forge_store_layout_indexes::BTreeLookupReadinessCaseId,
        >,
    ) {
        let case = observed.case_id();
        if case.name() == "layout.btree_lookup.readiness.stale" {
            self.record_executed_evidence(Evidence::BTreeReadinessStale);
        }
        self.record(super::LayoutOwnerFamily::BTreeLookupReadiness, case.name());
    }

    pub fn record_btree_lookup_execution(
        &mut self,
        observed: forge_store_layout_indexes::OwnerCaseObservation<
            forge_store_layout_indexes::BTreeLookupExecutionCaseId,
        >,
    ) {
        use forge_store_layout_indexes::{
            BTreeLookupExecutionCaseId as Case, BTreeSeparatorPartitionDenial as Partition,
            BaselineBTreeExecutionDenialKind as Denial,
        };
        let case = observed.case_id();
        match case {
            Case::Denied(Denial::Physical) => {
                self.record_executed_evidence(Evidence::BTreePhysicalReadDenied);
            }
            Case::Denied(Denial::SeparatorPartition(Partition::LeafSlotsNotCanonical)) => {
                self.record_executed_evidence(Evidence::BTreeLeafOrderDenied);
            }
            Case::Denied(Denial::SeparatorPartition(Partition::LeftChildCrossesSeparator)) => {
                self.record_executed_evidence(Evidence::BTreeLeftPartitionDenied);
            }
            Case::Denied(Denial::SeparatorPartition(Partition::RightChildPrecedesSeparator)) => {
                self.record_executed_evidence(Evidence::BTreeRightPartitionDenied);
            }
            _ => {}
        }
        self.record(super::LayoutOwnerFamily::BTreeLookupExecution, case.name());
    }
    record_layout_observation!(
        record_btree_replay_execution,
        BTreeReplayExecution,
        forge_store_layout_indexes::BTreeReplayCaseId,
        as_str
    );
    record_layout_observation!(
        record_degraded_scan_readiness,
        DegradedScanReadiness,
        forge_store_layout_indexes::DegradedScanReadinessCaseId,
        name
    );
    record_layout_observation!(
        record_lsm_lookup_readiness,
        LsmLookupReadiness,
        forge_store_layout_indexes::BaselineLsmLookupAdmissionCaseId,
        name
    );
    record_layout_observation!(
        record_lsm_lookup_execution,
        LsmLookupExecution,
        forge_store_layout_indexes::BaselineLsmLookupCaseId,
        name
    );
    record_layout_observation!(
        record_imported_blob_read_admission,
        ImportedBlobReadAdmission,
        forge_store_layout_indexes::ImportedBlobReadAdmissionCaseId,
        as_str
    );

    pub fn record_pre_execution_budget_admission(
        &mut self,
        observed: forge_store_budgets::PreExecutionBudgetAdmissionObservation,
    ) {
        self.record(
            super::LayoutOwnerFamily::PreExecutionBudgetAdmission,
            observed.case_id().as_str(),
        );
    }
}
