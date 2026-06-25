use std::collections::BTreeMap;

use forge_query::facade::{
    runtime_subscription_support_evidence_identity, DeclarativeLiveQueryRequest,
    ForgeQueryBasisAdmissionEvidenceRow, ForgeQueryEffectPolicy, ForgeQueryEntity,
    ForgeQueryLiveArtifactTarget, ForgeQueryLivePatch, ForgeQueryLiveViewHandle,
    ForgeQueryMutationReceipt, ForgeQueryMutationTargetCollectionIdentity,
    ForgeQueryPreviewBasisAdmission, ForgeQueryRuntimeEvidenceAuthority,
    ForgeQueryRuntimeInspectionEvidence, ForgeQueryRuntimeInspectorEvidenceAdapter,
    ForgeQueryRuntimePreviewBasisAdapter, ForgeQueryRuntimeSchemaAdapter,
    ForgeQueryRuntimeSignalSinkAdapter, ForgeQueryRuntimeSnapshotIdentityAdapter,
    ForgeQueryRuntimeSourceAdapter, ForgeQueryRuntimeSubscriptionActivationAdapter,
    ForgeQuerySessionLabel, ForgeQuerySnapshotIdentity, ForgeQueryWorkspaceError,
    ForgeQueryWriteReceipt, QuerySchemaView, SubscriptionActivationInput,
};
use forge_runtime_bridge::facade::{
    BridgeDeliveryReceipt, BridgeWritebackOutcomeClass, InvalidationSink, RuntimeBridge,
    SignalBridgeSinkError, TruthWritebackAuthority, TruthWritebackAuthorityError,
    TruthWritebackReceipt, TruthWritebackRequest,
};

mod binding;
pub(super) mod bridge_source;
mod bridge_source_support;
mod declaration_initialization;
mod existing_truth_verification;
mod query_rows;
mod schema_write_boundary;
pub(super) mod write_authority;
pub(super) mod write_support;

pub use self::binding::TopologyRuntimeBinding;
use self::bridge_source::TopologyRuntimeBridgeSource;
pub(crate) use self::declaration_initialization::{
    TopologyRuntimeDeclarationInitialization, TopologyRuntimeDeclarationInitializationAdapter,
};
pub(crate) use self::existing_truth_verification::TopologyExistingTruthVerificationAdapter;
use self::query_rows::{persistent_name_rows, topology_entity_rows, topology_relation_rows};
use crate::projection::runtime_boundary::bridge::{
    milestone_one_bridge_aspect_registrations, milestone_one_bridge_mapping_registrations,
};

pub(crate) fn build_runtime_bridge(
    binding: TopologyRuntimeBinding,
) -> Result<RuntimeBridge, forge_runtime_bridge::facade::BridgeBuildError> {
    use forge_runtime_bridge::facade::RuntimeBridgeBuilder;

    let source = TopologyRuntimeBridgeSource::new(binding);
    let builder = RuntimeBridgeBuilder::new()
        .with_relational_source(source.clone())
        .with_truth_branch_head_source(source)
        .with_signal_sink(TopologyStaticBridgeSink)
        .with_writeback_authority(TopologyProductionWritebackAuthority);
    let mut mappings = milestone_one_bridge_mapping_registrations().into_iter();
    let first = mappings
        .next()
        .expect(" milestone 1 bridge mapping pack should not be empty");
    let builder = mappings.fold(builder.register_mapping(first), |builder, registration| {
        builder.register_mapping(registration)
    });
    let builder = milestone_one_bridge_aspect_registrations()
        .into_iter()
        .fold(builder, |builder, registration| {
            builder.register_aspect_mapping(registration)
        });
    builder.build()
}

pub struct TopologyRuntimeSchemaAdapter;

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
    live_views: BTreeMap<ForgeQueryLiveArtifactTarget, ForgeQueryMutationTargetCollectionIdentity>,
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
        let live_target =
            ForgeQueryLiveArtifactTarget::from_source_adapter_declared_view_name(name.clone());
        self.live_views
            .insert(live_target, request.target_collection_identity());
        Ok(ForgeQueryLiveViewHandle::new(name))
    }

    fn live_entities_for_target(
        &self,
        target: &ForgeQueryLiveArtifactTarget,
    ) -> Vec<ForgeQueryEntity> {
        let Some(collection) = self.live_views.get(target) else {
            return Vec::new();
        };
        match collection.as_str() {
            "TopologyEntity" => topology_entity_rows(&self.binding),
            "TopologyRelation" => topology_relation_rows(&self.binding),
            "PersistentName" => persistent_name_rows(&self.binding),
            _ => Vec::new(),
        }
    }

    fn drain_live_patches_for_target(
        &mut self,
        _target: &ForgeQueryLiveArtifactTarget,
    ) -> Vec<ForgeQueryLivePatch> {
        Vec::new()
    }

    fn affected_live_view_targets(
        &self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> Vec<ForgeQueryLiveArtifactTarget> {
        let mut affected = receipt
            .deltas()
            .iter()
            .flat_map(|delta| {
                self.live_views
                    .iter()
                    .filter(move |(_, collection)| {
                        delta
                            .target_collection_identity()
                            .same_target_collection_as(collection)
                    })
                    .map(|(target, _)| target.clone())
            })
            .collect::<Vec<_>>();
        affected.sort();
        affected.dedup();
        affected
    }
}

impl ForgeQueryRuntimeSnapshotIdentityAdapter for TopologyRuntimeSourceAdapter {
    fn current_snapshot_identity(&self) -> ForgeQuerySnapshotIdentity {
        self.binding.current_snapshot_identity()
    }
}

pub(super) struct TopologyRuntimeSnapshotIdentityAdapter {
    binding: TopologyRuntimeBinding,
}

impl TopologyRuntimeSnapshotIdentityAdapter {
    pub(super) fn new(binding: TopologyRuntimeBinding) -> Self {
        Self { binding }
    }
}

impl ForgeQueryRuntimeSnapshotIdentityAdapter for TopologyRuntimeSnapshotIdentityAdapter {
    fn current_snapshot_identity(&self) -> ForgeQuerySnapshotIdentity {
        self.binding.current_snapshot_identity()
    }
}

pub(super) struct TopologyStaticSignalSink;

impl ForgeQueryRuntimeSignalSinkAdapter for TopologyStaticSignalSink {
    fn route_write_receipt(
        &mut self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> Result<forge_query::facade::SignalInvalidationBoundaryReceipt, ForgeQueryWorkspaceError>
    {
        let routing_receipt = self.build_signal_invalidation_routing_receipt(receipt)?;
        Ok(self.build_signal_invalidation_boundary_receipt(receipt, routing_receipt)?)
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
    fn support_evidence_identity(&self) -> forge_query::facade::ForgeQueryEvidenceIdentity {
        runtime_subscription_support_evidence_identity(self.support_evidence)
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

pub(super) enum TopologyPreviewBasis {
    Supported { support_evidence: &'static str },
    Denied { denial_reason: &'static str },
}

impl TopologyPreviewBasis {
    pub(super) fn supported(support_evidence: &'static str) -> Self {
        Self::Supported { support_evidence }
    }

    pub(super) fn denied(denial_reason: &'static str) -> Self {
        Self::Denied { denial_reason }
    }
}

impl ForgeQueryRuntimePreviewBasisAdapter for TopologyPreviewBasis {
    fn admit_preview_basis(
        &self,
        label: &ForgeQuerySessionLabel,
        effect_policy: ForgeQueryEffectPolicy,
        authority: &ForgeQueryRuntimeEvidenceAuthority,
    ) -> Result<ForgeQueryPreviewBasisAdmission, ForgeQueryWorkspaceError> {
        match self {
            Self::Supported { support_evidence } => Ok(ForgeQueryPreviewBasisAdmission::new(
                authority,
                label.clone(),
                effect_policy,
                [ForgeQueryBasisAdmissionEvidenceRow::support_profile_token(
                    *support_evidence,
                )],
            )),
            Self::Denied { denial_reason } => Err(ForgeQueryWorkspaceError::new(*denial_reason)),
        }
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

#[derive(Clone, Debug)]
struct TopologyProductionWritebackAuthority;

impl TruthWritebackAuthority for TopologyProductionWritebackAuthority {
    fn execute_writeback(
        &self,
        request: TruthWritebackRequest,
    ) -> Result<TruthWritebackReceipt, TruthWritebackAuthorityError> {
        Ok(TruthWritebackReceipt::new(
            BridgeWritebackOutcomeClass::AuthoritativeCommit,
            &request,
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
