use std::collections::BTreeMap;

use worth_foundational::facade::{AspectValue, FieldKey, InternedString};
use worth_relational::facade::indexes::{BoundedEntityFieldLookupRequest, BoundedIndexParityMode};
use worth_relational::facade::runtime::{ProjectionAspectRequirement, ProjectionAspectScope};
use worth_relational::facade::storage::RecordLifecycleState;
use worth_relational::facade::transactions::{
    AspectFieldLocator, AspectFieldPatch, CreateIntent, EntitySpec, MutationIntent,
};

use super::WorthQueryPrimaryGraphProvider;
use crate::domain_computation::primary_graph::application_attempt::WorthQueryApplicationCommitOutcomeIdentity;
use crate::domain_computation::primary_graph::application_attempt::WorthQueryApplicationIdempotencyBinding;
use crate::domain_computation::primary_graph::provider::WorthQueryPrimaryGraphCommittedApplication;
use crate::domain_computation::primary_graph::schema_layout::WorthQueryProviderIdempotencyLayout;

#[derive(Debug)]
pub(in crate::domain_computation::primary_graph) enum WorthQueryProviderIdempotencyResolution {
    Absent,
    Equivalent(WorthQueryPrimaryGraphCommittedApplication),
    Drift,
}

pub(super) fn idempotency_create_intent(
    layout: &WorthQueryProviderIdempotencyLayout,
    binding: WorthQueryApplicationIdempotencyBinding,
    outcome_identity: WorthQueryApplicationCommitOutcomeIdentity,
    emitted_effect_count: u64,
) -> MutationIntent {
    let key = binding.key_text();
    let fields = BTreeMap::from([
        (
            layout.key_locator.clone(),
            AspectValue::String(InternedString::from(key.clone())),
        ),
        (
            layout.intent_locator.clone(),
            AspectValue::String(InternedString::from(binding.intent_text())),
        ),
        (
            layout.outcome_identity_locator.clone(),
            AspectValue::UInt64(outcome_identity.get()),
        ),
        (
            layout.emitted_effect_count_locator.clone(),
            AspectValue::UInt64(emitted_effect_count),
        ),
    ]);
    MutationIntent::Create(CreateIntent::Entity(EntitySpec {
        partition_id: worth_relational::facade::identity::PartitionId::main(),
        kind_id: layout.entity_kind,
        client_key: worth_relational::facade::symbols::ClientKey::raw(format!(
            "worth-query-idempotency:{key}"
        )),
        fields: AspectFieldPatch::from(fields),
    }))
}

impl WorthQueryPrimaryGraphProvider {
    pub(in crate::domain_computation::primary_graph) fn resolve_idempotency_binding(
        &self,
        binding: WorthQueryApplicationIdempotencyBinding,
        branch: &worth_relational::facade::history::BranchId,
    ) -> Result<WorthQueryProviderIdempotencyResolution, &'static str> {
        let layout = self.graph.layout.provider_idempotency().clone();
        self.graph.with_runtime_mut(|runtime| {
            self.graph
                .ensure_primary_indexes_current_for_branch(runtime, branch)?;
            let snapshot = runtime
                .snapshots()
                .snapshot_for_branch(branch)
                .ok_or("provider idempotency branch has no current snapshot")?;
            let resolution = resolve_at_snapshot(runtime, &snapshot, &layout, binding);
            runtime.snapshots().release_snapshot(&snapshot);
            resolution
        })
    }

    pub(in crate::domain_computation::primary_graph) fn resolve_application_idempotency(
        &self,
        session_identity: &str,
    ) -> Result<WorthQueryProviderIdempotencyResolution, &'static str> {
        let (binding, branch) = self
            .sessions
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .application_attempts
            .get(session_identity)
            .map(|attempt| (attempt.idempotency, attempt.branch.clone()))
            .ok_or("provider session lost its idempotency binding")?;
        self.resolve_idempotency_binding(binding, &branch)
    }
}

fn resolve_at_snapshot(
    runtime: &worth_relational::facade::runtime::RelationalRuntime,
    snapshot: &worth_relational::facade::snapshots::SnapshotHandle,
    layout: &WorthQueryProviderIdempotencyLayout,
    binding: WorthQueryApplicationIdempotencyBinding,
) -> Result<WorthQueryProviderIdempotencyResolution, &'static str> {
    let key = AspectValue::String(InternedString::from(binding.key_text()));
    let context = WorthQueryIdempotencySnapshotContext {
        runtime,
        snapshot,
        layout,
    };
    let Some(entity_id) = locate_idempotency_entity(&context, &key)? else {
        return Ok(WorthQueryProviderIdempotencyResolution::Absent);
    };
    let record = read_idempotency_record(&context, entity_id, &key)?;
    resolve_projected_idempotency(&context, binding, record)
}

struct WorthQueryIdempotencySnapshotContext<'a> {
    runtime: &'a worth_relational::facade::runtime::RelationalRuntime,
    snapshot: &'a worth_relational::facade::snapshots::SnapshotHandle,
    layout: &'a WorthQueryProviderIdempotencyLayout,
}

struct WorthQueryIdempotencyProjectionFields {
    key: FieldKey,
    intent: FieldKey,
    outcome_identity: FieldKey,
    emitted_effect_count: FieldKey,
}

struct WorthQueryProjectedIdempotencyRecord {
    created_at_version: worth_relational::facade::identity::VersionId,
    intent: Option<AspectValue>,
    outcome_identity: Option<AspectValue>,
    emitted_effect_count: Option<AspectValue>,
}

fn locate_idempotency_entity(
    context: &WorthQueryIdempotencySnapshotContext<'_>,
    key: &AspectValue,
) -> Result<Option<worth_relational::facade::identity::EntityId>, &'static str> {
    let request = BoundedEntityFieldLookupRequest::new(
        context.snapshot.clone(),
        context.layout.key_index_id,
        context.layout.entity_kind,
        context.layout.key_locator.clone(),
        key.clone(),
        2,
    )
    .map_err(|_| "provider idempotency lookup request was rejected")?;
    let lookup = context
        .runtime
        .index_access()
        .execute_bounded_entity_field_lookup(request, BoundedIndexParityMode::Production)
        .map_err(|_| "provider idempotency index lookup failed")?;
    if lookup.overflowed() || lookup.candidate_entity_ids().len() > 1 {
        return Err("provider idempotency key is not unique");
    }
    Ok(lookup.candidate_entity_ids().first().copied())
}

fn read_idempotency_record(
    context: &WorthQueryIdempotencySnapshotContext<'_>,
    entity_id: worth_relational::facade::identity::EntityId,
    key: &AspectValue,
) -> Result<WorthQueryProjectedIdempotencyRecord, &'static str> {
    let fields = idempotency_projection_fields(context.layout)?;
    let scope = ProjectionAspectScope::from_requirements([ProjectionAspectRequirement::fields(
        context.layout.key_locator.aspect().aspect_key().clone(),
        [
            fields.key.clone(),
            fields.intent.clone(),
            fields.outcome_identity.clone(),
            fields.emitted_effect_count.clone(),
        ],
    )]);
    context
        .runtime
        .read_truth()
        .project_snapshot(context.snapshot)
        .and_then(|view| {
            view.entity_record_with_projection_scope(entity_id, scope, |record| {
                (record.kind_id() == context.layout.entity_kind
                    && record.lifecycle() == RecordLifecycleState::Live
                    && record.aspect_field_value(
                        context.layout.key_locator.aspect().aspect_key(),
                        &fields.key,
                    ) == Some(key))
                .then(|| WorthQueryProjectedIdempotencyRecord {
                    created_at_version: record.created_at_version(),
                    intent: record
                        .aspect_field_value(
                            context.layout.intent_locator.aspect().aspect_key(),
                            &fields.intent,
                        )
                        .cloned(),
                    outcome_identity: record
                        .aspect_field_value(
                            context
                                .layout
                                .outcome_identity_locator
                                .aspect()
                                .aspect_key(),
                            &fields.outcome_identity,
                        )
                        .cloned(),
                    emitted_effect_count: record
                        .aspect_field_value(
                            context
                                .layout
                                .emitted_effect_count_locator
                                .aspect()
                                .aspect_key(),
                            &fields.emitted_effect_count,
                        )
                        .cloned(),
                })
            })
        })
        .ok_or("provider idempotency record is not authoritative")
}

fn idempotency_projection_fields(
    layout: &WorthQueryProviderIdempotencyLayout,
) -> Result<WorthQueryIdempotencyProjectionFields, &'static str> {
    Ok(WorthQueryIdempotencyProjectionFields {
        key: required_locator_field(
            &layout.key_locator,
            "provider idempotency key locator is empty",
        )?,
        intent: required_locator_field(
            &layout.intent_locator,
            "provider idempotency intent locator is empty",
        )?,
        outcome_identity: required_locator_field(
            &layout.outcome_identity_locator,
            "provider idempotency outcome-identity locator is empty",
        )?,
        emitted_effect_count: required_locator_field(
            &layout.emitted_effect_count_locator,
            "provider idempotency emitted-effect-count locator is empty",
        )?,
    })
}

fn required_locator_field(
    locator: &AspectFieldLocator,
    denial: &'static str,
) -> Result<FieldKey, &'static str> {
    locator.field_path().fields().first().cloned().ok_or(denial)
}

fn resolve_projected_idempotency(
    context: &WorthQueryIdempotencySnapshotContext<'_>,
    binding: WorthQueryApplicationIdempotencyBinding,
    record: WorthQueryProjectedIdempotencyRecord,
) -> Result<WorthQueryProviderIdempotencyResolution, &'static str> {
    let expected_intent = AspectValue::String(InternedString::from(binding.intent_text()));
    if record.intent.as_ref() != Some(&expected_intent) {
        return Ok(WorthQueryProviderIdempotencyResolution::Drift);
    }
    let committed = context
        .runtime
        .history()
        .committed_version(record.created_at_version)
        .ok_or("provider idempotency creation commit is unavailable")?;
    let Some(AspectValue::UInt64(outcome_identity)) = record.outcome_identity else {
        return Err("provider idempotency outcome identity is unavailable");
    };
    let outcome_identity = WorthQueryApplicationCommitOutcomeIdentity::restore(outcome_identity)
        .ok_or("provider idempotency outcome identity is invalid")?;
    let Some(AspectValue::UInt64(emitted)) = record.emitted_effect_count else {
        return Err("provider idempotency emitted-effect count is unavailable");
    };
    let emitted = usize::try_from(emitted)
        .map_err(|_| "provider idempotency emitted-effect count exceeds host representation")?;
    Ok(WorthQueryProviderIdempotencyResolution::Equivalent(
        WorthQueryPrimaryGraphCommittedApplication::new(
            outcome_identity,
            context.snapshot.runtime_instance_id,
            committed.commit().clone(),
            committed.changed_record_count(),
            emitted,
        ),
    ))
}

#[cfg(test)]
mod tests {
    use worth_relational::facade::history::BranchId;
    use worth_relational::facade::transactions::{
        RelationalTransaction, TransactionOptions, WorkerIntentBatch,
    };

    use super::*;
    use crate::domain_computation::primary_graph::{
        primary_relational_branch_id, tests::fixture::installed_authorization_world,
    };

    #[test]
    fn idempotency_lookup_never_substitutes_another_branch_head() {
        let world = installed_authorization_world(true);
        let provider = &world.application.primary_provider;
        let main = primary_relational_branch_id();
        let feature = BranchId("idempotency-feature".to_owned());
        let binding = WorthQueryApplicationIdempotencyBinding::new([91; 32], [92; 32]);
        provider.graph.with_runtime_mut(|runtime| {
            runtime
                .history_authority()
                .create_branch(feature.clone(), &main)
                .unwrap();
            let mut feature_transaction: RelationalTransaction<'_> =
                runtime.begin_transaction(TransactionOptions {
                    target_branch: Some(feature.clone()),
                    ..TransactionOptions::default()
                });
            feature_transaction.push_batch(WorkerIntentBatch::new("feature-branch-head").push(
                idempotency_create_intent(
                    provider.graph.layout.provider_idempotency(),
                    WorthQueryApplicationIdempotencyBinding::new([93; 32], [94; 32]),
                    WorthQueryApplicationCommitOutcomeIdentity::mint().unwrap(),
                    0,
                ),
            ));
            feature_transaction.commit().unwrap();
            let mut transaction: RelationalTransaction<'_> =
                runtime.begin_transaction(Default::default());
            transaction.push_batch(WorkerIntentBatch::new("branch-idempotency").push(
                idempotency_create_intent(
                    provider.graph.layout.provider_idempotency(),
                    binding,
                    WorthQueryApplicationCommitOutcomeIdentity::mint().unwrap(),
                    0,
                ),
            ));
            transaction.commit().unwrap();
            provider
                .graph
                .ensure_primary_indexes_current(runtime)
                .unwrap();
        });

        assert!(matches!(
            provider.resolve_idempotency_binding(binding, &main),
            Ok(WorthQueryProviderIdempotencyResolution::Equivalent(_))
        ));
        let feature_resolution = provider.resolve_idempotency_binding(binding, &feature);
        assert!(
            matches!(
                feature_resolution,
                Ok(WorthQueryProviderIdempotencyResolution::Absent)
            ),
            "feature branch resolution: {feature_resolution:?}",
        );
    }
}
