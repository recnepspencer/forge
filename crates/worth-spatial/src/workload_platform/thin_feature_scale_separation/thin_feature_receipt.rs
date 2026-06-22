use super::thin_feature_counters::ThinFeatureScaleSeparationCounters;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ThinFeatureScaleSeparationReceipt {
    thin_feature_digest: String,
    workload_identity: String,
    precision_identity: String,
    platform_projection_identity: String,
    local_frame_identity: String,
    projection_consumption_identity: String,
    projection_consumed_local_frame_identity: String,
    local_scale_orders: Vec<i32>,
    required_world_magnitude_order: i32,
    counters: ThinFeatureScaleSeparationCounters,
}

impl ThinFeatureScaleSeparationReceipt {
    pub(crate) fn new(
        thin_feature_digest: String,
        workload_identity: String,
        precision_identity: String,
        platform_projection_identity: String,
        local_frame_identity: String,
        projection_consumption_identity: String,
        projection_consumed_local_frame_identity: String,
        local_scale_orders: Vec<i32>,
        required_world_magnitude_order: i32,
        counters: ThinFeatureScaleSeparationCounters,
    ) -> Self {
        Self {
            thin_feature_digest,
            workload_identity,
            precision_identity,
            platform_projection_identity,
            local_frame_identity,
            projection_consumption_identity,
            projection_consumed_local_frame_identity,
            local_scale_orders,
            required_world_magnitude_order,
            counters,
        }
    }

    pub fn thin_feature_digest(&self) -> &str {
        &self.thin_feature_digest
    }

    pub fn workload_identity(&self) -> &str {
        &self.workload_identity
    }

    pub fn precision_identity(&self) -> &str {
        &self.precision_identity
    }

    pub fn platform_projection_identity(&self) -> &str {
        &self.platform_projection_identity
    }

    pub fn local_frame_identity(&self) -> &str {
        &self.local_frame_identity
    }

    pub fn projection_consumption_identity(&self) -> &str {
        &self.projection_consumption_identity
    }

    pub fn projection_consumed_local_frame_identity(&self) -> &str {
        &self.projection_consumed_local_frame_identity
    }

    pub fn local_scale_orders(&self) -> &[i32] {
        &self.local_scale_orders
    }

    pub fn required_world_magnitude_order(&self) -> i32 {
        self.required_world_magnitude_order
    }

    pub fn counters(&self) -> ThinFeatureScaleSeparationCounters {
        self.counters
    }
}
