use super::BridgeOwnedSignalRuntime;
use crate::facade::{
    AdmittedBridgeAsyncRequestIdentity, BridgeAsyncCompletionAdmissionReport,
    BridgeAsyncCompletionRejection, BridgeAsyncForwardCausalityRejection,
    BridgeAsyncForwardCausalityRejectionKind, BridgeAsyncRequestAdmissionRequest,
    BridgeAsyncRequestIdentityRejection, BridgeAsyncRequestTruthViewBasis, BridgeAsyncRetryLineage,
    BridgeAsyncRetryLineageRequest, BridgeAsyncRevalidationLineage,
    BridgeAsyncRevalidationLineageRequest, BridgeAsyncSourceDeclarationDraft,
    BridgeAsyncSourceDeclarationRejection, BridgeMixedCauseOrdering, BridgeMixedCauseOrderingInput,
    BridgeMixedCauseOrderingLaneKind, BridgeMixedCauseOrderingRequest,
    LoweredBridgeAsyncSourceDeclaration, ValidatedBridgeAsyncCompletionEnvelope,
    ValidatedBridgeAsyncRequestBasisBinding, ValidatedBridgeAsyncSourceDeclaration,
};
use std::sync::Arc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeOwnedAsyncRequestResponseDeclaration {
    identity: Arc<str>,
    legacy_identity: Arc<str>,
    payload_contract: u64,
    max_payload_bytes: u64,
    retry_max_attempts: u32,
}

impl BridgeOwnedAsyncRequestResponseDeclaration {
    pub fn new(
        identity: impl Into<Arc<str>>,
        legacy_identity: impl Into<Arc<str>>,
        payload_contract: u64,
        max_payload_bytes: u64,
        retry_max_attempts: u32,
    ) -> Self {
        Self {
            identity: identity.into(),
            legacy_identity: legacy_identity.into(),
            payload_contract,
            max_payload_bytes,
            retry_max_attempts,
        }
    }
}

impl BridgeOwnedSignalRuntime {
    pub fn install_owned_async_request_response(
        &mut self,
        declaration: BridgeOwnedAsyncRequestResponseDeclaration,
    ) -> Result<LoweredBridgeAsyncSourceDeclaration, BridgeAsyncSourceDeclarationRejection> {
        // Draft lowering only needs a structurally valid Signal node identity. The
        // live node is installed later, when the exact request is admitted into
        // this retained runtime. Keeping the draft node in the owned graph would
        // leave one unreachable node behind for every installed declaration.
        let mut draft_graph = worth_signal::facade::SignalGraph::new();
        let node = draft_graph.node().build();
        let draft = BridgeAsyncSourceDeclarationDraft::request_response(
            crate::facade::BridgeAsyncSourceDeclarationIdentity::admit_bridge_owned(
                declaration.identity,
            ),
            crate::facade::BridgeAsyncSourceLegacyDeclarationIdentity::admit_bridge_owned(
                declaration.legacy_identity,
            ),
            worth_signal::facade::ResourceNodeDeclaration::new(
                worth_signal::facade::ResourceNodeId::from_node(node),
                worth_signal::facade::ResourcePayloadContract::new(
                    worth_signal::facade::ResourcePayloadContractId::new(
                        declaration.payload_contract,
                    ),
                )
                .with_max_payload_bytes(declaration.max_payload_bytes),
            )
            .with_observation_policy(
                worth_signal::facade::ResourceObservationPolicyDeclaration::LifecycleOnly,
            )
            .with_retry_max_attempts(declaration.retry_max_attempts),
        );
        let validated = self.validate_owned_async_source_declaration(draft)?;
        self.lower_owned_async_source_declaration(&validated)
    }

    pub fn validate_owned_async_source_declaration(
        &self,
        draft: BridgeAsyncSourceDeclarationDraft,
    ) -> Result<ValidatedBridgeAsyncSourceDeclaration, BridgeAsyncSourceDeclarationRejection> {
        self.bridge.validate_async_source_declaration(draft)
    }

    pub fn lower_owned_async_source_declaration(
        &self,
        declaration: &ValidatedBridgeAsyncSourceDeclaration,
    ) -> Result<LoweredBridgeAsyncSourceDeclaration, BridgeAsyncSourceDeclarationRejection> {
        self.bridge.lower_async_source_declaration(declaration)
    }

    pub fn bind_owned_async_request_basis(
        &self,
        lowered: &LoweredBridgeAsyncSourceDeclaration,
        truth_view_basis: BridgeAsyncRequestTruthViewBasis,
    ) -> ValidatedBridgeAsyncRequestBasisBinding {
        self.bridge
            .bind_async_request_basis(lowered, truth_view_basis)
    }

    pub fn admit_owned_async_request_identity(
        &mut self,
        request: BridgeAsyncRequestAdmissionRequest,
    ) -> Result<super::BridgeOwnedAsyncRequestAdmission, BridgeAsyncRequestIdentityRejection> {
        let request = AdmittedBridgeAsyncRequestIdentity::admit_owned(
            self.bridge.signal_runtime_key,
            &mut self.signal_runtime,
            &mut self.async_declarations,
            request,
        )?;
        Ok(super::BridgeOwnedAsyncRequestAdmission::new(
            &self.async_observation_authority,
            request,
        ))
    }

    pub fn retire_owned_async_request(
        &mut self,
        request: &AdmittedBridgeAsyncRequestIdentity,
    ) -> Result<bool, worth_signal::facade::SignalError> {
        let _cancellation = self.signal_runtime.cancel_resource_request(
            request.request_handle(),
            worth_signal::facade::ResourceCancellationReason::HostRequested,
        )?;
        match request.family_admission() {
            crate::source::BridgeAsyncRequestFamilyAdmission::RequestResponse => {
                crate::source::retire_owned_resource_declaration_for_lowering(
                    &mut self.async_declarations,
                    &mut self.signal_runtime,
                    request.lowered().lowering_identity().as_str(),
                )
            }
            crate::source::BridgeAsyncRequestFamilyAdmission::SubscriptionBacked { .. } => {
                crate::source::retire_owned_async_declaration_for_lowering(
                    &mut self.async_declarations,
                    &mut self.signal_runtime,
                    request.lowered().lowering_identity().as_str(),
                )
            }
        }
    }

    pub fn owned_async_declaration_count(&self) -> usize {
        self.async_declarations.len()
    }

    pub fn owned_signal_active_node_count(&self) -> usize {
        self.signal_runtime.graph().active_node_count()
    }

    pub fn validate_owned_async_completion_envelope(
        &self,
        request: &AdmittedBridgeAsyncRequestIdentity,
        raw: worth_signal::facade::RawCompletionEnvelope,
    ) -> Result<ValidatedBridgeAsyncCompletionEnvelope, BridgeAsyncCompletionRejection> {
        self.bridge.validate_async_completion_envelope(request, raw)
    }

    pub fn admit_owned_async_completion(
        &mut self,
        request: &AdmittedBridgeAsyncRequestIdentity,
        validated: &ValidatedBridgeAsyncCompletionEnvelope,
    ) -> Result<BridgeAsyncCompletionAdmissionReport, BridgeAsyncCompletionRejection> {
        validated.admit(&mut self.signal_runtime, request)
    }

    /// Admits a Signal completion after the audience owner has certified that
    /// the corresponding external effects cannot be determined. The marker is
    /// minted only inside Bridge; callers cannot reclassify an arbitrary
    /// completion report after admission.
    pub fn admit_owned_async_effects_indeterminate(
        &mut self,
        observation: super::BridgeAsyncEffectsIndeterminateObservation,
    ) -> Result<BridgeAsyncCompletionAdmissionReport, BridgeAsyncCompletionRejection> {
        let (authority, request, raw) = observation.into_parts();
        if !Arc::ptr_eq(&authority, &self.async_observation_authority) {
            return Err(BridgeAsyncCompletionRejection::new(
                crate::facade::BridgeAsyncCompletionRejectionKind::ForeignOwnerObservationAuthority,
                "effects-indeterminate observation belongs to another owned runtime",
            ));
        }
        let validated = self.validate_owned_async_completion_envelope(&request, raw)?;
        let report = validated.admit(&mut self.signal_runtime, &request)?;
        Ok(report.from_owner_effects_indeterminate(
            crate::source::BridgeAsyncEffectsIndeterminateCompletion::from_owner_observation(),
        ))
    }

    pub fn order_owned_async_completion_report(
        &self,
        report: &BridgeAsyncCompletionAdmissionReport,
    ) -> BridgeMixedCauseOrdering {
        let input = match (report.admitted_completion(), report.denied_completion()) {
            (Some(completion), None) => {
                BridgeMixedCauseOrderingInput::AsyncCompletion(completion.clone())
            }
            (None, Some(completion)) => {
                BridgeMixedCauseOrderingInput::AsyncDeniedCompletion(completion.clone())
            }
            _ => unreachable!("Bridge completion report retains exactly one outcome"),
        };
        self.bridge
            .order_mixed_causes(&BridgeMixedCauseOrderingRequest::new(
                BridgeMixedCauseOrderingLaneKind::Authoritative,
                vec![input],
            ))
    }

    pub fn admit_owned_async_retry_lineage(
        &mut self,
        request: BridgeAsyncRetryLineageRequest,
    ) -> Result<BridgeAsyncRetryLineage, BridgeAsyncForwardCausalityRejection> {
        crate::source::admit_retry_lineage(&mut self.signal_runtime, request)
    }

    pub fn admit_owned_async_revalidation_lineage(
        &mut self,
        request: BridgeAsyncRevalidationLineageRequest,
    ) -> Result<BridgeAsyncRevalidationLineage, BridgeAsyncForwardCausalityRejection> {
        crate::source::admit_revalidation_lineage(&mut self.signal_runtime, request)
    }

    pub fn revalidate_owned_async_request(
        &mut self,
        prior: &AdmittedBridgeAsyncRequestIdentity,
        current_truth_view_basis: BridgeAsyncRequestTruthViewBasis,
    ) -> Result<BridgeAsyncRevalidationLineage, BridgeAsyncForwardCausalityRejection> {
        let report = self
            .signal_runtime
            .revalidate_resource_node(
                worth_signal::facade::ResourceRevalidationIntent::with_expected_active(
                    worth_signal::facade::ResourceNodeId::from_node(
                        prior.in_flight_identity().in_flight().node().node(),
                    ),
                    prior.request_handle(),
                ),
            )
            .map_err(|error| {
                BridgeAsyncForwardCausalityRejection::new(
                    BridgeAsyncForwardCausalityRejectionKind::RevalidationAdmissionMissing,
                    format!("Signal rejected owned async request revalidation: {error:?}"),
                )
            })?;
        self.admit_owned_async_revalidation_lineage(
            BridgeAsyncRevalidationLineageRequest::request_response(
                prior,
                current_truth_view_basis,
                &report,
            ),
        )
    }
}
