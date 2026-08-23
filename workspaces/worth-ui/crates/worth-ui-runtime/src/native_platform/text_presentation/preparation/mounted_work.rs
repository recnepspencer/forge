use worth_ui_host_contract::{
    UiMountedLogicalDamage, UiMountedPaintCommand, UiMountedPaintCommandChange,
    UiMountedPaintCommandIdentity, UiMountedPresentationWorkView, UiMountedSemanticTextMechanic,
};

pub(crate) type MountedSemanticTextCommand<'work> = (
    UiMountedPaintCommandIdentity,
    &'work UiMountedSemanticTextMechanic,
);

pub(crate) struct MountedSemanticTextWork<'work> {
    pub(crate) mechanics: Vec<MountedSemanticTextCommand<'work>>,
    pub(crate) removals: Vec<UiMountedPaintCommandIdentity>,
    pub(crate) complete: bool,
}

pub(crate) fn mounted_semantic_text(
    work: UiMountedPresentationWorkView<'_>,
) -> MountedSemanticTextWork<'_> {
    match work {
        UiMountedPresentationWorkView::Initial(initial) => complete_text_work(initial.commands()),
        UiMountedPresentationWorkView::Reconstruction(reconstruction) => {
            complete_text_work(reconstruction.commands())
        }
        UiMountedPresentationWorkView::Delta(delta) => delta_text_work(delta.changes()),
        UiMountedPresentationWorkView::Unchanged(_) => MountedSemanticTextWork {
            mechanics: Vec::new(),
            removals: Vec::new(),
            complete: false,
        },
    }
}

fn delta_text_work(changes: &[UiMountedPaintCommandChange]) -> MountedSemanticTextWork<'_> {
    let mut mechanics = Vec::new();
    let mut removals = Vec::new();
    for change in changes {
        match change {
            UiMountedPaintCommandChange::Insert(command) => {
                if let Some(mechanic) = semantic_text_mechanic(command) {
                    mechanics.push((command.identity(), mechanic));
                } else {
                    removals.push(command.identity());
                }
            }
            UiMountedPaintCommandChange::Replace {
                predecessor,
                successor,
            } => {
                if *predecessor != successor.identity() {
                    removals.push(*predecessor);
                }
                if let Some(mechanic) = semantic_text_mechanic(successor) {
                    mechanics.push((successor.identity(), mechanic));
                } else {
                    removals.push(successor.identity());
                }
            }
            UiMountedPaintCommandChange::Remove(identity) => removals.push(*identity),
        }
    }
    MountedSemanticTextWork {
        mechanics,
        removals,
        complete: false,
    }
}

fn complete_text_work(commands: &[UiMountedPaintCommand]) -> MountedSemanticTextWork<'_> {
    MountedSemanticTextWork {
        mechanics: commands
            .iter()
            .filter_map(|command| {
                semantic_text_mechanic(command).map(|mechanic| (command.identity(), mechanic))
            })
            .collect(),
        removals: Vec::new(),
        complete: true,
    }
}

fn semantic_text_mechanic(
    command: &UiMountedPaintCommand,
) -> Option<&UiMountedSemanticTextMechanic> {
    match command {
        UiMountedPaintCommand::SemanticText { mechanic, .. } => Some(mechanic),
        UiMountedPaintCommand::FilledRect { .. } => None,
    }
}

pub(super) fn logical_damage(work: UiMountedPresentationWorkView<'_>) -> &[UiMountedLogicalDamage] {
    match work {
        UiMountedPresentationWorkView::Initial(initial) => initial.damage(),
        UiMountedPresentationWorkView::Delta(delta) => delta.damage(),
        UiMountedPresentationWorkView::Reconstruction(reconstruction) => reconstruction.damage(),
        UiMountedPresentationWorkView::Unchanged(_) => &[],
    }
}
