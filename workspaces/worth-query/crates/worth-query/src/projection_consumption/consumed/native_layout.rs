use crate::projection_consumption::{MaterializedProjectionContract, ProjectionFactKind};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct ConsumedNativeLayoutProof {
    row_count: usize,
    display_selections: Vec<u64>,
    derived_selections: Vec<u64>,
}

impl ConsumedNativeLayoutProof {
    pub(crate) fn from_contract(
        contract: &MaterializedProjectionContract,
        row_count: usize,
    ) -> Self {
        let mut display_selections = Vec::new();
        let mut derived_selections = Vec::new();
        for family in contract.fact_families() {
            let Some(native) = family.native_contract() else {
                continue;
            };
            match family.kind() {
                ProjectionFactKind::DisplayField => {
                    display_selections.push(native.selection_identity())
                }
                ProjectionFactKind::DerivedField => {
                    derived_selections.push(native.selection_identity())
                }
                _ => {}
            }
        }
        Self {
            row_count,
            display_selections,
            derived_selections,
        }
    }

    pub(crate) fn row_count(&self) -> usize {
        self.row_count
    }

    pub(crate) fn display_selections(&self) -> &[u64] {
        &self.display_selections
    }

    pub(crate) fn derived_selections(&self) -> &[u64] {
        &self.derived_selections
    }
}
