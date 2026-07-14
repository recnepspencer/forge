use worth_query::facade::foundation::CanonicalQueryArtifact;
use worth_query::facade::policy::{lower_policy_aware_delivery_shape, DeliveryWidthClass, PolicyAwareDeliveryShape, PolicyAwareExecutionSeamError};

fn expects_raw_query_lowerer(
    _: fn(
        &CanonicalQueryArtifact,
        DeliveryWidthClass,
    ) -> Result<PolicyAwareDeliveryShape, PolicyAwareExecutionSeamError>,
) {
}

fn main() {
    expects_raw_query_lowerer(lower_policy_aware_delivery_shape);
}
