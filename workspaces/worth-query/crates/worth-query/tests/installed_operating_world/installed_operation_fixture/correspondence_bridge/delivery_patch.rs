use super::*;

pub(crate) fn conditional_runtime_bridge_with_change(
    dependency: &worth_query::facade::domain::WorthQuerySemanticTruthDependency,
) -> (
    RuntimeBridge,
    RelationalCommittedPatchRequest,
    worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts,
    [worth_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts; 2],
) {
    let (bridge, mut requests, record, mut snapshots) =
        conditional_runtime_bridge_with_change_sequence(dependency, false);
    (
        bridge,
        requests.remove(0),
        record,
        [snapshots.remove(0), snapshots.remove(0)],
    )
}

pub(crate) fn conditional_runtime_bridge_with_repeated_value_changes(
    dependency: &worth_query::facade::domain::WorthQuerySemanticTruthDependency,
) -> (
    RuntimeBridge,
    [RelationalCommittedPatchRequest; 2],
    worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts,
    [worth_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts; 3],
) {
    let (bridge, mut requests, record, mut snapshots) =
        conditional_runtime_bridge_with_change_sequence(dependency, true);
    (
        bridge,
        [requests.remove(0), requests.remove(0)],
        record,
        [
            snapshots.remove(0),
            snapshots.remove(0),
            snapshots.remove(0),
        ],
    )
}

fn conditional_runtime_bridge_with_change_sequence(
    dependency: &worth_query::facade::domain::WorthQuerySemanticTruthDependency,
    repeat_after_value: bool,
) -> (
    RuntimeBridge,
    Vec<RelationalCommittedPatchRequest>,
    worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts,
    Vec<worth_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts>,
) {
    let mut relational = relational_with_dependency(dependency);
    let field = dependency_field(dependency.binding());
    let (before, after) = fixture_values(dependency.contract());
    let snapshot_values = (before.clone(), after.clone());
    let locator = AspectFieldLocator::new(
        LocatorAuthority::Planned,
        dependency.contract().key().clone(),
        CanonicalFieldPath::single(field.clone()),
    );
    let mut create = {
        let transaction_validation_input = relational
            .admit_main_branch_basis()
            .expect("main branch binding");
        relational
            .begin_branch_transaction(
                &transaction_validation_input,
                worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
            )
            .expect("owner-admitted transaction context")
    };
    create
        .push_batch(
            WorkerIntentBatch::new("create-delivery-entity").push(MutationIntent::Create(
                CreateIntent::Entity(EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(1),
                    client_key: ClientKey::raw("conditional-delivery-entity"),
                    fields: AspectFieldPatch::new(BTreeMap::from([(locator.clone(), before)])),
                }),
            )),
        )
        .expect("test staging stays within configured resource budgets");
    let created = create
        .commit(&mut relational)
        .expect("delivery entity should commit");
    let entity = created
        .changed_records
        .iter()
        .find_map(|record| match record {
            RecordRef::Entity(entity) => Some(*entity),
            RecordRef::Relation(_) => None,
        })
        .expect("create commit should retain the delivery entity");
    let branch_identity = relational
        .branch_identity(&created.commit.branch_id)
        .expect("created branch identity");
    let (_, created_basis) = relational
        .observe_branch(&branch_identity)
        .expect("created owner-admitted branch basis");
    commit_snapshot_closeout::release_commit_snapshot(&mut relational, &created);
    let mut update = {
        let transaction_validation_input = relational
            .admit_main_branch_basis()
            .expect("main branch binding");
        relational
            .begin_branch_transaction(
                &transaction_validation_input,
                worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
            )
            .expect("owner-admitted transaction context")
    };
    update
        .push_batch(
            WorkerIntentBatch::new("update-delivery-entity").push(MutationIntent::Entity(
                EntityMutationIntent::UpdateFields(UpdateEntityFieldsIntent {
                    entity_id: entity,
                    fields: AspectFieldPatch::from_locator(locator.clone(), after.clone()),
                }),
            )),
        )
        .expect("test staging stays within configured resource budgets");
    let updated = update
        .commit(&mut relational)
        .expect("delivery update should commit");
    let (_, updated_basis) = relational
        .observe_branch(&branch_identity)
        .expect("updated owner-admitted branch basis");
    let mut requests = vec![request(&updated)];
    let mut retained_bases = vec![created_basis, updated_basis];
    commit_snapshot_closeout::release_commit_snapshot(&mut relational, &updated);
    if repeat_after_value {
        let mut repeated = {
            let transaction_validation_input = relational
                .admit_main_branch_basis()
                .expect("main branch binding");
            relational
                .begin_branch_transaction(
                    &transaction_validation_input,
                    worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
                )
                .expect("owner-admitted transaction context")
        };
        repeated
            .push_batch(WorkerIntentBatch::new("repeat-delivery-value").push(
                MutationIntent::Entity(EntityMutationIntent::UpdateFields(
                    UpdateEntityFieldsIntent {
                        entity_id: entity,
                        fields: AspectFieldPatch::from_locator(
                            locator,
                            repeated_raw_fixture_value(dependency.contract()),
                        ),
                    },
                )),
            ))
            .expect("test staging stays within configured resource budgets");
        let committed = repeated
            .commit(&mut relational)
            .expect("repeated delivery value should retain a distinct authoritative commit");
        requests.push(request(&committed));
        let (_, repeated_basis) = relational
            .observe_branch(&branch_identity)
            .expect("repeated owner-admitted branch basis");
        retained_bases.push(repeated_basis);
        commit_snapshot_closeout::release_commit_snapshot(&mut relational, &committed);
    }
    let record = worth_runtime_bridge::facade::RelationalBridgeRecordIdentityParts::entity(
        PartitionId::main().0,
        entity.local_slot_value(),
        entity.generation_value(),
    );
    let source = RuntimeBridgeRelationalSource::for_graph_role(Arc::new(relational), "model")
        .expect("model is a valid graph role");
    let (source, snapshots) = RetainedRelationalSource::new(source, retained_bases);
    (
        build_bridge(
            source,
            dependency.contract(),
            field,
            dependency.projection_mask().is_whole_aspect(),
            FixtureLocality::Record,
            Some(VersionedFixtureSnapshotSource::new(
                snapshots[0].version_id(),
                snapshot_values.0,
                snapshot_values.1,
            )),
            None,
        ),
        requests,
        record,
        snapshots,
    )
}

fn relational_with_dependency(
    dependency: &worth_query::facade::domain::WorthQuerySemanticTruthDependency,
) -> worth_relational::facade::runtime::RelationalRuntime {
    RelationalRuntimeApi::builder()
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
                .expect("conditional delivery schema should register"),
        )
        .build()
}

fn request(
    commit: &worth_relational::facade::transactions::CommitResult,
) -> RelationalCommittedPatchRequest {
    RelationalCommittedPatchRequest::new(TruthCommitIdentity::from_relational_commit_id(
        commit.commit.commit_id.0,
    ))
}
