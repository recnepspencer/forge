use std::collections::BTreeSet;

use crate::validation::facade::TopologyValidationReport;
use crate::validation::rule_registry::{validate_row_against_spec, DERIVED_TOPOLOGY_RULE_SPECS};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RegisteredTopologyValidationReport {
    report: TopologyValidationReport,
}

impl RegisteredTopologyValidationReport {
    pub(crate) fn from_report(report: TopologyValidationReport) -> Result<Self, String> {
        validate_report(&report)?;
        Ok(Self { report })
    }

    pub(crate) fn report(&self) -> &TopologyValidationReport {
        &self.report
    }

    pub(crate) fn registered_rule_count(&self) -> usize {
        self.report.rows.len()
    }
}

fn validate_report(report: &TopologyValidationReport) -> Result<(), String> {
    if report.rows.len() != DERIVED_TOPOLOGY_RULE_SPECS.len() {
        return Err(format!(
            "expected {} registered validation rows, found {}",
            DERIVED_TOPOLOGY_RULE_SPECS.len(),
            report.rows.len()
        ));
    }

    let mut seen = BTreeSet::new();
    for (index, spec) in DERIVED_TOPOLOGY_RULE_SPECS.iter().enumerate() {
        let Some(row) = report.rows.get(index) else {
            return Err(format!("missing validation row for `{}`", spec.name));
        };
        if !seen.insert(row.rule_identity.stable_key()) {
            return Err(format!(
                "duplicate validation rule identity `{}`",
                row.rule_identity.stable_key()
            ));
        }
        validate_row_against_spec(row, spec)?;
    }
    Ok(())
}
