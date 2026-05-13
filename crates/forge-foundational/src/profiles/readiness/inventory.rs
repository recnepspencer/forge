use super::vocabulary::{
    FoundationalProfileCertifiedSurface, FoundationalProfileCertifiedSurfaceEvidence,
    FoundationalProfileCompileFailBoundary, FoundationalProfileForgeProofApi,
    FoundationalProfileForgeProofForbiddenSurface, FoundationalProfileForgeProofSurface,
    FoundationalProfileMilestone3PhaseGate, FoundationalProfilePhaseGateEvidence,
    FoundationalProfileResidualDebt, FoundationalProfileRuntimeAssumption,
    FoundationalProfileRuntimeNonAssumption, FoundationalProfileSyntheticRuntimePressure,
};

pub(super) fn certified_surfaces() -> Vec<FoundationalProfileCertifiedSurface> {
    vec![
        FoundationalProfileCertifiedSurface::ProfileFamilies,
        FoundationalProfileCertifiedSurface::ProfileComposition,
        FoundationalProfileCertifiedSurface::ProgressionAndAttachment,
        FoundationalProfileCertifiedSurface::CanonicalIdentityAndDifference,
        FoundationalProfileCertifiedSurface::MaterializationAndElision,
        FoundationalProfileCertifiedSurface::CertificationStrengthening,
    ]
}

pub(super) fn synthetic_pressures() -> Vec<FoundationalProfileSyntheticRuntimePressure> {
    vec![
        FoundationalProfileSyntheticRuntimePressure::FamilyAdjacencyHostility,
        FoundationalProfileSyntheticRuntimePressure::IndependentConstructionParity,
        FoundationalProfileSyntheticRuntimePressure::ReducedRichnessSuppression,
        FoundationalProfileSyntheticRuntimePressure::AttachmentTargetLaw,
        FoundationalProfileSyntheticRuntimePressure::ProofBearingCertificationBoundary,
    ]
}

pub(super) fn certified_surface_evidence() -> Vec<FoundationalProfileCertifiedSurfaceEvidence> {
    vec![
        FoundationalProfileCertifiedSurfaceEvidence::new(
            FoundationalProfileCertifiedSurface::ProfileFamilies,
            FoundationalProfileSyntheticRuntimePressure::FamilyAdjacencyHostility,
            FoundationalProfileCompileFailBoundary::RawLabelsCannotSatisfyProfileFamilyApis,
            "tests/certification/profiles/composition.rs",
            "tests/ui/profiles/family_boundaries/raw_string_cannot_satisfy_richness_profile.rs",
        ),
        FoundationalProfileCertifiedSurfaceEvidence::new(
            FoundationalProfileCertifiedSurface::ProfileComposition,
            FoundationalProfileSyntheticRuntimePressure::IndependentConstructionParity,
            FoundationalProfileCompileFailBoundary::PartialOrBagConstructionCannotSatisfyProfileSetApis,
            "tests/certification/profiles/composition.rs",
            "tests/ui/profiles/set_construction/raw_collection_cannot_satisfy_profile_set_api.rs",
        ),
        FoundationalProfileCertifiedSurfaceEvidence::new(
            FoundationalProfileCertifiedSurface::ProgressionAndAttachment,
            FoundationalProfileSyntheticRuntimePressure::AttachmentTargetLaw,
            FoundationalProfileCompileFailBoundary::PlainPayloadCannotSatisfyAttachmentApis,
            "tests/certification/profiles/progression_and_attachment.rs",
            "tests/ui/profiles/attachment_boundaries/plain_payload_cannot_satisfy_boundary_profiled_api.rs",
        ),
        FoundationalProfileCertifiedSurfaceEvidence::new(
            FoundationalProfileCertifiedSurface::CanonicalIdentityAndDifference,
            FoundationalProfileSyntheticRuntimePressure::IndependentConstructionParity,
            FoundationalProfileCompileFailBoundary::RawDigestCannotSatisfyProfileIdentityApis,
            "tests/certification/profiles/identity_and_difference.rs",
            "tests/ui/profiles/identity_boundaries/raw_digest_cannot_satisfy_profile_identity_api.rs",
        ),
        FoundationalProfileCertifiedSurfaceEvidence::new(
            FoundationalProfileCertifiedSurface::MaterializationAndElision,
            FoundationalProfileSyntheticRuntimePressure::ReducedRichnessSuppression,
            FoundationalProfileCompileFailBoundary::IllegalTargetSurfaceInventoriesCannotBeForged,
            "tests/certification/profiles/materialization.rs",
            "tests/ui/profiles/materialization_boundaries/target_surface_inventory_fields_are_private.rs",
        ),
        FoundationalProfileCertifiedSurfaceEvidence::new(
            FoundationalProfileCertifiedSurface::CertificationStrengthening,
            FoundationalProfileSyntheticRuntimePressure::ProofBearingCertificationBoundary,
            FoundationalProfileCompileFailBoundary::WrongStrengthProofBearingCertificationCannotSatisfyStrongerApis,
            "tests/certification/profiles/certification_posture.rs",
            "tests/ui/profiles/certification_boundaries/evidence_backed_artifact_cannot_satisfy_production_certified_boundary_api.rs",
        ),
    ]
}

pub(super) fn compile_fail_boundaries() -> Vec<FoundationalProfileCompileFailBoundary> {
    vec![
        FoundationalProfileCompileFailBoundary::RawLabelsCannotSatisfyProfileFamilyApis,
        FoundationalProfileCompileFailBoundary::PartialOrBagConstructionCannotSatisfyProfileSetApis,
        FoundationalProfileCompileFailBoundary::PlainPayloadCannotSatisfyAttachmentApis,
        FoundationalProfileCompileFailBoundary::RawDigestCannotSatisfyProfileIdentityApis,
        FoundationalProfileCompileFailBoundary::IllegalTargetSurfaceInventoriesCannotBeForged,
        FoundationalProfileCompileFailBoundary::WrongStrengthProofBearingCertificationCannotSatisfyStrongerApis,
        FoundationalProfileCompileFailBoundary::ProfileReadinessRequiresCertifiedArtifact,
    ]
}

pub(super) fn forge_proof_required_surfaces() -> Vec<FoundationalProfileForgeProofSurface> {
    vec![
        FoundationalProfileForgeProofSurface::ArtifactCarrier,
        FoundationalProfileForgeProofSurface::TransitionOutcome,
        FoundationalProfileForgeProofSurface::AuthorityWitness,
        FoundationalProfileForgeProofSurface::BoundaryBridgeTrustBoundary,
        FoundationalProfileForgeProofSurface::BoundaryReadmitWithAuthority,
        FoundationalProfileForgeProofSurface::CurrentBasisArtifactConstructor,
    ]
}

pub(super) fn forge_proof_api_appendix() -> Vec<FoundationalProfileForgeProofApi> {
    vec![
        FoundationalProfileForgeProofApi::AuthorityWitnessFromAuthorityMarker,
        FoundationalProfileForgeProofApi::ArtifactNew,
        FoundationalProfileForgeProofApi::ArtifactWithCurrentBasis,
        FoundationalProfileForgeProofApi::ArtifactWithProofsAndCurrentBasis,
        FoundationalProfileForgeProofApi::TransitionOutcomeStructuredCategories,
        FoundationalProfileForgeProofApi::ArtifactBridgeTrustBoundary,
        FoundationalProfileForgeProofApi::ArtifactReadmitWithAuthority,
    ]
}

pub(super) fn forge_proof_forbidden_surfaces() -> Vec<FoundationalProfileForgeProofForbiddenSurface>
{
    vec![
        FoundationalProfileForgeProofForbiddenSurface::PlainProfileFamilyVocabulary,
        FoundationalProfileForgeProofForbiddenSurface::PlainProfileCompositionData,
        FoundationalProfileForgeProofForbiddenSurface::PlainDescriptiveSurfaceVocabulary,
        FoundationalProfileForgeProofForbiddenSurface::PlainProfileIdentityBasisEntries,
    ]
}

pub(super) fn runtime_assumptions() -> Vec<FoundationalProfileRuntimeAssumption> {
    vec![
        FoundationalProfileRuntimeAssumption::CanonicalBasisLawCertified,
        FoundationalProfileRuntimeAssumption::ProfileMeaningRemainsFacadeControlled,
        FoundationalProfileRuntimeAssumption::ReducedRichnessAffectsOnlyOptionalDescriptiveSurfaces,
        FoundationalProfileRuntimeAssumption::ProofBearingCertificationUsesExplicitAuthorityProgression,
    ]
}

pub(super) fn runtime_non_assumptions() -> Vec<FoundationalProfileRuntimeNonAssumption> {
    vec![
        FoundationalProfileRuntimeNonAssumption::RuntimePolicyExecutionExistsInFoundational,
        FoundationalProfileRuntimeNonAssumption::AdoptingCrateLoweringParityAlreadyProven,
        FoundationalProfileRuntimeNonAssumption::DiagnosticsOrProvenanceOntologyAlreadyOwnedHere,
        FoundationalProfileRuntimeNonAssumption::BoundaryCrossingPreservesStrongerCertificationWithoutReadmission,
    ]
}

pub(super) fn residual_debt() -> Vec<FoundationalProfileResidualDebt> {
    vec![
        FoundationalProfileResidualDebt::AdoptingCrateParityDeferred,
        FoundationalProfileResidualDebt::RealRuntimePolicyLoweringDeferred,
        FoundationalProfileResidualDebt::LaterArtifactDiagnosticsAndProvenanceOntologyDeferred,
    ]
}

pub(super) fn phase_gates() -> Vec<FoundationalProfilePhaseGateEvidence> {
    vec![
        FoundationalProfilePhaseGateEvidence::new(
            FoundationalProfileMilestone3PhaseGate::TypedFamilies,
            "tests/certification/profiles/composition.rs",
        ),
        FoundationalProfilePhaseGateEvidence::new(
            FoundationalProfileMilestone3PhaseGate::ComposedProfileSet,
            "tests/certification/profiles/composition.rs",
        ),
        FoundationalProfilePhaseGateEvidence::new(
            FoundationalProfileMilestone3PhaseGate::ProgressionAndAttachment,
            "tests/certification/profiles/progression_and_attachment.rs",
        ),
        FoundationalProfilePhaseGateEvidence::new(
            FoundationalProfileMilestone3PhaseGate::CanonicalIdentityAndDifference,
            "tests/certification/profiles/identity_and_difference.rs",
        ),
        FoundationalProfilePhaseGateEvidence::new(
            FoundationalProfileMilestone3PhaseGate::MaterializationAndElision,
            "tests/certification/profiles/materialization.rs",
        ),
        FoundationalProfilePhaseGateEvidence::new(
            FoundationalProfileMilestone3PhaseGate::CertificationStrengthening,
            "tests/certification/profiles/certification_posture.rs",
        ),
        FoundationalProfilePhaseGateEvidence::new(
            FoundationalProfileMilestone3PhaseGate::ProductionReadiness,
            "tests/certification/profiles/readiness.rs",
        ),
    ]
}
