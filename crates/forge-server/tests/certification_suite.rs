#[path = "support/direct_context_runtime.rs"]
mod direct_context_runtime;
#[path = "support/forge_native/assertions.rs"]
mod forge_native_assertions;
#[path = "support/forge_native/runtime.rs"]
mod forge_native_runtime;
#[path = "support/query_handoff/fixture.rs"]
mod query_handoff_fixture;
#[path = "support/query_handoff/runtime.rs"]
mod query_handoff_runtime;

#[path = "support/certification/bundle.rs"]
mod certification_bundle;
#[path = "support/certification/counter_assertions.rs"]
mod certification_counter_assertions;
#[path = "support/certification/digest_assertions.rs"]
mod certification_digest_assertions;
#[path = "support/certification/fixture.rs"]
mod certification_fixture;
#[path = "support/certification/forge_native_fixture.rs"]
mod certification_forge_native_fixture;

#[path = "certification/forge_native_hostility_matrix.rs"]
mod forge_native_hostility_matrix;
#[path = "certification/forge_native_no_glue_equivalence.rs"]
mod forge_native_no_glue_equivalence;
#[path = "certification/forge_native_retained_fact_parity.rs"]
mod forge_native_retained_fact_parity;
#[path = "certification/forge_native_sabotage_suite.rs"]
mod forge_native_sabotage_suite;
#[path = "certification/mixed_hostility_matrix.rs"]
mod mixed_hostility_matrix;
#[path = "certification/response_denial_localization.rs"]
mod response_denial_localization;
#[path = "certification/response_envelope_parity.rs"]
mod response_envelope_parity;
#[path = "certification/structural_sabotage_suite.rs"]
mod structural_sabotage_suite;
