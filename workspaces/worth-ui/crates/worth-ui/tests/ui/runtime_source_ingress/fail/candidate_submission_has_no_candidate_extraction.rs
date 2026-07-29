use worth_ui::facade::source::WorthUiWatchedCandidateSubmission;

fn extract(submission: WorthUiWatchedCandidateSubmission) {
    let _ = submission.into_candidate();
}

fn main() {}

// runtime lifecycle denials share one compiler process.
mod covered_001 { include!("candidate_basis_cannot_replace_prepared_generation_identity.rs"); }
mod covered_002 { include!("../../runtime_file_rust_replacement_parity/fail/rust_cannot_construct_replacement_candidate.rs"); }
mod covered_003 { include!("../../runtime_file_rust_replacement_parity/fail/rust_cannot_inject_active_plan_nodes.rs"); }
mod covered_004 { include!("../../runtime_plan_swap/fail/plan_swap_receipt_fields_not_public.rs"); }
mod covered_005 { include!("../../runtime_reload_failure/fail/reload_failure_fields_not_public.rs"); }
mod covered_006 { include!("../../runtime_reload_storm_certification/fail/reload_storm_certification_not_public_facade_api.rs"); }
mod covered_007 { include!("raw_watcher_event_cannot_declare_dependency_impact.rs"); }
mod covered_008 { include!("../../observation/fail/admitted_observation_cannot_be_constructed.rs"); }
mod covered_009 { include!("../../observation/fail/raw_host_batch_cannot_enter_semantic_admission.rs"); }
