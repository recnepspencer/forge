#[test]
fn spatial_bindings_boundary_no_longer_teaches_parallel_report_ecologies() {
    let bindings_mod = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/bindings/mod.rs"));
    let facade = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/facade/mod.rs"));
    let facade_bindings = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/facade/bindings.rs"
    ));
    let facade_anchor_binding = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/facade/anchor_binding.rs"
    ));
    let facade_binding = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/facade/binding.rs"
    ));
    let facade_continuation = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/facade/continuation.rs"
    ));
    let facade_neighborhood = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/facade/neighborhood.rs"
    ));
    let facade_placement = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/facade/placement.rs"
    ));
    let facade_support = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/facade/support.rs"
    ));
    let facade_projection = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/facade/projection.rs"
    ));
    let facade_rebinding = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/facade/rebinding.rs"
    ));
    let facade_recovery = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/facade/recovery.rs"
    ));
    let facade_tolerance = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/facade/tolerance.rs"
    ));
    let binding_authoring = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/bindings/query_native_binding_authoring.rs"
    ));
    let anchor_binding_authoring = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/bindings/query_native_anchor_binding_authoring.rs"
    ));
    let rebinding_authoring = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/bindings/query_native_rebinding_authoring.rs"
    ));
    let rebinding_neighborhood = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/bindings/rebinding/neighborhood.rs"
    ));

    assert!(bindings_mod.contains("mod authority;"));
    assert!(bindings_mod.contains("mod identity;"));
    assert!(!bindings_mod.contains("mod primitive_birth_completeness;"));
    assert!(!bindings_mod.contains("mod primitive_birth_mapping;"));
    assert!(!bindings_mod.contains("mod primitive_birth_rejection;"));
    assert!(!facade.contains("construction_birth_authority"));
    assert!(!facade.contains("certify_primitive_construction_birth_completeness"));
    assert!(!facade.contains("build_primitive_construction_birth_mapping_report"));
    assert!(!facade.contains("impossible_primitive_construction_birth_attachment"));
    assert!(!facade.contains("SpatialConstructionBirthAuthority"));
    assert!(!facade.contains("SpatialConstructionBirthCompletenessReport"));
    assert!(!facade.contains("SpatialConstructionBirthMappingReport"));
    assert!(!facade.contains("SpatialConstructionBirthRejectionRow"));
    assert!(!facade.contains("PrimitiveConstructionBirthContractCounts"));
    assert!(facade.contains("pub mod anchor_binding;"));
    assert!(facade.contains("pub mod binding;"));
    assert!(facade.contains("pub mod bindings;"));
    assert!(facade.contains("pub mod continuation;"));
    assert!(facade.contains("pub mod inspection;"));
    assert!(facade.contains("pub mod neighborhood;"));
    assert!(facade.contains("pub mod planar_predicates;"));
    assert!(facade.contains("pub mod projection;"));
    assert!(facade.contains("pub mod rebinding;"));
    assert!(facade.contains("pub mod recovery;"));
    assert!(facade.contains("pub mod support;"));
    assert!(facade.contains("pub mod tolerance;"));
    assert!(!facade.contains("pub mod birth;"));
    assert!(!facade_bindings.contains("materialize_primitive_construction_birth_assessment"));
    assert!(!facade_bindings.contains("attach_surface_to_face"));
    assert!(!facade_bindings.contains("attach_curve_to_edge"));
    assert!(!facade_bindings.contains("attach_pcurve_to_coedge"));
    assert!(!facade_bindings.contains("attach_vertex_geometry"));
    assert!(!facade_bindings.contains("attach_parameter_space_point_to_face"));
    assert!(!facade_bindings.contains("attach_parameter_space_direction_to_face"));
    assert!(!facade_bindings.contains("evaluate_continuity"));
    assert!(!facade_bindings.contains("evaluate_binding_motion_posture"));
    assert!(!facade_bindings.contains("explain_rebinding_decision"));
    assert!(facade_anchor_binding.contains("query_native_anchor_binding_mutation_evidence::{"));
    assert!(facade_anchor_binding.contains("query_native_anchor_binding_projection::{"));
    assert!(facade_anchor_binding.contains("query_native_rebinding_candidate_fact::{"));
    assert!(facade_anchor_binding.contains("query_native_rebinding_prior_fact::{"));
    assert!(facade_anchor_binding.contains("query_native_target_identity::{"));
    assert!(facade_binding.contains("query_native_binding_mutation_evidence::{"));
    assert!(facade_binding.contains("query_native_binding_projection::{"));
    assert!(facade_binding.contains("query_native_rebinding_candidate_fact::{"));
    assert!(facade_binding.contains("query_native_rebinding_prior_fact::{"));
    assert!(facade_binding.contains("query_native_target_identity::{"));
    assert!(facade_rebinding.contains("query_native_rebinding_mutation_evidence::{"));
    assert!(facade_rebinding.contains("query_native_rebinding_projection::{"));
    assert!(facade_rebinding.contains("query_native_rebinding_authoring::{"));
    assert!(facade_rebinding.contains("pub use crate::bindings::rebinding::{"));
    assert!(facade_bindings.contains("query_native_binding_mutation_evidence::{"));
    assert!(facade_bindings.contains("query_native_binding_projection::{"));
    assert!(facade_bindings.contains("query_native_anchor_binding_mutation_evidence::{"));
    assert!(facade_bindings.contains("query_native_anchor_binding_projection::{"));
    assert!(facade_bindings.contains("query_native_rebinding_candidate_fact::{"));
    assert!(facade_bindings.contains("query_native_rebinding_prior_fact::{"));
    assert!(facade_bindings.contains("query_native_target_identity::{"));
    assert!(facade_bindings.contains("PrimitiveBindingDeclarationEntry"));
    assert!(facade_bindings.contains("PrimitiveAnchorBindingDeclarationEntry"));
    assert!(facade_bindings.contains("PrimitiveRebindingDeclarationEntry"));
    assert!(!facade_bindings.contains("primitive_rebinding_contribution_workflow"));
    assert!(!facade_bindings.contains("primitive_rebinding_local_neighborhood"));
    assert!(!facade_bindings.contains("primitive_rebinding_local_neighborhood_contributions"));
    assert!(facade_neighborhood.contains("primitive_rebinding_contribution_workflow"));
    assert!(facade_neighborhood.contains("primitive_rebinding_local_neighborhood"));
    assert!(facade_neighborhood.contains("primitive_rebinding_local_neighborhood_contributions"));
    assert!(!facade_bindings.contains("query_native_geometry_support::{"));
    assert!(!facade_bindings.contains("geometry_public_support_matrix"));
    assert!(!facade_bindings.contains("admit_geometry_public_surface"));
    assert!(!facade_bindings.contains("GeometryPublicSurface"));
    assert!(facade_support.contains("GeometryPublicSurface"));
    assert!(!facade_bindings.contains("query_native_geometry_applicability::{"));
    assert!(!facade_bindings.contains("geometry_applicability_matrix"));
    assert!(!facade_bindings.contains("GeometryRuntimeConcern"));
    assert!(!facade_bindings.contains("GeometryApplicabilityStatus"));
    assert!(facade_support.contains("query_native_geometry_applicability::{"));
    assert!(facade_support.contains("geometry_applicability_matrix"));
    assert!(!facade_bindings.contains("SpatialAdmittedPrimitiveBinding"));
    assert!(!facade_bindings.contains("query_native_rebinding_projection_consumption::{"));
    assert!(!facade_bindings.contains("primitive_rebinding_geometry_projection_consumption"));
    assert!(!facade_bindings.contains("geometry_projection_consumption_entry"));
    assert!(!facade_bindings.contains("GeometryProjectionConsumptionReceipt"));
    assert!(!facade_bindings.contains("GeometryProjectionConsumptionDeclarationFamily"));
    assert!(facade_projection.contains("geometry_projection_consumption_entry"));
    assert!(!facade_bindings.contains("query_native_rebinding_neighborhood_replacement::{"));
    assert!(!facade_bindings.contains("primitive_rebinding_neighborhood_replacement_facts"));
    assert!(!facade_bindings.contains("topology_neighborhood_replacement_entry"));
    assert!(!facade_bindings.contains("TopologyNeighborhoodReplacementFactReceipt"));
    assert!(!facade_bindings.contains("TopologyNeighborhoodReplacementDeclarationFamily"));
    assert!(facade_neighborhood.contains("primitive_rebinding_neighborhood_replacement_facts"));
    assert!(facade_neighborhood.contains("topology_neighborhood_replacement_entry"));
    assert!(facade_neighborhood.contains("TopologyNeighborhoodReplacementFactReceipt"));
    assert!(facade_neighborhood.contains("TopologyNeighborhoodReplacementDeclarationFamily"));
    assert!(!facade_bindings.contains("query_native_geometry_recovery::{"));
    assert!(!facade_bindings.contains("primitive_rebinding_geometry_recovery_action"));
    assert!(!facade_bindings.contains("GeometryRecoveryActionFactReceipt"));
    assert!(!facade_bindings.contains("GeometryRecoveryActionDeclarationFamily"));
    assert!(facade_recovery.contains("primitive_rebinding_geometry_recovery_action"));
    assert!(!facade_bindings.contains("query_native_tolerance_precision::{"));
    assert!(!facade_bindings.contains("query_native_tolerance_precision_authoring::{"));
    assert!(!facade_bindings
        .contains("primitive_construction_tolerance_and_precision_certification_facts"));
    assert!(!facade_bindings.contains("ToleranceAndPrecisionCertificationFactReceipt"));
    assert!(!facade_bindings.contains("ToleranceAndPrecisionCertificationDeclarationFamily"));
    assert!(facade_tolerance
        .contains("primitive_construction_tolerance_and_precision_certification_facts"));
    let facade_planar_predicates = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/facade/planar_predicates.rs"
    ));
    assert!(!facade_bindings.contains("query_native_planar_predicate::{"));
    assert!(!facade_bindings.contains("planar_predicate_authority_facts"));
    assert!(!facade_bindings.contains("PlanarPredicateAuthorityDeclarationFamily"));
    assert!(facade_planar_predicates.contains("query_native_planar_predicate::{"));
    assert!(facade_planar_predicates.contains("planar_predicate_authority_facts"));
    assert!(facade_planar_predicates.contains("PlanarPredicateAuthorityDeclarationFamily"));
    assert!(!binding_authoring.contains("pub fn admit("));
    assert!(!anchor_binding_authoring.contains("pub fn admit("));
    assert!(!rebinding_authoring.contains("pub fn admit("));
    assert!(!binding_authoring.contains("fn admit_intent("));
    assert!(!anchor_binding_authoring.contains("fn admit_intent("));
    assert!(!rebinding_authoring.contains("fn admit_intent("));
    assert!(!rebinding_authoring.contains("impl Into<PrimitiveRebindingPriorBindingFact>"));
    assert!(rebinding_authoring.contains("pub fn replace_surface_binding("));
    assert!(rebinding_authoring.contains("prior_binding: PrimitiveRebindingPriorBindingFact"));
    assert!(!rebinding_authoring.contains("replace_surface_binding_from_admitted_binding"));
    assert!(!rebinding_authoring.contains("replace_curve_binding_from_admitted_binding"));
    assert!(!rebinding_authoring.contains("replace_pcurve_binding_from_admitted_binding"));
    assert!(!rebinding_authoring.contains("replace_geometry_binding_from_admitted_binding"));
    assert!(!facade_bindings.contains("PrimitiveConstructionBirthScaffoldInput"));
    assert!(!facade_bindings.contains("plan_primitive_construction_birth"));
    assert!(!facade_bindings.contains("SpatialConstructionBirthPlan"));
    assert!(!facade_bindings.contains("evaluate_primitive_construction_birth_consequence"));
    assert!(!facade_bindings.contains("AdmittedPrimitiveConstructionBirthConsequence"));
    assert!(!facade_bindings.contains("RejectedPrimitiveConstructionBirthConsequence"));
    assert!(!facade_bindings.contains("SpatialConstructionBirthConsequence"));
    assert!(!facade_bindings.contains("SpatialConstructionBirthMappingKind"));
    assert!(!facade_bindings.contains("SpatialConstructionBirthRejectionKind"));
    assert!(!facade_bindings.contains("assess_primitive_construction_birth"));
    assert!(!facade_bindings.contains("AdmittedPrimitiveConstructionBirthAssessment"));
    assert!(!facade_bindings.contains("SpatialConstructionBirthAssessmentError"));
    assert!(!facade_bindings.contains("PrimitiveConstructionBirthRealizationFacts"));
    assert!(!facade_bindings.contains("PrimitiveConstructionBirthRealizationPosture"));
    assert!(!facade_bindings.contains("SpatialConstructionBirthError"));
    assert!(!facade_bindings.contains("PrimitiveConstructionBirthPlacementFacts"));
    assert!(!facade_bindings.contains("PrimitiveConstructionBirthFamily"));
    assert!(!facade_bindings.contains("materialize_primitive_construction_birth_scaffold_input"));
    assert!(!facade_bindings.contains("PrimitiveConstructionBirthScaffoldMaterializationInput"));
    assert!(!facade_bindings.contains("PrimitiveConstructionBirthScaffoldRealization"));
    assert!(!facade_bindings.contains("PrimitiveConstructionBirthTopologyCounts"));
    assert!(!facade_bindings.contains("ToleranceAndPrecisionRealizationPosture"));
    assert!(facade_tolerance.contains("ToleranceAndPrecisionRealizationPosture"));
    assert!(!facade_placement.contains("PrimitiveConstructionBirthPlacementFacts"));
    assert!(!facade_bindings.contains("primitive_rebinding_signal_workflow"));
    assert!(!facade_bindings.contains("primitive_rebinding_continuation_target"));
    assert!(!facade_bindings.contains("PrimitiveRebindingSignalCompatibilityArtifact"));
    assert!(!facade_bindings.contains("PrimitiveRebindingPreparedContinuation"));
    assert!(!facade_bindings.contains("PrimitiveRebindingContinuationExecution"));
    assert!(facade_continuation.contains("primitive_rebinding_signal_workflow"));
    assert!(facade_continuation.contains("primitive_rebinding_continuation_target"));
    assert!(facade_continuation.contains("PrimitiveRebindingSignalCompatibilityArtifact"));
    assert!(facade_continuation.contains("PrimitiveRebindingPreparedContinuation"));
    assert!(facade_continuation.contains("PrimitiveRebindingContinuationExecution"));
    assert!(facade_bindings.contains("AnchorCarrierOwnership"));
    assert!(facade_bindings.contains("FaceSurfaceBindingSpec"));
    assert!(facade_bindings.contains("SpatialCanonicalDeclarationField"));
    assert!(!rebinding_neighborhood.contains("impl Into<PrimitiveRebindingCandidateFact>"));
    assert!(!rebinding_neighborhood.contains("pub fn from_admitted_binding("));
    let rebinding_runtime = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/bindings/rebinding/mod.rs"
    ));
    assert!(!rebinding_runtime.contains("rebind_surface_on_face_from_admitted_binding"));
    assert!(!rebinding_runtime.contains("rebind_curve_on_edge_from_admitted_binding"));
    assert!(!rebinding_runtime.contains("rebind_pcurve_on_coedge_from_admitted_binding"));
    assert!(!rebinding_runtime.contains("rebind_geometry_on_vertex_from_admitted_binding"));
}
