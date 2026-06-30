#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum KernelCompiledProductFutureCutoverLane {
    SpatialCompiledProductConsumerCutover,
    ReplayUndoCompiledProductConsumerCutover,
    OrdinarySweepConsumerCutover,
    PublicCloseoutCompiledProductConsumerCutover,
    QueryProjectionConsumerCutover,
    QueryBoundaryEnvelopeConsumerCutover,
}

impl KernelCompiledProductFutureCutoverLane {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SpatialCompiledProductConsumerCutover => {
                "spatial-compiled-product-consumer-cutover"
            }
            Self::ReplayUndoCompiledProductConsumerCutover => {
                "replay-undo-compiled-product-consumer-cutover"
            }
            Self::OrdinarySweepConsumerCutover => "ordinary-sweep-consumer-cutover",
            Self::PublicCloseoutCompiledProductConsumerCutover => {
                "public-closeout-compiled-product-consumer-cutover"
            }
            Self::QueryProjectionConsumerCutover => "query-projection-consumer-cutover",
            Self::QueryBoundaryEnvelopeConsumerCutover => {
                "query-boundary-envelope-consumer-cutover"
            }
        }
    }
}
