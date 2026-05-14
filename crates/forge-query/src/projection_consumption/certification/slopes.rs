use crate::identity::hash_parts;
use crate::projection_consumption::{
    discover_projection_consumption_support, ProjectionConsumptionSource,
};

use super::fixtures::{certification_row_set, control_row_set_lifecycle};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumptionCertificationCounterSnapshot {
    declared_fact_family_count: usize,
    admitted_fact_family_count: usize,
    extracted_fact_count: usize,
    source_row_width_consumed: usize,
    source_evidence_lookup_width: usize,
    authority_reopen_count: usize,
    digest: String,
}

impl ProjectionConsumptionCertificationCounterSnapshot {
    pub fn declared_fact_family_count(&self) -> usize {
        self.declared_fact_family_count
    }

    pub fn admitted_fact_family_count(&self) -> usize {
        self.admitted_fact_family_count
    }

    pub fn extracted_fact_count(&self) -> usize {
        self.extracted_fact_count
    }

    pub fn source_row_width_consumed(&self) -> usize {
        self.source_row_width_consumed
    }

    pub fn source_evidence_lookup_width(&self) -> usize {
        self.source_evidence_lookup_width
    }

    pub fn authority_reopen_count(&self) -> usize {
        self.authority_reopen_count
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumptionSlopeReport {
    counter_snapshot: ProjectionConsumptionCertificationCounterSnapshot,
    declaration_slope_digest: String,
    eligibility_slope_digest: String,
    contract_binding_slope_digest: String,
    fact_extraction_slope_digest: String,
    receipt_materialization_slope_digest: String,
    envelope_materialization_slope_digest: String,
    support_lookup_slope_digest: String,
}

impl ProjectionConsumptionSlopeReport {
    pub fn counter_snapshot(&self) -> &ProjectionConsumptionCertificationCounterSnapshot {
        &self.counter_snapshot
    }

    pub fn declaration_slope_digest(&self) -> &str {
        &self.declaration_slope_digest
    }

    pub fn eligibility_slope_digest(&self) -> &str {
        &self.eligibility_slope_digest
    }

    pub fn contract_binding_slope_digest(&self) -> &str {
        &self.contract_binding_slope_digest
    }

    pub fn fact_extraction_slope_digest(&self) -> &str {
        &self.fact_extraction_slope_digest
    }

    pub fn receipt_materialization_slope_digest(&self) -> &str {
        &self.receipt_materialization_slope_digest
    }

    pub fn envelope_materialization_slope_digest(&self) -> &str {
        &self.envelope_materialization_slope_digest
    }

    pub fn support_lookup_slope_digest(&self) -> &str {
        &self.support_lookup_slope_digest
    }
}

pub fn projection_consumption_slope_report() -> ProjectionConsumptionSlopeReport {
    let control = control_row_set_lifecycle(2);
    let counters = control.facts().counters();
    let counter_snapshot = ProjectionConsumptionCertificationCounterSnapshot {
        declared_fact_family_count: counters.declared_fact_family_count(),
        admitted_fact_family_count: counters.admitted_fact_family_count(),
        extracted_fact_count: counters.extracted_fact_count(),
        source_row_width_consumed: counters.source_row_width_consumed(),
        source_evidence_lookup_width: counters.source_evidence_lookup_width(),
        authority_reopen_count: counters.authority_reopen_count(),
        digest: hash_parts(&[
            "projection_consumption_counter_snapshot_v1".to_string(),
            format!("declared:{}", counters.declared_fact_family_count()),
            format!("admitted:{}", counters.admitted_fact_family_count()),
            format!("extracted:{}", counters.extracted_fact_count()),
            format!("row_width:{}", counters.source_row_width_consumed()),
            format!("evidence_width:{}", counters.source_evidence_lookup_width()),
            format!("authority_reopen:{}", counters.authority_reopen_count()),
        ]),
    };
    ProjectionConsumptionSlopeReport {
        counter_snapshot,
        declaration_slope_digest: slope_digest("declaration", |scale| {
            let lifecycle = control_row_set_lifecycle(scale);
            vec![
                format!("rows:{scale}"),
                format!(
                    "declared:{}",
                    lifecycle.facts().counters().declared_fact_family_count()
                ),
                format!(
                    "declaration:{}",
                    lifecycle.declaration().declaration_digest()
                ),
            ]
        }),
        eligibility_slope_digest: slope_digest("eligibility", |scale| {
            let lifecycle = control_row_set_lifecycle(scale);
            vec![
                format!("rows:{scale}"),
                format!(
                    "admitted:{}",
                    lifecycle.facts().counters().admitted_fact_family_count()
                ),
                format!("eligibility:{}", lifecycle.contract().eligibility_digest()),
            ]
        }),
        contract_binding_slope_digest: slope_digest("contract_binding", |scale| {
            let lifecycle = control_row_set_lifecycle(scale);
            vec![
                format!("rows:{scale}"),
                format!("contract:{}", lifecycle.contract().contract_digest()),
                format!(
                    "query:{}",
                    lifecycle.contract().query_digest().unwrap_or("none")
                ),
            ]
        }),
        fact_extraction_slope_digest: slope_digest("fact_extraction", |scale| {
            let lifecycle = control_row_set_lifecycle(scale);
            let counters = lifecycle.facts().counters();
            vec![
                format!("rows:{scale}"),
                format!("extracted:{}", counters.extracted_fact_count()),
                format!("row_width:{}", counters.source_row_width_consumed()),
            ]
        }),
        receipt_materialization_slope_digest: slope_digest("receipt_materialization", |scale| {
            let lifecycle = control_row_set_lifecycle(scale);
            vec![
                format!("rows:{scale}"),
                format!("receipt:{}", lifecycle.receipt().receipt_digest()),
                format!("extracted:{}", lifecycle.receipt().extracted_fact_count()),
            ]
        }),
        envelope_materialization_slope_digest: slope_digest("envelope_materialization", |scale| {
            let lifecycle = control_row_set_lifecycle(scale);
            vec![
                format!("rows:{scale}"),
                format!("envelope:{}", lifecycle.envelope().envelope_digest()),
                format!(
                    "authority_reopen:{}",
                    lifecycle.envelope().authority_reopen_count()
                ),
            ]
        }),
        support_lookup_slope_digest: slope_digest("support_lookup", |scale| {
            let row_set = certification_row_set(scale);
            let support = discover_projection_consumption_support(
                &ProjectionConsumptionSource::from_relational_row_set(&row_set),
            );
            vec![
                format!("rows:{scale}"),
                format!("support_width:{}", support.rows().len()),
                format!(
                    "support_digest:{}",
                    hash_parts(
                        &support
                            .rows()
                            .iter()
                            .map(|row| row.support_digest().to_string())
                            .collect::<Vec<_>>()
                    )
                ),
            ]
        }),
    }
}

fn slope_digest(label: &'static str, parts_for_scale: impl Fn(usize) -> Vec<String>) -> String {
    hash_parts(
        &(1..=3)
            .flat_map(|scale| {
                let mut parts = vec![format!("label:{label}")];
                parts.extend(parts_for_scale(scale));
                parts
            })
            .collect::<Vec<_>>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slope_report_emits_counter_snapshot_and_all_phase_slope_digests() {
        let report = projection_consumption_slope_report();
        assert_eq!(report.counter_snapshot().authority_reopen_count(), 0);
        assert!(!report.counter_snapshot().digest().is_empty());
        assert!(!report.declaration_slope_digest().is_empty());
        assert!(!report.eligibility_slope_digest().is_empty());
        assert!(!report.contract_binding_slope_digest().is_empty());
        assert!(!report.fact_extraction_slope_digest().is_empty());
        assert!(!report.receipt_materialization_slope_digest().is_empty());
        assert!(!report.envelope_materialization_slope_digest().is_empty());
        assert!(!report.support_lookup_slope_digest().is_empty());
    }
}
