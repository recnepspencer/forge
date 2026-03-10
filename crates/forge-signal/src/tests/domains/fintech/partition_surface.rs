use crate::facade::NodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MarketPartition {
    Rates,
    Credit,
}

impl MarketPartition {
    pub(super) fn token(self) -> &'static str {
        match self {
            Self::Rates => "rates",
            Self::Credit => "credit",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PartitionDetail {
    Bucket0,
}

impl PartitionDetail {
    pub(super) fn token(self) -> &'static str {
        match self {
            Self::Bucket0 => "bucket-0",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct PartitionSurfaceNodes {
    pub market_regions: NodeId,
    pub rates_partition: NodeId,
    pub credit_partition: NodeId,
    pub rates_bucket_zero: NodeId,
    pub coarse_book: NodeId,
}
