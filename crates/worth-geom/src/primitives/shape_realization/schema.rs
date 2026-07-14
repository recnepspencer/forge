use crate::primitives::plane::Plane;
use crate::spatial::coordinate::local_space::ScaleAnalysis;

use super::geometry_identity::geometry_identity_bundle;
use super::private_support::{truth_digest_parts, PrimitiveDigestScope};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveRealizationStrategy {
    DirectWorld,
    LocalNormalized,
    ExactSupport,
}

impl PrimitiveRealizationStrategy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DirectWorld => "direct_world",
            Self::LocalNormalized => "local_normalized",
            Self::ExactSupport => "exact_support",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveStabilityClass {
    StableDirect,
    StableAfterEscalation,
    RejectedBelowConditioningFloor,
}

impl PrimitiveStabilityClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::StableDirect => "stable_direct",
            Self::StableAfterEscalation => "stable_after_escalation",
            Self::RejectedBelowConditioningFloor => "rejected_below_conditioning_floor",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveFeatureConditioningClass {
    Healthy,
    NearThreshold,
    Collapsed,
}

impl PrimitiveFeatureConditioningClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::NearThreshold => "near_threshold",
            Self::Collapsed => "collapsed",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveSupportNormalClass {
    Robust,
    NearDegenerate,
    Degenerate,
}

impl PrimitiveSupportNormalClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Robust => "robust",
            Self::NearDegenerate => "near_degenerate",
            Self::Degenerate => "degenerate",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimitiveNormalizationDisposition {
    WorldSpaceSufficient,
    LocalTransformationRecommended,
    LocalTransformationApplied,
}

impl PrimitiveNormalizationDisposition {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WorldSpaceSufficient => "world_space_sufficient",
            Self::LocalTransformationRecommended => "local_transformation_recommended",
            Self::LocalTransformationApplied => "local_transformation_applied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveConditioningWitness {
    coordinate_magnitude_bits: u64,
    feature_size_bits: u64,
    condition_number_bits: u64,
    machine_epsilon_bits: u64,
    precision_headroom_ratio_bits: u64,
    minimum_support_normal_magnitude_bits: u64,
    support_normal_headroom_ratio_bits: u64,
    normalization_scale_bits: u64,
    feature_size_collapsed: bool,
    needs_local_transform: bool,
    feature_conditioning_class: PrimitiveFeatureConditioningClass,
    support_normal_class: PrimitiveSupportNormalClass,
    normalization_disposition: PrimitiveNormalizationDisposition,
}

impl PrimitiveConditioningWitness {
    pub(crate) fn new(
        analysis: &ScaleAnalysis,
        precision_headroom_ratio: f64,
        minimum_support_normal_magnitude: f64,
        support_normal_headroom_ratio: f64,
        normalization_scale: f64,
        feature_size_collapsed: bool,
        feature_conditioning_class: PrimitiveFeatureConditioningClass,
        support_normal_class: PrimitiveSupportNormalClass,
        normalization_disposition: PrimitiveNormalizationDisposition,
    ) -> Self {
        Self {
            coordinate_magnitude_bits: analysis.get_coordinate_magnitude().to_bits(),
            feature_size_bits: analysis.get_feature_size().to_bits(),
            condition_number_bits: analysis.get_condition_number().to_bits(),
            machine_epsilon_bits: analysis.get_machine_epsilon_at_scale().to_bits(),
            precision_headroom_ratio_bits: precision_headroom_ratio.to_bits(),
            minimum_support_normal_magnitude_bits: minimum_support_normal_magnitude.to_bits(),
            support_normal_headroom_ratio_bits: support_normal_headroom_ratio.to_bits(),
            normalization_scale_bits: normalization_scale.to_bits(),
            feature_size_collapsed,
            needs_local_transform: analysis.get_needs_local_transform(),
            feature_conditioning_class,
            support_normal_class,
            normalization_disposition,
        }
    }

    pub fn coordinate_magnitude(&self) -> f64 {
        f64::from_bits(self.coordinate_magnitude_bits)
    }

    pub fn feature_size(&self) -> f64 {
        f64::from_bits(self.feature_size_bits)
    }

    pub fn condition_number(&self) -> f64 {
        f64::from_bits(self.condition_number_bits)
    }

    pub fn machine_epsilon_at_scale(&self) -> f64 {
        f64::from_bits(self.machine_epsilon_bits)
    }

    pub fn precision_headroom_ratio(&self) -> f64 {
        f64::from_bits(self.precision_headroom_ratio_bits)
    }

    pub fn minimum_support_normal_magnitude(&self) -> f64 {
        f64::from_bits(self.minimum_support_normal_magnitude_bits)
    }

    pub fn support_normal_headroom_ratio(&self) -> f64 {
        f64::from_bits(self.support_normal_headroom_ratio_bits)
    }

    pub fn normalization_scale_applied(&self) -> f64 {
        f64::from_bits(self.normalization_scale_bits)
    }

    pub fn feature_size_collapsed(&self) -> bool {
        self.feature_size_collapsed
    }

    pub fn needs_local_transform(&self) -> bool {
        self.needs_local_transform
    }

    pub fn feature_conditioning_class(&self) -> PrimitiveFeatureConditioningClass {
        self.feature_conditioning_class
    }

    pub fn support_normal_class(&self) -> PrimitiveSupportNormalClass {
        self.support_normal_class
    }

    pub fn normalization_disposition(&self) -> PrimitiveNormalizationDisposition {
        self.normalization_disposition
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimitiveRealizationReport {
    family: &'static str,
    strategy: PrimitiveRealizationStrategy,
    attempted_strategies: Vec<PrimitiveRealizationStrategy>,
    stability_class: PrimitiveStabilityClass,
    conditioning_witness: PrimitiveConditioningWitness,
    geometry_digest: String,
    report_digest: String,
}

impl PrimitiveRealizationReport {
    pub(crate) fn new(
        family: &'static str,
        strategy: PrimitiveRealizationStrategy,
        attempted_strategies: Vec<PrimitiveRealizationStrategy>,
        stability_class: PrimitiveStabilityClass,
        conditioning_witness: PrimitiveConditioningWitness,
        points: &[[f64; 3]],
        planes: &[Plane],
    ) -> Self {
        let geometry_digest = geometry_identity_bundle(planes, points)
            .realization_geometry_digest()
            .as_str()
            .to_string();
        let report_digest = digest_parts(&[
            family.to_string(),
            strategy.as_str().to_string(),
            attempted_strategies
                .iter()
                .map(|attempt| attempt.as_str())
                .collect::<Vec<_>>()
                .join("->"),
            stability_class.as_str().to_string(),
            conditioning_witness
                .coordinate_magnitude()
                .to_bits()
                .to_string(),
            conditioning_witness.feature_size().to_bits().to_string(),
            conditioning_witness
                .condition_number()
                .to_bits()
                .to_string(),
            conditioning_witness
                .machine_epsilon_at_scale()
                .to_bits()
                .to_string(),
            conditioning_witness
                .precision_headroom_ratio()
                .to_bits()
                .to_string(),
            conditioning_witness
                .minimum_support_normal_magnitude()
                .to_bits()
                .to_string(),
            conditioning_witness
                .support_normal_headroom_ratio()
                .to_bits()
                .to_string(),
            conditioning_witness
                .normalization_scale_applied()
                .to_bits()
                .to_string(),
            conditioning_witness.feature_size_collapsed().to_string(),
            conditioning_witness.needs_local_transform().to_string(),
            conditioning_witness
                .feature_conditioning_class()
                .as_str()
                .to_string(),
            conditioning_witness
                .support_normal_class()
                .as_str()
                .to_string(),
            conditioning_witness
                .normalization_disposition()
                .as_str()
                .to_string(),
            geometry_digest.clone(),
        ]);
        Self {
            family,
            strategy,
            attempted_strategies,
            stability_class,
            conditioning_witness,
            geometry_digest,
            report_digest,
        }
    }

    pub fn family(&self) -> &'static str {
        self.family
    }

    pub fn strategy(&self) -> PrimitiveRealizationStrategy {
        self.strategy
    }

    pub fn attempted_strategies(&self) -> &[PrimitiveRealizationStrategy] {
        &self.attempted_strategies
    }

    pub fn stability_class(&self) -> PrimitiveStabilityClass {
        self.stability_class
    }

    pub fn conditioning_witness(&self) -> &PrimitiveConditioningWitness {
        &self.conditioning_witness
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }

    pub fn geometry_digest(&self) -> &str {
        &self.geometry_digest
    }
}

#[derive(Clone, Debug)]
pub struct PrimitiveSupportRealization {
    planes: Vec<Plane>,
    report: PrimitiveRealizationReport,
}

impl PrimitiveSupportRealization {
    pub(crate) fn new(planes: Vec<Plane>, report: PrimitiveRealizationReport) -> Self {
        Self { planes, report }
    }

    pub fn planes(&self) -> &[Plane] {
        &self.planes
    }

    pub fn into_planes(self) -> Vec<Plane> {
        self.planes
    }

    pub fn report(&self) -> &PrimitiveRealizationReport {
        &self.report
    }
}

pub fn build_direct_realization_report(
    family: &'static str,
    points: &[[f64; 3]],
    planes: &[Plane],
) -> PrimitiveRealizationReport {
    PrimitiveRealizationReport::new(
        family,
        PrimitiveRealizationStrategy::DirectWorld,
        vec![PrimitiveRealizationStrategy::DirectWorld],
        PrimitiveStabilityClass::StableDirect,
        super::conditioning_witness(points, planes),
        points,
        planes,
    )
}

fn digest_parts(parts: &[String]) -> String {
    truth_digest_parts(PrimitiveDigestScope::ArtifactIdentity, parts)
}
