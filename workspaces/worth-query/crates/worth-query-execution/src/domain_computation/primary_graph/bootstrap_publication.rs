use std::collections::BTreeMap;

use worth_query_installation::facade::TypedApplicationValue;
use worth_relational::facade::identity::PartitionId;
use worth_relational::facade::indexes::{DerivedIndexBuildRequest, DerivedIndexId};
use worth_relational::facade::symbols::ClientKey;
use worth_relational::facade::transactions::{
    AspectFieldPatch, CreateIntent, CreatedEntityRef, EntityReference, EntitySpec, MutationIntent,
    RelationSpec, TransactionOptions, WorkerIntentBatch,
};

use super::bootstrap::WorthQueryPrincipalBootstrapRow;
use super::typed_bootstrap::{
    WorthQueryTypedEntityBootstrapRow, WorthQueryTypedRelationBootstrapRow,
};
use super::{
    primary_relational_branch_id, WorthQueryPrimaryGraph, WorthQueryPrimaryGraphInstallationDenial,
    WorthQueryPrimaryGraphInstallationDenialKind,
};

pub(super) fn commit_bootstrap_rows(
    graph: &WorthQueryPrimaryGraph,
    rows: Vec<WorthQueryPrincipalBootstrapRow>,
    entity_rows: Vec<WorthQueryTypedEntityBootstrapRow>,
    relation_rows: Vec<WorthQueryTypedRelationBootstrapRow>,
) -> Result<worth_relational::facade::history::CommitId, WorthQueryPrimaryGraphInstallationDenial> {
    graph.integration_handle().with_runtime_mut(|runtime| {
        let mut transaction = runtime.begin_transaction(TransactionOptions::default());
        let mut batch = WorkerIntentBatch::new("application-principal-bootstrap");
        for (ordinal, row) in rows.into_iter().enumerate() {
            batch = append_principal_row(batch, ordinal, row);
        }
        for row in entity_rows {
            batch = append_typed_entity(batch, row);
        }
        for row in relation_rows {
            batch = append_typed_relation(batch, row);
        }
        transaction.push_batch(batch);
        transaction
            .commit()
            .map(|outcome| outcome.commit.commit_id)
            .map_err(|error| {
                primary_graph_denial(
                    WorthQueryPrimaryGraphInstallationDenialKind::RelationalCommitRejected,
                    format!(
                        "Relational rejected the application principal bootstrap transaction: {}",
                        error.detail()
                    ),
                )
            })
    })
}

fn append_typed_entity(
    batch: WorkerIntentBatch,
    row: WorthQueryTypedEntityBootstrapRow,
) -> WorkerIntentBatch {
    batch.push(MutationIntent::Create(CreateIntent::Entity(EntitySpec {
        partition_id: PartitionId::main(),
        kind_id: row.kind,
        client_key: ClientKey::raw(row.key),
        fields: AspectFieldPatch::from(row.fields),
    })))
}

fn append_typed_relation(
    batch: WorkerIntentBatch,
    row: WorthQueryTypedRelationBootstrapRow,
) -> WorkerIntentBatch {
    batch.push(MutationIntent::Create(CreateIntent::Relation(
        RelationSpec {
            partition_id: PartitionId::main(),
            kind_id: row.kind,
            client_key: ClientKey::raw(row.key),
            source: EntityReference::Created(CreatedEntityRef {
                partition_id: PartitionId::main(),
                kind_id: row.from_kind,
                client_key: ClientKey::raw(row.from_key),
            }),
            target: EntityReference::Created(CreatedEntityRef {
                partition_id: PartitionId::main(),
                kind_id: row.to_kind,
                client_key: ClientKey::raw(row.to_key),
            }),
            fields: AspectFieldPatch::default(),
        },
    )))
}

fn append_principal_row(
    batch: WorkerIntentBatch,
    ordinal: usize,
    row: WorthQueryPrincipalBootstrapRow,
) -> WorkerIntentBatch {
    let partition_id = PartitionId::main();
    let principal_client_key = ClientKey::raw(row.principal_key);
    let mapping_client_key = ClientKey::raw(format!("{}-mapping-{ordinal}", row.binding));
    let principal = CreatedEntityRef {
        partition_id,
        kind_id: row.layout.principal_kind,
        client_key: principal_client_key.clone(),
    };
    let mapping = CreatedEntityRef {
        partition_id,
        kind_id: row.layout.mapping_kind,
        client_key: mapping_client_key.clone(),
    };
    let mapping_fields = BTreeMap::from([
        (
            row.layout.identity_locator,
            row.identity.into_foundational_value(),
        ),
        (
            row.layout.status_locator,
            row.status.into_foundational_value(),
        ),
    ]);
    batch
        .push(MutationIntent::Create(CreateIntent::Entity(EntitySpec {
            partition_id,
            kind_id: principal.kind_id,
            client_key: principal_client_key,
            fields: AspectFieldPatch::from(BTreeMap::from([(
                row.layout.principal_identity_locator.clone(),
                row.principal_identity,
            )])),
        })))
        .push(MutationIntent::Create(CreateIntent::Entity(EntitySpec {
            partition_id,
            kind_id: mapping.kind_id,
            client_key: mapping_client_key,
            fields: AspectFieldPatch::from(mapping_fields),
        })))
        .push(MutationIntent::Create(CreateIntent::Relation(
            RelationSpec {
                partition_id,
                kind_id: row.layout.relation_kind,
                client_key: ClientKey::raw(format!("{}-target-{ordinal}", row.binding)),
                source: EntityReference::Created(mapping),
                target: EntityReference::Created(principal),
                fields: AspectFieldPatch::default(),
            },
        )))
}

pub(super) fn build_identity_indexes(
    graph: &WorthQueryPrimaryGraph,
    source_commit_id: worth_relational::facade::history::CommitId,
    index_ids: &[DerivedIndexId],
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial> {
    graph.integration_handle().with_runtime_mut(|runtime| {
        let build = runtime
            .index_authority()
            .build_for_commit(DerivedIndexBuildRequest {
                source_commit_id,
                branch_id: primary_relational_branch_id(),
                index_ids: index_ids.to_vec(),
            });
        if build.failed_indexes.is_empty() && build.generations.len() == index_ids.len() {
            Ok(())
        } else {
            Err(primary_graph_denial(
                WorthQueryPrimaryGraphInstallationDenialKind::IndexBuildRejected,
                format!(
                    "{} of {} identity indexes failed publication",
                    build.failed_indexes.len(),
                    index_ids.len()
                ),
            ))
        }
    })
}

fn primary_graph_denial(
    kind: WorthQueryPrimaryGraphInstallationDenialKind,
    subject: impl Into<String>,
) -> WorthQueryPrimaryGraphInstallationDenial {
    WorthQueryPrimaryGraphInstallationDenial::new(kind, subject)
}
