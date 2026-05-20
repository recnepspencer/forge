#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialAuthoredActKind {
    Move,
    Rotate,
    Reorient,
    Offset,
    Place,
    Align,
    Constrain,
}

impl SpatialAuthoredActKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Move => "move",
            Self::Rotate => "rotate",
            Self::Reorient => "reorient",
            Self::Offset => "offset",
            Self::Place => "place",
            Self::Align => "align",
            Self::Constrain => "constrain",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialObservedRelationFact {
    GrazingContact,
    FrameAligned,
    InsideTarget,
    Overlap,
    HostFaceContact,
    HostPenetration,
}

impl SpatialObservedRelationFact {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::GrazingContact => "grazing_contact",
            Self::FrameAligned => "frame_aligned",
            Self::InsideTarget => "inside_target",
            Self::Overlap => "overlap",
            Self::HostFaceContact => "host_face_contact",
            Self::HostPenetration => "host_penetration",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialIntentConflictClass {
    SingleClearIntent,
    MultiplePlausibleIntents,
    UnsafeToAssume,
    BlockedCandidateSet,
}
