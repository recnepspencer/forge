use super::{
    NamingEditContinuityMatrix, TopologyEditContract, TopologyEditDigest, TopologyEditFamily,
    TopologyEditNamingOutcome, TopologyEditNamingReport, TopologyEditNamingRow,
    TopologyOperatorDigest,
};

pub(crate) fn topology_edit_naming_report_for_contracts(
    contracts: &[TopologyEditContract],
) -> TopologyEditNamingReport {
    let rows = contracts
        .iter()
        .flat_map(|contract| contract.naming_report().rows)
        .collect();
    TopologyEditNamingReport { rows }
}

pub(crate) fn topology_edit_families_for_contracts(
    contracts: &[TopologyEditContract],
) -> Vec<TopologyEditFamily> {
    contracts.iter().map(|contract| contract.family).collect()
}

pub(crate) fn naming_edit_continuity_matrix_for_contracts(
    contracts: &[TopologyEditContract],
) -> NamingEditContinuityMatrix {
    let rows = topology_edit_naming_report_for_contracts(contracts).rows;
    naming_edit_continuity_matrix_from_rows(rows)
}

pub(crate) fn topology_edit_digest_for_contracts(
    contracts: &[TopologyEditContract],
) -> TopologyEditDigest {
    let rows = contracts.iter().map(contract_digest_row);
    let changed_scope_count = contracts
        .iter()
        .map(|contract| contract.changed_scopes().len())
        .sum();
    let naming_scope_count = contracts
        .iter()
        .map(|contract| contract.naming_scopes().len())
        .sum();
    let derived_region_count = contracts
        .iter()
        .map(|contract| contract.derived_regions().len())
        .sum();
    let fallback_policy_count = contracts.len();
    let fallback_rejection_policy_count = contracts
        .iter()
        .filter(|contract| {
            contract.derived_fallback_policy()
                == super::TopologyEditDerivedFallbackPolicy::RejectAnyFallback
        })
        .count();
    TopologyEditDigest {
        digest: digest_rows(rows),
        contract_count: contracts.len(),
        family_count: contracts.len(),
        changed_scope_count,
        naming_scope_count,
        derived_region_count,
        fallback_policy_count,
        fallback_rejection_policy_count,
    }
}

pub(crate) fn naming_edit_continuity_matrix_from_rows(
    rows: Vec<TopologyEditNamingRow>,
) -> NamingEditContinuityMatrix {
    let preserved_count = rows
        .iter()
        .filter(|row| row.outcome == TopologyEditNamingOutcome::Preserved)
        .count();
    let ambiguous_count = rows
        .iter()
        .filter(|row| row.outcome == TopologyEditNamingOutcome::Ambiguous)
        .count();
    let rejected_count = rows
        .iter()
        .filter(|row| row.outcome == TopologyEditNamingOutcome::Rejected)
        .count();
    NamingEditContinuityMatrix {
        rows,
        preserved_count,
        ambiguous_count,
        rejected_count,
    }
}

fn contract_digest_row(contract: &TopologyEditContract) -> String {
    serde_json::to_string(contract).expect(" topology edit contracts should serialize")
}

fn digest_rows(rows: impl IntoIterator<Item = String>) -> TopologyOperatorDigest {
    let mut count = 0usize;
    let mut hash = 0xcbf29ce484222325u64;
    for row in rows {
        count += 1;
        for byte in row.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(b'\n');
        hash = hash.wrapping_mul(0x100000001b3);
    }
    TopologyOperatorDigest {
        algorithm: "fnv1a64".to_string(),
        digest_hex: format!("{hash:016x}"),
        row_count: count,
    }
}
