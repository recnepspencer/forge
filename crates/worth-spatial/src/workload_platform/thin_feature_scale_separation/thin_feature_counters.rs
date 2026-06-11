#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ThinFeatureScaleSeparationCounters {
    thin_feature_count: usize,
    local_scale_order_count: usize,
    world_magnitude_order_count: usize,
    precision_escalation_count: usize,
    local_basis_part_count: usize,
    projected_entity_count: usize,
    transform_step_count: usize,
    tiny_rotation_pressure_count: usize,
    projection_consumed_basis_count: usize,
    diagnostic_count: usize,
    user_outcome_count: usize,
}

impl ThinFeatureScaleSeparationCounters {
    pub(crate) fn new(input: ThinFeatureScaleSeparationCounterInput) -> Self {
        Self {
            thin_feature_count: input.thin_feature_count,
            local_scale_order_count: input.local_scale_order_count,
            world_magnitude_order_count: input.world_magnitude_order_count,
            precision_escalation_count: input.precision_escalation_count,
            local_basis_part_count: input.local_basis_part_count,
            projected_entity_count: input.projected_entity_count,
            transform_step_count: input.transform_step_count,
            tiny_rotation_pressure_count: input.tiny_rotation_pressure_count,
            projection_consumed_basis_count: input.projection_consumed_basis_count,
            diagnostic_count: input.diagnostic_count,
            user_outcome_count: input.user_outcome_count,
        }
    }

    pub fn thin_feature_count(self) -> usize {
        self.thin_feature_count
    }

    pub fn local_scale_order_count(self) -> usize {
        self.local_scale_order_count
    }

    pub fn world_magnitude_order_count(self) -> usize {
        self.world_magnitude_order_count
    }

    pub fn precision_escalation_count(self) -> usize {
        self.precision_escalation_count
    }

    pub fn local_basis_part_count(self) -> usize {
        self.local_basis_part_count
    }

    pub fn projected_entity_count(self) -> usize {
        self.projected_entity_count
    }

    pub fn transform_step_count(self) -> usize {
        self.transform_step_count
    }

    pub fn tiny_rotation_pressure_count(self) -> usize {
        self.tiny_rotation_pressure_count
    }

    pub fn projection_consumed_basis_count(self) -> usize {
        self.projection_consumed_basis_count
    }

    pub fn diagnostic_count(self) -> usize {
        self.diagnostic_count
    }

    pub fn user_outcome_count(self) -> usize {
        self.user_outcome_count
    }
}

pub(crate) struct ThinFeatureScaleSeparationCounterInput {
    pub(crate) thin_feature_count: usize,
    pub(crate) local_scale_order_count: usize,
    pub(crate) world_magnitude_order_count: usize,
    pub(crate) precision_escalation_count: usize,
    pub(crate) local_basis_part_count: usize,
    pub(crate) projected_entity_count: usize,
    pub(crate) transform_step_count: usize,
    pub(crate) tiny_rotation_pressure_count: usize,
    pub(crate) projection_consumed_basis_count: usize,
    pub(crate) diagnostic_count: usize,
    pub(crate) user_outcome_count: usize,
}
