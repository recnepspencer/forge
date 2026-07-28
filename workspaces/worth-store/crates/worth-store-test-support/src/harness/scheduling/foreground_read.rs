use worth_store_contracts::QueueProducerResourceShape;
use worth_store_io_scheduler::foreground_reservation::ForegroundReservationReceipt;
use worth_store_io_scheduler::{
    lower_physical_foreground_work, PhysicalForegroundWorkDeclaration,
    QueueExecutionAdmissionDenial, QueueLocalityIdentity, QueueWorkDeclaration,
};

pub fn scheduler_foreground_read_work(
    reservation: ForegroundReservationReceipt,
    flush_epoch: u64,
    resources: QueueProducerResourceShape,
) -> Result<QueueWorkDeclaration, QueueExecutionAdmissionDenial> {
    lower_physical_foreground_work(PhysicalForegroundWorkDeclaration::read(
        reservation,
        QueueLocalityIdentity::from_digest([93; 32]),
        resources,
        flush_epoch,
    ))
}
