use super::{
    UiHeadlessFilledRectMechanic, UiHeadlessMountedFrameTranscript, UiHeadlessSemanticTextMechanic,
    UiHeadlessTranscriptSuccessorIdentity,
};
use worth_ui_host_contract::{UiMountedLogicalDamage, UiMountedPaintOrderIdentity};

impl UiHeadlessMountedFrameTranscript {
    pub(crate) fn successor_recorded_delta(
        &self,
        identity: UiHeadlessTranscriptSuccessorIdentity,
        changes: &[worth_ui_host_contract::UiMountedPaintCommandChange],
        order_edits: &[worth_ui_host_contract::UiMountedPaintOrderEdit],
        order_integrity: worth_ui_host_contract::UiMountedPaintOrderIntegrity,
        damage: &[UiMountedLogicalDamage],
    ) -> Result<Self, worth_ui_host_contract::UiHostSurfacePresentationDenial> {
        let mut order = self.paint_order.to_vec();
        apply_recorded_order_edits(&mut order, order_edits)?;
        if !order_integrity.admits(&order) {
            return Err(malformed());
        }
        let mut successor = self.successor_recorded_identity(identity);
        apply_mechanic_changes(&mut successor, changes)?;
        successor.paint_order = order.into_boxed_slice();
        successor.logical_damage = damage.into();
        Ok(successor)
    }

    fn successor_recorded_identity(&self, identity: UiHeadlessTranscriptSuccessorIdentity) -> Self {
        let mut successor = self.clone();
        successor.host_session_identity = identity.host_session_identity;
        successor.protocol = identity.protocol;
        successor.attempt = identity.attempt;
        successor.frame = identity.frame;
        successor.binding = identity.binding;
        successor
    }
}

fn apply_mechanic_changes(
    successor: &mut UiHeadlessMountedFrameTranscript,
    changes: &[worth_ui_host_contract::UiMountedPaintCommandChange],
) -> Result<(), worth_ui_host_contract::UiHostSurfacePresentationDenial> {
    let mut filled_rects = std::mem::take(&mut successor.filled_rects).into_vec();
    let mut semantic_text = std::mem::take(&mut successor.semantic_text).into_vec();
    remove_changed_commands(&mut filled_rects, &mut semantic_text, changes)?;
    insert_changed_commands(&mut filled_rects, &mut semantic_text, changes)?;
    successor.filled_rects = filled_rects.into_boxed_slice();
    successor.semantic_text = semantic_text.into_boxed_slice();
    Ok(())
}

fn apply_recorded_order_edits(
    order: &mut Vec<UiMountedPaintOrderIdentity>,
    edits: &[worth_ui_host_contract::UiMountedPaintOrderEdit],
) -> Result<(), worth_ui_host_contract::UiHostSurfacePresentationDenial> {
    for edit in edits {
        let identity = edit.identity();
        if let Some(index) = order.iter().position(|current| *current == identity) {
            order.remove(index);
        } else if edit.is_removal() {
            return Err(malformed());
        }
        if edit.is_removal() {
            continue;
        }
        let index = match edit.predecessor() {
            None => 0,
            Some(predecessor) => order
                .iter()
                .position(|current| *current == predecessor)
                .and_then(|index| index.checked_add(1))
                .ok_or_else(malformed)?,
        };
        order.insert(index, identity);
    }
    Ok(())
}

fn remove_changed_commands(
    filled_rects: &mut Vec<UiHeadlessFilledRectMechanic>,
    semantic_text: &mut Vec<UiHeadlessSemanticTextMechanic>,
    changes: &[worth_ui_host_contract::UiMountedPaintCommandChange],
) -> Result<(), worth_ui_host_contract::UiHostSurfacePresentationDenial> {
    for change in changes {
        let identity = match change {
            worth_ui_host_contract::UiMountedPaintCommandChange::Insert(_) => continue,
            worth_ui_host_contract::UiMountedPaintCommandChange::Replace(command) => {
                command.identity()
            }
            worth_ui_host_contract::UiMountedPaintCommandChange::Remove(identity) => *identity,
        };
        if let Some(index) = filled_rects
            .iter()
            .position(|mechanic| mechanic.command_identity() == identity)
        {
            filled_rects.remove(index);
        } else if let Some(index) = semantic_text
            .iter()
            .position(|mechanic| mechanic.command_identity() == identity)
        {
            semantic_text.remove(index);
        } else {
            return Err(malformed());
        }
    }
    Ok(())
}

fn insert_changed_commands(
    filled_rects: &mut Vec<UiHeadlessFilledRectMechanic>,
    semantic_text: &mut Vec<UiHeadlessSemanticTextMechanic>,
    changes: &[worth_ui_host_contract::UiMountedPaintCommandChange],
) -> Result<(), worth_ui_host_contract::UiHostSurfacePresentationDenial> {
    for command in changes.iter().filter_map(changed_command) {
        match command {
            worth_ui_host_contract::UiMountedPaintCommand::FilledRect {
                mechanic,
                ..
            } => filled_rects.push(
                crate::headless_translation::static_paint::translate_command(*mechanic),
            ),
            worth_ui_host_contract::UiMountedPaintCommand::SemanticText {
                mechanic,
                ..
            } => semantic_text.push(
                crate::headless_translation::semantic_text::translate_command(mechanic),
            ),
        }
    }
    Ok(())
}

fn changed_command(
    change: &worth_ui_host_contract::UiMountedPaintCommandChange,
) -> Option<&worth_ui_host_contract::UiMountedPaintCommand> {
    match change {
        worth_ui_host_contract::UiMountedPaintCommandChange::Insert(command)
        | worth_ui_host_contract::UiMountedPaintCommandChange::Replace(command) => Some(command),
        worth_ui_host_contract::UiMountedPaintCommandChange::Remove(_) => None,
    }
}

fn malformed() -> worth_ui_host_contract::UiHostSurfacePresentationDenial {
    worth_ui_host_contract::UiHostSurfacePresentationDenial::MalformedProjection
}
