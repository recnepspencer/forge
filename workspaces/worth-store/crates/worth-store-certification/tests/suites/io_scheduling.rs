#[path = "../support/physical_isolation/checkpoint_read_fixture/checkpoint_read_fixture.rs"]
mod checkpoint_read_fixture;

#[path = "../scenarios/scheduling/access_policy/access_policy.rs"]
mod access_policy;

#[path = "../scenarios/scheduling/evidence_materialization/evidence_materialization.rs"]
mod evidence_materialization;

#[path = "../scenarios/scheduling/flush_durability/flush_durability.rs"]
mod flush_durability;

#[path = "../scenarios/scheduling/io_qos_readiness_handoff/io_qos_readiness_handoff.rs"]
mod io_qos_readiness_handoff;

#[path = "../scenarios/scheduling/latency_interference/latency_interference.rs"]
mod latency_interference;

#[path = "../scenarios/scheduling/reclaim_policy/reclaim_policy.rs"]
mod reclaim_policy;

#[path = "../support/scheduling/access_policy_support/access_policy_support.rs"]
mod access_policy_support;
#[path = "../scenarios/scheduling/secure_io_execution/secure_io_execution.rs"]
mod secure_io_execution;

#[path = "../scenarios/scheduling/producer_declarations/producer_declarations.rs"]
mod producer_declarations;
