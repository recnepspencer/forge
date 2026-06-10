#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PlanarRuntimeConcern {
    SupportMatrixAdmission,
    PredicateClassification,
    LocalFrameDerivation,
    CertifiedProjection,
    SegmentContactClassification,
    WindingContainment,
    SignedAreaDegeneracy,
    CoplanarOverlapExtraction,
    StructuralIdentity,
    MovementRotationPosture,
    RetainedFactReplay,
    ProjectionConsumption,
    RecoveryAction,
    DiagnosticsLocalization,
    BooleanReadinessBundle,
}

impl PlanarRuntimeConcern {
    pub const fn all() -> [Self; 15] {
        [
            Self::SupportMatrixAdmission,
            Self::PredicateClassification,
            Self::LocalFrameDerivation,
            Self::CertifiedProjection,
            Self::SegmentContactClassification,
            Self::WindingContainment,
            Self::SignedAreaDegeneracy,
            Self::CoplanarOverlapExtraction,
            Self::StructuralIdentity,
            Self::MovementRotationPosture,
            Self::RetainedFactReplay,
            Self::ProjectionConsumption,
            Self::RecoveryAction,
            Self::DiagnosticsLocalization,
            Self::BooleanReadinessBundle,
        ]
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportMatrixAdmission => "support-matrix-admission",
            Self::PredicateClassification => "predicate-classification",
            Self::LocalFrameDerivation => "local-frame-derivation",
            Self::CertifiedProjection => "certified-projection",
            Self::SegmentContactClassification => "segment-contact-classification",
            Self::WindingContainment => "winding-containment",
            Self::SignedAreaDegeneracy => "signed-area-degeneracy",
            Self::CoplanarOverlapExtraction => "coplanar-overlap-extraction",
            Self::StructuralIdentity => "structural-identity",
            Self::MovementRotationPosture => "movement-rotation-posture",
            Self::RetainedFactReplay => "retained-fact-replay",
            Self::ProjectionConsumption => "projection-consumption",
            Self::RecoveryAction => "recovery-action",
            Self::DiagnosticsLocalization => "diagnostics-localization",
            Self::BooleanReadinessBundle => "boolean-readiness-bundle",
        }
    }
}
