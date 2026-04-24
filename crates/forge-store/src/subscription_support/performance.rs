use super::{
    classification_error, SubscriptionSupportAllocationScope, SubscriptionSupportDensityClass,
    SubscriptionSupportPlanFamily, SubscriptionSupportResultCostSurface,
};
use crate::failure::StoreError;
use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportProgramDensityClass {
    SingleSupportArtifact,
    FamilyLocalBatch,
    BasisLocalBatch,
    PortabilityScopeBatch,
    MaintenanceKeyBatch,
    StoreGlobalDebt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportPathClass {
    ForegroundResume,
    ForegroundRead,
    OperationalPlanning,
    MaintenanceExecution,
    ReplicationExport,
    ImportAdmission,
    OperatorReporting,
}

impl SupportPathClass {
    fn admits_operational_work(self) -> bool {
        !matches!(self, Self::ForegroundResume | Self::ForegroundRead)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SupportAllocationScope {
    NoAllocation,
    ActionLocal,
    FamilyLocalBatch,
    PortabilityManifest,
    OperatorReport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct SupportActionBreadthBudget {
    max_support_artifacts: u64,
    max_payload_header_bytes: u64,
}

impl SupportActionBreadthBudget {
    pub fn new(
        max_support_artifacts: u64,
        max_payload_header_bytes: u64,
    ) -> Result<Self, StoreError> {
        if max_support_artifacts == 0 || max_payload_header_bytes == 0 {
            return Err(classification_error(
                "subscription-support action breadth budgets must be non-zero",
            ));
        }
        Ok(Self {
            max_support_artifacts,
            max_payload_header_bytes,
        })
    }

    pub fn admits(&self, support_artifacts: u64, payload_header_bytes: u64) -> bool {
        support_artifacts <= self.max_support_artifacts
            && payload_header_bytes <= self.max_payload_header_bytes
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportBatchAdmissionReceipt {
    density_class: SupportProgramDensityClass,
    affected_entries: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum SupportBatchProofKind {
    CompatibilityReceipt,
    BasisEvidence,
    CursorCheckpointEvidence,
    PortabilityScopeEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportBatchReceiptReuseReport {
    density_class: SupportProgramDensityClass,
    affected_entries: u64,
    reused_proofs: Vec<SupportBatchProofKind>,
}

impl SupportBatchReceiptReuseReport {
    pub(crate) fn new(
        receipt: &SupportBatchAdmissionReceipt,
        reused_proofs: Vec<SupportBatchProofKind>,
    ) -> Result<Self, StoreError> {
        if reused_proofs.is_empty() {
            return Err(classification_error(
                "subscription-support batch receipt reuse must name at least one reused proof",
            ));
        }
        let unique = reused_proofs.iter().copied().collect::<BTreeSet<_>>();
        if unique.len() != reused_proofs.len() {
            return Err(classification_error(
                "subscription-support batch receipt reuse proofs must be unique",
            ));
        }
        Ok(Self {
            density_class: receipt.density_class(),
            affected_entries: receipt.affected_entries(),
            reused_proofs,
        })
    }

    pub fn density_class(&self) -> SupportProgramDensityClass {
        self.density_class
    }

    pub fn affected_entries(&self) -> u64 {
        self.affected_entries
    }

    pub fn reused_proofs(&self) -> &[SupportBatchProofKind] {
        &self.reused_proofs
    }
}

impl SupportBatchAdmissionReceipt {
    pub(crate) fn new(
        density_class: SupportProgramDensityClass,
        affected_entries: u64,
    ) -> Result<Self, StoreError> {
        if density_class == SupportProgramDensityClass::StoreGlobalDebt {
            return Err(classification_error(
                "store-global subscription-support batches are explicit debt and cannot produce admitted receipts",
            ));
        }
        if affected_entries == 0 {
            return Err(classification_error(
                "subscription-support batch admission receipts require affected entries",
            ));
        }
        Ok(Self {
            density_class,
            affected_entries,
        })
    }

    pub fn density_class(&self) -> SupportProgramDensityClass {
        self.density_class
    }

    pub fn affected_entries(&self) -> u64 {
        self.affected_entries
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SupportProgramPathPlan {
    path_class: SupportPathClass,
    density_class: SupportProgramDensityClass,
    allocation_scope: SupportAllocationScope,
    budget: SupportActionBreadthBudget,
    receipt: Option<SupportBatchAdmissionReceipt>,
}

impl SupportProgramPathPlan {
    pub(crate) fn new(
        path_class: SupportPathClass,
        density_class: SupportProgramDensityClass,
        allocation_scope: SupportAllocationScope,
        budget: SupportActionBreadthBudget,
        affected_entries: u64,
        payload_header_bytes: u64,
    ) -> Result<Self, StoreError> {
        if !path_class.admits_operational_work() {
            return Err(classification_error(
                "foreground subscription-support paths cannot admit operational work",
            ));
        }
        if density_class == SupportProgramDensityClass::StoreGlobalDebt {
            return Err(classification_error(
                "store-global subscription-support density is debt and cannot close Phase 1 admission",
            ));
        }
        if !budget.admits(affected_entries, payload_header_bytes) {
            return Err(classification_error(
                "subscription-support path plan exceeds its breadth budget before execution",
            ));
        }
        let receipt = Some(SupportBatchAdmissionReceipt::new(
            density_class,
            affected_entries,
        )?);
        Ok(Self {
            path_class,
            density_class,
            allocation_scope,
            budget,
            receipt,
        })
    }

    pub fn path_class(&self) -> SupportPathClass {
        self.path_class
    }

    pub fn density_class(&self) -> SupportProgramDensityClass {
        self.density_class
    }

    pub fn allocation_scope(&self) -> SupportAllocationScope {
        self.allocation_scope
    }

    pub fn batch_width(&self) -> u64 {
        self.receipt
            .as_ref()
            .map(SupportBatchAdmissionReceipt::affected_entries)
            .unwrap_or(0)
    }

    pub(crate) fn batch_receipt(&self) -> Option<&SupportBatchAdmissionReceipt> {
        self.receipt.as_ref()
    }
}

pub(crate) fn cost_surface_for_program_path(
    plan_family: SubscriptionSupportPlanFamily,
    path_plan: &SupportProgramPathPlan,
) -> SubscriptionSupportResultCostSurface {
    SubscriptionSupportResultCostSurface::new(
        plan_family,
        match path_plan.density_class() {
            SupportProgramDensityClass::SingleSupportArtifact => {
                SubscriptionSupportDensityClass::SparseIdentityClassification
            }
            SupportProgramDensityClass::FamilyLocalBatch => {
                SubscriptionSupportDensityClass::FamilyLocalBatch
            }
            SupportProgramDensityClass::BasisLocalBatch => {
                SubscriptionSupportDensityClass::BasisLocalBatch
            }
            SupportProgramDensityClass::PortabilityScopeBatch => {
                SubscriptionSupportDensityClass::PortabilityScopeBatch
            }
            SupportProgramDensityClass::MaintenanceKeyBatch => {
                SubscriptionSupportDensityClass::MaintenanceKeyBatch
            }
            SupportProgramDensityClass::StoreGlobalDebt => {
                SubscriptionSupportDensityClass::StoreGlobalDebt
            }
        },
        0,
        path_plan.batch_width(),
        0,
        match path_plan.allocation_scope() {
            SupportAllocationScope::NoAllocation => {
                SubscriptionSupportAllocationScope::NoAllocation
            }
            SupportAllocationScope::ActionLocal => SubscriptionSupportAllocationScope::ActionLocal,
            SupportAllocationScope::FamilyLocalBatch => {
                SubscriptionSupportAllocationScope::FamilyLocalBatch
            }
            SupportAllocationScope::PortabilityManifest => {
                SubscriptionSupportAllocationScope::PortabilityManifest
            }
            SupportAllocationScope::OperatorReport => {
                SubscriptionSupportAllocationScope::OperatorReport
            }
        },
    )
}
