use std::collections::BTreeMap;
use std::sync::Arc;

use worth_foundational::facade::{
    AspectBinding, AspectContract, AspectFieldLocator, AspectValue, CanonicalF64,
    CanonicalFieldPath, FieldKey, LocatorAuthority, ScalarAspectType,
};
use worth_relational::facade::branch::AdmittedRelationalBranchBasis;
use worth_relational::facade::bridge::RuntimeBridgeRelationalSource;
use worth_relational::facade::identity::{KindId, PartitionId};
use worth_relational::facade::runtime::RelationalRuntimeApi;
use worth_relational::facade::schema::{
    DeclaredAspectContractBinding, EntityKindRegistration, KindAspectContractDeclarations,
    RelationalSchemaRegistry, SchemaId, SchemaVersionId,
};
use worth_relational::facade::symbols::ClientKey;
use worth_relational::facade::transactions::{
    AspectFieldPatch, CreateIntent, EntityMutationIntent, EntitySpec, MutationIntent, RecordRef,
    UpdateEntityFieldsIntent, WorkerIntentBatch,
};
use worth_runtime_bridge::facade::{
    AspectKeySelector, BridgeAspectRegistration, BridgeAspectRegistrationId,
    BridgeAuthoritativeSourceProfile, BridgeCommittedPatchEnvelope, BridgeDeliveryReceipt,
    BridgeMappingId, BridgeMappingRegistration, BridgeSemanticCorrespondenceRegistration,
    BridgeSemanticLocality, CoarseRoutingMode, CommittedPatchSource, InvalidationSink,
    MappingSelector, RelationalBridgeSourceError, RelationalCommittedPatchRequest, RuntimeBridge,
    RuntimeBridgeBuilder, SignalBridgeSinkError, SignalInvalidationScope, SliceWideningPolicy,
    SnapshotReadContract, SnapshotReadSource, SubscriptionSliceKind, TruthCommitIdentity,
    TruthDeltaSurfaceKind, TruthPatchScope, TruthPatchTargetSelector, TruthSnapshotIdentity,
    TruthSnapshotReader,
};

mod delivery_patch;
pub(super) mod versioned_snapshot;

pub(crate) use delivery_patch::{
    conditional_runtime_bridge_with_change, conditional_runtime_bridge_with_repeated_value_changes,
};

use versioned_snapshot::VersionedFixtureSnapshotSource;

struct CorrespondenceSink;

impl InvalidationSink for CorrespondenceSink {
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

pub(crate) fn correspondence_bridge(
    registration: BridgeSemanticCorrespondenceRegistration,
) -> (RuntimeBridge, RelationalCommittedPatchRequest) {
    let dependency = registration.dependency();
    let mut relational = RelationalRuntimeApi::builder()
        .schema_registry(
            RelationalSchemaRegistry::new()
                .register_entity_kind(EntityKindRegistration {
                    kind_id: KindId(1),
                    kind_name: "conditional.geometry".into(),
                    schema_id: SchemaId("conditional-geometry".into()),
                    schema_version_id: SchemaVersionId(1),
                    aspect_contract_declarations: KindAspectContractDeclarations::new(vec![
                        DeclaredAspectContractBinding {
                            binding: dependency.binding().clone(),
                            contract: dependency.contract().clone(),
                        },
                    ]),
                })
                .expect("conditional correspondence schema should register"),
        )
        .build();
    let field = dependency_field(dependency.binding());
    let locator = AspectFieldLocator::new(
        LocatorAuthority::Planned,
        dependency.contract().key().clone(),
        CanonicalFieldPath::single(field.clone()),
    );
    let mut create = relational.begin_transaction(
        relational
            .transaction_options_for_main()
            .expect("main branch binding"),
    );
    create.push_batch(WorkerIntentBatch::new("create-conditional-entity").push(
        MutationIntent::Create(CreateIntent::Entity(EntitySpec {
            partition_id: PartitionId::main(),
            kind_id: KindId(1),
            client_key: ClientKey::raw("conditional-entity"),
            fields: AspectFieldPatch::new(BTreeMap::from([(
                locator.clone(),
                AspectValue::String("before".into()),
            )])),
        })),
    ));
    let created = create.commit().expect("conditional entity should commit");
    let entity = created
        .changed_records
        .iter()
        .find_map(|record| match record {
            RecordRef::Entity(entity) => Some(*entity),
            RecordRef::Relation(_) => None,
        })
        .expect("create commit should retain its entity identity");

    let mut update = relational.begin_transaction(
        relational
            .transaction_options_for_main()
            .expect("main branch binding"),
    );
    update.push_batch(WorkerIntentBatch::new("update-conditional-identity").push(
        MutationIntent::Entity(EntityMutationIntent::UpdateFields(
            UpdateEntityFieldsIntent {
                entity_id: entity,
                fields: AspectFieldPatch::from_locator(
                    locator,
                    AspectValue::String("after".into()),
                ),
            },
        )),
    ));
    let updated = update
        .commit()
        .expect("conditional identity field should commit");
    let branch_identity = relational
        .branch_identity(&updated.commit.branch_id)
        .expect("updated branch identity");
    let (_, updated_basis) = relational
        .observe_branch(&branch_identity)
        .expect("updated owner-admitted branch basis");
    let request = RelationalCommittedPatchRequest::new(
        TruthCommitIdentity::from_relational_commit_id(updated.commit.commit_id.0),
    );
    let source = match dependency.locality() {
        BridgeSemanticLocality::SourcePartition(role) => {
            RuntimeBridgeRelationalSource::for_graph_partition(
                Arc::new(relational),
                "model",
                PartitionId::main(),
                role.clone(),
            )
        }
        BridgeSemanticLocality::SourceRecord
        | BridgeSemanticLocality::ManagedSourceRecord
        | BridgeSemanticLocality::WholeLogicalGraph => {
            RuntimeBridgeRelationalSource::for_graph_role(Arc::new(relational), "model")
        }
    }
    .expect("model is a valid graph role");
    let locality = match dependency.locality() {
        BridgeSemanticLocality::SourceRecord | BridgeSemanticLocality::ManagedSourceRecord => {
            FixtureLocality::Record
        }
        BridgeSemanticLocality::SourcePartition(_) => FixtureLocality::Partition,
        BridgeSemanticLocality::WholeLogicalGraph => FixtureLocality::Graph,
    };
    let contract = dependency.contract().clone();
    let (source, _) = RetainedRelationalSource::new(source, vec![updated_basis]);
    let bridge = build_bridge(
        source,
        &contract,
        field,
        dependency.projection_mask().is_whole_aspect(),
        locality,
        None,
        Some(registration),
    );
    (bridge, request)
}

pub(crate) fn conditional_runtime_bridge(
    dependency: &worth_query::facade::domain::WorthQuerySemanticTruthDependency,
) -> RuntimeBridge {
    let relational = RelationalRuntimeApi::builder()
        .schema_registry(
            RelationalSchemaRegistry::new()
                .register_entity_kind(EntityKindRegistration {
                    kind_id: KindId(1),
                    kind_name: "conditional.geometry".into(),
                    schema_id: SchemaId("conditional-geometry".into()),
                    schema_version_id: SchemaVersionId(1),
                    aspect_contract_declarations: KindAspectContractDeclarations::new(vec![
                        DeclaredAspectContractBinding {
                            binding: dependency.binding().clone(),
                            contract: dependency.contract().clone(),
                        },
                    ]),
                })
                .expect("conditional correspondence schema should register"),
        )
        .build();
    let locality = match dependency.locality() {
        worth_query::facade::domain::WorthQuerySemanticLocality::SourceRecord => {
            FixtureLocality::Record
        }
        worth_query::facade::domain::WorthQuerySemanticLocality::SourcePartition(_) => {
            FixtureLocality::Partition
        }
        worth_query::facade::domain::WorthQuerySemanticLocality::WholeLogicalGraph => {
            FixtureLocality::Graph
        }
    };
    let source = RuntimeBridgeRelationalSource::for_graph_role(Arc::new(relational), "model")
        .expect("model is a valid graph role");
    let (source, _) = RetainedRelationalSource::new(source, Vec::new());
    build_bridge(
        source,
        dependency.contract(),
        dependency_field(dependency.binding()),
        dependency.projection_mask().is_whole_aspect(),
        locality,
        None,
        None,
    )
}

#[derive(Clone, Copy)]
enum FixtureLocality {
    Record,
    Partition,
    Graph,
}

fn build_bridge(
    source: RetainedRelationalSource,
    contract: &AspectContract,
    field: FieldKey,
    whole_aspect: bool,
    locality: FixtureLocality,
    snapshot_source: Option<VersionedFixtureSnapshotSource>,
    registration: Option<BridgeSemanticCorrespondenceRegistration>,
) -> RuntimeBridge {
    let target = if whole_aspect {
        TruthPatchTargetSelector::authoritative_aspect()
    } else {
        TruthPatchTargetSelector::entity_field(field)
    };
    let truth_surface = if whole_aspect {
        TruthDeltaSurfaceKind::AuthoritativeAspect
    } else {
        TruthDeltaSurfaceKind::EntityField
    };
    let entity_selector = match locality {
        FixtureLocality::Record => MappingSelector::exact(
            worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts::entity(0, 0, 1)
                .terminal_projection_for_reporting(),
        ),
        FixtureLocality::Partition | FixtureLocality::Graph => MappingSelector::any(),
    };
    let mapping = BridgeMappingRegistration::new(
        BridgeMappingId::from_stable_name("conditional-identity"),
        TruthPatchScope::new(
            entity_selector,
            AspectKeySelector::exact(contract.key().clone()),
            target,
        ),
        SnapshotReadContract::new(contract.clone()),
        SignalInvalidationScope::from_stable_name("conditional-identity"),
        CoarseRoutingMode::Direct,
    );
    let (slice, widening) = match locality {
        FixtureLocality::Partition => (
            SubscriptionSliceKind::SignalPartition,
            SliceWideningPolicy::RegisteredPartitionWidening,
        ),
        FixtureLocality::Record => (
            SubscriptionSliceKind::SignalField,
            SliceWideningPolicy::Disallow,
        ),
        FixtureLocality::Graph => (
            SubscriptionSliceKind::SignalField,
            SliceWideningPolicy::Disallow,
        ),
    };
    let aspect_mapping = BridgeAspectRegistration::new(
        BridgeAspectRegistrationId::from_stable_name("conditional-identity"),
        mapping.truth_scope().clone(),
        mapping.snapshot_read_contract().clone(),
        truth_surface,
        slice,
        widening,
    );
    let builder = match snapshot_source {
        Some(snapshot_source) => RuntimeBridgeBuilder::new()
            .with_committed_patch_source(source)
            .with_snapshot_read_source(snapshot_source),
        None => RuntimeBridgeBuilder::new().with_relational_source(source),
    }
    .with_signal_sink(CorrespondenceSink)
    .register_mapping(mapping)
    .register_aspect_mapping(aspect_mapping);
    match registration {
        Some(registration) => builder
            .register_semantic_correspondence(registration)
            .build(),
        None => builder.build(),
    }
    .expect("conditional correspondence bridge should build")
}

struct RetainedRelationalSource {
    source: RuntimeBridgeRelationalSource,
    _observations: Vec<worth_relational::facade::bridge::RelationalBridgeObservationLease>,
}

impl RetainedRelationalSource {
    fn new(
        source: RuntimeBridgeRelationalSource,
        retained_bases: Vec<AdmittedRelationalBranchBasis>,
    ) -> (
        Self,
        Vec<worth_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts>,
    ) {
        let observations: Vec<_> = retained_bases
            .iter()
            .map(|basis| {
                source
                    .retain_branch_basis_for_bridge(basis)
                    .expect("fixture Bridge observation should retain")
            })
            .collect();
        let snapshots = observations
            .iter()
            .map(|observation| {
                observation
                    .snapshot_identity()
                    .relational_snapshot_parts()
                    .expect("Relational Bridge observation identity")
            })
            .collect();
        (
            Self {
                source,
                _observations: observations,
            },
            snapshots,
        )
    }
}

impl CommittedPatchSource for RetainedRelationalSource {
    fn authoritative_source_profile(&self) -> Option<BridgeAuthoritativeSourceProfile> {
        CommittedPatchSource::authoritative_source_profile(&self.source)
    }

    fn load_committed_patch(
        &self,
        request: RelationalCommittedPatchRequest,
    ) -> Result<BridgeCommittedPatchEnvelope, RelationalBridgeSourceError> {
        self.source.load_committed_patch(request)
    }
}

impl SnapshotReadSource for RetainedRelationalSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, RelationalBridgeSourceError> {
        self.source.open_snapshot(identity)
    }
}

fn dependency_field(binding: &AspectBinding) -> FieldKey {
    match binding {
        AspectBinding::EntityField { field } | AspectBinding::RelationField { field } => {
            field.clone()
        }
        _ => FieldKey::new("id").unwrap(),
    }
}

fn fixture_values(contract: &AspectContract) -> (AspectValue, AspectValue) {
    match contract.shape() {
        worth_foundational::facade::AspectShape::Scalar(ScalarAspectType::Float64) => (
            AspectValue::Float64(CanonicalF64::from_f64(10.0)),
            AspectValue::Float64(CanonicalF64::from_f64(10.02)),
        ),
        _ => (
            AspectValue::String("before".into()),
            AspectValue::String("after".into()),
        ),
    }
}

fn repeated_raw_fixture_value(contract: &AspectContract) -> AspectValue {
    match contract.shape() {
        worth_foundational::facade::AspectShape::Scalar(ScalarAspectType::Float64) => {
            AspectValue::Float64(CanonicalF64::from_f64(10.03))
        }
        _ => AspectValue::String("after-raw-revision".into()),
    }
}
