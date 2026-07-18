//! Semantic integration suite. Individual responsibilities remain in named child modules.

#[path = "../obligation_catalog_runtime.rs"]
mod obligation_catalog_runtime;
#[path = "../obligation_closeout_runtime.rs"]
mod obligation_closeout_runtime;
#[path = "../obligation_diagnostic_projection_runtime.rs"]
mod obligation_diagnostic_projection_runtime;
#[path = "../obligation_dispatch_determinism_runtime.rs"]
mod obligation_dispatch_determinism_runtime;
#[path = "../obligation_dispatch_prerequisite_runtime.rs"]
mod obligation_dispatch_prerequisite_runtime;
#[path = "../obligation_dispatch_runtime.rs"]
mod obligation_dispatch_runtime;
#[path = "../obligation_evidence_runtime.rs"]
mod obligation_evidence_runtime;
#[path = "../obligation_global_stop_runtime.rs"]
mod obligation_global_stop_runtime;
#[path = "../obligation_handoff_determinism_runtime.rs"]
mod obligation_handoff_determinism_runtime;
#[path = "../obligation_handoff_runtime.rs"]
mod obligation_handoff_runtime;
#[path = "../obligation_inspection_runtime.rs"]
mod obligation_inspection_runtime;
#[path = "../obligation_query_prerequisite_runtime.rs"]
mod obligation_query_prerequisite_runtime;
#[path = "../obligation_requirement_posture_runtime.rs"]
mod obligation_requirement_posture_runtime;
#[path = "../obligation_selected_membership_runtime.rs"]
mod obligation_selected_membership_runtime;
#[path = "../obligation_selection_host_runtime.rs"]
mod obligation_selection_host_runtime;
#[path = "../obligation_selection_matrix_runtime.rs"]
mod obligation_selection_matrix_runtime;
#[path = "../obligation_selection_rejection_runtime.rs"]
mod obligation_selection_rejection_runtime;
#[path = "../obligation_touch_isolation_runtime.rs"]
mod obligation_touch_isolation_runtime;
#[path = "../obligation_verdict_runtime.rs"]
mod obligation_verdict_runtime;
