use crate::construction::graph_obligation_adoption::{
    primitive_construction_family_cardinality_closeout,
    primitive_construction_graph_obligation_residue_contract,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryPrimitiveConstructionFamilyCardinalityCloseout {
    spec_expected_family_count: usize,
    runtime_family_count: usize,
    missing_family_count: usize,
    capped_residue_class: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryPrimitiveConstructionResidueContract {
    rows: Vec<QueryPrimitiveConstructionResidueContractRow>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryPrimitiveConstructionResidueContractRow {
    class: &'static str,
    owner: &'static str,
    introduced_in: &'static str,
    current_count: usize,
    must_not_exceed_count: usize,
    blocker: &'static str,
    removal_trigger: &'static str,
    decision: &'static str,
}

pub fn query_primitive_construction_family_cardinality_closeout(
) -> QueryPrimitiveConstructionFamilyCardinalityCloseout {
    let closeout = primitive_construction_family_cardinality_closeout();
    QueryPrimitiveConstructionFamilyCardinalityCloseout {
        spec_expected_family_count: closeout.spec_expected_family_count(),
        runtime_family_count: closeout.runtime_family_count(),
        missing_family_count: closeout.missing_family_count(),
        capped_residue_class: closeout.capped_residue_class(),
    }
}

pub fn query_primitive_construction_residue_contract() -> QueryPrimitiveConstructionResidueContract
{
    QueryPrimitiveConstructionResidueContract {
        rows: primitive_construction_graph_obligation_residue_contract()
            .into_iter()
            .map(|row| QueryPrimitiveConstructionResidueContractRow {
                class: row.class(),
                owner: row.owner(),
                introduced_in: row.introduced_in(),
                current_count: row.current_count(),
                must_not_exceed_count: row.must_not_exceed_count(),
                blocker: row.blocker(),
                removal_trigger: row.removal_trigger(),
                decision: row.decision(),
            })
            .collect(),
    }
}

impl QueryPrimitiveConstructionFamilyCardinalityCloseout {
    pub const fn spec_expected_family_count(&self) -> usize {
        self.spec_expected_family_count
    }

    pub const fn runtime_family_count(&self) -> usize {
        self.runtime_family_count
    }

    pub const fn missing_family_count(&self) -> usize {
        self.missing_family_count
    }

    pub const fn capped_residue_class(&self) -> &'static str {
        self.capped_residue_class
    }
}

impl QueryPrimitiveConstructionResidueContract {
    pub fn rows(&self) -> &[QueryPrimitiveConstructionResidueContractRow] {
        &self.rows
    }
}

impl QueryPrimitiveConstructionResidueContractRow {
    pub const fn class(&self) -> &'static str {
        self.class
    }

    pub const fn owner(&self) -> &'static str {
        self.owner
    }

    pub const fn introduced_in(&self) -> &'static str {
        self.introduced_in
    }

    pub const fn current_count(&self) -> usize {
        self.current_count
    }

    pub const fn must_not_exceed_count(&self) -> usize {
        self.must_not_exceed_count
    }

    pub const fn blocker(&self) -> &'static str {
        self.blocker
    }

    pub const fn removal_trigger(&self) -> &'static str {
        self.removal_trigger
    }

    pub const fn decision(&self) -> &'static str {
        self.decision
    }
}
