use std::collections::BTreeSet;

use crate::indexes::data::{
    BoundedIndexParityMode, BoundedRelationJoinLookupDenial, BoundedRelationJoinLookupDenialKind,
    BoundedRelationJoinLookupOutcome, BoundedRelationJoinLookupRequest,
    BoundedRelationJoinLookupWork, DerivedIndexEntries, DerivedIndexKind, RelationJoinEntry,
    RelationJoinKey,
};
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
        let projection = runtime
            .read_truth()
            .project_snapshot(&snapshot)
            .ok_or_else(|| {
                denial(
                    BoundedRelationJoinLookupDenialKind::SnapshotUnavailable,
                    &request,
                )
            })?;
        let source =
            super::super::projected_field_values::IndexProjectionSource::exact(&projection)
                .expect("resolved snapshot projection must carry an exact basis");
        let prepared = prepare_relation_join_lookup(runtime, &snapshot, &request)?;
        let overflowed = prepared.indexed_entries.len() > request.candidate_limit();
        let examined_entry_count = prepared
            .indexed_entries
            .len()
            .min(request.candidate_limit());
        let candidate_entity_ids = prepared
            .verification
            .verify_entries(&source, &prepared.indexed_entries[..examined_entry_count])?;
        let outcome = BoundedRelationJoinLookupOutcome::new(
            prepared.generation_id,
            candidate_entity_ids,
            BoundedRelationJoinLookupWork::for_verified_candidates(examined_entry_count),
            overflowed,
            parity_mode,
        );
        if parity_mode == BoundedIndexParityMode::Certification {
            certify_storage_parity(&source, &prepared.verification, &request, &outcome)?;
            runtime
                .performance_access()
                .count_query_index_parity_verification();
        }
        runtime.performance_access().count_query_index_path();
        Ok(outcome)
    }
}

struct PreparedRelationJoinLookup<'runtime> {
    generation_id: crate::indexes::data::DerivedIndexGenerationId,
    indexed_entries: &'runtime [RelationJoinEntry],
    verification: RelationJoinVerification,
}

fn prepare_relation_join_lookup<'runtime>(
    runtime: &'runtime crate::runtime::RelationalRuntime,
    snapshot: &crate::snapshots::data::SnapshotHandle,
    request: &BoundedRelationJoinLookupRequest,
) -> Result<PreparedRelationJoinLookup<'runtime>, BoundedRelationJoinLookupDenial> {
    let definition = runtime
        .indexes
        .definitions
        .get(&request.index_id())
        .ok_or_else(|| {
            denial(
                BoundedRelationJoinLookupDenialKind::IndexNotInstalled,
                request,
            )
        })?;
    let verification = RelationJoinVerification::new(definition, request)?;
    let generation =
        super::generation_selection::exact_published_generation(runtime, snapshot, definition)
            .ok_or_else(|| {
                denial(
                    BoundedRelationJoinLookupDenialKind::ExactGenerationUnavailable,
                    request,
                )
            })?;
    let DerivedIndexEntries::RelationJoin(entries) = &generation.entries else {
        return Err(denial(
            BoundedRelationJoinLookupDenialKind::WrongIndexKind,
            request,
        ));
    };
    let key = RelationJoinKey::new(request.left_entity_id(), request.right_entity_id());
    Ok(PreparedRelationJoinLookup {
        generation_id: generation.generation_id,
        indexed_entries: entries.get(&key).map(Vec::as_slice).unwrap_or(&[]),
        verification,
    })
}

struct RelationJoinVerification {
    join: crate::indexes::data::RelationJoinDefinition,
    left_entity_id: crate::identity::data::EntityId,
    right_entity_id: crate::identity::data::EntityId,
    index_id: crate::indexes::data::DerivedIndexId,
}

impl RelationJoinVerification {
    fn new(
        definition: &crate::indexes::data::DerivedIndexDefinition,
        request: &BoundedRelationJoinLookupRequest,
    ) -> Result<Self, BoundedRelationJoinLookupDenial> {
        let DerivedIndexKind::RelationJoin(join) = definition.kind else {
            return Err(corrupt(request));
        };
        Ok(Self {
            join,
            left_entity_id: request.left_entity_id(),
            right_entity_id: request.right_entity_id(),
            index_id: request.index_id(),
        })
    }

    fn verify_external_endpoints(
        &self,
        source: &super::super::projected_field_values::IndexProjectionSource<'_, '_>,
    ) -> Result<(), BoundedRelationJoinLookupDenial> {
        let left = source
            .with_entity(self.left_entity_id, Clone::clone)
            .ok_or_else(|| self.corrupt())?;
        let right = source
            .with_entity(self.right_entity_id, Clone::clone)
            .ok_or_else(|| self.corrupt())?;
        if left.kind.kind_id == self.join.left().external_entity_kind()
            && right.kind.kind_id == self.join.right().external_entity_kind()
        {
            Ok(())
        } else {
            Err(self.corrupt())
        }
    }

    fn verify_entries(
        &self,
        source: &super::super::projected_field_values::IndexProjectionSource<'_, '_>,
        entries: &[RelationJoinEntry],
    ) -> Result<Vec<crate::identity::data::EntityId>, BoundedRelationJoinLookupDenial> {
        self.verify_external_endpoints(source)?;
        let mut seen = BTreeSet::new();
        let mut candidates = Vec::with_capacity(entries.len());
        for entry in entries {
            let candidate = self.verify_entry(source, entry)?;
            if !seen.insert(candidate) {
                return Err(self.corrupt());
            }
            candidates.push(candidate);
        }
        Ok(candidates)
    }

    fn verify_entry(
        &self,
        source: &super::super::projected_field_values::IndexProjectionSource<'_, '_>,
        entry: &RelationJoinEntry,
    ) -> Result<crate::identity::data::EntityId, BoundedRelationJoinLookupDenial> {
        let shared = source
            .with_entity(entry.shared_entity_id(), Clone::clone)
            .ok_or_else(|| self.corrupt())?;
        let left = source
            .with_relation(entry.left_relation_id(), Clone::clone)
            .ok_or_else(|| self.corrupt())?;
        let right = source
            .with_relation(entry.right_relation_id(), Clone::clone)
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
    source: &super::super::projected_field_values::IndexProjectionSource<'_, '_>,
    verification: &RelationJoinVerification,
    request: &BoundedRelationJoinLookupRequest,
    indexed: &BoundedRelationJoinLookupOutcome,
) -> Result<(), BoundedRelationJoinLookupDenial> {
    let expected =
        super::super::projected_field_values::build_relation_join_index(source, verification.join);
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
