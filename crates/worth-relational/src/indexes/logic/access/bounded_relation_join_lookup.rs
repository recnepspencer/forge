use std::collections::BTreeSet;

use crate::indexes::data::{
    BoundedIndexParityMode, BoundedRelationJoinLookupDenial, BoundedRelationJoinLookupDenialKind,
    BoundedRelationJoinLookupOutcome, BoundedRelationJoinLookupRequest,
    BoundedRelationJoinLookupWork, DerivedIndexEntries, DerivedIndexKind, RelationJoinEntry,
    RelationJoinKey,
};
use crate::logic::runtime::RelationalRuntime;
use crate::visibility::snapshot_states::resolve_snapshot_handle;

use super::IndexAccess;

impl IndexAccess<'_> {
    pub fn execute_bounded_relation_join_lookup(
        &self,
        request: BoundedRelationJoinLookupRequest,
        parity_mode: BoundedIndexParityMode,
    ) -> Result<BoundedRelationJoinLookupOutcome, BoundedRelationJoinLookupDenial> {
        let runtime = self.runtime;
        runtime.performance_access().count_query_index_attempt();
        let snapshot = resolve_snapshot_handle(runtime, request.snapshot()).ok_or_else(|| {
            denial(
                BoundedRelationJoinLookupDenialKind::SnapshotUnavailable,
                &request,
            )
        })?;
        let definition = runtime
            .indexes
            .definitions
            .get(&request.index_id())
            .ok_or_else(|| {
                denial(
                    BoundedRelationJoinLookupDenialKind::IndexNotInstalled,
                    &request,
                )
            })?;
        if !matches!(&definition.kind, DerivedIndexKind::RelationJoin(_)) {
            return Err(denial(
                BoundedRelationJoinLookupDenialKind::WrongIndexKind,
                &request,
            ));
        }
        let generation =
            super::generation_selection::exact_published_generation(runtime, &snapshot, definition)
                .ok_or_else(|| {
                    denial(
                        BoundedRelationJoinLookupDenialKind::ExactGenerationUnavailable,
                        &request,
                    )
                })?;
        let DerivedIndexEntries::RelationJoin(entries) = &generation.entries else {
            return Err(denial(
                BoundedRelationJoinLookupDenialKind::WrongIndexKind,
                &request,
            ));
        };
        let key = RelationJoinKey::new(request.left_entity_id(), request.right_entity_id());
        let indexed = entries.get(&key).map(Vec::as_slice).unwrap_or(&[]);
        let overflowed = indexed.len() > request.candidate_limit();
        let examined_entry_count = indexed.len().min(request.candidate_limit());
        let selected = &indexed[..examined_entry_count];
        let candidate_entity_ids =
            verify_entries(runtime, &snapshot, definition, &request, selected)?;
        let outcome = BoundedRelationJoinLookupOutcome::new(
            generation.generation_id,
            candidate_entity_ids,
            BoundedRelationJoinLookupWork::for_verified_candidates(examined_entry_count),
            overflowed,
            parity_mode,
        );
        if parity_mode == BoundedIndexParityMode::Certification {
            certify_storage_parity(runtime, &snapshot, definition, &request, &outcome)?;
            runtime
                .performance_access()
                .count_query_index_parity_verification();
        }
        runtime.performance_access().count_query_index_path();
        Ok(outcome)
    }
}

fn verify_entries(
    runtime: &RelationalRuntime,
    snapshot: &crate::snapshots::data::SnapshotHandle,
    definition: &crate::indexes::data::DerivedIndexDefinition,
    request: &BoundedRelationJoinLookupRequest,
    entries: &[RelationJoinEntry],
) -> Result<Vec<crate::identity::data::EntityId>, BoundedRelationJoinLookupDenial> {
    let verification = RelationJoinVerification::new(definition, snapshot, request)?;
    let state = runtime.storage_access().current_state();
    verification.verify_external_endpoints(runtime, &state)?;
    let mut seen = BTreeSet::new();
    let mut candidates = Vec::with_capacity(entries.len());
    for entry in entries {
        let candidate = verification.verify_entry(runtime, &state, entry)?;
        if !seen.insert(candidate) {
            return Err(verification.corrupt());
        }
        candidates.push(candidate);
    }
    Ok(candidates)
}

struct RelationJoinVerification {
    join: crate::indexes::data::RelationJoinDefinition,
    version_id: crate::identity::data::VersionId,
    left_entity_id: crate::identity::data::EntityId,
    right_entity_id: crate::identity::data::EntityId,
    index_id: crate::indexes::data::DerivedIndexId,
}

impl RelationJoinVerification {
    fn new(
        definition: &crate::indexes::data::DerivedIndexDefinition,
        snapshot: &crate::snapshots::data::SnapshotHandle,
        request: &BoundedRelationJoinLookupRequest,
    ) -> Result<Self, BoundedRelationJoinLookupDenial> {
        let DerivedIndexKind::RelationJoin(join) = definition.kind else {
            return Err(corrupt(request));
        };
        Ok(Self {
            join,
            version_id: snapshot.version_id,
            left_entity_id: request.left_entity_id(),
            right_entity_id: request.right_entity_id(),
            index_id: request.index_id(),
        })
    }

    fn verify_external_endpoints(
        &self,
        runtime: &RelationalRuntime,
        state: &crate::storage::overlay::BorrowedWorkingState<'_>,
    ) -> Result<(), BoundedRelationJoinLookupDenial> {
        let left = runtime
            .read_truth()
            .authoritative_entity_record_for_id_at_version(
                state,
                self.left_entity_id,
                self.version_id,
            )
            .ok_or_else(|| self.corrupt())?;
        let right = runtime
            .read_truth()
            .authoritative_entity_record_for_id_at_version(
                state,
                self.right_entity_id,
                self.version_id,
            )
            .ok_or_else(|| self.corrupt())?;
        if left.kind.kind_id == self.join.left().external_entity_kind()
            && right.kind.kind_id == self.join.right().external_entity_kind()
        {
            Ok(())
        } else {
            Err(self.corrupt())
        }
    }

    fn verify_entry(
        &self,
        runtime: &RelationalRuntime,
        state: &crate::storage::overlay::BorrowedWorkingState<'_>,
        entry: &RelationJoinEntry,
    ) -> Result<crate::identity::data::EntityId, BoundedRelationJoinLookupDenial> {
        let shared = runtime
            .read_truth()
            .authoritative_entity_record_for_id_at_version(
                state,
                entry.shared_entity_id(),
                self.version_id,
            )
            .ok_or_else(|| self.corrupt())?;
        let left = runtime
            .read_truth()
            .authoritative_relation_record_for_id_at_version(
                state,
                entry.left_relation_id(),
                self.version_id,
            )
            .ok_or_else(|| self.corrupt())?;
        let right = runtime
            .read_truth()
            .authoritative_relation_record_for_id_at_version(
                state,
                entry.right_relation_id(),
                self.version_id,
            )
            .ok_or_else(|| self.corrupt())?;
        if self.entry_matches(entry, &shared, &left, &right) {
            Ok(entry.shared_entity_id())
        } else {
            Err(self.corrupt())
        }
    }

    fn entry_matches(
        &self,
        entry: &RelationJoinEntry,
        shared: &crate::storage::data::EntityReadRecord,
        left: &crate::storage::data::RelationReadRecord,
        right: &crate::storage::data::RelationReadRecord,
    ) -> bool {
        let left_endpoints = super::super::projected_field_values::join_endpoints(
            left,
            self.join.left().shared_endpoint(),
        );
        let right_endpoints = super::super::projected_field_values::join_endpoints(
            right,
            self.join.right().shared_endpoint(),
        );
        shared.kind.kind_id == self.join.shared_entity_kind()
            && left.kind.kind_id == self.join.left().relation_kind()
            && right.kind.kind_id == self.join.right().relation_kind()
            && left_endpoints == (entry.shared_entity_id(), self.left_entity_id)
            && right_endpoints == (entry.shared_entity_id(), self.right_entity_id)
    }

    const fn corrupt(&self) -> BoundedRelationJoinLookupDenial {
        BoundedRelationJoinLookupDenial::new(
            BoundedRelationJoinLookupDenialKind::CorruptIndexEntries,
            self.index_id,
        )
    }
}

fn certify_storage_parity(
    runtime: &RelationalRuntime,
    snapshot: &crate::snapshots::data::SnapshotHandle,
    definition: &crate::indexes::data::DerivedIndexDefinition,
    request: &BoundedRelationJoinLookupRequest,
    indexed: &BoundedRelationJoinLookupOutcome,
) -> Result<(), BoundedRelationJoinLookupDenial> {
    let DerivedIndexKind::RelationJoin(join) = definition.kind else {
        return Err(corrupt(request));
    };
    let read = runtime
        .read_truth()
        .read_snapshot(snapshot)
        .ok_or_else(|| {
            denial(
                BoundedRelationJoinLookupDenialKind::SnapshotUnavailable,
                request,
            )
        })?;
    let projection =
        super::super::projected_field_values::IndexProjectionSource::Reconstructed(&read);
    let expected =
        super::super::projected_field_values::build_relation_join_index(&projection, join);
    let key = RelationJoinKey::new(request.left_entity_id(), request.right_entity_id());
    let expected = expected.get(&key).map(Vec::as_slice).unwrap_or(&[]);
    let expected_overflowed = expected.len() > request.candidate_limit();
    let expected_ids = expected
        .iter()
        .take(request.candidate_limit())
        .map(|entry| entry.shared_entity_id())
        .collect::<Vec<_>>();
    if expected_ids == indexed.candidate_entity_ids() && expected_overflowed == indexed.overflowed()
    {
        Ok(())
    } else {
        Err(denial(
            BoundedRelationJoinLookupDenialKind::StorageParityMismatch,
            request,
        ))
    }
}

fn corrupt(request: &BoundedRelationJoinLookupRequest) -> BoundedRelationJoinLookupDenial {
    denial(
        BoundedRelationJoinLookupDenialKind::CorruptIndexEntries,
        request,
    )
}

fn denial(
    kind: BoundedRelationJoinLookupDenialKind,
    request: &BoundedRelationJoinLookupRequest,
) -> BoundedRelationJoinLookupDenial {
    BoundedRelationJoinLookupDenial::new(kind, request.index_id())
}
