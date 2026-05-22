use super::facts::SpatialAuthoredActKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum SpatialIntentCandidate {
    MoveOnly,
    SnapFlush,
    AlignFrames,
    AttachRelationally,
    NestInside,
    MergeCandidate,
    SubtractCandidate,
    CutOpeningCandidate,
    JoinCandidate,
}

impl SpatialIntentCandidate {
    pub fn baseline_for(authored_act: SpatialAuthoredActKind) -> Self {
        match authored_act {
            SpatialAuthoredActKind::Move
            | SpatialAuthoredActKind::Offset
            | SpatialAuthoredActKind::Place => Self::MoveOnly,
            SpatialAuthoredActKind::Rotate
            | SpatialAuthoredActKind::Reorient
            | SpatialAuthoredActKind::Align
            | SpatialAuthoredActKind::Constrain => Self::AlignFrames,
        }
    }

    pub fn default_priority(self) -> u8 {
        match self {
            Self::MoveOnly => 10,
            Self::AlignFrames => 20,
            Self::SnapFlush => 40,
            Self::AttachRelationally => 50,
            Self::NestInside => 60,
            Self::JoinCandidate => 70,
            Self::MergeCandidate | Self::SubtractCandidate => 80,
            Self::CutOpeningCandidate => 90,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MoveOnly => "move_only",
            Self::SnapFlush => "snap_flush",
            Self::AlignFrames => "align_frames",
            Self::AttachRelationally => "attach_relationally",
            Self::NestInside => "nest_inside",
            Self::MergeCandidate => "merge_candidate",
            Self::SubtractCandidate => "subtract_candidate",
            Self::CutOpeningCandidate => "cut_opening_candidate",
            Self::JoinCandidate => "join_candidate",
        }
    }
}
