pub(crate) mod anchors;
pub(crate) mod authority;
pub(crate) mod canonical_projection;
#[cfg(test)]
mod certification;
pub(crate) mod identity;
#[cfg(test)]
pub(crate) mod primitive_birth;
#[cfg(test)]
pub(crate) mod primitive_birth_assessment;
#[cfg(test)]
pub(crate) mod primitive_birth_consequence;
#[cfg(test)]
mod primitive_birth_contract;
#[cfg(test)]
pub(crate) mod primitive_birth_placement;
#[cfg(test)]
pub(crate) mod primitive_birth_runtime;
#[cfg(test)]
pub(crate) mod primitive_birth_scaffold_materialization;
#[cfg(test)]
mod primitive_birth_validation;
pub(crate) mod query_native;
pub(crate) mod query_native_anchor_binding_authoring;
pub(crate) mod query_native_anchor_binding_mutation_evidence;
pub(crate) mod query_native_anchor_binding_projection;
pub(crate) mod query_native_binding_authoring;
pub(crate) mod query_native_binding_mutation_evidence;
pub(crate) mod query_native_binding_projection;
pub(crate) mod query_native_binding_projection_payload;
pub(crate) mod query_native_branch_local_geometry_inspection;
pub(crate) mod query_native_declared_target_identity_fact;
pub(crate) mod query_native_geometry_applicability;
pub(crate) mod query_native_geometry_applicability_planar;
pub(crate) mod query_native_geometry_inventory;
pub(crate) mod query_native_geometry_recovery;
pub(crate) mod query_native_geometry_replay_parity;
pub(crate) mod query_native_geometry_replay_parity_artifact;
pub(crate) mod query_native_historical_geometry_inspection;
pub(crate) mod query_native_planar_clean_fail_boundary;
pub(crate) mod query_native_planar_contract_bundle;
pub(crate) mod query_native_planar_diagnostics;
pub(crate) mod query_native_planar_local_frame;
pub(crate) mod query_native_planar_local_rebuild_parity;
pub(crate) mod query_native_planar_motion_posture;
pub(crate) mod query_native_planar_overlap;
pub(crate) mod query_native_planar_precision;
pub(crate) mod query_native_planar_predicate;
pub(crate) mod query_native_planar_predicate_consumption;
pub(crate) mod query_native_planar_projection;
pub(crate) mod query_native_planar_projection_consumption;
pub(crate) mod query_native_planar_recovery;
pub(crate) mod query_native_planar_segment_segment;
pub(crate) mod query_native_planar_signed_area;
pub(crate) mod query_native_planar_structural_identity;
pub(crate) mod query_native_planar_topology_contract;
pub(crate) mod query_native_planar_winding;
pub(crate) mod query_native_rebinding;
pub(crate) mod query_native_rebinding_authoring;
pub(crate) mod query_native_rebinding_candidate_fact;
pub(crate) mod query_native_rebinding_contribution;
pub(crate) mod query_native_rebinding_declaration_support;
pub(crate) mod query_native_rebinding_declared_binding_fact;
pub(crate) mod query_native_rebinding_grouped;
pub(crate) mod query_native_rebinding_grouped_contribution;
pub(crate) mod query_native_rebinding_mutation_evidence;
pub(crate) mod query_native_rebinding_neighborhood_replacement;
pub(crate) mod query_native_rebinding_prior_fact;
pub(crate) mod query_native_rebinding_projection;
pub(crate) mod query_native_rebinding_projection_consumption;
pub(crate) mod query_native_rebinding_projection_logic;
pub(crate) mod query_native_rebinding_signal_and_continuation;
pub(crate) mod query_native_retained_geometry;
pub(crate) mod query_native_retained_planar_facts;
pub(crate) mod query_native_retained_view_payload;
pub(crate) mod query_native_target_identity;
pub(crate) mod query_native_tolerance_precision;
pub(crate) mod query_native_tolerance_precision_authoring;
pub(crate) mod query_native_tolerance_precision_facts;
pub(crate) mod rebinding;
