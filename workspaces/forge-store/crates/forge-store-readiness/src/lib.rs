#![forbid(unsafe_code)]
//!
//! ```compile_fail
//! use forge_store_readiness::FoundationalVocabularyAdoptionMap;
//!
//! struct LocalLookalike;
//!
//! fn requires_public_foundational_adoption(_: FoundationalVocabularyAdoptionMap) {}
//!
//! requires_public_foundational_adoption(LocalLookalike);
//! ```
//!
//! S.1 closeout receipts cannot be synthesized from raw fields:
//!
//! ```compile_fail
//! use forge_store_contracts::ROADMAP_2_S1_SCOPE;
//! use forge_store_readiness::S1PhysicalSubstrateCloseoutReceipt;
//!
//! let _forged = S1PhysicalSubstrateCloseoutReceipt {
//!     scope: ROADMAP_2_S1_SCOPE,
//!     evidence: todo!(),
//! };
//! ```
//!
//! S.2 readiness facts are not public count authority:
//!
//! ```compile_fail
//! use forge_store_readiness::S2PhysicalReadinessFacts;
//!
//! let _forged = S2PhysicalReadinessFacts::from_s1_closeout_counts(4, 2, 2, 3, 1, 9);
//! ```
//!
//! S.2 handoff evidence cannot be synthesized by ordinary callers:
//!
//! ```compile_fail
//! use forge_store_readiness::{
//!     S2PhysicalSubstrateEvidenceCounts, S2PhysicalSubstrateHandoffEvidence,
//! };
//! ```
//!
//! S.2 readiness is not minted from arbitrary fact bags:
//!
//! ```compile_fail
//! use forge_store_contracts::ROADMAP_2_S1_SCOPE;
//! use forge_store_readiness::{S2PhysicalReadinessFacts, S2PhysicalSubstrateReadiness};
//!
//! let facts: S2PhysicalReadinessFacts = todo!();
//! let _forged = S2PhysicalSubstrateReadiness::from_admitted_physical_substrate_closeout(
//!     ROADMAP_2_S1_SCOPE,
//!     facts,
//! );
//! ```
//!
//! S.2 readiness cannot be minted directly from S.0-to-S.1 handoff readiness:
//!
//! ```compile_fail
//! use forge_store_contracts::AcceptedHandoffReadiness;
//! use forge_store_readiness::prove_s2_physical_substrate_readiness;
//!
//! let readiness: AcceptedHandoffReadiness = todo!();
//! let _forged = prove_s2_physical_substrate_readiness(readiness);
//! ```
//!
//! S.3 physical integrity readiness cannot be synthesized from raw fields:
//!
//! ```compile_fail
//! use forge_store_readiness::S3PhysicalIntegrityReadiness;
//!
//! let _forged = S3PhysicalIntegrityReadiness {
//!     s2_readiness: todo!(),
//!     payload: todo!(),
//! };
//! ```
//!
//! S.3 physical integrity readiness cannot be copied and replayed:
//!
//! ```compile_fail
//! use forge_store_readiness::S3PhysicalIntegrityReadiness;
//!
//! fn copy_readiness(
//!     readiness: S3PhysicalIntegrityReadiness,
//! ) -> (S3PhysicalIntegrityReadiness, S3PhysicalIntegrityReadiness) {
//!     (readiness, readiness)
//! }
//! ```
//!
//! S.6 certification closeout adoption cannot be minted from public scalar rows:
//!
//! ```compile_fail
//! use forge_store_readiness::{
//!     S6MaterializedCertificationAdoptionReceipt, S6ReadinessCertificationProofSummary,
//!     S6ReadinessCertificationProofTopology,
//! };
//!
//! let _from_fields = S6MaterializedCertificationAdoptionReceipt {
//!     canonical_execution_identity_tag: 7,
//!     proof_execution_identity_tag: 7,
//!     canonical_lane_binding_mask: 0b111,
//!     proof_lane_binding_mask: 0b111,
//!     profile_count: 6,
//!     profile_boundary_certification_only: true,
//!     performance_receipt_count: 5,
//!     counter_strengths: vec![],
//!     canonical_access_policy_rows: 1,
//!     canonical_post_admission_violation_rows: 1,
//!     proof: S6ReadinessCertificationProofSummary::new(true, 5, 1, 1),
//!     proof_topology: S6ReadinessCertificationProofTopology::new(
//!         true, true, true, true, true, true, true, true, true, true, true, true, 5, 5, 5,
//!     ),
//!     residual_debt_rows: vec![],
//! };
//! ```
//!
//! S.6/S.7 placement admission authority cannot be forged from raw fields:
//!
//! ```compile_fail
//! use forge_store_readiness::S6S7PlacementAdmissionAuthority;
//!
//! let _forged = S6S7PlacementAdmissionAuthority {
//!     current_authority: todo!(),
//! };
//! ```
//!
//! S.7 capsule readiness handoff cannot be synthesized from raw fields:
//!
//! ```compile_fail
//! use forge_store_readiness::S7CapsuleReadinessHandoff;
//!
//! let _forged = S7CapsuleReadinessHandoff {
//!     readiness_digest: String::new(),
//!     declared_chunk_count: 0,
//!     declared_bytes: 0,
//!     planned_chunks: 0,
//!     materialized_chunks: 0,
//!     skipped_chunks: 0,
//!     denied_chunks: 0,
//!     readiness_publications: 0,
//!     non_claims: todo!(),
//! };
//! ```
//!
//! S.7 capsule readiness handoff cannot be minted through a public constructor:
//!
//! ```compile_fail
//! use forge_store_readiness::S7CapsuleReadinessHandoff;
//!
//! let _forged = S7CapsuleReadinessHandoff::from_lower_capsule_readiness(
//!     String::new(),
//!     0,
//!     0,
//!     0,
//!     0,
//!     0,
//!     0,
//!     0,
//! );
//! ```
//!
//! S.7 capsule readiness handoff cannot be admitted from a forged lower witness:
//!
//! ```compile_fail
//! use forge_store_blob_chunks::BlobCapsuleReadinessWitness;
//! use forge_store_readiness::admit_s7_capsule_readiness_handoff;
//!
//! let forged = BlobCapsuleReadinessWitness {
//!     object_id: todo!(),
//!     generation: todo!(),
//!     chunk_tree_root: todo!(),
//!     logical_content_digest: todo!(),
//!     selected_chunks: vec![],
//!     readiness_digest: String::new(),
//!     declared_bytes: 0,
//!     counters: todo!(),
//! };
//! let _handoff = admit_s7_capsule_readiness_handoff(&forged);
//! ```
//!
//! S.7 closeout downstream non-claim vocabulary is typed and fixed-shape:
//!
//! ```compile_fail
//! use forge_store_readiness::S8LayoutReadinessNonClaim;
//!
//! let _ = S8LayoutReadinessNonClaim::ImaginaryShortcut;
//! ```
//!
mod adoption_denial;
mod aspect_native_vocabulary_readiness;
mod evidence_fields;
#[cfg(test)]
mod evidence_fields_tests;
mod foundational_adoption;
mod foundational_lanes;
mod proof_vocabulary;
mod s0_handoff;
mod s2_physical_substrate_proof;
mod s2_physical_substrate_readiness;
mod s2_readiness_denial;
mod s2_readiness_facts;
mod s3_physical_integrity_readiness;
mod s3_readiness_denial;
mod s3_readiness_payload;
mod s3_readiness_recap;
mod s5_1_later_milestone_handoffs;
mod s5_1_security_scope_admission;
mod s5_1_security_scope_vocabulary;
mod s5_simulation_harness_denial;
mod s5_simulation_harness_readiness;
mod s6_later_milestone_non_claims;
mod s6_materialized_certification_closeout;
mod s6_production_readiness_closeout;
mod s6_s7_placement_admission;
mod s7_capsule_readiness_handoff;
mod s7_closeout_handoffs;
mod s8_layout_handoff_readiness;

pub use adoption_denial::FoundationalAdoptionDenial;
pub use aspect_native_vocabulary_readiness::{
    AspectNativeVocabularyFamily, AspectNativeVocabularyPosture,
    StoreAspectNativeVocabularyReadiness,
};
pub use evidence_fields::PhysicalFoundationEvidenceField;
pub use foundational_adoption::{
    FoundationalAdoptionFamily, FoundationalAdoptionRow, FoundationalAdoptionStatus,
    FoundationalVocabularyAdoptionMap, FoundationalVocabularyAdoptionMapBuilder,
};
pub use foundational_lanes::FoundationalPublicLaneSet;
pub use proof_vocabulary::{FoundationalAdoptionDigest, ProofVocabularyAdoptionMap};
pub use s0_handoff::{
    accept_s0_aspect_native_gate_handoff, reconstruct_s0_handoff_verdict_from_native_evidence,
    reject_terminal_json_projection_as_s0_handoff, S0AspectNativeGateHandoff,
    S0AspectNativeGateHandoffDenial, S0AspectNativeGateHandoffVerdict,
};
pub use s2_physical_substrate_proof::{
    close_s1_physical_substrate_readiness, prove_s2_physical_substrate_readiness,
    S1PhysicalSubstrateCloseoutReceipt,
};
pub use s2_physical_substrate_readiness::S2PhysicalSubstrateReadiness;
pub use s2_readiness_denial::{S2ReadinessDenial, S2ReadinessDenialKind};
pub use s2_readiness_facts::{
    S2PhysicalReadinessFact, S2PhysicalReadinessFacts, S2ReadinessFactPosture,
};
pub use s3_physical_integrity_readiness::S3PhysicalIntegrityReadiness;
pub use s3_readiness_denial::{S3ReadinessDenial, S3ReadinessDenialKind};
pub use s3_readiness_payload::{
    IntegrityInspectionLifetimeLaw, ProtectedIntegrityViewCapability, S2NoMaterializationWitness,
    S3PhysicalIntegrityReadinessPayload, ScrubPlanningAllocationEnvelope, VerifierResidentEnvelope,
};
pub use s3_readiness_recap::{
    BufferPoolAuthorityRecap, PhysicalAuthorityRecap, S2BoundedCounterRecap, S2DenialBehaviorRecap,
    S2DeniedBoundaryKind,
};
pub use s5_1_later_milestone_handoffs::{
    S51LaterMilestoneHandoffCounterSnapshot, S51LaterMilestoneHandoffDenial,
    S51SecurityFoundationHandoff, S51SecurityFoundationLifecyclePermission,
    S51SecurityFoundationNonClaim,
};
pub use s5_1_security_scope_admission::{
    accept_s5_1_admitted_security_scope_readiness, S51AdmittedSecurityScopeReadiness,
};
pub use s5_1_security_scope_vocabulary::{
    S51SecurityScopeReadinessFamily, S51SecurityScopeReadinessReservation,
};
pub use s5_simulation_harness_denial::{
    reject_copied_s5_simulation_harness_readiness_fields, reject_missing_s5_correctness_non_claim,
    S5HarnessMaturityDependency, S5SimulationHarnessReadinessDenial,
};
pub use s5_simulation_harness_readiness::S5CorrectnessNonClaimEvidence;
pub use s6_later_milestone_non_claims::{
    S10BackupExportReadinessNonClaim, S10CompactionReadinessNonClaim,
    S10RepairScanReadinessNonClaim, S11OperatorReadinessNonClaim, S6LaterMilestoneDestination,
    S6LaterMilestoneHandoffDenial, S7CapsuleReadinessNonClaim, S7PlacementReadinessNonClaim,
};
pub use s6_materialized_certification_closeout::{
    reject_materialized_s6_certification_as_runtime_authority,
    S6MaterializedCertificationAdoptionDenial, S6MaterializedCertificationAdoptionReceipt,
    S6ReadinessCertificationCounterEvidence, S6ReadinessCertificationCounterFamily,
    S6ReadinessCertificationCounterStrength, S6ReadinessCertificationProofSummary,
    S6ReadinessCertificationProofTopology, S6ReadinessResidualDebtEvidenceKind,
    S6ReadinessResidualDebtEvidenceRow,
};
pub use s6_production_readiness_closeout::{
    close_s6_production_readiness, S6ClosedS10BackupExportAdmissionSeed,
    S6ClosedS10RepairAdmissionSeed, S6ClosedS11SecureIoFoundationAdmissionSeed,
    S6ClosedS7PlacementAdmissionSeed, S6ProductionReadinessClosure,
    S6ProductionReadinessClosureDenial, S6ProductionReadinessClosureInput,
    S6ProductionReadinessPosture, S6ProductionReadinessProof, S6ResidualDebtKind,
    S6ResidualDebtLedger, S6ResidualDebtRow,
};
pub use s6_s7_placement_admission::{
    admit_s6_s7_placement_handoff, S6S7PlacementAdmissionAuthority,
};
pub use s7_capsule_readiness_handoff::{
    admit_s7_capsule_readiness_handoff, S7CapsuleReadinessHandoff,
};
pub use s7_closeout_handoffs::{
    S10BackupRepairReadinessNonClaim, S11KeyLifecycleReadinessNonClaim,
    S12FullCertificationNonClaim, S8LayoutReadinessNonClaim,
};
pub use s8_layout_handoff_readiness::{
    admit_s8_layout_handoff_readiness, S8LayoutHandoffReadiness, S8LayoutHandoffReadinessDenial,
};
