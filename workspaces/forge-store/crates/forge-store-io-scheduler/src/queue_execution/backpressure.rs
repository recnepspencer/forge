#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueueBackpressureCause {
    QueueDepthSaturated,
    BandwidthSaturated,
    FlushDelayed,
    WriteBackWindowSaturated,
    ReadAheadDenied,
    BackgroundYielded,
    BackendTemporarilySaturated,
}
