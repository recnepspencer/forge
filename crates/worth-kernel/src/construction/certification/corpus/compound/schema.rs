#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionCompoundWorkloadFamily {
    SimplexSolid,
    Orthotope,
    RegularPrism,
    RegularPyramid,
    SheetPatch,
    WireOpen,
    MixedTopologyClassBatch,
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
            Self::MixedTopologyClassBatch => "mixed_topology_class_batch",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionCompoundTopologyClass {
    ClosedSolid,
    OpenShell,
    OpenWire,
    MixedBatch,
}

impl PrimitiveConstructionCompoundTopologyClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ClosedSolid => "closed_solid",
            Self::OpenShell => "open_shell",
            Self::OpenWire => "open_wire",
            Self::MixedBatch => "mixed_batch",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionCompoundRowClass {
    DirectStable,
    EscalatedStableLocalNormalized,
    EscalatedStableExactSupport,
    StructuredRealizationExhaustion,
    StructuredAdmissionRejection,
    BoundaryDriftGuardCase,
    MotionStableRelocation,
    MotionHostileReorientation,
    PreBooleanGrazingCase,
    MixedTopologyBatch,
}

impl PrimitiveConstructionCompoundRowClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DirectStable => "direct_stable",
            Self::EscalatedStableLocalNormalized => "escalated_stable_local_normalized",
            Self::EscalatedStableExactSupport => "escalated_stable_exact_support",
            Self::StructuredRealizationExhaustion => "structured_realization_exhaustion",
            Self::StructuredAdmissionRejection => "structured_admission_rejection",
            Self::BoundaryDriftGuardCase => "boundary_drift_guard_case",
            Self::MotionStableRelocation => "motion_stable_relocation",
            Self::MotionHostileReorientation => "motion_hostile_reorientation",
            Self::PreBooleanGrazingCase => "pre_boolean_grazing_case",
            Self::MixedTopologyBatch => "mixed_topology_batch",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveConstructionCompoundMotionKind {
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
pub enum PrimitiveConstructionCompoundGrazingKind {
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
