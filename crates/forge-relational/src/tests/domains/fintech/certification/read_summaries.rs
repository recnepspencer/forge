use std::collections::BTreeMap;

use serde_json::{Map, Number, Value};

use super::super::fixture::FintechCaseRole;
use super::super::probes::{capture_case_truth_probe, CaseTruthProbe, ProbeStage};
use super::session::CertifiedRelationalFintechSession;
use crate::facade::snapshots::SnapshotHandle;
use crate::tests::support::read_entity_field;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct CertifiedFintechReadSummary {
    snapshot_id: u64,
    entity_count: usize,
    relation_count: usize,
    corrected_trade_count: usize,
    repaired_settlement_count: usize,
    open_breach_count: usize,
    audit_record_count: usize,
    case_role: Option<FintechCaseRole>,
}

impl CertifiedFintechReadSummary {
    pub(super) fn snapshot_read(
        snapshot_id: u64,
        entity_count: usize,
        relation_count: usize,
        corrected_trade_count: usize,
        repaired_settlement_count: usize,
        open_breach_count: usize,
    ) -> Self {
        Self {
            snapshot_id,
            entity_count,
            relation_count,
            corrected_trade_count,
            repaired_settlement_count,
            open_breach_count,
            audit_record_count: 0,
            case_role: None,
        }
    }

    pub(super) fn case_probe(probe: &CaseTruthProbe) -> Self {
        Self {
            snapshot_id: probe.snapshot_id,
            entity_count: probe.entity_count,
            relation_count: probe.relation_count,
            corrected_trade_count: probe.corrected_trade_count,
            repaired_settlement_count: probe.repaired_settlement_count,
            open_breach_count: probe.open_breach_count,
            audit_record_count: probe.audit_record_count,
            case_role: Some(probe.case_role),
        }
    }

    pub(super) fn entity_count(&self) -> usize {
        self.entity_count
    }

    pub(super) fn corrected_trade_count(&self) -> usize {
        self.corrected_trade_count
    }

    pub(super) fn repaired_settlement_count(&self) -> usize {
        self.repaired_settlement_count
    }

    pub(super) fn open_breach_count(&self) -> usize {
        self.open_breach_count
    }

    pub(super) fn audit_record_count(&self) -> usize {
        self.audit_record_count
    }

    pub(super) fn matches_case_role(&self, expected: &str) -> bool {
        self.case_role
            .map(|case_role| format!("{case_role:?}") == expected)
            .unwrap_or(false)
    }

    pub(super) fn to_harness_json_artifact(&self) -> Value {
        let mut fields = Map::new();
        fields.insert("snapshot_id".to_string(), u64_value(self.snapshot_id));
        fields.insert("entity_count".to_string(), usize_value(self.entity_count));
        fields.insert(
            "relation_count".to_string(),
            usize_value(self.relation_count),
        );
        fields.insert(
            "corrected_trade_count".to_string(),
            usize_value(self.corrected_trade_count),
        );
        fields.insert(
            "repaired_settlement_count".to_string(),
            usize_value(self.repaired_settlement_count),
        );
        fields.insert(
            "open_breach_count".to_string(),
            usize_value(self.open_breach_count),
        );
        fields.insert(
            "audit_record_count".to_string(),
            usize_value(self.audit_record_count),
        );
        if let Some(case_role) = self.case_role {
            fields.insert(
                "case_role".to_string(),
                Value::String(format!("{case_role:?}")),
            );
        }
        Value::Object(fields)
    }
}

pub(super) fn read_summary_json_artifacts(
    summaries: &BTreeMap<String, CertifiedFintechReadSummary>,
) -> Value {
    Value::Object(
        summaries
            .iter()
            .map(|(alias, summary)| (alias.clone(), summary.to_harness_json_artifact()))
            .collect(),
    )
}

pub(super) fn read_summary(
    session: &CertifiedRelationalFintechSession,
    snapshot: SnapshotHandle,
) -> Result<CertifiedFintechReadSummary, String> {
    let read = session
        .world
        .runtime
        .read_truth()
        .read_snapshot(&snapshot)
        .ok_or_else(|| format!("snapshot `{}` is unavailable", snapshot.snapshot_id.0))?;
    let corrected_trades = read
        .entities()
        .iter()
        .filter(|entity| read_entity_field(entity, "corrected") == Some("true".into()))
        .count();
    let repaired_settlements = read
        .entities()
        .iter()
        .filter(|entity| {
            read_entity_field(entity, "entity_type") == Some("settlement".into())
                && read_entity_field(entity, "status") == Some("repaired".into())
        })
        .count();
    let open_breaches = read
        .entities()
        .iter()
        .filter(|entity| {
            read_entity_field(entity, "entity_type") == Some("limit_breach".into())
                && read_entity_field(entity, "status") == Some("open".into())
        })
        .count();
    Ok(CertifiedFintechReadSummary::snapshot_read(
        snapshot.snapshot_id.0,
        read.entities().len(),
        read.relations().len(),
        corrected_trades,
        repaired_settlements,
        open_breaches,
    ))
}

pub(super) fn case_read_summary(
    session: &CertifiedRelationalFintechSession,
    case_role: FintechCaseRole,
) -> CertifiedFintechReadSummary {
    let probe = capture_case_truth_probe(&session.world, case_role, ProbeStage::PostMutation);
    CertifiedFintechReadSummary::case_probe(&probe)
}

fn usize_value(value: usize) -> Value {
    u64_value(value as u64)
}

fn u64_value(value: u64) -> Value {
    Value::Number(Number::from(value))
}
