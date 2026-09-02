use worth_runtime_world::facade::{
    OwnerExecutionSettlement, ReservedCompositePublicationAttempt,
};

fn consume_settlement(_: OwnerExecutionSettlement) {}

fn illegal_skip(attempt: ReservedCompositePublicationAttempt) {
    consume_settlement(attempt);
}

fn main() {}
