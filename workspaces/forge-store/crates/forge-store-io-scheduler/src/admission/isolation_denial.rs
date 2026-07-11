#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoSchedulerIsolationAdmissionDenial {
    MissingUnsupportedQosNonClaim,
    MissingLatchCounters,
    MissingReclaimCounters,
    MissingProtectedByteFootprint,
    LogOrMetricProjection,
    HardwareQueueDepthClaim,
    MediaQosClaim,
}
