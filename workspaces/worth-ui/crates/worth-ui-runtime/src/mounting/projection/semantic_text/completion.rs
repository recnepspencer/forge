use std::sync::Arc;

use worth_ui_host_contract::{
    UiMountedAllocationProjection, UiMountedCollectionRowCorrelation,
    UiMountedSemanticTextCompletionInput, UiMountedSemanticTextMechanic, UiSemanticTextProfile,
    UiSemanticTextSlot,
};

use super::super::frame_storage::{UiMountedProjectionNodeRecord, UiMountedSemanticProjection};
use super::super::UiMountedProjectionDenial;

pub(in crate::mounting::projection) fn complete_semantic_text(
    input: UiMountedSemanticTextCompletionContext<'_>,
) -> Result<Vec<UiMountedSemanticTextMechanic>, UiMountedProjectionDenial> {
    let mut rows = Vec::new();
    for node in input.semantic.nodes_in_order() {
        let Some(seed) = node.semantic_text.as_ref() else {
            continue;
        };
        push_node_rows(&input, node, seed, &mut rows)?;
    }
    Ok(rows)
}

pub(in crate::mounting::projection) struct UiMountedSemanticTextCompletionContext<'a> {
    pub frame: worth_ui_host_contract::UiMountedFrameIdentity,
    pub content_generation: worth_ui_host_contract::UiMountedContentGeneration,
    pub receipt_basis: &'a super::super::super::UiMountedNodeReceiptBasis,
    pub semantic: &'a UiMountedSemanticProjection,
    pub capability_generation: worth_ui_host_contract::WorthUiHostCapabilityObservationGeneration,
    pub capability_profile_digest: u64,
}

fn push_node_rows(
    context: &UiMountedSemanticTextCompletionContext<'_>,
    node: &UiMountedProjectionNodeRecord,
    seed: &super::UiMountedSemanticTextSeed,
    rows: &mut Vec<UiMountedSemanticTextMechanic>,
) -> Result<(), UiMountedProjectionDenial> {
    let (bounds, allocation_basis) = require_allocation(node)?;
    let surface = context
        .semantic
        .surface_for(node.receipt.semantic_surface())
        .ok_or(UiMountedProjectionDenial::MissingSurfaceBinding)?;
    let mounted_instance = node.receipt.mounted_instance();
    let node_receipt = context
        .receipt_basis
        .receipt_for(mounted_instance)
        .ok_or(UiMountedProjectionDenial::SemanticTextNodeReceiptMismatch)?;
    let value_row_count = value_row_count(seed)?;
    push_value_rows(
        context,
        seed,
        UiMountedNodeTextBasis {
            surface,
            mounted_instance,
            node_receipt,
            allocation_basis,
            bounds,
        },
        rows,
    )?;
    push_row(
        context,
        UiMountedSemanticTextRowBasis {
            surface,
            mounted_instance,
            node_receipt,
            allocation_basis,
            bounds,
            origin_x: bounds.x(),
            origin_y: row_origin(bounds, value_row_count, value_row_count + 1),
            text: Arc::clone(seed.posture()),
            slot: UiSemanticTextSlot::Posture,
            collection_row: None,
            layer_semantic_order: seed
                .layer_semantic_order()
                .checked_add(
                    u32::try_from(value_row_count)
                        .map_err(|_| UiMountedProjectionDenial::SemanticTextLayerOrderExceeded)?,
                )
                .ok_or(UiMountedProjectionDenial::SemanticTextLayerOrderExceeded)?,
            color: seed.color(),
        },
        rows,
    )
}

pub(in crate::mounting::projection) fn rebind_semantic_text(
    rows: &mut [UiMountedSemanticTextMechanic],
    replacements: &[(
        worth_ui_host_contract::UiSurfaceBindingGeneration,
        super::super::super::UiSurfaceBindingIdentityView,
    )],
) -> Result<(), UiMountedProjectionDenial> {
    for row in rows {
        let Some((_, replacement)) = replacements
            .iter()
            .find(|(affected, _)| *affected == row.binding())
        else {
            continue;
        };
        if replacement.semantic_surface_identity() != row.surface() {
            return Err(UiMountedProjectionDenial::MissingSurfaceBinding);
        }
        *row = UiMountedSemanticTextMechanic::complete_from_runtime_mounting(
            UiMountedSemanticTextCompletionInput {
                content_generation: row.content_generation(),
                frame: row.frame(),
                surface: row.surface(),
                binding: replacement.binding_generation(),
                mounted_instance: row.mounted_instance(),
                node_receipt: row.node_receipt(),
                allocation_basis: row.allocation_basis(),
                bounds: row.bounds(),
                clip_bounds: row.clip_bounds(),
                origin_x: row.origin_x(),
                origin_y: row.origin_y(),
                text: Arc::from(row.text()),
                slot: row.slot(),
                collection_row: row.collection_row().cloned(),
                color: row.color(),
                profile: row.profile(),
                layer_semantic_order: row.layer_semantic_order(),
                capability_generation: row.capability_generation(),
                capability_profile_digest: row.capability_profile_digest(),
            },
        )
        .map_err(UiMountedProjectionDenial::SemanticTextCompletion)?;
    }
    Ok(())
}

#[derive(Clone, Copy)]
struct UiMountedNodeTextBasis {
    surface: super::super::frame_storage::UiMountedProjectionSurface,
    mounted_instance: worth_ui_host_contract::UiMountedInstanceIdentity,
    node_receipt: worth_ui_host_contract::UiMountedNodeReceiptIdentity,
    allocation_basis: worth_ui_host_contract::UiMountedAllocationBasis,
    bounds: worth_ui_host_contract::UiMountedCanonicalBox,
}

struct UiMountedSemanticTextRowBasis {
    surface: super::super::frame_storage::UiMountedProjectionSurface,
    mounted_instance: worth_ui_host_contract::UiMountedInstanceIdentity,
    node_receipt: worth_ui_host_contract::UiMountedNodeReceiptIdentity,
    allocation_basis: worth_ui_host_contract::UiMountedAllocationBasis,
    bounds: worth_ui_host_contract::UiMountedCanonicalBox,
    origin_x: f32,
    origin_y: f32,
    text: Arc<str>,
    slot: UiSemanticTextSlot,
    collection_row: Option<UiMountedCollectionRowCorrelation>,
    layer_semantic_order: u32,
    color: worth_ui_host_contract::UiMountedRgba8,
}

enum UiMountedSemanticTextValueMeaning {
    Scalar(Arc<str>),
    Collection {
        text: Arc<str>,
        row: UiMountedCollectionRowCorrelation,
        selected_field_ordinal: u16,
    },
}

struct UiMountedSemanticTextValueRowInput {
    basis: UiMountedNodeTextBasis,
    meaning: UiMountedSemanticTextValueMeaning,
    index: usize,
    total: usize,
}

fn push_row(
    context: &UiMountedSemanticTextCompletionContext<'_>,
    row: UiMountedSemanticTextRowBasis,
    rows: &mut Vec<UiMountedSemanticTextMechanic>,
) -> Result<(), UiMountedProjectionDenial> {
    if rows.len() >= worth_ui_host_contract::UiMountedSemanticTextTable::MAX_ROWS {
        return Err(UiMountedProjectionDenial::SemanticTextCapacityExceeded);
    }
    let mechanic = UiMountedSemanticTextMechanic::complete_from_runtime_mounting(
        UiMountedSemanticTextCompletionInput {
            content_generation: context.content_generation,
            frame: context.frame,
            surface: row.surface.surface,
            binding: row.surface.binding,
            mounted_instance: row.mounted_instance,
            node_receipt: row.node_receipt,
            allocation_basis: row.allocation_basis,
            bounds: row.bounds,
            clip_bounds: row.bounds,
            origin_x: row.origin_x,
            origin_y: row.origin_y,
            text: row.text,
            slot: row.slot,
            collection_row: row.collection_row,
            color: row.color,
            profile: UiSemanticTextProfile::BodyDefault,
            layer_semantic_order: row.layer_semantic_order,
            capability_generation: context.capability_generation,
            capability_profile_digest: context.capability_profile_digest,
        },
    )
    .map_err(UiMountedProjectionDenial::SemanticTextCompletion)?;
    rows.push(mechanic);
    Ok(())
}

fn value_row_count(
    seed: &super::UiMountedSemanticTextSeed,
) -> Result<usize, UiMountedProjectionDenial> {
    match seed.content() {
        super::UiMountedSemanticTextSeedContent::Scalar(value) => Ok(usize::from(value.is_some())),
        super::UiMountedSemanticTextSeedContent::Collection(rows) => rows
            .iter()
            .try_fold(0usize, |count, row| {
                count.checked_add(row.selected_values().len())
            })
            .ok_or(UiMountedProjectionDenial::SemanticTextCapacityExceeded),
    }
}

fn push_value_rows(
    context: &UiMountedSemanticTextCompletionContext<'_>,
    seed: &super::UiMountedSemanticTextSeed,
    basis: UiMountedNodeTextBasis,
    rows: &mut Vec<UiMountedSemanticTextMechanic>,
) -> Result<(), UiMountedProjectionDenial> {
    match seed.content() {
        super::UiMountedSemanticTextSeedContent::Scalar(Some(value)) => push_value_row(
            context,
            seed,
            UiMountedSemanticTextValueRowInput {
                basis,
                meaning: UiMountedSemanticTextValueMeaning::Scalar(Arc::clone(value)),
                index: 0,
                total: 2,
            },
            rows,
        ),
        super::UiMountedSemanticTextSeedContent::Scalar(None) => Ok(()),
        super::UiMountedSemanticTextSeedContent::Collection(collection) => {
            let total = value_row_count(seed)? + 1;
            let mut index = 0usize;
            for row in collection {
                for (field_ordinal, value) in row.selected_values().iter().enumerate() {
                    push_value_row(
                        context,
                        seed,
                        UiMountedSemanticTextValueRowInput {
                            basis,
                            meaning: UiMountedSemanticTextValueMeaning::Collection {
                                text: Arc::clone(value),
                                row: UiMountedCollectionRowCorrelation::from_runtime_mounting(
                                    row.identity()
                                        .query_reference()
                                        .query_identity()
                                        .operational_key()
                                        .correlation_digest(),
                                ),
                                selected_field_ordinal: u16::try_from(field_ordinal).map_err(
                                    |_| UiMountedProjectionDenial::SemanticTextCapacityExceeded,
                                )?,
                            },
                            index,
                            total,
                        },
                        rows,
                    )?;
                    index += 1;
                }
            }
            Ok(())
        }
    }
}

fn push_value_row(
    context: &UiMountedSemanticTextCompletionContext<'_>,
    seed: &super::UiMountedSemanticTextSeed,
    input: UiMountedSemanticTextValueRowInput,
    rows: &mut Vec<UiMountedSemanticTextMechanic>,
) -> Result<(), UiMountedProjectionDenial> {
    let (text, slot, collection_row) = match input.meaning {
        UiMountedSemanticTextValueMeaning::Scalar(text) => (text, UiSemanticTextSlot::Value, None),
        UiMountedSemanticTextValueMeaning::Collection {
            text,
            row,
            selected_field_ordinal,
        } => (
            text,
            UiSemanticTextSlot::CollectionValue {
                selected_field_ordinal,
            },
            Some(row),
        ),
    };
    push_row(
        context,
        UiMountedSemanticTextRowBasis {
            surface: input.basis.surface,
            mounted_instance: input.basis.mounted_instance,
            node_receipt: input.basis.node_receipt,
            allocation_basis: input.basis.allocation_basis,
            bounds: input.basis.bounds,
            origin_x: input.basis.bounds.x(),
            origin_y: row_origin(input.basis.bounds, input.index, input.total),
            text,
            slot,
            collection_row,
            layer_semantic_order: seed
                .layer_semantic_order()
                .checked_add(
                    u32::try_from(input.index)
                        .map_err(|_| UiMountedProjectionDenial::SemanticTextLayerOrderExceeded)?,
                )
                .ok_or(UiMountedProjectionDenial::SemanticTextLayerOrderExceeded)?,
            color: seed.color(),
        },
        rows,
    )
}

fn row_origin(
    bounds: worth_ui_host_contract::UiMountedCanonicalBox,
    index: usize,
    total: usize,
) -> f32 {
    bounds.y() + bounds.height() * (index as f32 / total as f32)
}

fn require_allocation(
    node: &UiMountedProjectionNodeRecord,
) -> Result<
    (
        worth_ui_host_contract::UiMountedCanonicalBox,
        worth_ui_host_contract::UiMountedAllocationBasis,
    ),
    UiMountedProjectionDenial,
> {
    match node.receipt.allocation() {
        UiMountedAllocationProjection::Known { bounds, basis } => Ok((bounds, basis)),
        UiMountedAllocationProjection::PortalAnchorObservation { .. } => Err(
            UiMountedProjectionDenial::UnsupportedSemanticTextAllocation(node.receipt.graph_node()),
        ),
        UiMountedAllocationProjection::Omitted(_) => Err(
            UiMountedProjectionDenial::MissingSemanticTextAllocation(node.receipt.graph_node()),
        ),
    }
}
