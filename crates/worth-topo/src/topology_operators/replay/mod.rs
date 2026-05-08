use serde::{Deserialize, Serialize};

use super::{TopologyEditBatch, TopologyEditContract};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyOperatorDigest {
    pub algorithm: String,
    pub digest_hex: String,
    pub row_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyEditDigest {
    pub digest: TopologyOperatorDigest,
    pub contract_count: usize,
    pub family_count: usize,
    pub changed_scope_count: usize,
    pub naming_scope_count: usize,
    pub derived_region_count: usize,
}

impl TopologyEditBatch {
    pub fn topology_edit_digest(&self) -> TopologyEditDigest {
        let rows = self.contracts().iter().map(contract_digest_row);
        let changed_scope_count = self
            .contracts()
            .iter()
            .map(|contract| contract.changed_scopes().len())
            .sum();
        let naming_scope_count = self
            .contracts()
            .iter()
            .map(|contract| contract.naming_scopes().len())
            .sum();
        let derived_region_count = self
            .contracts()
            .iter()
            .map(|contract| contract.derived_regions().len())
            .sum();
        TopologyEditDigest {
            digest: digest_rows(rows),
            contract_count: self.contracts().len(),
            family_count: self.families().len(),
            changed_scope_count,
            naming_scope_count,
            derived_region_count,
        }
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
