use worth_ui_host_contract::{UiMountedPaintCommand, UiMountedPaintCommandChange};

use super::UiMountedSemanticMechanicRows;

pub(super) fn diff_rows(
    predecessor: &UiMountedSemanticMechanicRows,
    successor: &UiMountedSemanticMechanicRows,
) -> Vec<UiMountedPaintCommandChange> {
    let mut changes = Vec::new();
    for (key, before) in predecessor.rows.iter() {
        match successor.rows.get(key) {
            None => changes.push(UiMountedPaintCommandChange::Remove(
                worth_ui_host_contract::UiMountedPaintCommandIdentity::semantic_text(before),
            )),
            Some(after) if before.semantic_digest() != after.semantic_digest() => {
                changes.push(UiMountedPaintCommandChange::Replace(
                    UiMountedPaintCommand::SemanticText {
                        identity:
                            worth_ui_host_contract::UiMountedPaintCommandIdentity::semantic_text(
                                after,
                            ),
                        mechanic: after.mechanic_clone(),
                    },
                ));
            }
            Some(_) => {}
        }
    }
    for (key, after) in successor.rows.iter() {
        if predecessor.rows.get(key).is_none() {
            changes.push(UiMountedPaintCommandChange::Insert(
                UiMountedPaintCommand::SemanticText {
                    identity: worth_ui_host_contract::UiMountedPaintCommandIdentity::semantic_text(
                        after,
                    ),
                    mechanic: after.mechanic_clone(),
                },
            ));
        }
    }
    changes
}
