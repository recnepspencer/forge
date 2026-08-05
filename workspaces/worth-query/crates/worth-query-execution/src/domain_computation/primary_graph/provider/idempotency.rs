use std::collections::BTreeMap;

use worth_foundational::facade::{AspectValue, InternedString};
use worth_relational::facade::indexes::{BoundedEntityFieldLookupRequest, BoundedIndexParityMode};
use worth_relational::facade::runtime::{ProjectionAspectRequirement, ProjectionAspectScope};
use worth_relational::facade::storage::RecordLifecycleState;
use worth_relational::facade::transactions::{
    AspectFieldPatch, CreateIntent, EntitySpec, MutationIntent,
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
    let request = BoundedEntityFieldLookupRequest::new(
        snapshot.clone(),
        layout.key_index_id,
        layout.entity_kind,
        layout.key_locator.clone(),
        key.clone(),
        2,
    )
    .map_err(|_| "provider idempotency lookup request was rejected")?;
    let lookup = runtime
        .index_access()
        .execute_bounded_entity_field_lookup(request, BoundedIndexParityMode::Production)
        .map_err(|_| "provider idempotency index lookup failed")?;
    if lookup.overflowed() || lookup.candidate_entity_ids().len() > 1 {
        return Err("provider idempotency key is not unique");
    }
    let Some(entity_id) = lookup.candidate_entity_ids().first().copied() else {
        return Ok(WorthQueryProviderIdempotencyResolution::Absent);
    };
    let key_field = layout
        .key_locator
        .field_path()
        .fields()
        .first()
        .cloned()
        .ok_or("provider idempotency key locator is empty")?;
    let intent_field = layout
        .intent_locator
        .field_path()
        .fields()
        .first()
        .cloned()
        .ok_or("provider idempotency intent locator is empty")?;
    let emitted_effect_count_field = layout
        .emitted_effect_count_locator
        .field_path()
        .fields()
        .first()
        .cloned()
        .ok_or("provider idempotency emitted-effect-count locator is empty")?;
    let outcome_identity_field = layout
        .outcome_identity_locator
        .field_path()
        .fields()
        .first()
        .cloned()
        .ok_or("provider idempotency outcome-identity locator is empty")?;
    let scope = ProjectionAspectScope::from_requirements([ProjectionAspectRequirement::fields(
        layout.key_locator.aspect().aspect_key().clone(),
        [
            key_field.clone(),
            intent_field.clone(),
            outcome_identity_field.clone(),
            emitted_effect_count_field.clone(),
        ],
    )]);
    let record = runtime
        .read_truth()
        .project_snapshot(snapshot)
        .and_then(|view| {
            view.entity_record_with_projection_scope(entity_id, scope, |record| {
                (record.kind_id() == layout.entity_kind
                    && record.lifecycle() == RecordLifecycleState::Live
                    && record
                        .aspect_field_value(layout.key_locator.aspect().aspect_key(), &key_field)
                        == Some(&key))
                .then(|| {
                    (
                        record.created_at_version(),
                        record
                            .aspect_field_value(
                                layout.intent_locator.aspect().aspect_key(),
                                &intent_field,
                            )
                            .cloned(),
                        record
                            .aspect_field_value(
                                layout.outcome_identity_locator.aspect().aspect_key(),
                                &outcome_identity_field,
                            )
                            .cloned(),
                        record
                            .aspect_field_value(
                                layout.emitted_effect_count_locator.aspect().aspect_key(),
                                &emitted_effect_count_field,
                            )
                            .cloned(),
                    )
                })
            })
        })
        .ok_or("provider idempotency record is not authoritative")?;
    let expected_intent = AspectValue::String(InternedString::from(binding.intent_text()));
    if record.1.as_ref() != Some(&expected_intent) {
        return Ok(WorthQueryProviderIdempotencyResolution::Drift);
    }
    let committed = runtime
        .history()
        .committed_version(record.0)
        .ok_or("provider idempotency creation commit is unavailable")?;
    let Some(AspectValue::UInt64(outcome_identity)) = record.2 else {
        return Err("provider idempotency outcome identity is unavailable");
    };
    let outcome_identity = WorthQueryApplicationCommitOutcomeIdentity::restore(outcome_identity)
        .ok_or("provider idempotency outcome identity is invalid")?;
    let Some(AspectValue::UInt64(emitted)) = record.3 else {
        return Err("provider idempotency emitted-effect count is unavailable");
    };
    let emitted = usize::try_from(emitted)
        .map_err(|_| "provider idempotency emitted-effect count exceeds host representation")?;
    Ok(WorthQueryProviderIdempotencyResolution::Equivalent(
        WorthQueryPrimaryGraphCommittedApplication::new(
            outcome_identity,
            snapshot.runtime_instance_id,
            snapshot.branch_id.clone(),
            committed.commit().commit_id,
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
    use crate::domain_computation::primary_graph::tests::fixture::installed_authorization_world;

    #[test]
    fn idempotency_lookup_never_substitutes_another_branch_head() {
        let world = installed_authorization_world(true);
        let provider = &world.application.primary_provider;
        let main = BranchId("main".to_owned());
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
