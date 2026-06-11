use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::thin_feature_counters::ThinFeatureScaleSeparationCounters;

pub(crate) fn thin_feature_digest(
    workload_identity: &str,
    precision_identity: &str,
    local_frame_identity: &str,
    projection_consumption_identity: &str,
    projection_consumed_local_frame_identity: &str,
    local_scale_orders: &[i32],
    required_world_magnitude_order: i32,
    counters: ThinFeatureScaleSeparationCounters,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "thin-feature-scale-separation-workload".to_string(),
            format!("workload:{workload_identity}"),
            format!("precision:{precision_identity}"),
            format!("local_frame:{local_frame_identity}"),
            format!("projection_consumption:{projection_consumption_identity}"),
            format!("projection_consumed_local_frame:{projection_consumed_local_frame_identity}"),
            format!("local_scale_orders:{local_scale_orders:?}"),
            format!("required_world_magnitude_order:{required_world_magnitude_order}"),
            format!("thin_features:{}", counters.thin_feature_count()),
            format!("local_scales:{}", counters.local_scale_order_count()),
            format!(
                "world_magnitudes:{}",
                counters.world_magnitude_order_count()
            ),
            format!(
                "precision_escalations:{}",
                counters.precision_escalation_count()
            ),
            format!("local_basis_parts:{}", counters.local_basis_part_count()),
            format!(
                "projection_consumed_basis:{}",
                counters.projection_consumed_basis_count()
            ),
            format!(
                "tiny_rotation_pressure:{}",
                counters.tiny_rotation_pressure_count()
            ),
        ],
    )
}
