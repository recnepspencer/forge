use worth_spatial::facade::transform_workload::{TransformSequence, TransformedWorkload};

fn main() {
    let label_motion = TransformSequence::identity_label_only("moved");
    consume_transformed_workload(label_motion);
}

fn consume_transformed_workload(_workload: TransformedWorkload) {}
