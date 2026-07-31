use crate::indexes::data::{
    BoundedIndexParityMode, BoundedRelatedEntityOrderedLookupDenial,
    BoundedRelatedEntityOrderedLookupDenialKind, BoundedRelatedEntityOrderedLookupOutcome,
    BoundedRelatedEntityOrderedLookupRequest, DerivedIndexEntries, DerivedIndexKind,
    DerivedIndexPublicationStatus, RelatedEntityEndpoint, RelatedEntityOrderingBoundary,
    RelatedEntityOrderingEntry, RelatedEntityOrderingField,
};
use crate::logic::runtime::RelationalRuntime;
use crate::visibility::snapshot_states::resolve_snapshot_handle;

use super::super::projected_field_values::{
    build_related_entity_ordering_index, compare_related_entries,
};
use super::IndexAccess;

impl IndexAccess<'_> {
    pub fn execute_bounded_related_entity_ordered_lookup(
        &self,
        request: BoundedRelatedEntityOrderedLookupRequest,
        parity_mode: BoundedIndexParityMode,
    ) -> Result<BoundedRelatedEntityOrderedLookupOutcome, BoundedRelatedEntityOrderedLookupDenial>
    {
        let runtime = self.runtime;
        runtime.performance_access().count_query_index_attempt();
        let snapshot = resolve_snapshot_handle(runtime, request.snapshot()).ok_or_else(|| {
            denial(
                BoundedRelatedEntityOrderedLookupDenialKind::SnapshotUnavailable,
                &request,
            )
        })?;
        let definition = runtime
            .indexes
            .definitions
            .get(&request.index_id())
            .ok_or_else(|| {
                denial(
                    BoundedRelatedEntityOrderedLookupDenialKind::IndexNotInstalled,
                    &request,
                )
            })?;
        let DerivedIndexKind::RelatedEntityOrdering {
            relation_kind,
            parent_endpoint,
            child_kind,
            ordering,
        } = &definition.kind
        else {
            return Err(denial(
                BoundedRelatedEntityOrderedLookupDenialKind::WrongIndexKind,
                &request,
            ));
        };
        if *child_kind != request.child_kind() {
            return Err(denial(
                BoundedRelatedEntityOrderedLookupDenialKind::WrongIndexKind,
                &request,
            ));
        }
        let generation = exact_generation(runtime, &snapshot, definition).ok_or_else(|| {
            denial(
                BoundedRelatedEntityOrderedLookupDenialKind::ExactGenerationUnavailable,
                &request,
            )
        })?;
        if request
            .expected_generation()
            .is_some_and(|expected| expected != generation.generation_id)
        {
            return Err(denial(
                BoundedRelatedEntityOrderedLookupDenialKind::ExpectedGenerationMismatch,
                &request,
            ));
        }
        let DerivedIndexEntries::RelatedEntityOrdering(entries) = &generation.entries else {
            return Err(denial(
                BoundedRelatedEntityOrderedLookupDenialKind::WrongIndexKind,
                &request,
            ));
        };
        let parent_entries = entries
            .get(&request.parent_entity_id())
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        let page = select_page(parent_entries, ordering, &request)?;
        verify_entries(
            runtime,
            &snapshot,
            *relation_kind,
            *parent_endpoint,
            *child_kind,
            ordering,
            request.parent_entity_id(),
            page.rows,
            &request,
        )?;
        let outcome = outcome_from_page(generation.generation_id, page, parity_mode);
        if parity_mode == BoundedIndexParityMode::Certification {
            certify_storage_parity(
                runtime,
                &snapshot,
                *relation_kind,
                *parent_endpoint,
                *child_kind,
                ordering,
                &request,
                &outcome,
            )?;
            runtime
                .performance_access()
                .count_query_index_parity_verification();
        }
        runtime.performance_access().count_query_index_path();
        Ok(outcome)
    }
}

struct SelectedPage<'a> {
    rows: &'a [RelatedEntityOrderingEntry],
    has_more: bool,
    seek_comparison_count: usize,
}

fn select_page<'a>(
    entries: &'a [RelatedEntityOrderingEntry],
    ordering: &[RelatedEntityOrderingField],
    request: &BoundedRelatedEntityOrderedLookupRequest,
) -> Result<SelectedPage<'a>, BoundedRelatedEntityOrderedLookupDenial> {
    let mut seek_comparison_count = 0;
    let start = match request.after() {
        None => 0,
        Some(boundary) => entries
            .binary_search_by(|entry| {
                seek_comparison_count += 1;
                compare_related_entries(entry, boundary.entry(), ordering)
            })
            .map(|index| index + 1)
            .map_err(|_| {
                denial(
                    BoundedRelatedEntityOrderedLookupDenialKind::ForeignBoundary,
                    request,
                )
            })?,
    };
    let available = &entries[start.min(entries.len())..];
    let returned = available.len().min(request.page_width());
    Ok(SelectedPage {
        rows: &available[..returned],
        has_more: available.len() > returned,
        seek_comparison_count,
    })
}

fn outcome_from_page(
    generation_id: crate::indexes::data::DerivedIndexGenerationId,
    page: SelectedPage<'_>,
    parity_mode: BoundedIndexParityMode,
) -> BoundedRelatedEntityOrderedLookupOutcome {
    let child_entity_ids = page
        .rows
        .iter()
        .map(RelatedEntityOrderingEntry::child_entity_id)
        .collect();
    let next_boundary = page
        .has_more
        .then(|| {
            page.rows
                .last()
                .map(RelatedEntityOrderingBoundary::from_entry)
        })
        .flatten();
    BoundedRelatedEntityOrderedLookupOutcome::new(
        generation_id,
        child_entity_ids,
        next_boundary,
        page.rows.len() + page.seek_comparison_count,
        page.seek_comparison_count,
        parity_mode,
    )
}

#[allow(clippy::too_many_arguments)]
fn verify_entries(
    runtime: &RelationalRuntime,
    snapshot: &crate::snapshots::data::SnapshotHandle,
    relation_kind: crate::identity::data::KindId,
    parent_endpoint: RelatedEntityEndpoint,
    child_kind: crate::identity::data::KindId,
    ordering: &[RelatedEntityOrderingField],
    parent: crate::identity::data::EntityId,
    entries: &[RelatedEntityOrderingEntry],
    request: &BoundedRelatedEntityOrderedLookupRequest,
) -> Result<(), BoundedRelatedEntityOrderedLookupDenial> {
    let read = runtime
        .read_truth()
        .read_snapshot(snapshot)
        .ok_or_else(|| {
            denial(
                BoundedRelatedEntityOrderedLookupDenialKind::SnapshotUnavailable,
                request,
            )
        })?;
    let projection =
        super::super::projected_field_values::IndexProjectionSource::Reconstructed(&read);
    for entry in entries {
        let relation = projection
            .with_relation(entry.relation_id(), Clone::clone)
            .ok_or_else(|| corrupt(request))?;
        let (observed_parent, observed_child) = match parent_endpoint {
            RelatedEntityEndpoint::SourceParent => (relation.source, relation.target),
            RelatedEntityEndpoint::TargetParent => (relation.target, relation.source),
        };
        if relation.kind.kind_id != relation_kind
            || observed_parent != parent
            || observed_child != entry.child_entity_id()
            || projection.with_entity(observed_child, |record| record.kind.kind_id)
                != Some(child_kind)
        {
            return Err(corrupt(request));
        }
        let values = ordering
            .iter()
            .map(|field| {
                super::super::projected_field_values::entity_aspect_field_ordering_value(
                    runtime,
                    &projection,
                    observed_child,
                    field.locator(),
                )
            })
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| corrupt(request))?;
        if !values
            .iter()
            .zip(entry.ordering_values())
            .all(|(value, expected)| value == expected.value())
            || values.len() != entry.ordering_values().len()
        {
            return Err(corrupt(request));
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn certify_storage_parity(
    runtime: &RelationalRuntime,
    snapshot: &crate::snapshots::data::SnapshotHandle,
    relation_kind: crate::identity::data::KindId,
    parent_endpoint: RelatedEntityEndpoint,
    child_kind: crate::identity::data::KindId,
    ordering: &[RelatedEntityOrderingField],
    request: &BoundedRelatedEntityOrderedLookupRequest,
    indexed: &BoundedRelatedEntityOrderedLookupOutcome,
) -> Result<(), BoundedRelatedEntityOrderedLookupDenial> {
    let read = runtime
        .read_truth()
        .read_snapshot(snapshot)
        .ok_or_else(|| {
            denial(
                BoundedRelatedEntityOrderedLookupDenialKind::SnapshotUnavailable,
                request,
            )
        })?;
    let projection =
        super::super::projected_field_values::IndexProjectionSource::Reconstructed(&read);
    let storage = build_related_entity_ordering_index(
        runtime,
        &projection,
        relation_kind,
        parent_endpoint,
        child_kind,
        ordering,
    );
    let expected = select_page(
        storage
            .get(&request.parent_entity_id())
            .map(Vec::as_slice)
            .unwrap_or(&[]),
        ordering,
        request,
    )?;
    let expected = outcome_from_page(indexed.generation_id(), expected, indexed.parity_mode());
    if expected.child_entity_ids() == indexed.child_entity_ids()
        && expected.next_boundary() == indexed.next_boundary()
    {
        Ok(())
    } else {
        Err(denial(
            BoundedRelatedEntityOrderedLookupDenialKind::StorageParityMismatch,
            request,
        ))
    }
}

fn exact_generation<'a>(
    runtime: &'a RelationalRuntime,
    snapshot: &crate::snapshots::data::SnapshotHandle,
    definition: &crate::indexes::data::DerivedIndexDefinition,
) -> Option<&'a crate::indexes::data::DerivedIndexGeneration> {
    let branch_id = runtime
        .history
        .commit_graph
        .values()
        .find(|node| node.commit.version_id == snapshot.version_id)
        .map(|node| &node.commit.branch_id);
    let schema_version = runtime
        .read_truth()
        .query_plan_context(snapshot)?
        .schema_version;
    runtime
        .indexes
        .generations
        .get(&definition.index_id)?
        .iter()
        .rev()
        .find(|generation| {
            generation.status == DerivedIndexPublicationStatus::Published
                && generation.applicability.version_id == snapshot.version_id
                && generation.applicability.schema_version == schema_version
                && (!definition.branch_scoped
                    || branch_id
                        .is_some_and(|branch| generation.applicability.branch_id == *branch))
        })
}

fn corrupt(
    request: &BoundedRelatedEntityOrderedLookupRequest,
) -> BoundedRelatedEntityOrderedLookupDenial {
    denial(
        BoundedRelatedEntityOrderedLookupDenialKind::CorruptIndexEntries,
        request,
    )
}

fn denial(
    kind: BoundedRelatedEntityOrderedLookupDenialKind,
    request: &BoundedRelatedEntityOrderedLookupRequest,
) -> BoundedRelatedEntityOrderedLookupDenial {
    BoundedRelatedEntityOrderedLookupDenial::new(kind, request.index_id())
}
