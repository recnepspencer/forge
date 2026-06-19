use super::super::classification::PlanarBooleanLoopValidatorRuntimeLane as Lane;
use super::super::proof_obligation::PlanarBooleanLoopValidatorProofObligation as ValidatorProof;
use super::super::validator_row::PlanarBooleanLoopValidatorRow;

pub(super) fn phase_2_validators() -> Vec<PlanarBooleanLoopValidatorRow> {
    vec![
        query_invariant_validator("ValidatePlanarBooleanSplitLedgerConsumption"),
        query_invariant_validator("ValidateLoopReceiptEnvelopeConsistency"),
        query_invariant_validator("ValidateLoopLedgerReceiptChain"),
        query_invariant_validator("RejectLoopLedgerMissingDecisionLogReceipt"),
        query_invariant_validator("RejectLoopLedgerMissingPersistentNamingReceipt"),
        query_invariant_validator("RejectLoopLedgerForeignProductLineage"),
        spatial_validator("ValidateLoopCarrierCoverage"),
        spatial_validator("ValidateFragmentMembershipCoverage"),
        spatial_validator("ValidateLoopContinuationIndexCoverage"),
        spatial_validator("ValidateNoDanglingLoopFragmentReferences"),
        spatial_validator("ValidateCanonicalContinuationOrderingStable"),
        spatial_validator("ValidateNoNPlusOneLoopContinuationDiscovery"),
        spatial_validator("ValidateLoopContinuationOutcomeConsistency"),
        spatial_validator("ValidateLoopAmbiguityClassificationConsistency"),
        spatial_validator("ValidateCanonicalLoopSeedSelection"),
        spatial_validator("ValidateClosedWalkFragmentConsumption"),
        spatial_validator("ValidateWalkClosure"),
        spatial_validator("ValidateWalkOutcomeLocalization"),
        spatial_validator("ValidateClosedWalkPromotionBoundary"),
        spatial_validator("ValidateDeniedLoopCandidateLocalization"),
        spatial_validator("ValidateClosedIsNotAutomaticallyAdmittedLoop"),
        spatial_validator("ValidateBornLoopAttributionCoverage"),
        spatial_validator("ValidateLoopIslandPartitionConsistency"),
        spatial_validator("ValidateSourceLoopSplitAttributionConsistency"),
        spatial_validator("ValidateInnerOuterLoopFlagsConsistent"),
        spatial_validator("ValidateLoopRoleOutcomeConsistency"),
        spatial_validator("ValidateLoopContainmentEvidencePostureConsistency"),
        spatial_validator("ValidateDegenerateLoopPolicyConsistency"),
        spatial_validator("ValidateLoopHasMinimumCardinality"),
        spatial_validator("ValidateNoUnexpectedZeroAreaLoops"),
        query_invariant_validator("ValidateLoopIdentityCanonicality"),
        query_invariant_validator("ValidatePersistentNameUniqueness"),
        query_invariant_validator("ValidateNameSurvivalThroughLoopReconstruction"),
        query_invariant_validator("ValidateNoDanglingNameReferences"),
        query_invariant_validator("ValidateLoopDecisionLogCoverage"),
        query_invariant_validator("ValidateLoopFailureLocalizationConsistency"),
        query_invariant_validator("ValidateCanonicalOrderingStable"),
        query_invariant_validator("ValidateHashStabilityAcrossRuns"),
        query_invariant_validator("ValidateTieBreakerCoverage"),
        query_invariant_validator("ValidatePlanarBooleanLoopReplayParity"),
        query_invariant_validator("ValidatePlanarBooleanLoopCheckpointParity"),
        topology_review_validator("ValidateLoopOperatorQueryProgression"),
        query_invariant_validator("ValidateLoopValidatorRuntimeRegistration"),
        query_invariant_validator("ValidateLoopGraphInvariantPackRegistration"),
        query_invariant_validator("ValidatePreparedSpatialLoopProductsCannotMutateTopologyTruth"),
        topology_review_validator("ValidateTopologyDeclarationFamilyCanonicalEntries"),
    ]
}

fn spatial_validator(validator_name: &'static str) -> PlanarBooleanLoopValidatorRow {
    validator(
        validator_name,
        Lane::SpatialPreparedProductValidation,
        false,
        &[ValidatorProof::RuntimeFacingDenialPathTypedAndInspectable],
    )
}

fn query_invariant_validator(validator_name: &'static str) -> PlanarBooleanLoopValidatorRow {
    validator(
        validator_name,
        Lane::QueryGraphInvariantPack,
        true,
        &[
            ValidatorProof::RuntimeFacingDenialPathTypedAndInspectable,
            ValidatorProof::QueryInvariantRuntimeRegistration,
        ],
    )
}

fn topology_review_validator(validator_name: &'static str) -> PlanarBooleanLoopValidatorRow {
    validator(
        validator_name,
        Lane::TopologyDeclarationReview,
        true,
        &[
            ValidatorProof::RuntimeFacingDenialPathTypedAndInspectable,
            ValidatorProof::TopologyDeclarationReviewDenial,
        ],
    )
}

fn validator(
    validator_name: &'static str,
    runtime_lane: Lane,
    governs_topology_legality: bool,
    proof_obligations: &'static [ValidatorProof],
) -> PlanarBooleanLoopValidatorRow {
    PlanarBooleanLoopValidatorRow::new(
        validator_name,
        runtime_lane,
        governs_topology_legality,
        proof_obligations,
    )
}
