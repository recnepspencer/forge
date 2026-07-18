use std::collections::BTreeSet;

use crate::evidence::sha256_serialized;

use super::CiCertificationAggregate;

impl CiCertificationAggregate {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema_version != 2 {
            return Err(format!(
                "unsupported CI aggregate schema {}",
                self.schema_version
            ));
        }
        if self.source_identity.len() != 64
            || !self
                .source_identity
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit())
            || self.required_lanes.is_empty()
        {
            return Err("CI aggregate has incomplete source or lane identity".to_owned());
        }
        let history: BTreeSet<_> = self
            .evidence_history
            .iter()
            .map(|entry| &entry.lane)
            .collect();
        let promoted: BTreeSet<_> = self
            .promoted_evidence
            .iter()
            .map(|entry| &entry.lane)
            .collect();
        let required: BTreeSet<_> = self.required_lanes.iter().collect();
        if history != required
            || promoted != required
            || self.evidence_history.len() != required.len()
            || self.promoted_evidence.len() != required.len()
            || self
                .promoted_evidence
                .iter()
                .any(|entry| entry.evidence_identities.is_empty())
        {
            return Err(
                "CI aggregate does not promote every required lane exactly once".to_owned(),
            );
        }
        let mut basis = self.clone();
        basis.aggregate_identity.clear();
        if sha256_serialized(&basis)? != self.aggregate_identity {
            return Err("CI aggregate identity does not match its contents".to_owned());
        }
        Ok(())
    }
}
