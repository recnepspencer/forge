#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarLocalFrameDenialKind {
    MissingFrameIdentity,
    MissingMovementRotationPostureIdentity,
    MissingTolerancePolicyIdentity,
    MissingTransformChainDigest,
    MissingPrecisionReceipt,
    MissingLocalFeatureScaleOrder,
    MissingWorldMagnitudeOrder,
    PrecisionBasisMismatch,
    NonFiniteOrigin,
    InvalidNormal,
    InvalidLocalFeatureScaleOrder,
    InvalidWorldMagnitudeOrder,
    InvalidNormalizationScale,
    NormalizationScaleLocalFeatureMismatch,
    ContradictoryScaleSeparation,
    SemanticRotationInvalidatedPlanarClass,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarLocalFrameDenial {
    kind: PlanarLocalFrameDenialKind,
    reason: &'static str,
}

impl PlanarLocalFrameDenial {
    pub(crate) const fn new(kind: PlanarLocalFrameDenialKind, reason: &'static str) -> Self {
        Self { kind, reason }
    }

    pub fn kind(&self) -> PlanarLocalFrameDenialKind {
        self.kind
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}
