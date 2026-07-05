#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SecureIoOperation {
    ForegroundReservation,
    QueueGrouping,
    BatchedWrite,
    ReadAhead,
    WriteBack,
    BackgroundLease,
    RepairScan,
    VerificationPressure,
    BackendExecution,
}
