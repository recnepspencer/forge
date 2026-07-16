use super::{OwnerBoundaryBinding, OwnerOperationFamily, ProductionOwner, ProtocolFamily};
use crate::protocol_bindings::OwnerEvidenceClass;

pub(super) fn current() -> Vec<OwnerBoundaryBinding> {
    use OwnerEvidenceClass::{DurableAuthoritativeReceipt, EphemeralDiagnosticTrace};
    use OwnerOperationFamily::*;

    vec![
        OwnerBoundaryBinding::to::<worth_store_replication::ReplicationSourceAdmissionOutcome>(
            ProtocolFamily::ReplicationAdmission,
            ProductionOwner::Replication,
            ReplicationSourceAdmission,
            EphemeralDiagnosticTrace,
        ),
        OwnerBoundaryBinding::to::<worth_store_replication::ReplicationProgressOutcome>(
            ProtocolFamily::ReplicationAdmission,
            ProductionOwner::Replication,
            ReplicationProgressObservation,
            EphemeralDiagnosticTrace,
        ),
        OwnerBoundaryBinding::to::<worth_store_replication::ReplicationPublicationReadiness>(
            ProtocolFamily::ReplicationAdmission,
            ProductionOwner::Replication,
            ReplicationPublicationReadiness,
            EphemeralDiagnosticTrace,
        ),
        OwnerBoundaryBinding::to::<worth_store_replication::ReplicationPublicationOutcome>(
            ProtocolFamily::ReplicationAdmission,
            ProductionOwner::Replication,
            ReplicationPublicationCompletion,
            EphemeralDiagnosticTrace,
        ),
        OwnerBoundaryBinding::to::<worth_store_replication::PublishedReplication>(
            ProtocolFamily::ReplicationAdmission,
            ProductionOwner::Replication,
            ReplicationDurablePublication,
            DurableAuthoritativeReceipt,
        ),
        OwnerBoundaryBinding::to::<worth_store_replication::ReplicationPeerProgress>(
            ProtocolFamily::ReplicationAdmission,
            ProductionOwner::Replication,
            ReplicationPeerProgress,
            EphemeralDiagnosticTrace,
        ),
    ]
}
