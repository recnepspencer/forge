#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoSchedulerS6ReadinessDenial {
    MissingUnsupportedQosNonClaim,
    MissingLatchCounters,
    MissingReclaimCounters,
    MissingProtectedByteFootprint,
    LogOrMetricProjection,
    HardwareQueueDepthClaim,
    MediaQosClaim,
}
