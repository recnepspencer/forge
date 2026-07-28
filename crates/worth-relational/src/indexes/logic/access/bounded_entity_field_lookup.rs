use std::collections::BTreeSet;

use crate::indexes::data::{
    BoundedEntityFieldLookupDenial, BoundedEntityFieldLookupDenialKind,
    BoundedEntityFieldLookupOutcome, BoundedEntityFieldLookupRequest, BoundedIndexParityMode,
    DerivedIndexEntries, DerivedIndexKind, DerivedIndexPublicationStatus,
};
use crate::logic::runtime::RelationalRuntime;
use crate::storage::data::AuthoritativeFieldComparisonKey;
use crate::visibility::materialization::read_records::entity_query_locus_comparison_key;
use crate::visibility::snapshot_states::resolve_snapshot_handle;

use super::IndexAccess;

impl IndexAccess<'_> {
    pub fn execute_bounded_entity_field_lookup(
        &self,
        request: BoundedEntityFieldLookupRequest,
        parity_mode: BoundedIndexParityMode,
    ) -> Result<BoundedEntityFieldLookupOutcome, BoundedEntityFieldLookupDenial> {
        let runtime = self.runtime;
        runtime.performance_access().count_query_index_attempt();
        let snapshot = resolve_snapshot_handle(runtime, request.snapshot()).ok_or_else(|| {
            lookup_denial(
                BoundedEntityFieldLookupDenialKind::SnapshotUnavailable,
                &request,
            )
        })?;
        let definition = runtime
            .indexes
            .definitions
            .get(&request.index_id())
            .ok_or_else(|| {
                lookup_denial(
                    BoundedEntityFieldLookupDenialKind::IndexNotInstalled,
                    &request,
                )
            })?;
        if !matches!(
            &definition.kind,
            DerivedIndexKind::EntityField { field_locator }
                if field_locator == request.field_locator()
        ) {
            return Err(lookup_denial(
                BoundedEntityFieldLookupDenialKind::WrongIndexKind,
                &request,
            ));
        }
        let branch_id = branch_id_for_version(runtime, snapshot.version_id);
        let schema_version = runtime
            .read_truth()
            .query_plan_context(&snapshot)
            .ok_or_else(|| {
                lookup_denial(
                    BoundedEntityFieldLookupDenialKind::SnapshotUnavailable,
                    &request,
                )
            })?
            .schema_version;
        let generation = runtime
            .indexes
            .generations
            .get(&request.index_id())
            .into_iter()
            .flatten()
            .rev()
            .find(|generation| {
                generation.status == DerivedIndexPublicationStatus::Published
                    && generation.applicability.version_id == snapshot.version_id
                    && generation.applicability.schema_version == schema_version
                    && (!definition.branch_scoped
                        || branch_id
                            .as_ref()
                            .is_some_and(|branch| generation.applicability.branch_id == *branch))
            })
            .ok_or_else(|| {
                lookup_denial(
                    BoundedEntityFieldLookupDenialKind::ExactGenerationUnavailable,
                    &request,
                )
            })?;
        let DerivedIndexEntries::EntityField(entries) = &generation.entries else {
            return Err(lookup_denial(
                BoundedEntityFieldLookupDenialKind::WrongIndexKind,
                &request,
            ));
        };
        let comparison_key = AuthoritativeFieldComparisonKey::from_aspect_value(request.value());
        let indexed_ids = entries
            .get(&comparison_key)
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        let overflowed = indexed_ids.len() > request.candidate_limit();
        let examined_entry_count = indexed_ids.len().min(request.candidate_limit());
        let candidate_entity_ids =
            verify_bounded_index_entries(runtime, &snapshot, &request, indexed_ids)?;
        let outcome = BoundedEntityFieldLookupOutcome::new(
            generation.generation_id,
            candidate_entity_ids,
            examined_entry_count,
            overflowed,
            parity_mode,
        );
        if parity_mode == BoundedIndexParityMode::Certification {
            certify_storage_parity(runtime, &snapshot, &request, &outcome)?;
            runtime
                .performance_access()
                .count_query_index_parity_verification();
        }
        runtime.performance_access().count_query_index_path();
        Ok(outcome)
    }
}

fn verify_bounded_index_entries(
    runtime: &RelationalRuntime,
    snapshot: &crate::snapshots::data::SnapshotHandle,
    request: &BoundedEntityFieldLookupRequest,
    indexed_ids: &[crate::identity::data::EntityId],
) -> Result<Vec<crate::identity::data::EntityId>, BoundedEntityFieldLookupDenial> {
    let state = runtime.storage_access().current_state();
    let expected = AuthoritativeFieldComparisonKey::from_aspect_value(request.value());
    let mut seen = BTreeSet::new();
    let mut candidates = Vec::with_capacity(indexed_ids.len().min(request.candidate_limit()));
    for entity_id in indexed_ids.iter().take(request.candidate_limit()) {
        let record = runtime
            .read_truth()
            .authoritative_entity_record_for_id_at_version(&state, *entity_id, snapshot.version_id)
            .ok_or_else(|| corrupt_index_denial(request))?;
        if !seen.insert(record.entity_id)
            || entity_query_locus_comparison_key(&record, request.field_locator())
                != Some(expected.clone())
        {
            return Err(corrupt_index_denial(request));
        }
        if record.kind.kind_id == request.entity_kind() {
            candidates.push(record.entity_id);
        }
    }
    Ok(candidates)
}

fn certify_storage_parity(
    runtime: &RelationalRuntime,
    snapshot: &crate::snapshots::data::SnapshotHandle,
    request: &BoundedEntityFieldLookupRequest,
    indexed: &BoundedEntityFieldLookupOutcome,
) -> Result<(), BoundedEntityFieldLookupDenial> {
    let expected = AuthoritativeFieldComparisonKey::from_aspect_value(request.value());
    let view = runtime
        .read_truth()
        .project_snapshot(snapshot)
        .ok_or_else(|| {
            lookup_denial(
                BoundedEntityFieldLookupDenialKind::SnapshotUnavailable,
                request,
            )
        })?;
    let mut storage_ids = view
        .authoritative_entity_records(request.entity_kind())
        .into_iter()
        .filter(|record| {
            entity_query_locus_comparison_key(record, request.field_locator())
                == Some(expected.clone())
        })
        .map(|record| record.entity_id)
        .collect::<Vec<_>>();
    storage_ids.sort();
    let storage_overflowed = storage_ids.len() > request.candidate_limit();
    storage_ids.truncate(request.candidate_limit());
    if storage_ids == indexed.candidate_entity_ids() && storage_overflowed == indexed.overflowed() {
        Ok(())
    } else {
        Err(lookup_denial(
            BoundedEntityFieldLookupDenialKind::StorageParityMismatch,
            request,
        ))
    }
}

fn branch_id_for_version(
    runtime: &RelationalRuntime,
    version_id: crate::identity::data::VersionId,
) -> Option<crate::history::data::BranchId> {
    runtime
        .history
        .commit_graph
        .values()
        .find(|node| node.commit.version_id == version_id)
        .map(|node| node.commit.branch_id.clone())
}

fn corrupt_index_denial(
    request: &BoundedEntityFieldLookupRequest,
) -> BoundedEntityFieldLookupDenial {
    lookup_denial(
        BoundedEntityFieldLookupDenialKind::CorruptIndexEntries,
        request,
    )
}

fn lookup_denial(
    kind: BoundedEntityFieldLookupDenialKind,
    request: &BoundedEntityFieldLookupRequest,
) -> BoundedEntityFieldLookupDenial {
    BoundedEntityFieldLookupDenial::new(kind, request.index_id())
}
