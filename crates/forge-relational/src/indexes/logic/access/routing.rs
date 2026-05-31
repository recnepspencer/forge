use crate::history::data::BranchId;
use crate::indexes::data::{DerivedIndexEntries, DerivedIndexGeneration, DerivedIndexKind};
use crate::logic::runtime::RelationalRuntime;
use crate::query::data::{
    IndexQueryRejectionClass, PlannedQueryPacket, QueryAccessContract, QueryAccessPath, QueryScope,
    SnapshotPinnedQueryPlan,
};

const SAMPLED_PARITY_MODULUS: u128 = 8;
const SAMPLED_PARITY_REMAINDER: u128 = 0;

pub(crate) fn admissible_access_path(
    runtime: &RelationalRuntime,
    plan: &SnapshotPinnedQueryPlan,
) -> QueryAccessPath {
    if plan.packet.access_contract == QueryAccessContract::AuthoritativeStorageOnly {
        return QueryAccessPath::AuthoritativeStorage;
    }

    let branch_id = branch_id_for_version(runtime, plan.snapshot.version_id)
        .unwrap_or_else(|| runtime.config.history.main_branch.clone());
    let Some(generation) = candidate_generation_for_packet(runtime, &plan.packet, &branch_id)
    else {
        return QueryAccessPath::DerivedIndexRejectedStorageRead {
            rejection: if matching_index_definition_exists(runtime, &plan.packet) {
                IndexQueryRejectionClass::MissingGeneration
            } else if runtime
                .indexes
                .generations
                .values()
                .flat_map(|generations| generations.iter())
                .any(|generation| generation.compatibility.version_id <= plan.snapshot.version_id)
            {
                IndexQueryRejectionClass::UnsupportedScope
            } else {
                IndexQueryRejectionClass::MissingGeneration
            },
        };
    };

    match index_rejection_for_packet(runtime, &plan.packet, generation, &branch_id) {
        Some(rejection) => QueryAccessPath::DerivedIndexRejectedStorageRead { rejection },
        None => QueryAccessPath::DerivedIndexGeneration {
            generation_id: generation.generation_id,
        },
    }
}

pub(crate) fn should_verify_sampled_parity(
    plan: &SnapshotPinnedQueryPlan,
    generation_id: crate::indexes::data::DerivedIndexGenerationId,
) -> bool {
    let sample_key = plan.packet.plan_key.0
        ^ ((generation_id.0 as u128) << 64)
        ^ (plan.snapshot.version_id.0 as u128);
    sample_key % SAMPLED_PARITY_MODULUS == SAMPLED_PARITY_REMAINDER
}

fn branch_id_for_version(
    runtime: &RelationalRuntime,
    version_id: crate::identity::data::VersionId,
) -> Option<BranchId> {
    runtime
        .history
        .commit_graph
        .values()
        .find(|node| node.commit.version_id == version_id)
        .map(|node| node.commit.branch_id.clone())
}

fn index_rejection_for_packet(
    runtime: &RelationalRuntime,
    packet: &PlannedQueryPacket,
    generation: &DerivedIndexGeneration,
    branch_id: &BranchId,
) -> Option<IndexQueryRejectionClass> {
    if generation.status != crate::indexes::data::DerivedIndexPublicationStatus::Published {
        return Some(IndexQueryRejectionClass::CorruptIndexEntries);
    }
    if generation.compatibility.branch_id != *branch_id
        && runtime
            .indexes
            .definitions
            .get(&generation.index_id)
            .is_some_and(|definition| definition.branch_scoped)
    {
        return Some(IndexQueryRejectionClass::IncompatibleBranch);
    }
    if generation.compatibility.version_id > packet.context_id.version_id {
        return Some(IndexQueryRejectionClass::IncompatibleVersion);
    }
    if generation.compatibility.schema_version != packet.context_id.schema_version {
        return Some(IndexQueryRejectionClass::IncompatibleVersion);
    }
    match &packet.scope {
        QueryScope::EntityFieldEquals { .. } | QueryScope::EntityFieldAnyOf { .. } => {
            if !matches!(
                packet.ordering,
                crate::query::data::QueryOrderingContract::CanonicalRecordRefOrder
                    | crate::query::data::QueryOrderingContract::CanonicalEntityIdOrder
            ) {
                return Some(IndexQueryRejectionClass::UnsupportedOrderingContract);
            }
        }
        QueryScope::RelationFieldEquals { .. } | QueryScope::RelationFieldAnyOf { .. } => {
            if !matches!(
                packet.ordering,
                crate::query::data::QueryOrderingContract::CanonicalRecordRefOrder
                    | crate::query::data::QueryOrderingContract::CanonicalRelationIdOrder
            ) {
                return Some(IndexQueryRejectionClass::UnsupportedOrderingContract);
            }
        }
        _ => {
            if !matches!(
                packet.ordering,
                crate::query::data::QueryOrderingContract::CanonicalRecordRefOrder
                    | crate::query::data::QueryOrderingContract::CanonicalEntityIdOrder
                    | crate::query::data::QueryOrderingContract::CanonicalRelationIdOrder
            ) {
                return Some(IndexQueryRejectionClass::UnsupportedOrderingContract);
            }
        }
    }
    match (
        &packet.scope,
        &generation.entries,
        runtime.indexes.definitions.get(&generation.index_id),
    ) {
        (
            QueryScope::EntityFieldEquals { field_locator, .. },
            DerivedIndexEntries::EntityField(_),
            Some(definition),
        )
        | (
            QueryScope::EntityFieldAnyOf { field_locator, .. },
            DerivedIndexEntries::EntityField(_),
            Some(definition),
        ) => match &definition.kind {
            DerivedIndexKind::EntityField {
                field_locator: indexed_field_locator,
            } if indexed_field_locator == field_locator => None,
            _ => Some(IndexQueryRejectionClass::UnsupportedScope),
        },
        (
            QueryScope::RelationFieldEquals { field_locator, .. },
            DerivedIndexEntries::RelationField(_),
            Some(definition),
        )
        | (
            QueryScope::RelationFieldAnyOf { field_locator, .. },
            DerivedIndexEntries::RelationField(_),
            Some(definition),
        ) => match &definition.kind {
            DerivedIndexKind::RelationField {
                field_locator: indexed_field_locator,
            } if indexed_field_locator == field_locator => None,
            _ => Some(IndexQueryRejectionClass::UnsupportedScope),
        },
        _ => Some(IndexQueryRejectionClass::UnsupportedScope),
    }
}

fn candidate_generation_for_packet<'a>(
    runtime: &'a RelationalRuntime,
    packet: &PlannedQueryPacket,
    branch_id: &BranchId,
) -> Option<&'a DerivedIndexGeneration> {
    match &packet.scope {
        QueryScope::EntityFieldEquals { field_locator, .. }
        | QueryScope::EntityFieldAnyOf { field_locator, .. } => runtime
            .indexes
            .definitions
            .values()
            .filter(|definition| {
                matches!(
                    &definition.kind,
                    DerivedIndexKind::EntityField { field_locator: indexed_field_locator }
                        if indexed_field_locator == field_locator
                )
            })
            .flat_map(|definition| {
                runtime
                    .indexes
                    .generations
                    .get(&definition.index_id)
                    .into_iter()
                    .flatten()
            })
            .max_by(|left, right| {
                generation_preference(runtime, left, packet, branch_id)
                    .cmp(&generation_preference(runtime, right, packet, branch_id))
                    .then_with(|| left.generation_id.cmp(&right.generation_id))
            }),
        QueryScope::RelationFieldEquals { field_locator, .. }
        | QueryScope::RelationFieldAnyOf { field_locator, .. } => runtime
            .indexes
            .definitions
            .values()
            .filter(|definition| {
                matches!(
                    &definition.kind,
                    DerivedIndexKind::RelationField { field_locator: indexed_field_locator }
                        if indexed_field_locator == field_locator
                )
            })
            .flat_map(|definition| {
                runtime
                    .indexes
                    .generations
                    .get(&definition.index_id)
                    .into_iter()
                    .flatten()
            })
            .max_by(|left, right| {
                generation_preference(runtime, left, packet, branch_id)
                    .cmp(&generation_preference(runtime, right, packet, branch_id))
                    .then_with(|| left.generation_id.cmp(&right.generation_id))
            }),
        _ => None,
    }
}

fn matching_index_definition_exists(
    runtime: &RelationalRuntime,
    packet: &PlannedQueryPacket,
) -> bool {
    match &packet.scope {
        QueryScope::EntityFieldEquals { field_locator, .. }
        | QueryScope::EntityFieldAnyOf { field_locator, .. } => {
            runtime.indexes.definitions.values().any(|definition| {
                matches!(
                    &definition.kind,
                    DerivedIndexKind::EntityField { field_locator: indexed_field_locator }
                        if indexed_field_locator == field_locator
                )
            })
        }
        QueryScope::RelationFieldEquals { field_locator, .. }
        | QueryScope::RelationFieldAnyOf { field_locator, .. } => {
            runtime.indexes.definitions.values().any(|definition| {
                matches!(
                    &definition.kind,
                    DerivedIndexKind::RelationField { field_locator: indexed_field_locator }
                        if indexed_field_locator == field_locator
                )
            })
        }
        _ => false,
    }
}

fn generation_preference(
    runtime: &RelationalRuntime,
    generation: &DerivedIndexGeneration,
    packet: &PlannedQueryPacket,
    branch_id: &BranchId,
) -> (bool, bool, bool, bool) {
    let branch_compatible = runtime
        .indexes
        .definitions
        .get(&generation.index_id)
        .is_none_or(|definition| {
            !definition.branch_scoped || generation.compatibility.branch_id == *branch_id
        });
    let version_compatible = generation.compatibility.version_id <= packet.context_id.version_id;
    let schema_compatible =
        generation.compatibility.schema_version == packet.context_id.schema_version;
    let published =
        generation.status == crate::indexes::data::DerivedIndexPublicationStatus::Published;
    (
        published,
        branch_compatible,
        version_compatible,
        schema_compatible,
    )
}
