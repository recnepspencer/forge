use super::super::resource_budget::budget_from_shape;
use super::super::{
    QueueExecutionAdmissionDenial, QueueGroupingBasis, QueueWorkClass, QueueWorkDeclaration,
};
use super::PhysicalForegroundWorkDeclaration;

pub fn lower_physical_foreground_work(
    declaration: PhysicalForegroundWorkDeclaration,
) -> Result<QueueWorkDeclaration, QueueExecutionAdmissionDenial> {
    let PhysicalForegroundWorkDeclaration {
        reservation,
        locality,
        resources,
        durability,
        flush_epoch,
        recovery,
        writeback,
        secure_io,
    } = declaration;
    let security = reservation.security_scope_identity();
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
            QueueWorkClass::Foreground(reservation.lane()),
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
