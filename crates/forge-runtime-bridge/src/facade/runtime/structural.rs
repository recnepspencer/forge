use super::*;
use crate::diagnostics::{
    validate_structural_replay_contract, validate_structural_replay_outcome,
    BridgeCanonicalStructuralBranchComparisonRecord, BridgeCanonicalStructuralRemapRecord,
    BridgeStructuralBranchComparisonRecord, BridgeStructuralBranchComparisonReplaySummary,
    BridgeStructuralCounters, BridgeStructuralRemapRecord, BridgeStructuralRemapReplaySummary,
};
use crate::structural::{
    classify_advisory_candidates, classify_branch_comparison, PlannedStructuralMatchPacketSet,
    PublishedBranchComparisonArtifact, PublishedStructuralRemapArtifact, ReducedStructuralMatchSet,
    StructuralFingerprint, StructuralMatchCandidate, StructuralMatchCandidateKind,
    StructuralTruthViewBasis, ValidatedStructuralIdentityDeclaration,
};

impl RuntimeBridge {
    pub fn validate_structural_declaration(
        &self,
        declaration: StructuralIdentityDeclaration,
    ) -> Result<ValidatedStructuralIdentityDeclaration, BridgeDeliveryError> {
        let contract = self.admit_structural_comparison(declaration)?;
        Ok(ValidatedStructuralIdentityDeclaration::from_contract(
            &contract,
        ))
    }

    pub fn admit_structural_comparison(
        &self,
        declaration: StructuralIdentityDeclaration,
    ) -> Result<AdmittedStructuralComparisonContract, BridgeDeliveryError> {
        self.structural_registry
            .contract_for_declaration(&declaration)
            .cloned()
            .ok_or_else(|| {
                BridgeDeliveryError::new(
                    BridgeDeliveryErrorKind::StructuralContractMismatch,
                    format!(
                        "Structural declaration `{}` was not admitted by the runtime structural registry.",
                        declaration.declaration_identity().as_str()
                    ),
                )
            })
    }

    pub fn plan_structural_match_packet_set(
        &self,
        contract: &AdmittedStructuralComparisonContract,
        candidates: Vec<StructuralMatchCandidate>,
    ) -> Result<PlannedStructuralMatchPacketSet, BridgeDeliveryError> {
        validate_candidate_kinds(contract, &candidates)?;
        let validated = ValidatedStructuralIdentityDeclaration::from_contract(contract);
        Ok(PlannedStructuralMatchPacketSet::new(
            contract.clone(),
            validated,
            None,
            None,
            candidates,
        ))
    }

    pub fn materialize_structural_fingerprint(
        &self,
        contract: &AdmittedStructuralComparisonContract,
        read_packet: SnapshotReadPacket,
    ) -> Result<StructuralFingerprint, BridgeDeliveryError> {
        let declaration = contract.validated_declaration().declaration();
        let selector = match declaration.truth_view_basis() {
            StructuralTruthViewBasis::Single { selector, .. } => selector.clone(),
            StructuralTruthViewBasis::BranchPair { .. } => {
                return Err(BridgeDeliveryError::new(
                    BridgeDeliveryErrorKind::StructuralPlanRejected,
                    format!(
                        "Structural contract `{}` requires a branch-pair basis and cannot materialize a single structural fingerprint.",
                        contract.contract_identity().as_str()
                    ),
                ))
            }
        };

        let observation = self.materialize_truth_view_observation(self.plan_truth_view_packet(
            HistoricalEvaluationDeclaration::new(
                selector,
                BridgeReplayMode::Enabled,
                BridgeDiagnosticsTier::Standard,
                BridgeDeliveryIntent::PrepareSignalEvaluation,
            ),
            read_packet,
        )?)?;

        StructuralFingerprint::from_observation(contract, &observation).map_err(|error| {
            BridgeDeliveryError::new(
                BridgeDeliveryErrorKind::SnapshotReadContractViolation,
                format!("Structural fingerprint materialization could not validate reads: {error}"),
            )
        })
    }

    pub fn materialize_structural_branch_fingerprints(
        &self,
        contract: &AdmittedStructuralComparisonContract,
        read_packet: SnapshotReadPacket,
    ) -> Result<(StructuralFingerprint, StructuralFingerprint), BridgeDeliveryError> {
        let declaration = contract.validated_declaration().declaration();
        let (left_selector, right_selector) = match declaration.truth_view_basis() {
            StructuralTruthViewBasis::BranchPair {
                left_selector,
                right_selector,
                ..
            } => (left_selector.clone(), right_selector.clone()),
            StructuralTruthViewBasis::Single { .. } => {
                return Err(BridgeDeliveryError::new(
                    BridgeDeliveryErrorKind::StructuralPlanRejected,
                    format!(
                    "Structural contract `{}` does not admit branch-pair structural comparison.",
                    contract.contract_identity().as_str()
                ),
                ))
            }
        };

        let left = self.materialize_truth_view_observation(self.plan_truth_view_packet(
            HistoricalEvaluationDeclaration::new(
                left_selector,
                BridgeReplayMode::Enabled,
                BridgeDiagnosticsTier::Standard,
                BridgeDeliveryIntent::PrepareSignalEvaluation,
            ),
            read_packet.clone(),
        )?)?;
        let right = self.materialize_truth_view_observation(self.plan_truth_view_packet(
            HistoricalEvaluationDeclaration::new(
                right_selector,
                BridgeReplayMode::Enabled,
                BridgeDiagnosticsTier::Standard,
                BridgeDeliveryIntent::PrepareSignalEvaluation,
            ),
            read_packet,
        )?)?;

        let left = StructuralFingerprint::from_observation(contract, &left).map_err(|error| {
            BridgeDeliveryError::new(
                BridgeDeliveryErrorKind::SnapshotReadContractViolation,
                format!("Structural branch comparison could not validate left-side reads: {error}"),
            )
        })?;
        let right = StructuralFingerprint::from_observation(contract, &right).map_err(|error| {
            BridgeDeliveryError::new(
                BridgeDeliveryErrorKind::SnapshotReadContractViolation,
                format!(
                    "Structural branch comparison could not validate right-side reads: {error}"
                ),
            )
        })?;

        Ok((left, right))
    }

    pub fn plan_structural_match_packet_set_from_read_packets(
        &self,
        contract: &AdmittedStructuralComparisonContract,
        target_read_packet: SnapshotReadPacket,
        candidate_read_packets: Vec<SnapshotReadPacket>,
    ) -> Result<PlannedStructuralMatchPacketSet, BridgeDeliveryError> {
        let target = self.materialize_structural_fingerprint(contract, target_read_packet)?;
        let mut candidate_fingerprints = Vec::with_capacity(candidate_read_packets.len());
        for read_packet in candidate_read_packets {
            candidate_fingerprints
                .push(self.materialize_structural_fingerprint(contract, read_packet)?);
        }

        self.plan_structural_match_packet_set(
            contract,
            classify_advisory_candidates(&target, candidate_fingerprints),
        )
        .map(|planned| {
            PlannedStructuralMatchPacketSet::new(
                planned.contract().clone(),
                planned.validated_declaration().clone(),
                Some(target),
                None,
                planned.candidates().to_vec(),
            )
        })
    }

    pub fn plan_structural_branch_comparison_from_read_packet(
        &self,
        contract: &AdmittedStructuralComparisonContract,
        read_packet: SnapshotReadPacket,
    ) -> Result<PlannedStructuralMatchPacketSet, BridgeDeliveryError> {
        let (left, right) =
            self.materialize_structural_branch_fingerprints(contract, read_packet)?;
        self.plan_structural_match_packet_set(contract, classify_branch_comparison(&left, &right))
            .map(|planned| {
                PlannedStructuralMatchPacketSet::new(
                    planned.contract().clone(),
                    planned.validated_declaration().clone(),
                    Some(left),
                    Some(right),
                    planned.candidates().to_vec(),
                )
            })
    }

    pub fn reduce_structural_match_set(
        &self,
        planned_packet_set: &PlannedStructuralMatchPacketSet,
    ) -> Result<ReducedStructuralMatchSet, BridgeDeliveryError> {
        validate_candidate_kinds(
            planned_packet_set.contract(),
            planned_packet_set.candidates(),
        )?;
        Ok(ReducedStructuralMatchSet::from_planned_packet_set(
            planned_packet_set.clone(),
        ))
    }

    pub fn publish_structural_remap_artifact(
        &self,
        reduced_match_set: &ReducedStructuralMatchSet,
    ) -> Result<PublishedStructuralRemapArtifact, BridgeDeliveryError> {
        PublishedStructuralRemapArtifact::from_reduced_match_set(reduced_match_set.clone())
            .ok_or_else(|| {
                BridgeDeliveryError::new(
                    BridgeDeliveryErrorKind::StructuralPlanRejected,
                    format!(
                        "Reduced structural match set `{}` does not describe an advisory remap publication outcome.",
                        reduced_match_set.digest()
                    ),
                )
            })
    }

    pub fn publish_branch_comparison_artifact(
        &self,
        reduced_match_set: &ReducedStructuralMatchSet,
    ) -> Result<PublishedBranchComparisonArtifact, BridgeDeliveryError> {
        PublishedBranchComparisonArtifact::from_reduced_match_set(reduced_match_set.clone())
            .ok_or_else(|| {
                BridgeDeliveryError::new(
                    BridgeDeliveryErrorKind::StructuralPlanRejected,
                    format!(
                        "Reduced structural match set `{}` does not describe a branch comparison publication outcome.",
                        reduced_match_set.digest()
                    ),
                )
            })
    }

    pub fn canonicalize_structural_remap_record(
        &self,
        contract: &AdmittedStructuralComparisonContract,
        planned_packet_set: &PlannedStructuralMatchPacketSet,
        reduced_match_set: &ReducedStructuralMatchSet,
        artifact: &PublishedStructuralRemapArtifact,
    ) -> BridgeCanonicalStructuralRemapRecord {
        let counters = BridgeStructuralCounters::from_structural_outcome(
            contract,
            planned_packet_set,
            reduced_match_set,
        );
        let record = BridgeCanonicalStructuralRemapRecord::new(BridgeStructuralRemapRecord::new(
            contract.clone(),
            planned_packet_set.clone(),
            reduced_match_set.clone(),
            artifact.clone(),
            counters,
        ));
        self.diagnostics.record_structural_remap(record.clone());
        record
    }

    pub fn canonicalize_structural_branch_comparison_record(
        &self,
        contract: &AdmittedStructuralComparisonContract,
        planned_packet_set: &PlannedStructuralMatchPacketSet,
        reduced_match_set: &ReducedStructuralMatchSet,
        artifact: &PublishedBranchComparisonArtifact,
    ) -> BridgeCanonicalStructuralBranchComparisonRecord {
        let counters = BridgeStructuralCounters::from_structural_outcome(
            contract,
            planned_packet_set,
            reduced_match_set,
        );
        let record = BridgeCanonicalStructuralBranchComparisonRecord::new(
            BridgeStructuralBranchComparisonRecord::new(
                contract.clone(),
                planned_packet_set.clone(),
                reduced_match_set.clone(),
                artifact.clone(),
                counters,
            ),
        );
        self.diagnostics
            .record_structural_branch_comparison(record.clone());
        record
    }

    pub fn replay_canonical_structural_remap_record(
        &self,
        record: &BridgeCanonicalStructuralRemapRecord,
    ) -> Result<BridgeStructuralRemapReplaySummary, BridgeReplayError> {
        let record = record.decode()?;
        let reconstructed_contract = self
            .admit_structural_comparison(
                record
                    .contract()
                    .validated_declaration()
                    .declaration()
                    .clone(),
            )
            .map_err(|error| {
                BridgeReplayError::new(
                    BridgeReplayErrorKind::PlanningContractMismatch,
                    format!(
                        "Bridge structural remap replay could not reconstruct the admitted structural contract: {error}"
                    ),
                )
                .with_context(BridgeErrorContext::default())
            })?;
        validate_structural_replay_contract(record.contract(), &reconstructed_contract)?;

        let target_fingerprint = record
            .planned_packet_set()
            .target_fingerprint()
            .ok_or_else(|| {
                BridgeReplayError::new(
                    BridgeReplayErrorKind::StructuralReplayBasisTruncated,
                    "Bridge structural remap replay requires a retained target fingerprint basis.",
                )
                .with_context(BridgeErrorContext::default())
            })?;
        let replayed_target = self
            .materialize_structural_fingerprint(
                &reconstructed_contract,
                target_fingerprint.read_packet().clone(),
            )
            .map_err(|error| {
                BridgeReplayError::new(
                    BridgeReplayErrorKind::PlanningContractMismatch,
                    format!(
                        "Bridge structural remap replay could not reconstruct the target structural fingerprint: {error}"
                    ),
                )
                .with_context(BridgeErrorContext::default())
            })?;
        if replayed_target.digest() != target_fingerprint.digest() {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::DigestMismatch,
                format!(
                    "Bridge structural remap replay reconstructed target fingerprint `{}` but original fingerprint was `{}`.",
                    replayed_target.digest(),
                    target_fingerprint.digest()
                ),
            )
            .with_context(BridgeErrorContext::default()));
        }
        let mut replayed_candidates =
            Vec::with_capacity(record.planned_packet_set().candidates().len());
        for candidate in record.planned_packet_set().candidates() {
            let fingerprint = candidate.fingerprint().ok_or_else(|| {
                BridgeReplayError::new(
                    BridgeReplayErrorKind::StructuralReplayBasisTruncated,
                    format!(
                        "Bridge structural remap replay requires retained fingerprint basis for candidate `{}`.",
                        candidate.candidate_identity().as_str()
                    ),
                )
                .with_context(BridgeErrorContext::default())
            })?;
            let replayed = self
                .materialize_structural_fingerprint(
                    &reconstructed_contract,
                    fingerprint.read_packet().clone(),
                )
                .map_err(|error| {
                    BridgeReplayError::new(
                        BridgeReplayErrorKind::PlanningContractMismatch,
                        format!(
                            "Bridge structural remap replay could not reconstruct candidate fingerprint `{}`: {error}",
                            candidate.candidate_identity().as_str()
                        ),
                    )
                    .with_context(BridgeErrorContext::default())
                })?;
            if replayed.digest() != fingerprint.digest() {
                return Err(BridgeReplayError::new(
                    BridgeReplayErrorKind::DigestMismatch,
                    format!(
                        "Bridge structural remap replay reconstructed candidate fingerprint `{}` but original fingerprint was `{}`.",
                        replayed.digest(),
                        fingerprint.digest()
                    ),
                )
                .with_context(BridgeErrorContext::default()));
            }
            replayed_candidates.push(replayed);
        }
        let planned = self
            .plan_structural_match_packet_set_from_read_packets(
                &reconstructed_contract,
                target_fingerprint.read_packet().clone(),
                replayed_candidates
                    .iter()
                    .map(|fingerprint| fingerprint.read_packet().clone())
                    .collect(),
            )
            .map_err(|error| {
                BridgeReplayError::new(
                    BridgeReplayErrorKind::PlanningContractMismatch,
                    format!(
                        "Bridge structural remap replay could not reconstruct the planned structural packet set: {error}"
                    ),
                )
                .with_context(BridgeErrorContext::default())
            })?;
        if planned.digest() != record.planned_packet_set().digest() {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::DigestMismatch,
                format!(
                    "Bridge structural remap replay reconstructed planned packet set `{}` but original packet set was `{}`.",
                    planned.digest(),
                    record.planned_packet_set().digest()
                ),
            )
            .with_context(BridgeErrorContext::default()));
        }

        let reduced = self
            .reduce_structural_match_set(&planned)
            .map_err(|error| {
                BridgeReplayError::new(
                    BridgeReplayErrorKind::PlanningContractMismatch,
                    format!(
                        "Bridge structural remap replay could not reduce the planned structural packet set: {error}"
                    ),
                )
                .with_context(BridgeErrorContext::default())
            })?;
        validate_structural_replay_outcome(
            &planned,
            &reduced,
            StructuralComparisonMode::AdvisoryRemap,
        )?;
        if reduced.digest() != record.reduced_match_set().digest() {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::DigestMismatch,
                format!(
                    "Bridge structural remap replay reconstructed reduced match set `{}` but original reduced match set was `{}`.",
                    reduced.digest(),
                    record.reduced_match_set().digest()
                ),
            )
            .with_context(BridgeErrorContext::default()));
        }

        let artifact = self.publish_structural_remap_artifact(&reduced).map_err(|error| {
            BridgeReplayError::new(
                BridgeReplayErrorKind::LoweringContractMismatch,
                format!(
                    "Bridge structural remap replay could not republish the structural remap artifact: {error}"
                ),
            )
            .with_context(BridgeErrorContext::default())
        })?;
        if artifact.digest() != record.artifact().digest() {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::LoweringContractMismatch,
                format!(
                    "Bridge structural remap replay reconstructed artifact `{}` but original artifact was `{}`.",
                    artifact.digest(),
                    record.artifact().digest()
                ),
            )
            .with_context(BridgeErrorContext::default()));
        }

        Ok(artifact)
    }

    pub fn replay_canonical_structural_branch_comparison_record(
        &self,
        record: &BridgeCanonicalStructuralBranchComparisonRecord,
    ) -> Result<BridgeStructuralBranchComparisonReplaySummary, BridgeReplayError> {
        let record = record.decode()?;
        let reconstructed_contract = self
            .admit_structural_comparison(
                record
                    .contract()
                    .validated_declaration()
                    .declaration()
                    .clone(),
            )
            .map_err(|error| {
                BridgeReplayError::new(
                    BridgeReplayErrorKind::PlanningContractMismatch,
                    format!(
                        "Bridge structural branch comparison replay could not reconstruct the admitted structural contract: {error}"
                    ),
                )
                .with_context(BridgeErrorContext::default())
            })?;
        validate_structural_replay_contract(record.contract(), &reconstructed_contract)?;

        let left_fingerprint = record
            .planned_packet_set()
            .target_fingerprint()
            .ok_or_else(|| {
                BridgeReplayError::new(
                    BridgeReplayErrorKind::StructuralReplayBasisTruncated,
                    "Bridge structural branch comparison replay requires a retained left-side fingerprint basis.",
                )
                .with_context(BridgeErrorContext::default())
            })?;
        let right_fingerprint = record
            .planned_packet_set()
            .comparison_fingerprint()
            .ok_or_else(|| {
                BridgeReplayError::new(
                    BridgeReplayErrorKind::StructuralReplayBasisTruncated,
                    "Bridge structural branch comparison replay requires a retained right-side fingerprint basis.",
                )
                .with_context(BridgeErrorContext::default())
            })?;
        let (left_replayed, right_replayed) = self
            .materialize_structural_branch_fingerprints(
                &reconstructed_contract,
                left_fingerprint.read_packet().clone(),
            )
            .map_err(|error| {
                BridgeReplayError::new(
                    BridgeReplayErrorKind::PlanningContractMismatch,
                    format!(
                        "Bridge structural branch comparison replay could not reconstruct the branch-pair fingerprint basis: {error}"
                    ),
                )
                .with_context(BridgeErrorContext::default())
            })?;
        if left_replayed.digest() != left_fingerprint.digest() {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::DigestMismatch,
                format!(
                    "Bridge structural branch comparison replay reconstructed left fingerprint `{}` but original fingerprint was `{}`.",
                    left_replayed.digest(),
                    left_fingerprint.digest()
                ),
            )
            .with_context(BridgeErrorContext::default()));
        }
        if right_replayed.digest() != right_fingerprint.digest() {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::DigestMismatch,
                format!(
                    "Bridge structural branch comparison replay reconstructed right fingerprint `{}` but original fingerprint was `{}`.",
                    right_replayed.digest(),
                    right_fingerprint.digest()
                ),
            )
            .with_context(BridgeErrorContext::default()));
        }
        let planned = self
            .plan_structural_branch_comparison_from_read_packet(
                &reconstructed_contract,
                left_fingerprint.read_packet().clone(),
            )
            .map_err(|error| {
                BridgeReplayError::new(
                    BridgeReplayErrorKind::PlanningContractMismatch,
                    format!(
                        "Bridge structural branch comparison replay could not reconstruct the planned structural packet set: {error}"
                    ),
                )
                .with_context(BridgeErrorContext::default())
            })?;
        if planned.digest() != record.planned_packet_set().digest() {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::DigestMismatch,
                format!(
                    "Bridge structural branch comparison replay reconstructed planned packet set `{}` but original packet set was `{}`.",
                    planned.digest(),
                    record.planned_packet_set().digest()
                ),
            )
            .with_context(BridgeErrorContext::default()));
        }

        let reduced = self
            .reduce_structural_match_set(&planned)
            .map_err(|error| {
                BridgeReplayError::new(
                    BridgeReplayErrorKind::PlanningContractMismatch,
                    format!(
                        "Bridge structural branch comparison replay could not reduce the planned structural packet set: {error}"
                    ),
                )
                .with_context(BridgeErrorContext::default())
            })?;
        validate_structural_replay_outcome(
            &planned,
            &reduced,
            StructuralComparisonMode::BranchComparison,
        )?;
        if reduced.digest() != record.reduced_match_set().digest() {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::DigestMismatch,
                format!(
                    "Bridge structural branch comparison replay reconstructed reduced match set `{}` but original reduced match set was `{}`.",
                    reduced.digest(),
                    record.reduced_match_set().digest()
                ),
            )
            .with_context(BridgeErrorContext::default()));
        }

        let artifact = self
            .publish_branch_comparison_artifact(&reduced)
            .map_err(|error| {
                BridgeReplayError::new(
                    BridgeReplayErrorKind::LoweringContractMismatch,
                    format!(
                        "Bridge structural branch comparison replay could not republish the branch comparison artifact: {error}"
                    ),
                )
                .with_context(BridgeErrorContext::default())
            })?;
        if artifact.digest() != record.artifact().digest() {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::LoweringContractMismatch,
                format!(
                    "Bridge structural branch comparison replay reconstructed artifact `{}` but original artifact was `{}`.",
                    artifact.digest(),
                    record.artifact().digest()
                ),
            )
            .with_context(BridgeErrorContext::default()));
        }

        Ok(artifact)
    }
}

fn validate_candidate_kinds(
    contract: &AdmittedStructuralComparisonContract,
    candidates: &[StructuralMatchCandidate],
) -> Result<(), BridgeDeliveryError> {
    let comparison_mode = contract
        .validated_declaration()
        .declaration()
        .comparison_mode();

    for candidate in candidates {
        match (comparison_mode, candidate.candidate_kind()) {
            (
                StructuralComparisonMode::AdvisoryRemap,
                StructuralMatchCandidateKind::ExactAdvisoryMatch
                | StructuralMatchCandidateKind::AdvisoryReuseCandidate
                | StructuralMatchCandidateKind::IdentityAuthorityConflict
                | StructuralMatchCandidateKind::LineageStructuralDivergence,
            )
            | (
                StructuralComparisonMode::BranchComparison,
                StructuralMatchCandidateKind::BranchDiff,
            ) => {}
            (StructuralComparisonMode::AdvisoryRemap, StructuralMatchCandidateKind::BranchDiff) => {
                return Err(BridgeDeliveryError::new(
                    BridgeDeliveryErrorKind::StructuralPlanRejected,
                    format!(
                        "Structural contract `{}` is advisory remap but candidate `{}` was classified as a branch diff.",
                        contract.contract_identity().as_str(),
                        candidate.candidate_identity().as_str()
                    ),
                ))
            }
            (StructuralComparisonMode::BranchComparison, _) => {
                return Err(BridgeDeliveryError::new(
                    BridgeDeliveryErrorKind::StructuralPlanRejected,
                    format!(
                        "Structural contract `{}` is branch comparison but candidate `{}` was not classified as a branch diff.",
                        contract.contract_identity().as_str(),
                        candidate.candidate_identity().as_str()
                    ),
                ))
            }
        }
    }

    Ok(())
}
