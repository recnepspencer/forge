use std::collections::BTreeMap;

use forge_query::facade::{
    DeclarativeLiveQueryRequest, ForgeQueryEffectPolicy, ForgeQueryEntity, ForgeQueryLivePatch,
    ForgeQueryLiveViewHandle, ForgeQueryMutationReceipt, ForgeQueryPreviewBasisAdmission,
    ForgeQueryRuntimeEvidenceAuthority, ForgeQueryRuntimeInspectionEvidence,
    ForgeQueryRuntimeInspectorEvidenceAdapter, ForgeQueryRuntimePreviewBasisAdapter,
    ForgeQueryRuntimeSchemaAdapter, ForgeQueryRuntimeSignalSinkAdapter,
    ForgeQueryRuntimeSourceAdapter, ForgeQueryRuntimeSubscriptionActivationAdapter,
    ForgeQueryWorkspaceError, ForgeQueryWriteReceipt, QuerySchemaView, SubscriptionActivationInput,
};
use forge_runtime_bridge::facade::{
    BridgeDeliveryReceipt, InvalidationSink, RuntimeBridge, SignalBridgeSinkError,
};

mod binding;
pub(super) mod bridge_source;
mod bridge_source_support;
mod existing_truth_verification;
mod query_rows;
pub(super) mod write_authority;
pub(super) mod write_support;

pub(crate) use self::binding::TopologyRuntimeBinding;
use self::bridge_source::TopologyRuntimeBridgeSource;
pub(crate) use self::existing_truth_verification::TopologyExistingTruthVerificationAdapter;
use self::query_rows::{persistent_name_rows, topology_entity_rows, topology_relation_rows};

pub(super) fn build_runtime_bridge(
    binding: TopologyRuntimeBinding,
) -> Result<RuntimeBridge, forge_runtime_bridge::facade::BridgeBuildError> {
    use forge_runtime_bridge::facade::RuntimeBridgeBuilder;

    let source = TopologyRuntimeBridgeSource::new(binding);
    let builder = RuntimeBridgeBuilder::new()
        .with_relational_source(source.clone())
        .with_truth_branch_head_source(source)
        .with_signal_sink(TopologyStaticBridgeSink);
    let mut mappings =
        crate::projection::runtime_boundary::bridge::milestone_one_bridge_mapping_registrations()
            .into_iter();
    let first = mappings
        .next()
        .expect(" milestone 1 bridge mapping pack should not be empty");
    let builder = mappings.fold(builder.register_mapping(first), |builder, registration| {
        builder.register_mapping(registration)
    });
    let builder =
        crate::projection::runtime_boundary::bridge::milestone_one_bridge_aspect_registrations()
            .into_iter()
            .fold(builder, |builder, registration| {
                builder.register_aspect_mapping(registration)
            });
    builder.build()
}

pub(super) struct TopologyRuntimeSchemaAdapter;

impl ForgeQueryRuntimeSchemaAdapter for TopologyRuntimeSchemaAdapter {
    fn admit_live_view(
        &self,
        name: &str,
        request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<
        forge_query::facade::LiveViewDeclarationAdmissionBoundaryReceipt,
        ForgeQueryWorkspaceError,
    > {
        match request.target() {
            "TopologyEntity" | "TopologyRelation" | "PersistentName" => {
                let admission = self.build_live_view_declaration_admission_receipt(name, request);
                Ok(self.build_live_view_declaration_boundary_receipt(name, request, admission))
            }
            other => Err(ForgeQueryWorkspaceError::new(format!(
                "topology production runtime does not admit live view target `{other}` yet"
            ))),
        }
    }
}

pub(super) struct TopologyRuntimeSourceAdapter {
    binding: TopologyRuntimeBinding,
    live_views: BTreeMap<String, String>,
}

impl TopologyRuntimeSourceAdapter {
    pub(super) fn new(binding: TopologyRuntimeBinding) -> Self {
        Self {
            binding,
            live_views: BTreeMap::new(),
        }
    }
}

impl ForgeQueryRuntimeSourceAdapter for TopologyRuntimeSourceAdapter {
    fn declare_live_view(
        &mut self,
        name: String,
        request: DeclarativeLiveQueryRequest,
        _schema_view: QuerySchemaView,
    ) -> Result<ForgeQueryLiveViewHandle, ForgeQueryWorkspaceError> {
        self.live_views
            .insert(name.clone(), request.target().to_string());
        Ok(ForgeQueryLiveViewHandle::new(name))
    }

    fn live_entities(&self, view_name: &str) -> Vec<ForgeQueryEntity> {
        let Some(target) = self.live_views.get(view_name) else {
            return Vec::new();
        };
        match target.as_str() {
            "TopologyEntity" => topology_entity_rows(&self.binding),
            "TopologyRelation" => topology_relation_rows(&self.binding),
            "PersistentName" => persistent_name_rows(&self.binding),
            _ => Vec::new(),
        }
    }

    fn drain_live_patches(&mut self, _view_name: &str) -> Vec<ForgeQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_ids(&self, receipt: &ForgeQueryMutationReceipt) -> Vec<String> {
        let mut affected = receipt
            .deltas
            .iter()
            .flat_map(|delta| {
                self.live_views
                    .iter()
                    .filter(move |(_, target)| *target == &delta.collection)
                    .map(|(name, _)| name.clone())
            })
            .collect::<Vec<_>>();
        affected.sort();
        affected.dedup();
        affected
    }

    fn snapshot_token(&self) -> String {
        self.binding.snapshot_token()
    }
}

pub(super) struct TopologyStaticSignalSink;

impl ForgeQueryRuntimeSignalSinkAdapter for TopologyStaticSignalSink {
    fn route_write_receipt(
        &mut self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> Result<forge_query::facade::SignalInvalidationBoundaryReceipt, ForgeQueryWorkspaceError>
    {
        let routing_receipt = self.build_signal_invalidation_routing_receipt(receipt);
        Ok(self.build_signal_invalidation_boundary_receipt(receipt, routing_receipt))
    }
}

pub(super) struct TopologySubscriptionActivation {
    support_evidence: &'static str,
}

impl TopologySubscriptionActivation {
    pub(super) fn new(support_evidence: &'static str) -> Self {
        Self { support_evidence }
    }
}

impl ForgeQueryRuntimeSubscriptionActivationAdapter for TopologySubscriptionActivation {
    fn support_evidence(&self) -> String {
        self.support_evidence.to_string()
    }
    fn admit_activation(
        &mut self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Result<forge_query::facade::SubscriptionActivationBoundaryReceipt, ForgeQueryWorkspaceError>
    {
        let activation_receipt = self.build_subscription_activation_receipt(view_name, activation);
        Ok(self.build_subscription_activation_boundary_receipt(
            view_name,
            activation,
            activation_receipt,
        ))
    }
}

pub(super) struct TopologyPreviewBasis {
    denial_reason: &'static str,
}

impl TopologyPreviewBasis {
    pub(super) fn new(denial_reason: &'static str) -> Self {
        Self { denial_reason }
    }
}

impl ForgeQueryRuntimePreviewBasisAdapter for TopologyPreviewBasis {
    fn admit_preview_basis(
        &self,
        _label: &str,
        _effect_policy: ForgeQueryEffectPolicy,
        _authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError> {
        Err(ForgeQueryWorkspaceError::new(self.denial_reason))
    }
}

pub(super) struct TopologyInspectorEvidence {
    receipt_label: &'static str,
    evidence_label: &'static str,
}

impl TopologyInspectorEvidence {
    pub(super) fn new(receipt_label: &'static str, evidence_label: &'static str) -> Self {
        Self {
            receipt_label,
            evidence_label,
        }
    }
}

impl ForgeQueryRuntimeInspectorEvidenceAdapter for TopologyInspectorEvidence {
    fn inspect_write_receipt(
        &self,
        receipt: &ForgeQueryWriteReceipt,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryRuntimeInspectionEvidence::new(
            authority,
            self.receipt_label,
            receipt.authority_lane(),
            [self.evidence_label],
        ))
    }
}

#[derive(Clone)]
struct TopologyStaticBridgeSink;

impl InvalidationSink for TopologyStaticBridgeSink {
    fn deliver_invalidation(
        &self,
        delivery: forge_runtime_bridge::facade::BridgeSignalInvalidationDelivery,
    ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError> {
        Ok(BridgeDeliveryReceipt::new(
            delivery.invalidation_targets().len(),
            delivery.source_snapshot().clone(),
        ))
    }
}
