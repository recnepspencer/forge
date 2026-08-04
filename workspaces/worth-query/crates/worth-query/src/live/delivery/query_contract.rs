use super::super::promotion::LiveQueryFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DeliveryLocalityOutcome {
    InRegionRegion,
    InRegionRegionWithPeerWidening { peer_scopes: Vec<String> },
    InRegionPartition,
    InRegionPartitionWithPeerWidening { peer_scopes: Vec<String> },
    OffRegionSuppressed,
}

impl DeliveryLocalityOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InRegionRegion => "in_region_region",
            Self::InRegionRegionWithPeerWidening { .. } => "in_region_region_widened",
            Self::InRegionPartition => "in_region_partition",
            Self::InRegionPartitionWithPeerWidening { .. } => "in_region_partition_widened",
            Self::OffRegionSuppressed => "off_region_suppressed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryDeliveryContract {
    pub(in crate::live) digest: String,
    pub(in crate::live) query_digest: String,
    pub(in crate::live) locality_digest: String,
    pub(in crate::live) delivery_digest: String,
    pub(in crate::live) family: LiveQueryFamily,
    pub(in crate::live) locality_outcome: DeliveryLocalityOutcome,
}

impl QueryDeliveryContract {
    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn locality_digest(&self) -> &str {
        &self.locality_digest
    }

    pub fn delivery_digest(&self) -> &str {
        &self.delivery_digest
    }

    pub fn family(&self) -> &LiveQueryFamily {
        &self.family
    }

    pub fn locality_outcome(&self) -> &DeliveryLocalityOutcome {
        &self.locality_outcome
    }
}
