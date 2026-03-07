use serde::{Deserialize, Serialize};

macro_rules! define_projected_handle {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        pub struct $name(u32);

        impl $name {
            pub const fn new(index: u32) -> Self {
                Self(index)
            }

            pub const fn index(self) -> usize {
                self.0 as usize
            }

            pub const fn raw(self) -> u32 {
                self.0
            }
        }
    };
}

define_projected_handle!(ProjectedBodyId);
define_projected_handle!(ProjectedLumpId);
define_projected_handle!(ProjectedRegionId);
define_projected_handle!(ProjectedShellId);
define_projected_handle!(ProjectedFaceId);
define_projected_handle!(ProjectedLoopId);
define_projected_handle!(ProjectedHalfEdgeId);
define_projected_handle!(ProjectedEdgeId);
define_projected_handle!(ProjectedVertexId);
