#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarPrecisionBasisDenialKind {
    MissingLocalFrameIdentity,
    MissingTopologyBasisIdentity,
    MissingMovementRotationPostureIdentity,
    MissingTolerancePolicyIdentity,
    MissingPredicateReceipt,
    MissingLocalFeatureScaleOrder,
    MissingWorldMagnitudeOrder,
    PredicateBasisMismatch,
    InvalidLocalFeatureScaleOrder,
    InvalidWorldMagnitudeOrder,
    InvalidNormalizationScale,
    NormalizationScaleLocalFeatureMismatch,
    ContradictoryScaleSeparation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarPrecisionBasisDenial {
    kind: PlanarPrecisionBasisDenialKind,
    reason: &'static str,
}

impl PlanarPrecisionBasisDenial {
    pub(crate) const fn new(kind: PlanarPrecisionBasisDenialKind, reason: &'static str) -> Self {
        Self { kind, reason }
    }

    pub fn kind(&self) -> PlanarPrecisionBasisDenialKind {
        self.kind
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}
