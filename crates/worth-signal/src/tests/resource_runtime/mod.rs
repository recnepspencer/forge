use std::sync::{Arc, Mutex};

use crate::data::resource::{
    built_in_policy_registrations, FrozenResourcePolicyDescriptorSet, LoweredResourcePolicyBundle,
    ResourceReplayAvailabilityClass, ResourceReplayAvailabilityDenialClass,
    ResourceReplayDecisionClass, ResourceReplayDecisionPlan, ResourceReplayPolicyDeclaration,
    ValidatedResourcePolicyDeclaration,
};
use crate::facade::*;
use crate::tests::support::version_ab;

use super::resource_closeout_assertions::{
    assert_hostile_evidence_shape, assert_milestone_c_policy_performance_closeout_claim_shape,
    assert_performance_closeout_claim_shape, required_hostile_evidence_row,
    required_milestone_c_policy_performance_claim_row, required_performance_claim_row,
    required_scenario_row,
};

mod cancellation_and_supersession;
mod completion_admission;
mod declaration_and_visibility;
mod diagnostics;
mod milestone_b;
mod milestone_c;
mod observation;
mod policy_descriptor;
mod policy_restore_compatibility;
mod replay_availability;
mod retention;
mod revalidation;
mod safe_point;
mod support;
mod timeout_and_retry;

use support::*;

type TestRuntime = SignalRuntime<(), (), (), (), ()>;
