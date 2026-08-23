use worth_ui_host_contract::{UiMountedPaintCommand, UiMountedPaintCommandChange};

use super::super::super::semantic_text::complete_semantic_text_replacement;
use super::*;

pub(super) fn apply_row_update(
    context: &UiMountedSemanticTextCompletionContext<'_>,
    node: &UiMountedProjectionNodeRecord,
    seed: &UiMountedSemanticTextSeed,
    source: &super::super::super::semantic_text::UiMountedCollectionTextSource,
    row: &crate::mounting::UiMountedCollectionTextRow,
    rows: &mut UiMountedSemanticMechanicRows,
    changes: &mut UiMountedSparseSemanticChanges,
) -> Result<(), UiMountedProjectionDenial> {
    let row_key = UiMountedCollectionTextKey::for_row(row);
    let successor = source
        .row(row_key)
        .ok_or(UiMountedProjectionDenial::InvalidSemanticCollectionPatch)?;
    for (field, text) in successor.selected_values().iter().enumerate() {
        let key = UiMountedSemanticMechanicKey::collection(
            row_key,
            u16::try_from(field)
                .map_err(|_| UiMountedProjectionDenial::SemanticTextCapacityExceeded)?,
        );
        let predecessor = rows
            .rows
            .get(&key)
            .cloned()
            .ok_or(UiMountedProjectionDenial::InvalidSemanticCollectionPatch)?;
        if predecessor.text() == text.as_ref() {
            continue;
        }
        let replacement = complete_semantic_text_replacement(
            context,
            node,
            &predecessor,
            text,
            seed.formatting().default_row(),
        )?;
        let predecessor_identity =
            worth_ui_host_contract::UiMountedPaintCommandIdentity::semantic_text(&predecessor);
        rows.replace(key, replacement.clone())?;
        changes.layouts.push((predecessor, replacement.clone()));
        changes
            .commands
            .push(UiMountedPaintCommandChange::replacement(
                predecessor_identity,
                UiMountedPaintCommand::SemanticText {
                    identity: worth_ui_host_contract::UiMountedPaintCommandIdentity::semantic_text(
                        &replacement,
                    ),
                    mechanic: replacement.mechanic_clone(),
                },
            ));
    }
    Ok(())
}

pub(super) fn apply_posture_update(
    context: &UiMountedSemanticTextCompletionContext<'_>,
    node: &UiMountedProjectionNodeRecord,
    seed: &UiMountedSemanticTextSeed,
    rows: &mut UiMountedSemanticMechanicRows,
    changes: &mut UiMountedSparseSemanticChanges,
) -> Result<(), UiMountedProjectionDenial> {
    let key = UiMountedSemanticMechanicKey::posture();
    let predecessor = rows
        .rows
        .get(&key)
        .cloned()
        .ok_or(UiMountedProjectionDenial::InvalidSemanticCollectionPatch)?;
    if predecessor.text() == seed.posture().as_ref() {
        return Ok(());
    }
    let replacement = complete_semantic_text_replacement(
        context,
        node,
        &predecessor,
        seed.posture(),
        seed.formatting().default_row(),
    )?;
    let predecessor_identity =
        worth_ui_host_contract::UiMountedPaintCommandIdentity::semantic_text(&predecessor);
    rows.replace(key, replacement.clone())?;
    changes.layouts.push((predecessor, replacement.clone()));
    changes
        .commands
        .push(UiMountedPaintCommandChange::replacement(
            predecessor_identity,
            UiMountedPaintCommand::SemanticText {
                identity: worth_ui_host_contract::UiMountedPaintCommandIdentity::semantic_text(
                    &replacement,
                ),
                mechanic: replacement.mechanic_clone(),
            },
        ));
    Ok(())
}
