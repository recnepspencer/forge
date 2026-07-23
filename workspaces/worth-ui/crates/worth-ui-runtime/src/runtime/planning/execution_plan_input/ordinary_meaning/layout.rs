use crate::capability::{MosaicRegionKindDescriptor, SurfaceDescriptor};

use super::digest::fold_text;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiLayoutPlanMeaning {
    Region {
        descriptor: MosaicRegionKindDescriptor,
        child_range_identity: Option<String>,
    },
    Surface {
        descriptor: SurfaceDescriptor,
        child_range_identity: Option<String>,
    },
}

impl WorthUiLayoutPlanMeaning {
    pub(crate) fn region(
        descriptor: MosaicRegionKindDescriptor,
        child_range_identity: Option<String>,
    ) -> Self {
        Self::Region {
            descriptor,
            child_range_identity,
        }
    }

    pub(crate) fn surface(
        descriptor: SurfaceDescriptor,
        child_range_identity: Option<String>,
    ) -> Self {
        Self::Surface {
            descriptor,
            child_range_identity,
        }
    }

    pub(crate) fn child_range_identity(&self) -> Option<&str> {
        match self {
            Self::Region {
                child_range_identity,
                ..
            }
            | Self::Surface {
                child_range_identity,
                ..
            } => child_range_identity.as_deref(),
        }
    }

    pub(crate) fn semantic_digest(&self) -> u64 {
        match self {
            Self::Region { descriptor, .. } => {
                fold_text(0x7265_6769_6f6e_0001, descriptor.id().as_str())
            }
            Self::Surface { descriptor, .. } => {
                fold_text(0x7375_7266_6163_6501, descriptor.id().as_str())
            }
        }
    }
}
