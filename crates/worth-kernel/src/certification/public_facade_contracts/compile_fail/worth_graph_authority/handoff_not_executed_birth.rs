use worth_kernel::construction::result::{
    ExecutedPrimitiveConstructionGraphAuthorityResult, PreparedPrimitiveConstructionHandoffResult,
};

fn requires_executed_birth_authority(_: ExecutedPrimitiveConstructionGraphAuthorityResult) {}

fn promote_handoff(handoff: PreparedPrimitiveConstructionHandoffResult) {
    requires_executed_birth_authority(handoff);
}

fn main() {}
