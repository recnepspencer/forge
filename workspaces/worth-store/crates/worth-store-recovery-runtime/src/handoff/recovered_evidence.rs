use worth_store::physical_runtime::StoreRecoveryBindingFreshnessSample;
use worth_store_recovery_physics::{PhysicalSourceSelection, RecoveryPlanningCounters};

use crate::entry::{
    PhysicalRecoveryPublicationCounters, PhysicalRecoveryPublicationSettlementLedger,
    PhysicalRecoveryReopenCounters, PhysicalRecoveryStagingCounters,
    PhysicalRecoveryStagingSettlementLedger, RecoveredRecoverySessionReceipt,
};
use crate::progression::{
    ClosedRecoveryStagingGeneration, PhysicalRecoveryDiscoveryCounters, RecoveryBaseImagePlan,
    RecoveryPublicationExpectation, RecoveryQuiescencePlan,
};

use super::{RecoveryCleanupPosture, RecoveryOperationFateSet};

pub(crate) struct RecoveredPhysicalRuntimeHandoffEvidence {
    pub(crate) session: RecoveredRecoverySessionReceipt,
    pub(crate) selection: PhysicalSourceSelection,
    pub(crate) discovery: PhysicalRecoveryDiscoveryCounters,
    pub(crate) freshness: StoreRecoveryBindingFreshnessSample,
    pub(crate) fates: RecoveryOperationFateSet,
    pub(crate) planning: RecoveryPlanningCounters,
    pub(crate) base: RecoveryBaseImagePlan,
    pub(crate) quiescence: RecoveryQuiescencePlan,
    pub(crate) closed: ClosedRecoveryStagingGeneration,
    pub(crate) staging: PhysicalRecoveryStagingCounters,
    pub(crate) staging_settlements: PhysicalRecoveryStagingSettlementLedger,
    pub(crate) publication_expectation: RecoveryPublicationExpectation,
    pub(crate) publication: PhysicalRecoveryPublicationCounters,
    pub(crate) publication_settlement: PhysicalRecoveryPublicationSettlementLedger,
    pub(crate) reopen: PhysicalRecoveryReopenCounters,
    pub(crate) cleanup: RecoveryCleanupPosture,
}
