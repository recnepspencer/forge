use crate::primitives::plane::Plane;
use crate::primitives::shape_realization::schema::{
    PrimitiveConditioningWitness, PrimitiveFeatureConditioningClass,
    PrimitiveNormalizationDisposition, PrimitiveSupportNormalClass,
};
use crate::spatial::coordinate::local_space::ScaleAnalysis;

pub(crate) fn conditioning_witness(
    points: &[[f64; 3]],
    planes: &[Plane],
) -> PrimitiveConditioningWitness {
    conditioning_witness_with_normalization(points, planes, 1.0, false)
}

pub(crate) fn conditioning_witness_with_normalization(
    points: &[[f64; 3]],
    planes: &[Plane],
    normalization_scale: f64,
    normalization_applied: bool,
) -> PrimitiveConditioningWitness {
    let minimum_feature_size = minimum_pairwise_distance(points).unwrap_or(f64::MIN_POSITIVE);
    let analysis = ScaleAnalysis::compute(points, minimum_feature_size);
    let minimum_support_normal_magnitude = planes
        .iter()
        .map(|plane| {
            let normal = plane.raw_normal();
            (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt()
        })
        .fold(f64::INFINITY, f64::min);
    let feature_size_collapsed = feature_size_collapsed(points);
    let precision_headroom_ratio = stable_ratio(
        analysis.get_feature_size(),
        analysis.get_machine_epsilon_at_scale(),
    );
    let support_normal_headroom_ratio = stable_support_normal_headroom_ratio(
        minimum_support_normal_magnitude,
        analysis.get_feature_size(),
    );
    PrimitiveConditioningWitness::new(
        &analysis,
        precision_headroom_ratio,
        if minimum_support_normal_magnitude.is_finite() {
            minimum_support_normal_magnitude
        } else {
            0.0
        },
        support_normal_headroom_ratio,
        normalization_scale,
        feature_size_collapsed,
        classify_feature_conditioning(precision_headroom_ratio, feature_size_collapsed),
        classify_support_normals(support_normal_headroom_ratio, feature_size_collapsed),
        classify_normalization_disposition(&analysis, normalization_applied),
    )
}

pub(crate) fn feature_size_collapsed(points: &[[f64; 3]]) -> bool {
    points.len() > 1 && minimum_pairwise_distance(points).is_none()
}

fn minimum_pairwise_distance(points: &[[f64; 3]]) -> Option<f64> {
    let mut minimum = f64::INFINITY;
    for left in 0..points.len() {
        for right in (left + 1)..points.len() {
            let delta = worth_math::linalg::sub(points[left], points[right]);
            let distance = stable_vector_length(delta);
            if distance > 0.0 && distance < minimum {
                minimum = distance;
            }
        }
    }
    if minimum.is_finite() {
        Some(minimum)
    } else {
        None
    }
}

fn stable_vector_length(delta: [f64; 3]) -> f64 {
    let dx = delta[0].abs();
    let dy = delta[1].abs();
    let dz = delta[2].abs();
    let max_component = dx.max(dy).max(dz);
    if max_component == 0.0 {
        return 0.0;
    }
    let sx = dx / max_component;
    let sy = dy / max_component;
    let sz = dz / max_component;
    max_component * (sx * sx + sy * sy + sz * sz).sqrt()
}

fn classify_feature_conditioning(
    precision_headroom_ratio: f64,
    feature_size_collapsed: bool,
) -> PrimitiveFeatureConditioningClass {
    if feature_size_collapsed
        || !precision_headroom_ratio.is_finite()
        || precision_headroom_ratio <= 1.0
    {
        PrimitiveFeatureConditioningClass::Collapsed
    } else if precision_headroom_ratio <= 1.0e8 {
        PrimitiveFeatureConditioningClass::NearThreshold
    } else {
        PrimitiveFeatureConditioningClass::Healthy
    }
}

fn classify_support_normals(
    support_normal_headroom_ratio: f64,
    feature_size_collapsed: bool,
) -> PrimitiveSupportNormalClass {
    if feature_size_collapsed
        || !support_normal_headroom_ratio.is_finite()
        || support_normal_headroom_ratio <= 0.0
    {
        PrimitiveSupportNormalClass::Degenerate
    } else if support_normal_headroom_ratio <= 1.0e-8 {
        PrimitiveSupportNormalClass::NearDegenerate
    } else {
        PrimitiveSupportNormalClass::Robust
    }
}

fn classify_normalization_disposition(
    analysis: &ScaleAnalysis,
    normalization_applied: bool,
) -> PrimitiveNormalizationDisposition {
    if normalization_applied {
        PrimitiveNormalizationDisposition::LocalTransformationApplied
    } else if analysis.get_needs_local_transform() {
        PrimitiveNormalizationDisposition::LocalTransformationRecommended
    } else {
        PrimitiveNormalizationDisposition::WorldSpaceSufficient
    }
}

fn stable_ratio(numerator: f64, denominator: f64) -> f64 {
    if numerator <= 0.0 {
        0.0
    } else if denominator <= 0.0 {
        f64::INFINITY
    } else {
        numerator / denominator
    }
}

fn stable_support_normal_headroom_ratio(
    minimum_support_normal_magnitude: f64,
    feature_size: f64,
) -> f64 {
    if minimum_support_normal_magnitude <= 0.0 || feature_size <= 0.0 {
        return 0.0;
    }
    let log_ratio = minimum_support_normal_magnitude.log10() - (2.0 * feature_size.log10());
    if !log_ratio.is_finite() {
        return 0.0;
    }
    if log_ratio > 308.0 {
        f64::INFINITY
    } else if log_ratio < -308.0 {
        0.0
    } else {
        10f64.powf(log_ratio)
    }
}
