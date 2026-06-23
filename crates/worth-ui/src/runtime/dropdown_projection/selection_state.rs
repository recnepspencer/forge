use crate::capability::CommandProjectionSelectionMode;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiDropdownSelectionState {
    None,
    Single(String),
    Multi(Vec<String>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiDropdownSelectionStateReconciliationReceipt {
    status: WorthUiDropdownSelectionStateStatus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiDropdownSelectionStateStatus {
    Empty,
    PreservedSingle,
    PreservedMulti,
    PromotedSingleToMulti,
    NarrowedMultiToSingle {
        survivor_command_id: String,
    },
    DroppedSelection {
        reason: WorthUiDropdownStateDropReason,
    },
    DeniedModeTransition(WorthUiDropdownModeTransitionDenial),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiDropdownStateDropReason {
    SelectedCommandUnavailable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiDropdownModeTransitionDenial {
    AmbiguousSingleSelectNarrowing { surviving_command_ids: Vec<String> },
}

impl WorthUiDropdownSelectionState {
    pub(crate) fn empty_for_mode(_mode: CommandProjectionSelectionMode) -> Self {
        Self::None
    }

    pub(crate) fn reconcile(
        previous: &Self,
        next_mode: CommandProjectionSelectionMode,
        valid_command_ids: &[String],
    ) -> (
        WorthUiDropdownSelectionState,
        WorthUiDropdownSelectionStateReconciliationReceipt,
    ) {
        let mut surviving = previous
            .selected_command_ids()
            .into_iter()
            .filter(|command_id| valid_command_ids.contains(command_id))
            .collect::<Vec<_>>();

        if surviving.is_empty() {
            let status = if previous.selected_command_ids().is_empty() {
                WorthUiDropdownSelectionStateStatus::Empty
            } else {
                WorthUiDropdownSelectionStateStatus::DroppedSelection {
                    reason: WorthUiDropdownStateDropReason::SelectedCommandUnavailable,
                }
            };
            return (
                Self::None,
                WorthUiDropdownSelectionStateReconciliationReceipt { status },
            );
        }

        match next_mode {
            CommandProjectionSelectionMode::SingleSelect => {
                if surviving.len() == 1 {
                    let survivor = surviving.remove(0);
                    let status = match previous {
                        Self::Single(_) => WorthUiDropdownSelectionStateStatus::PreservedSingle,
                        Self::Multi(_) => {
                            WorthUiDropdownSelectionStateStatus::NarrowedMultiToSingle {
                                survivor_command_id: survivor.clone(),
                            }
                        }
                        Self::None => WorthUiDropdownSelectionStateStatus::PreservedSingle,
                    };
                    (
                        Self::Single(survivor),
                        WorthUiDropdownSelectionStateReconciliationReceipt { status },
                    )
                } else {
                    let denial =
                        WorthUiDropdownModeTransitionDenial::AmbiguousSingleSelectNarrowing {
                            surviving_command_ids: surviving,
                        };
                    (
                        Self::None,
                        WorthUiDropdownSelectionStateReconciliationReceipt {
                            status: WorthUiDropdownSelectionStateStatus::DeniedModeTransition(
                                denial,
                            ),
                        },
                    )
                }
            }
            CommandProjectionSelectionMode::MultiSelect => {
                let status = match previous {
                    Self::Single(_) => WorthUiDropdownSelectionStateStatus::PromotedSingleToMulti,
                    Self::Multi(_) => WorthUiDropdownSelectionStateStatus::PreservedMulti,
                    Self::None => WorthUiDropdownSelectionStateStatus::Empty,
                };
                (
                    Self::Multi(surviving),
                    WorthUiDropdownSelectionStateReconciliationReceipt { status },
                )
            }
        }
    }

    pub fn selected_command_ids(&self) -> Vec<String> {
        match self {
            Self::None => Vec::new(),
            Self::Single(command_id) => vec![command_id.clone()],
            Self::Multi(command_ids) => command_ids.clone(),
        }
    }

    pub fn contains(&self, command_id: &str) -> bool {
        match self {
            Self::None => false,
            Self::Single(selected) => selected == command_id,
            Self::Multi(selected) => selected.iter().any(|candidate| candidate == command_id),
        }
    }
}

impl WorthUiDropdownSelectionStateReconciliationReceipt {
    pub(crate) fn new(status: WorthUiDropdownSelectionStateStatus) -> Self {
        Self { status }
    }

    pub fn status(&self) -> &WorthUiDropdownSelectionStateStatus {
        &self.status
    }
}
