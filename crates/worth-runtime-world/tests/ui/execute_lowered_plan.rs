use worth_runtime_world::facade::{LoweredOwnerComponentPlan, ReservedCompositePublicationAttempt};

fn consume_reserved(_: ReservedCompositePublicationAttempt) {}

fn illegal_skip(plan: LoweredOwnerComponentPlan) {
    consume_reserved(plan);
}

fn main() {}
