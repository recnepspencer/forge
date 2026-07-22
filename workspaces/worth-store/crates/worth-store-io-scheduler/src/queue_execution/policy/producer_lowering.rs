use worth_store_buffer_pool::BufferPoolQueueExecutionDeclaration;
use worth_store_wal::WalQueueExecutionDeclaration;

use crate::foreground_reservation::ForegroundReservationReceipt;

use super::work::QueueProducerExecutionDeclaration;
use super::{QueueExecutionAdmissionDenial, QueueWorkDeclaration};

pub fn lower_wal_queue_declaration(
    declaration: WalQueueExecutionDeclaration,
    reservation: ForegroundReservationReceipt,
) -> Result<QueueWorkDeclaration, QueueExecutionAdmissionDenial> {
    let grouping_scope = declaration.grouping_scope();
    QueueProducerExecutionDeclaration::new(
        declaration.producer_kind(),
        declaration.resource_shape(),
        declaration.flush_epoch(),
        grouping_scope.tenant_scope(),
        grouping_scope.key_scope(),
        grouping_scope.authenticity_requirement(),
    )
    .lower_foreground(reservation)
}

pub fn lower_buffer_pool_queue_declaration(
    declaration: BufferPoolQueueExecutionDeclaration,
    reservation: ForegroundReservationReceipt,
) -> Result<QueueWorkDeclaration, QueueExecutionAdmissionDenial> {
    let security_identity = reservation.security_scope_identity();
    let grouping_scope = declaration.grouping_scope();
    if grouping_scope.security_scope_identity() != security_identity {
        return Err(QueueExecutionAdmissionDenial::ProducerSecurityScopeMismatch);
    }
    QueueProducerExecutionDeclaration::new(
        declaration.producer_kind(),
        declaration.resource_shape(),
        declaration.flush_epoch(),
        grouping_scope.tenant_scope(),
        grouping_scope.key_scope(),
        grouping_scope.authenticity_requirement(),
    )
    .lower_foreground(reservation)
    .map(|work| work.with_buffer_pool_declaration(declaration))
}
