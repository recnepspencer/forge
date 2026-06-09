#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PrimitiveConstructionCompoundWorkloadFamily {
    SimplexSolid,
    Orthotope,
    RegularPrism,
    RegularPyramid,
    SheetPatch,
    WireOpen,
}

impl PrimitiveConstructionCompoundWorkloadFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SimplexSolid => "simplex_solid",
            Self::Orthotope => "orthotope",
            Self::RegularPrism => "regular_prism",
            Self::RegularPyramid => "regular_pyramid",
            Self::SheetPatch => "sheet_patch",
            Self::WireOpen => "wire_open",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PrimitiveConstructionCompoundTopologyClass {
    ClosedSolid,
    OpenShell,
    OpenWire,
}

impl PrimitiveConstructionCompoundTopologyClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ClosedSolid => "closed_solid",
            Self::OpenShell => "open_shell",
            Self::OpenWire => "open_wire",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PrimitiveConstructionCompoundRowClass {
    DirectStable,
    EscalatedStableExactSupport,
    StructuredRealizationExhaustion,
    BoundaryDriftGuardCase,
    MotionStableRelocation,
    MotionHostileReorientation,
    PreBooleanGrazingCase,
}

impl PrimitiveConstructionCompoundRowClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DirectStable => "direct_stable",
            Self::EscalatedStableExactSupport => "escalated_stable_exact_support",
            Self::StructuredRealizationExhaustion => "structured_realization_exhaustion",
            Self::BoundaryDriftGuardCase => "boundary_drift_guard_case",
            Self::MotionStableRelocation => "motion_stable_relocation",
            Self::MotionHostileReorientation => "motion_hostile_reorientation",
            Self::PreBooleanGrazingCase => "pre_boolean_grazing_case",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PrimitiveConstructionCompoundMotionKind {
    Move,
    Reorient,
    Offset,
}

impl PrimitiveConstructionCompoundMotionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Move => "move",
            Self::Reorient => "reorient",
            Self::Offset => "offset",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum PrimitiveConstructionCompoundGrazingKind {
    NearFrameNormalAlignment,
    NearReferenceAnchorDistance,
}

impl PrimitiveConstructionCompoundGrazingKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::NearFrameNormalAlignment => "near_frame_normal_alignment",
            Self::NearReferenceAnchorDistance => "near_reference_anchor_distance",
        }
    }
}
