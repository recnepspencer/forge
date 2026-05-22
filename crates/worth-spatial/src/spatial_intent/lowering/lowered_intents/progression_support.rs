use super::runtime_declaration::{
    LoweredSpatialIntent, LoweredSpatialIntentFamily, LoweredSpatialNumericPosture,
    LoweredSpatialOperation, LoweredSpatialRuntimeDeclaration, LoweredSpatialTargetBindingPosture,
    RuntimeAnchorSemantic,
};
use super::runtime_payload::LoweredSpatialRuntimePayload;

pub(super) fn runtime(
    family: LoweredSpatialIntentFamily,
    subject_anchor: Option<RuntimeAnchorSemantic>,
    target_anchor: Option<RuntimeAnchorSemantic>,
    numeric_posture: LoweredSpatialNumericPosture,
    target_binding: LoweredSpatialTargetBindingPosture,
    payload: LoweredSpatialRuntimePayload,
) -> LoweredSpatialRuntimeDeclaration {
    LoweredSpatialRuntimeDeclaration::new(
        family,
        subject_anchor,
        target_anchor,
        numeric_posture,
        target_binding,
        payload,
    )
}

pub(super) fn build(
    runtime_declaration: LoweredSpatialRuntimeDeclaration,
    operation: LoweredSpatialOperation,
) -> LoweredSpatialIntent {
    LoweredSpatialIntent::new(runtime_declaration, operation)
}

pub(super) fn reorient_posture(source: [f64; 3], target: [f64; 3]) -> LoweredSpatialNumericPosture {
    let dot = source[0] * target[0] + source[1] * target[1] + source[2] * target[2];
    if dot <= -1.0 + 1.0e-12 {
        LoweredSpatialNumericPosture::FallbackDerived
    } else {
        LoweredSpatialNumericPosture::Normalized
    }
}

pub(super) fn reorient_point_like_posture(
    _source_facing: [f64; 3],
    _target_direction_witness: &crate::spatial_intent::refs::SpatialDirectionWitnessRef,
) -> LoweredSpatialNumericPosture {
    LoweredSpatialNumericPosture::Normalized
}

pub(super) fn coincident(a: [f64; 3], b: [f64; 3]) -> bool {
    [a[0] - b[0], a[1] - b[1], a[2] - b[2]]
        .iter()
        .all(|v| v.abs() <= f64::MIN_POSITIVE)
}
