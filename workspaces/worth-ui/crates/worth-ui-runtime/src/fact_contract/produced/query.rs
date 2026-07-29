#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiQueryIncrementalChangedFact {
    graph_effects: usize,
    measurement_effects: usize,
    allocation_effects: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiQueryResetChangedFact {
    reason: worth_ui_query_binding::WorthUiCollectionResetReason,
    fresh_execution_required: bool,
    maximum_replacement_rows: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiQueryChangedFactKind {
    Incremental(UiQueryIncrementalChangedFact),
    Reset(UiQueryResetChangedFact),
}

pub struct UiQueryChangedFact {
    kind: UiQueryChangedFactKind,
    consequence: worth_ui_query_binding::WorthUiCollectionChangeConsequence,
}

impl UiQueryChangedFact {
    pub(crate) fn from_owner_consequence(
        consequence: worth_ui_query_binding::WorthUiCollectionChangeConsequence,
    ) -> Self {
        let kind = match consequence.kind() {
            worth_ui_query_binding::WorthUiCollectionChangeKind::Incremental(incremental) => {
                UiQueryChangedFactKind::Incremental(UiQueryIncrementalChangedFact {
                    graph_effects: incremental.graph().len(),
                    measurement_effects: incremental.measurement().len(),
                    allocation_effects: incremental.allocation().len(),
                })
            }
            worth_ui_query_binding::WorthUiCollectionChangeKind::Reset(reset) => {
                UiQueryChangedFactKind::Reset(UiQueryResetChangedFact {
                    reason: reset.reason(),
                    fresh_execution_required: reset.fresh_execution_required(),
                    maximum_replacement_rows: reset.maximum_replacement_rows(),
                })
            }
        };
        Self { kind, consequence }
    }

    pub const fn kind(&self) -> UiQueryChangedFactKind {
        self.kind
    }

    pub fn change_order(&self) -> u64 {
        self.consequence.change_order()
    }
}

impl UiQueryIncrementalChangedFact {
    pub const fn graph_effects(self) -> usize {
        self.graph_effects
    }

    pub const fn measurement_effects(self) -> usize {
        self.measurement_effects
    }

    pub const fn allocation_effects(self) -> usize {
        self.allocation_effects
    }
}

impl UiQueryResetChangedFact {
    pub const fn reason(self) -> worth_ui_query_binding::WorthUiCollectionResetReason {
        self.reason
    }

    pub const fn fresh_execution_required(self) -> bool {
        self.fresh_execution_required
    }

    pub const fn maximum_replacement_rows(self) -> usize {
        self.maximum_replacement_rows
    }
}
