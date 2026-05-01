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
mod query_rows;
pub(super) mod write_authority;
pub(super) mod write_support;

pub(crate) use self::binding::WorthTopologyRuntimeBinding;
use self::bridge_source::WorthTopologyRuntimeBridgeSource;
use self::query_rows::{persistent_name_rows, topology_entity_rows, topology_relation_rows};

pub(super) fn build_runtime_bridge(
    binding: WorthTopologyRuntimeBinding,
) -> Result<RuntimeBridge, forge_runtime_bridge::facade::BridgeBuildError> {
    use forge_runtime_bridge::facade::RuntimeBridgeBuilder;

    let source = WorthTopologyRuntimeBridgeSource::new(binding);
    let builder = RuntimeBridgeBuilder::new()
        .with_relational_source(source.clone())
        .with_truth_branch_head_source(source)
        .with_signal_sink(WorthTopologyStaticBridgeSink);
    let mut mappings =
        crate::bridge::worth_milestone_one_bridge_mapping_registrations().into_iter();
    let first = mappings
        .next()
        .expect("worth milestone 1 bridge mapping pack should not be empty");
    let builder = mappings.fold(builder.register_mapping(first), |builder, registration| {
        builder.register_mapping(registration)
    });
    let builder = crate::bridge::worth_milestone_one_bridge_aspect_registrations()
        .into_iter()
        .fold(builder, |builder, registration| {
            builder.register_aspect_mapping(registration)
        });
    builder.build()
}

pub(super) struct WorthTopologyRuntimeSchemaAdapter;

impl ForgeQueryRuntimeSchemaAdapter for WorthTopologyRuntimeSchemaAdapter {
    fn admit_live_view(
        &self,
        _name: &str,
        request: &DeclarativeLiveQueryRequest,
        _schema_view: &QuerySchemaView,
    ) -> Result<(), ForgeQueryWorkspaceError> {
        match request.target() {
            "WorthTopologyEntity" | "WorthTopologyRelation" | "WorthPersistentName" => Ok(()),
            other => Err(ForgeQueryWorkspaceError::new(format!(
                "worth topology production runtime does not admit live view target `{other}` yet"
            ))),
        }
    }
}

pub(super) struct WorthTopologyRuntimeSourceAdapter {
    binding: WorthTopologyRuntimeBinding,
    live_views: BTreeMap<String, String>,
}

impl WorthTopologyRuntimeSourceAdapter {
    pub(super) fn new(binding: WorthTopologyRuntimeBinding) -> Self {
        Self {
            binding,
            live_views: BTreeMap::new(),
        }
    }
}

impl ForgeQueryRuntimeSourceAdapter for WorthTopologyRuntimeSourceAdapter {
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
            "WorthTopologyEntity" => topology_entity_rows(&self.binding),
            "WorthTopologyRelation" => topology_relation_rows(&self.binding),
            "WorthPersistentName" => persistent_name_rows(&self.binding),
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

pub(super) struct WorthTopologyStaticSignalSink;

impl ForgeQueryRuntimeSignalSinkAdapter for WorthTopologyStaticSignalSink {
    fn route_write_receipt(
        &mut self,
        _receipt: &ForgeQueryMutationReceipt,
    ) -> Result<(), ForgeQueryWorkspaceError> {
        Ok(())
    }
}

pub(super) struct WorthTopologySubscriptionActivation;
impl ForgeQueryRuntimeSubscriptionActivationAdapter for WorthTopologySubscriptionActivation {
    fn support_evidence(&self) -> String {
        "worth-topology-current-head-subscription-activation".to_string()
    }
    fn admit_activation(
        &mut self,
        view_name: &str,
        activation: &SubscriptionActivationInput,
    ) -> Result<String, ForgeQueryWorkspaceError> {
        Ok(format!(
            "worth-topology-subscription:{view_name}:{}",
            activation.activation_digest()
        ))
    }
}

pub(super) struct WorthTopologyPreviewBasis;
impl ForgeQueryRuntimePreviewBasisAdapter for WorthTopologyPreviewBasis {
    fn admit_preview_basis(
        &self,
        _label: &str,
        _effect_policy: ForgeQueryEffectPolicy,
        _authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError> {
        Err(ForgeQueryWorkspaceError::new(
            "worth topology production runtime current-head slice does not admit preview bases yet",
        ))
    }
}

pub(super) struct WorthTopologyInspectorEvidence;
impl ForgeQueryRuntimeInspectorEvidenceAdapter for WorthTopologyInspectorEvidence {
    fn inspect_write_receipt(
        &self,
        receipt: &ForgeQueryWriteReceipt,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryRuntimeInspectionEvidence, ForgeQueryWorkspaceError> {
        Ok(ForgeQueryRuntimeInspectionEvidence::new(
            authority,
            "worth-topology-current-head-write-receipt",
            receipt.authority_lane(),
            ["worth-topology-current-head-inspector-evidence"],
        ))
    }
}

#[derive(Clone)]
struct WorthTopologyStaticBridgeSink;

impl InvalidationSink for WorthTopologyStaticBridgeSink {
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
