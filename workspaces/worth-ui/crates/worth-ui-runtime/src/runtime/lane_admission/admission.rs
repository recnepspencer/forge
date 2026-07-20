use crate::runtime::{
    WorthUiExecutionLane, WorthUiLaneAdmissionCounters, WorthUiLaneSupportRow,
    WorthUiQueryLaneSupportLinks,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLaneAdmission {
    rows: Vec<WorthUiLaneSupportRow>,
    query_support_links: Vec<WorthUiQueryLaneSupportLinks>,
    plan_input_basis_digest: u64,
    support_digest: u64,
    counters: WorthUiLaneAdmissionCounters,
}

impl WorthUiLaneAdmission {
    pub(crate) fn new(
        mut rows: Vec<WorthUiLaneSupportRow>,
        query_support_links: Vec<WorthUiQueryLaneSupportLinks>,
        plan_input_basis_digest: u64,
        counters: WorthUiLaneAdmissionCounters,
    ) -> Self {
        rows.sort_by_key(WorthUiLaneSupportRow::lane);
        let support_digest = digest_rows(&rows);
        Self {
            rows,
            query_support_links,
            plan_input_basis_digest,
            support_digest,
            counters,
        }
    }

    pub fn rows(&self) -> &[WorthUiLaneSupportRow] {
        &self.rows
    }

    pub fn query_support_links(&self) -> &[WorthUiQueryLaneSupportLinks] {
        &self.query_support_links
    }

    pub fn support_digest(&self) -> u64 {
        self.support_digest
    }

    pub(crate) fn plan_input_basis_digest(&self) -> u64 {
        self.plan_input_basis_digest
    }

    pub fn counters(&self) -> WorthUiLaneAdmissionCounters {
        self.counters
    }

    pub fn posture_for(&self, lane: WorthUiExecutionLane) -> Option<&WorthUiLaneSupportRow> {
        self.rows.iter().find(|row| row.lane() == lane)
    }

    pub(crate) fn includes_lane(&self, lane: WorthUiExecutionLane) -> bool {
        self.posture_for(lane).is_some()
    }

    pub(crate) fn executable_contract_matches(&self, other: &Self) -> bool {
        self.rows == other.rows && self.query_support_links == other.query_support_links
    }
}

fn digest_rows(rows: &[WorthUiLaneSupportRow]) -> u64 {
    rows.iter().fold(0xcbf29ce484222325u64, |digest, row| {
        fold(
            fold(digest, row.lane().canonical_tag()),
            row.support_contract_digest(),
        )
    })
}

fn fold(mut digest: u64, value: u64) -> u64 {
    digest ^= value;
    digest.wrapping_mul(0x100000001b3)
}
