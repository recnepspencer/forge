use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use worth_foundational::facade::{
    AspectBinding, AspectFieldLocator, AspectValue, AuthoritativeAspectChangeKind,
    CanonicalFieldPath, FieldKey, LocatorAuthority, ScalarAspectType,
};
use worth_query::facade::domain;
use worth_runtime_bridge::facade::{
    AspectKeySelector, BridgeAspectRegistration, BridgeAspectRegistrationId,
    BridgeAuthoritativeSourceProfile, BridgeAuthoritativeSourceProvenance,
    BridgeCommittedPatchEnvelope, BridgeCommittedPatchEnvelopeIdentity, BridgeCommittedPatchItem,
    BridgeCommittedPatchTarget, BridgeDeliveryReceipt, BridgeMappingId, BridgeMappingRegistration,
    BridgeProducerMetadata, BridgeSemanticAspectChange, CoarseRoutingMode, CommittedPatchSource,
    InvalidationSink, MappingSelector, RelationalBridgeRecordIdentityParts,
    RelationalBridgeSnapshotIdentityParts, RelationalBridgeSourceError,
    RelationalCommittedPatchRequest, RuntimeBridgeBuilder, SignalBridgeSinkError,
    SignalInvalidationScope, SliceWideningPolicy, SnapshotReadContract, SubscriptionSliceKind,
    TruthBranchIdentity, TruthCommitIdentity, TruthDeltaSurfaceKind, TruthPatchIdentity,
    TruthPatchScope, TruthPatchTargetSelector, TruthSnapshotIdentity,
};

use super::super::correspondence_bridge::versioned_snapshot::VersionedFixtureSnapshotSource;
use super::installation::{conditional_installation_on_bridge, ConditionalInstallation};

const COMMIT_ID: u64 = 42;

#[derive(Clone)]
pub(crate) struct CausalMismatchSwitch(Arc<AtomicBool>);

impl CausalMismatchSwitch {
    pub(crate) fn include_conflicting_change(&self) {
        self.0.store(true, Ordering::SeqCst);
    }
}

#[derive(Clone)]
struct SwitchingCommittedPatchSource {
    include_conflicting_change: Arc<AtomicBool>,
    owner_record: RelationalBridgeRecordIdentityParts,
    conflicting_record: RelationalBridgeRecordIdentityParts,
    target: BridgeCommittedPatchTarget,
    semantic_change: BridgeSemanticAspectChange,
}

impl CommittedPatchSource for SwitchingCommittedPatchSource {
    fn authoritative_source_profile(&self) -> Option<BridgeAuthoritativeSourceProfile> {
        Some(
            BridgeAuthoritativeSourceProfile::new(42, "relational-adapter:42")
                .expect("valid causal mismatch source profile"),
        )
    }

    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        let source = BridgeAuthoritativeSourceProvenance::from_owner_publication(
            42,
            "model",
            "relational-adapter:42",
            "commit:42",
        );
        let identity = BridgeCommittedPatchEnvelopeIdentity::new_with_metadata(
            BridgeProducerMetadata::registered_authoritative_source()
                .with_authoritative_source(source),
            request.commit_identity().clone(),
            TruthPatchIdentity::from_relational_patch_position(COMMIT_ID),
            TruthSnapshotIdentity::from_relational_snapshot(
                RelationalBridgeSnapshotIdentityParts::new(COMMIT_ID, 2),
            ),
            TruthBranchIdentity::from_relational_branch_id("main"),
        );
        let record = if self.include_conflicting_change.load(Ordering::SeqCst) {
            self.conflicting_record
        } else {
            self.owner_record
        };
        let item = BridgeCommittedPatchItem::with_relational_semantic_change(
            record,
            self.target.clone(),
            self.semantic_change.clone(),
        );
        BridgeCommittedPatchEnvelope::new(identity, vec![item])
            .map_err(|error| RelationalBridgeSourceError::new(error.to_string()))
    }
}

struct CausalMismatchSink;

impl InvalidationSink for CausalMismatchSink {
    fn deliver_invalidation(
        &self,
        delivery: worth_runtime_bridge::facade::BridgeSignalInvalidationDelivery,
    ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError> {
        Ok(BridgeDeliveryReceipt::new(
            delivery.invalidation_targets().len(),
            delivery.source_snapshot().clone(),
        ))
    }
}

pub(crate) fn conditional_causal_mismatch_installation(
    node: &domain::WorthQueryPortableConditionalNodeDeclaration,
) -> (
    ConditionalInstallation,
    RelationalCommittedPatchRequest,
    [RelationalBridgeSnapshotIdentityParts; 2],
    CausalMismatchSwitch,
) {
    let dependency = &node.dependencies()[0];
    let field = dependency_field(dependency.binding());
    let owner_record = RelationalBridgeRecordIdentityParts::entity(0, 0, 1);
    let conflicting_record = RelationalBridgeRecordIdentityParts::entity(0, 1, 1);
    let switch = CausalMismatchSwitch(Arc::new(AtomicBool::new(false)));
    let target = BridgeCommittedPatchTarget::entity_field(AspectFieldLocator::new(
        LocatorAuthority::Authoritative,
        dependency.contract().key().clone(),
        CanonicalFieldPath::single(field.clone()),
    ));
    let semantic_change = BridgeSemanticAspectChange::from_authoritative_publication(
        dependency.contract().key().clone(),
        dependency.contract().identity(),
        dependency.contract().revision(),
        dependency.binding().clone(),
        AuthoritativeAspectChangeKind::FieldSet,
        Some(CanonicalFieldPath::single(field.clone())),
    );
    let source = SwitchingCommittedPatchSource {
        include_conflicting_change: switch.0.clone(),
        owner_record,
        conflicting_record,
        target,
        semantic_change,
    };
    let (before, after) = fixture_values(dependency.contract());
    let truth_scope = TruthPatchScope::new(
        MappingSelector::any(),
        AspectKeySelector::exact(dependency.contract().key().clone()),
        TruthPatchTargetSelector::entity_field(field),
    );
    let bridge = RuntimeBridgeBuilder::new()
        .with_committed_patch_source(source)
        .with_snapshot_read_source(VersionedFixtureSnapshotSource::new(1, before, after))
        .with_signal_sink(CausalMismatchSink)
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::from_stable_name("conditional-identity"),
            truth_scope.clone(),
            SnapshotReadContract::new(dependency.contract().clone()),
            SignalInvalidationScope::from_stable_name("conditional-identity"),
            CoarseRoutingMode::Direct,
        ))
        .register_aspect_mapping(BridgeAspectRegistration::new(
            BridgeAspectRegistrationId::from_stable_name("conditional-identity"),
            truth_scope,
            SnapshotReadContract::new(dependency.contract().clone()),
            TruthDeltaSurfaceKind::EntityField,
            SubscriptionSliceKind::SignalField,
            SliceWideningPolicy::Disallow,
        ))
        .build()
        .expect("causal mismatch bridge should build");
    let snapshots = [
        RelationalBridgeSnapshotIdentityParts::new(COMMIT_ID - 1, 1),
        RelationalBridgeSnapshotIdentityParts::new(COMMIT_ID, 2),
    ];
    (
        conditional_installation_on_bridge(node, "geometry-signal", bridge),
        RelationalCommittedPatchRequest::new(TruthCommitIdentity::from_relational_commit_id(
            COMMIT_ID,
        )),
        snapshots,
        switch,
    )
}

fn dependency_field(binding: &AspectBinding) -> FieldKey {
    match binding {
        AspectBinding::EntityField { field } | AspectBinding::RelationField { field } => {
            field.clone()
        }
        _ => FieldKey::new("id").unwrap(),
    }
}

fn fixture_values(
    contract: &worth_foundational::facade::AspectContract,
) -> (AspectValue, AspectValue) {
    match contract.shape() {
        worth_foundational::facade::AspectShape::Scalar(ScalarAspectType::Float64) => (
            AspectValue::Float64(worth_foundational::facade::CanonicalF64::from_f64(10.0)),
            AspectValue::Float64(worth_foundational::facade::CanonicalF64::from_f64(10.02)),
        ),
        _ => (
            AspectValue::String("before".into()),
            AspectValue::String("after".into()),
        ),
    }
}
