use std::collections::BTreeSet;

use forge_query::facade::{
    ForgeQueryGraphObligationKind, ForgeQueryGraphObligationSupportLane,
    ForgeQueryGraphObligationSupportStatus,
};

use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalog, WorthTopologyLegalityFamilyRecord,
    WorthTopologySelectedLegalityObligationPlan,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologySelectedRelationalInvariantFamilyRow {
    worth_family_identity_digest: String,
    query_rule_identity_digest: String,
    support_lane: ForgeQueryGraphObligationSupportLane,
    support_status: ForgeQueryGraphObligationSupportStatus,
    registration_digest: String,
    selected_obligation_row_digest: String,
    row_digest: String,
}

impl WorthTopologySelectedRelationalInvariantFamilyRow {
    fn from_selected_obligation(
        selected: &crate::validator_invariant_catalog::WorthTopologySelectedLegalityObligationRow,
    ) -> Self {
        let row_digest = [
            "worth-topo-selected-relational-invariant-family-row-v1",
            selected.worth_family_identity_digest(),
            selected.query_rule_identity_digest(),
            selected.support_lane().as_str(),
            selected.support_status().as_str(),
            selected.registration_digest(),
            selected.row_digest(),
        ]
        .join("|");
        Self {
            worth_family_identity_digest: selected.worth_family_identity_digest().to_string(),
            query_rule_identity_digest: selected.query_rule_identity_digest().to_string(),
            support_lane: selected.support_lane(),
            support_status: selected.support_status(),
            registration_digest: selected.registration_digest().to_string(),
            selected_obligation_row_digest: selected.row_digest().to_string(),
            row_digest,
        }
    }

    pub fn worth_family_identity_digest(&self) -> &str {
        &self.worth_family_identity_digest
    }

    pub fn query_rule_identity_digest(&self) -> &str {
        &self.query_rule_identity_digest
    }

    pub const fn support_lane(&self) -> ForgeQueryGraphObligationSupportLane {
        self.support_lane
    }

    pub const fn support_status(&self) -> ForgeQueryGraphObligationSupportStatus {
        self.support_status
    }

    pub fn registration_digest(&self) -> &str {
        &self.registration_digest
    }

    pub fn selected_obligation_row_digest(&self) -> &str {
        &self.selected_obligation_row_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

pub(in crate::validator_invariant_catalog) fn select_relational_invariant_family_rows(
    catalog: &WorthTopologyLegalityCatalog,
    selected_plan: &WorthTopologySelectedLegalityObligationPlan,
) -> Vec<WorthTopologySelectedRelationalInvariantFamilyRow> {
    let invariant_identity_digests = catalog
        .records()
        .iter()
        .filter_map(invariant_identity_digest)
        .collect::<BTreeSet<_>>();

    selected_plan
        .selected_obligation_rows()
        .iter()
        .filter(|row| {
            row.query_obligation_kind() == ForgeQueryGraphObligationKind::BlockingInvariant
        })
        .filter(|row| invariant_identity_digests.contains(row.worth_family_identity_digest()))
        .map(WorthTopologySelectedRelationalInvariantFamilyRow::from_selected_obligation)
        .collect()
}

fn invariant_identity_digest(record: &WorthTopologyLegalityFamilyRecord) -> Option<String> {
    match record {
        WorthTopologyLegalityFamilyRecord::Invariant(_) => {
            Some(record.identity().identity_digest().to_string())
        }
        WorthTopologyLegalityFamilyRecord::Validator(_) => None,
    }
}
