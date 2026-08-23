use super::{OwnerBoundaryBinding, OwnerOperationFamily, ProductionOwner, ProtocolFamily};
use crate::protocol_bindings::OwnerEvidenceClass;

pub(super) fn current() -> Vec<OwnerBoundaryBinding> {
    use OwnerEvidenceClass::{
        DurableAuthoritativeReceipt, EphemeralDiagnosticTrace, ReopenedObservedReceipt,
    };
    use OwnerOperationFamily::*;
    use ProductionOwner::*;
    use ProtocolFamily::{CompactionVisibility, LeaseReclaim};

    vec![
        OwnerBoundaryBinding::to::<worth_store_lsm_authority::LsmMembershipOwnerCaseObservation>(
            CompactionVisibility,
            LsmAuthority,
            LsmMembership,
            EphemeralDiagnosticTrace,
        ),
        OwnerBoundaryBinding::to::<worth_store_layout_indexes::LsmExecutionOwnerCaseObservation>(
            CompactionVisibility,
            LayoutIndexes,
            LsmCompactionExecution,
            EphemeralDiagnosticTrace,
        ),
        OwnerBoundaryBinding::to::<worth_store_layout_indexes::LsmMaintenanceOwnerCaseObservation>(
            CompactionVisibility,
            LayoutIndexes,
            LsmMaintenanceAdmission,
            EphemeralDiagnosticTrace,
        ),
        OwnerBoundaryBinding::to::<worth_store_physical_isolation::CompactionOwnerCaseObservation>(
            CompactionVisibility,
            PhysicalIsolation,
            PhysicalCompactionCutover,
            EphemeralDiagnosticTrace,
        ),
        OwnerBoundaryBinding::to::<worth_store_physical_isolation::CompactionMutationLaneReceipt>(
            CompactionVisibility,
            PhysicalIsolation,
            CompactionMutationOutcome,
            EphemeralDiagnosticTrace,
        ),
        OwnerBoundaryBinding::to::<worth_store_physical_isolation::CompactionCutoverStabilityProof>(
            CompactionVisibility,
            PhysicalIsolation,
            CompactionStability,
            ReopenedObservedReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_physical_isolation::CompactionRewritePublication>(
            CompactionVisibility,
            PhysicalIsolation,
            CompactionPublication,
            DurableAuthoritativeReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_physical_isolation::PhysicalPublicationReceipt>(
            CompactionVisibility,
            PhysicalIsolation,
            PhysicalPublication,
            DurableAuthoritativeReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_physical_isolation::PublicationCrashRecoveryOutcome>(
            CompactionVisibility,
            PhysicalIsolation,
            PublicationCrashRecovery,
            ReopenedObservedReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_physical_isolation::ReclaimEligibilityProof>(
            LeaseReclaim,
            PhysicalIsolation,
            ReclaimEligibility,
            DurableAuthoritativeReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_physical_isolation::DeferredReclaimReceipt>(
            LeaseReclaim,
            PhysicalIsolation,
            DeferredReclaim,
            EphemeralDiagnosticTrace,
        ),
        OwnerBoundaryBinding::to::<worth_store_physical_isolation::DrainedCompactionReclaim>(
            LeaseReclaim,
            PhysicalIsolation,
            ReclaimDrain,
            EphemeralDiagnosticTrace,
        ),
        OwnerBoundaryBinding::to::<worth_store_physical_isolation::CrashStableReclaimReuseFence>(
            LeaseReclaim,
            PhysicalIsolation,
            ReclaimReuseFence,
            DurableAuthoritativeReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_physical_isolation::GenerationAdvanceReceipt>(
            LeaseReclaim,
            PhysicalIsolation,
            GenerationAdvance,
            DurableAuthoritativeReceipt,
        ),
    ]
}
