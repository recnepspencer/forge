mod support;

use forge_relational::facade::{
    durability::RecoveryVerificationMode, runtime::RelationalRuntimeApi,
};

fn main() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(support::demo_schema_registry())
        .build();

    let (_created, entity_id) = support::create_entity(&mut runtime, "durable");
    let _updated = support::update_entity(&mut runtime, entity_id, "durable-updated");
    let _deleted = support::delete_entity(&mut runtime, entity_id);

    let validation = runtime.validation().certification_state();
    let recovery_plan = runtime
        .durability()
        .recovery_plan(RecoveryVerificationMode::NormalRecoveryVerification);

    println!(
        "validation_results={} blocking_violation={} durable_log_len={} recovery_segments={}",
        validation.summary().result_count(),
        validation.summary().has_blocking_violation(),
        runtime.durability().durable_log().len(),
        recovery_plan.cursor.segment_ids.len()
    );
}
