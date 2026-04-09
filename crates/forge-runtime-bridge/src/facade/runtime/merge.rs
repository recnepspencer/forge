use super::*;

impl RuntimeBridge {
    fn merge_publication_error_kind(
        reduced_routing_artifact: &ReducedMergeRoutingArtifact,
    ) -> BridgeMergeErrorKind {
        match reduced_routing_artifact.lowered_packet_set().denial_class() {
            Some(BridgeMergeDenialClass::CausalFrontierTruncated) => {
                BridgeMergeErrorKind::MergeCausalFrontierTruncated
            }
            Some(BridgeMergeDenialClass::SchemaPolicyRejected) => {
                BridgeMergeErrorKind::MergePolicyRejected
            }
            Some(BridgeMergeDenialClass::DeletionGate) => BridgeMergeErrorKind::MergeDeletionDenied,
            Some(BridgeMergeDenialClass::TopologyRewireGate) => {
                BridgeMergeErrorKind::MergeTopologyRewireDenied
            }
            Some(BridgeMergeDenialClass::UnsupportedMergeClass)
            | Some(BridgeMergeDenialClass::NoAuthoritativeSuccessor) => {
                BridgeMergeErrorKind::MergeContinuityDenied
            }
            None if reduced_routing_artifact.outcome_class()
                == BridgeMergeRoutingOutcomeClass::StructuralContradiction =>
            {
                BridgeMergeErrorKind::MergeStructuralContradiction
            }
            None => BridgeMergeErrorKind::MergeContinuityDenied,
        }
    }

    pub fn merge_registry(&self) -> &AdmittedMergeRegistry {
        &self.merge_registry
    }

    pub fn validate_merge_declaration(
        &self,
        declaration: MergeHistoryDeclaration,
    ) -> Result<ValidatedMergeHistoryDeclaration, BridgeMergeError> {
        let contract = self.admit_merge_history(declaration)?;
        Ok(contract.validated_declaration().clone())
    }

    pub fn admit_merge_history(
        &self,
        declaration: MergeHistoryDeclaration,
    ) -> Result<AdmittedMergeHistoryContract, BridgeMergeError> {
        self.merge_registry
            .contract_for_declaration(&declaration)
            .cloned()
            .ok_or_else(|| {
                BridgeMergeError::new(
                    BridgeMergeErrorKind::MergeContractMismatch,
                    format!(
                        "Merge declaration `{}` was not admitted by the runtime merge registry.",
                        declaration.declaration_identity().as_str()
                    ),
                )
            })
    }

    pub fn lower_merge_history(
        &self,
        contract: &AdmittedMergeHistoryContract,
    ) -> Result<LoweredMergeHistoryPacketSet, BridgeMergeError> {
        self.merge_registry
            .contract_for_identity(contract.contract_identity().as_str())
            .ok_or_else(|| {
                BridgeMergeError::new(
                    BridgeMergeErrorKind::MergeContractMismatch,
                    format!(
                        "Merge contract `{}` was not admitted by the runtime merge registry.",
                        contract.contract_identity().as_str()
                    ),
                )
            })?;
        Ok(LoweredMergeHistoryPacketSet::from_contract(contract))
    }

    pub fn reduce_merge_routing(
        &self,
        lowered_packet_set: &LoweredMergeHistoryPacketSet,
    ) -> Result<ReducedMergeRoutingArtifact, BridgeMergeError> {
        self.merge_registry
            .contract_for_identity(
                lowered_packet_set
                    .contract()
                    .contract_identity()
                    .as_str(),
            )
            .ok_or_else(|| {
                BridgeMergeError::new(
                    BridgeMergeErrorKind::MergeContractMismatch,
                    format!(
                        "Lowered merge packet set `{}` referenced contract `{}` that is not admitted by the runtime merge registry.",
                        lowered_packet_set.digest(),
                        lowered_packet_set.contract().contract_identity().as_str()
                    ),
                )
            })?;
        Ok(ReducedMergeRoutingArtifact::from_lowered_packet_set(
            lowered_packet_set.clone(),
        ))
    }

    pub fn publish_merge_continuity_artifact(
        &self,
        reduced_routing_artifact: &ReducedMergeRoutingArtifact,
    ) -> Result<PublishedMergeContinuityArtifact, BridgeMergeError> {
        PublishedMergeContinuityArtifact::from_reduced_routing_artifact(
            reduced_routing_artifact.clone(),
        )
        .ok_or_else(|| {
            BridgeMergeError::new(
                Self::merge_publication_error_kind(reduced_routing_artifact),
                format!(
                    "Reduced merge routing artifact `{}` does not admit continuity publication.",
                    reduced_routing_artifact.digest()
                ),
            )
        })
    }

    pub fn publish_merge_remap_artifact(
        &self,
        reduced_routing_artifact: &ReducedMergeRoutingArtifact,
    ) -> Result<PublishedMergeRemapArtifact, BridgeMergeError> {
        PublishedMergeRemapArtifact::from_reduced_routing_artifact(
            reduced_routing_artifact.clone(),
        )
        .ok_or_else(|| {
            BridgeMergeError::new(
                Self::merge_publication_error_kind(reduced_routing_artifact),
                format!(
                    "Reduced merge routing artifact `{}` does not admit advisory remap publication.",
                    reduced_routing_artifact.digest()
                ),
            )
        })
    }

    pub fn publish_merge_explanation_artifact(
        &self,
        lowered_packet_set: &LoweredMergeHistoryPacketSet,
        reduced_routing_artifact: &ReducedMergeRoutingArtifact,
        continuity_artifact: Option<&PublishedMergeContinuityArtifact>,
        remap_artifact: Option<&PublishedMergeRemapArtifact>,
    ) -> PublishedMergeExplanationArtifact {
        PublishedMergeExplanationArtifact::from_merge_result(
            lowered_packet_set,
            reduced_routing_artifact,
            continuity_artifact,
            remap_artifact,
        )
    }

    pub fn replay_merge_history(
        &self,
        contract: &AdmittedMergeHistoryContract,
    ) -> Result<MergeReplayCertificationBundle, BridgeMergeError> {
        let lowered = self.lower_merge_history(contract)?;
        let mut reduced = self.reduce_merge_routing(&lowered)?;
        let continuity =
            PublishedMergeContinuityArtifact::from_reduced_routing_artifact(reduced.clone());
        let remap = PublishedMergeRemapArtifact::from_reduced_routing_artifact(reduced.clone());
        reduced = reduced
            .clone()
            .with_counters(reduced.counters().with_explanation_request());
        if remap.is_some() {
            reduced = reduced
                .clone()
                .with_counters(reduced.counters().with_remap_publication());
        }
        let explanation = self.publish_merge_explanation_artifact(
            &lowered,
            &reduced,
            continuity.as_ref(),
            remap.as_ref(),
        );

        Ok(MergeReplayCertificationBundle::new(
            contract.clone(),
            lowered,
            reduced,
            continuity,
            remap,
            explanation,
        ))
    }

    pub fn canonicalize_merge_record(
        &self,
        bundle: &MergeReplayCertificationBundle,
    ) -> BridgeCanonicalMergeRecord {
        let record = BridgeCanonicalMergeRecord::new(BridgeMergeRecord::new(bundle.clone()));
        self.diagnostics.record_merge(record.clone());
        record
    }

    pub fn replay_canonical_merge_record(
        &self,
        record: &BridgeCanonicalMergeRecord,
    ) -> Result<BridgeMergeReplaySummary, BridgeReplayError> {
        let record = record.decode()?;
        let reconstructed_contract = self
            .admit_merge_history(
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
                        "Bridge merge replay could not reconstruct the admitted merge contract: {error}"
                    ),
                )
                .with_context(BridgeErrorContext::default())
            })?;
        if reconstructed_contract.digest() != record.contract().digest() {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::PlanningContractMismatch,
                format!(
                    "Bridge merge replay reconstructed contract `{}` but original contract was `{}`.",
                    reconstructed_contract.contract_identity().as_str(),
                    record.contract().contract_identity().as_str()
                ),
            )
            .with_context(BridgeErrorContext::default()));
        }

        let replayed = self
            .replay_merge_history(&reconstructed_contract)
            .map_err(|error| {
                BridgeReplayError::new(
                    BridgeReplayErrorKind::LoweringContractMismatch,
                    format!("Bridge merge replay could not reconstruct the merge bundle: {error}"),
                )
                .with_context(BridgeErrorContext::default())
            })?;
        let replayed = replayed.with_replay_request();

        if replayed.digest() != record.bundle().digest() {
            let replayed = replayed.with_replay_mismatch();
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::DigestMismatch,
                format!(
                    "Bridge merge replay reconstructed bundle `{}` but original bundle was `{}`.",
                    replayed.digest(),
                    record.bundle().digest()
                ),
            )
            .with_context(BridgeErrorContext::default()));
        }

        Ok(replayed)
    }
}
