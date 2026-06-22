mod canonical_segment;
mod canonical_segment_set;
mod denial;
mod identity;

pub use canonical_segment::PlanarBooleanCanonicalSegment;
pub use canonical_segment_set::PlanarBooleanCanonicalSegmentSet;
pub use denial::{
    PlanarBooleanCanonicalSegmentSetDenial, PlanarBooleanCanonicalSegmentSetDenialKind,
};
