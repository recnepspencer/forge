use super::{OwnerBoundaryBinding, OwnerOperationFamily, ProductionOwner, ProtocolFamily};
use crate::protocol_bindings::OwnerEvidenceClass;

pub(super) fn current() -> Vec<OwnerBoundaryBinding> {
    use OwnerEvidenceClass::{
        DurableAuthoritativeReceipt, EphemeralDiagnosticTrace, ForbiddenAuthoritySubstitute,
        ReopenedObservedReceipt,
    };
    use OwnerOperationFamily::*;
    use ProductionOwner::*;
    use ProtocolFamily::{ImportPublication, QuarantineReadmission};

    vec![
        OwnerBoundaryBinding::to::<worth_store_physical_integrity::ExecutedQuarantineFinding>(
            QuarantineReadmission,
            PhysicalIntegrity,
            QuarantineFinding,
            EphemeralDiagnosticTrace,
        ),
        OwnerBoundaryBinding::to::<worth_store_physical_integrity::QuarantineRecord>(
            QuarantineReadmission,
            PhysicalIntegrity,
            QuarantineRecord,
            DurableAuthoritativeReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_physical_integrity::QuarantineReceipt>(
            QuarantineReadmission,
            PhysicalIntegrity,
            QuarantineEntry,
            DurableAuthoritativeReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_physical_integrity::QuarantineHandoffPosture>(
            QuarantineReadmission,
            PhysicalIntegrity,
            QuarantineHandoff,
            ForbiddenAuthoritySubstitute,
        ),
        OwnerBoundaryBinding::to::<
            worth_store_physical_integrity::RecoveryCorruptionReadmissionHandoff,
        >(
            QuarantineReadmission,
            PhysicalIntegrity,
            CorruptionReadmission,
            ReopenedObservedReceipt,
        ),
        OwnerBoundaryBinding::to::<
            worth_store_layout_indexes::integrity::RecoveryLayoutReadmissionOutcomeView<'static>,
        >(
            QuarantineReadmission,
            LayoutIndexes,
            LayoutReadmission,
            ReopenedObservedReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_operations::BackupImportCustodyReadmission>(
            ImportPublication,
            Operations,
            ImportCustodyReadmission,
            ReopenedObservedReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_operations::BackupExportCustodyAdmission>(
            ImportPublication,
            Operations,
            ExportCustodyAdmission,
            ReopenedObservedReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_security::StoreTrustBoundaryReadmissionTrigger>(
            ImportPublication,
            Security,
            TrustBoundaryReadmission,
            ForbiddenAuthoritySubstitute,
        ),
        OwnerBoundaryBinding::to::<worth_store_security::StoreReadmittedSecurityScope>(
            ImportPublication,
            Security,
            SecurityScopeReadmission,
            ReopenedObservedReceipt,
        ),
    ]
}
