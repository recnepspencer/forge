use super::lowered_authority::WorthGraphReadLoweredTouchedAuthority;
use super::source_family::WorthGraphReadTouchedAuthoritySourceFamily;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadTouchedAuthorityLoweringSummary {
    topology_lowered_count: usize,
    spatial_lowered_count: usize,
    lowered_record_count: usize,
}

impl WorthGraphReadTouchedAuthorityLoweringSummary {
    pub fn from_lowered_authorities<'a>(
        lowered_authorities: impl Iterator<Item = &'a WorthGraphReadLoweredTouchedAuthority>,
    ) -> Self {
        let mut topology_lowered_count = 0;
        let mut spatial_lowered_count = 0;
        let mut lowered_record_count = 0;

        for lowered_authority in lowered_authorities {
            lowered_record_count += 1;
            match lowered_authority.source_family() {
                WorthGraphReadTouchedAuthoritySourceFamily::TopologyClosure => {
                    topology_lowered_count += 1;
                }
                WorthGraphReadTouchedAuthoritySourceFamily::SpatialContinuation => {
                    spatial_lowered_count += 1;
                }
            }
        }

        Self {
            topology_lowered_count,
            spatial_lowered_count,
            lowered_record_count,
        }
    }

    pub const fn topology_lowered_count(&self) -> usize {
        self.topology_lowered_count
    }

    pub const fn spatial_lowered_count(&self) -> usize {
        self.spatial_lowered_count
    }

    pub const fn lowered_record_count(&self) -> usize {
        self.lowered_record_count
    }
}
