use worth_store_contracts::QueueProducerResourceShape;
use worth_store_security::StoreSecurityScopeIdentity;

use crate::foreground_reservation::ForegroundReservationReceipt;
use crate::SecureIoPreservationReceipt;

use super::{
    work::budget_from_shape, QueueDurabilityClass, QueueExecutionAdmissionDenial,
    QueueGroupingBasis, QueueLocalityIdentity, QueueRecoveryOrdering, QueueWorkDeclaration,
    QueueWritebackPolicy,
};

#[allow(clippy::too_many_arguments)]
pub fn lower_physical_foreground_work(
    reservation: ForegroundReservationReceipt,
    security: StoreSecurityScopeIdentity,
    locality: QueueLocalityIdentity,
    resources: QueueProducerResourceShape,
    durability: QueueDurabilityClass,
    flush_epoch: u64,
    recovery: QueueRecoveryOrdering,
    writeback: QueueWritebackPolicy,
    secure_io: Option<SecureIoPreservationReceipt>,
) -> Result<QueueWorkDeclaration, QueueExecutionAdmissionDenial> {
    if reservation.security_scope_identity() != security {
        return Err(QueueExecutionAdmissionDenial::ProducerSecurityScopeMismatch);
    }
    let mut work = QueueWorkDeclaration::foreground(
        reservation.execution_ready(),
        durability,
        budget_from_shape(resources)?,
    )
    .with_grouping_basis(
        QueueGroupingBasis::new(
            security,
            security.tenant_scope(),
            security.key_scope(),
            security.authenticity_requirement(),
            durability,
            flush_epoch,
            super::QueueWorkClass::Foreground(reservation.lane()),
            recovery,
            writeback,
        )
        .with_locality(locality),
    );
    if let Some(secure_io) = secure_io {
        work = work.with_secure_io_scope(secure_io);
    }
    Ok(work)
}
