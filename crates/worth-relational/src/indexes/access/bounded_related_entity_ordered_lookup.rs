use crate::indexes::data::{
    BoundedIndexParityMode, BoundedRelatedEntityOrderedLookupDenial,
    BoundedRelatedEntityOrderedLookupDenialKind, BoundedRelatedEntityOrderedLookupOutcome,
    BoundedRelatedEntityOrderedLookupRequest, DerivedIndexEntries, DerivedIndexKind,
    RelatedEntityEndpoint, RelatedEntityOrderingBoundary, RelatedEntityOrderingEntry,
    RelatedEntityOrderingField,
};
use crate::runtime::RelationalRuntime;
use crate::visibility::snapshot_states::resolve_snapshot_handle;

use super::super::projected_field_values::{
    build_related_entity_ordering_index, compare_related_entries, RelatedEntityOrderingProjection,
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
        let projection = runtime
            .read_truth()
            .project_snapshot(&snapshot)
            .ok_or_else(|| {
                denial(
                    BoundedRelatedEntityOrderedLookupDenialKind::SnapshotUnavailable,
                    &request,
                )
            })?;
        let source =
            super::super::projected_field_values::IndexProjectionSource::exact(&projection)
                .expect("resolved snapshot projection must carry an exact basis");
        let prepared = prepare_related_lookup(runtime, &snapshot, &request)?;
        let page = select_page(
            prepared.parent_entries,
            prepared.contract.ordering,
            &request,
        )?;
        verify_entries(&source, &prepared.contract, page.rows)?;
        let outcome = outcome_from_page(prepared.generation_id, page, parity_mode);
        if parity_mode == BoundedIndexParityMode::Certification {
            certify_storage_parity(&source, &prepared.contract, &outcome)?;
            runtime
                .performance_access()
                .count_query_index_parity_verification();
        }
        runtime.performance_access().count_query_index_path();
        Ok(outcome)
    }
}

struct PreparedRelatedLookup<'definition, 'request> {
    generation_id: crate::indexes::data::DerivedIndexGenerationId,
    parent_entries: &'definition [RelatedEntityOrderingEntry],
    contract: RelatedLookupContract<'definition, 'request>,
}

struct RelatedLookupContract<'definition, 'request> {
    relation_kind: crate::identity::data::KindId,
    parent_endpoint: RelatedEntityEndpoint,
    child_kind: crate::identity::data::KindId,
    ordering: &'definition [RelatedEntityOrderingField],
    parent: crate::identity::data::EntityId,
    request: &'request BoundedRelatedEntityOrderedLookupRequest,
}

fn prepare_related_lookup<'definition, 'request>(
    runtime: &'definition RelationalRuntime,
    snapshot: &crate::snapshots::data::SnapshotHandle,
    request: &'request BoundedRelatedEntityOrderedLookupRequest,
) -> Result<PreparedRelatedLookup<'definition, 'request>, BoundedRelatedEntityOrderedLookupDenial> {
    let (definition, contract) = resolve_related_lookup_contract(runtime, request)?;
    let generation =
        super::generation_selection::exact_published_generation(runtime, snapshot, definition)
            .ok_or_else(|| {
                denial(
                    BoundedRelatedEntityOrderedLookupDenialKind::ExactGenerationUnavailable,
                    request,
                )
            })?;
    if request
        .expected_generation()
        .is_some_and(|expected| expected != generation.generation_id)
    {
        return Err(denial(
            BoundedRelatedEntityOrderedLookupDenialKind::ExpectedGenerationMismatch,
            request,
        ));
    }
    let DerivedIndexEntries::RelatedEntityOrdering(entries) = &generation.entries else {
        return Err(denial(
            BoundedRelatedEntityOrderedLookupDenialKind::WrongIndexKind,
            request,
        ));
    };
    Ok(PreparedRelatedLookup {
        generation_id: generation.generation_id,
        parent_entries: entries
            .get(&contract.parent)
            .map(Vec::as_slice)
            .unwrap_or(&[]),
        contract,
    })
}

fn resolve_related_lookup_contract<'definition, 'request>(
    runtime: &'definition RelationalRuntime,
    request: &'request BoundedRelatedEntityOrderedLookupRequest,
) -> Result<
    (
        &'definition crate::indexes::data::DerivedIndexDefinition,
        RelatedLookupContract<'definition, 'request>,
    ),
    BoundedRelatedEntityOrderedLookupDenial,
> {
    let definition = runtime
        .indexes
        .definitions
        .get(&request.index_id())
        .ok_or_else(|| {
            denial(
                BoundedRelatedEntityOrderedLookupDenialKind::IndexNotInstalled,
                request,
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
            request,
        ));
    };
    if *child_kind != request.child_kind() {
        return Err(denial(
            BoundedRelatedEntityOrderedLookupDenialKind::WrongIndexKind,
            request,
        ));
    }
    Ok((
        definition,
        RelatedLookupContract {
            relation_kind: *relation_kind,
            parent_endpoint: *parent_endpoint,
            child_kind: *child_kind,
            ordering,
            parent: request.parent_entity_id(),
            request,
        },
    ))
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

fn verify_entries(
    projection: &super::super::projected_field_values::IndexProjectionSource<'_, '_>,
    contract: &RelatedLookupContract<'_, '_>,
    entries: &[RelatedEntityOrderingEntry],
) -> Result<(), BoundedRelatedEntityOrderedLookupDenial> {
    for entry in entries {
        let relation = projection
            .with_relation(entry.relation_id(), Clone::clone)
            .ok_or_else(|| corrupt(contract.request))?;
        let (observed_parent, observed_child) = match contract.parent_endpoint {
            RelatedEntityEndpoint::SourceParent => (relation.source, relation.target),
            RelatedEntityEndpoint::TargetParent => (relation.target, relation.source),
        };
        if relation.kind.kind_id != contract.relation_kind
            || observed_parent != contract.parent
            || observed_child != entry.child_entity_id()
            || projection.with_entity(observed_child, |record| record.kind.kind_id)
                != Some(contract.child_kind)
        {
            return Err(corrupt(contract.request));
        }
        let values = contract
            .ordering
            .iter()
            .map(|field| {
                super::super::projected_field_values::entity_aspect_field_ordering_value(
                    projection,
                    observed_child,
                    field.locator(),
                )
            })
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| corrupt(contract.request))?;
        if !values
            .iter()
            .zip(entry.ordering_values())
            .all(|(value, expected)| value == expected.value())
            || values.len() != entry.ordering_values().len()
        {
            return Err(corrupt(contract.request));
        }
    }
    Ok(())
}

fn certify_storage_parity(
    projection: &super::super::projected_field_values::IndexProjectionSource<'_, '_>,
    contract: &RelatedLookupContract<'_, '_>,
    indexed: &BoundedRelatedEntityOrderedLookupOutcome,
) -> Result<(), BoundedRelatedEntityOrderedLookupDenial> {
    let storage = build_related_entity_ordering_index(
        projection,
        &RelatedEntityOrderingProjection::new(
            contract.relation_kind,
            contract.parent_endpoint,
            contract.child_kind,
            contract.ordering,
        ),
    );
    let expected = select_page(
        storage
            .get(&contract.parent)
            .map(Vec::as_slice)
            .unwrap_or(&[]),
        contract.ordering,
        contract.request,
    )?;
    let expected = outcome_from_page(indexed.generation_id(), expected, indexed.parity_mode());
    if expected.child_entity_ids() == indexed.child_entity_ids()
        && expected.next_boundary() == indexed.next_boundary()
    {
        Ok(())
    } else {
        Err(denial(
            BoundedRelatedEntityOrderedLookupDenialKind::StorageParityMismatch,
            contract.request,
        ))
    }
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
