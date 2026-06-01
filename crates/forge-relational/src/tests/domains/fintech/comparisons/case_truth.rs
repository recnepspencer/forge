use super::super::probes::CaseTruthProbe;

pub(crate) fn compare_case_truth(left: &CaseTruthProbe, right: &CaseTruthProbe) -> Vec<String> {
    let mut mismatches = Vec::new();
    if left.case_role != right.case_role {
        mismatches.push("case_role".to_string());
    }
    if left.entity_count != right.entity_count {
        mismatches.push("entity_count".to_string());
    }
    if left.relation_count != right.relation_count {
        mismatches.push("relation_count".to_string());
    }
    if left.corrected_trade_count != right.corrected_trade_count {
        mismatches.push("corrected_trade_count".to_string());
    }
    if left.repaired_settlement_count != right.repaired_settlement_count {
        mismatches.push("repaired_settlement_count".to_string());
    }
    if left.open_breach_count != right.open_breach_count {
        mismatches.push("open_breach_count".to_string());
    }
    if left.audit_record_count != right.audit_record_count {
        mismatches.push("audit_record_count".to_string());
    }
    if left.aspect_state_fingerprints != right.aspect_state_fingerprints {
        mismatches.push("aspect_state_fingerprints".to_string());
    }
    mismatches
}
