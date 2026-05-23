use crate::identity::hash_parts;

use super::representative::ForgeQueryDomainCapabilityRepresentativeReport;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDomainCapabilityCertificationCounterSnapshot {
    contribution_width: usize,
    trace_width: usize,
    category_width: usize,
    support_width: usize,
    digest: String,
}

impl ForgeQueryDomainCapabilityCertificationCounterSnapshot {
    pub fn contribution_width(&self) -> usize {
        self.contribution_width
    }

    pub fn trace_width(&self) -> usize {
        self.trace_width
    }

    pub fn category_width(&self) -> usize {
        self.category_width
    }

    pub fn support_width(&self) -> usize {
        self.support_width
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDomainCapabilitySlopeReport {
    counter_snapshot: ForgeQueryDomainCapabilityCertificationCounterSnapshot,
    contribution_materialization_slope_digest: String,
    trace_materialization_slope_digest: String,
    category_materialization_slope_digest: String,
    support_materialization_slope_digest: String,
}

impl ForgeQueryDomainCapabilitySlopeReport {
    pub fn counter_snapshot(&self) -> &ForgeQueryDomainCapabilityCertificationCounterSnapshot {
        &self.counter_snapshot
    }

    pub fn contribution_materialization_slope_digest(&self) -> &str {
        &self.contribution_materialization_slope_digest
    }

    pub fn trace_materialization_slope_digest(&self) -> &str {
        &self.trace_materialization_slope_digest
    }

    pub fn category_materialization_slope_digest(&self) -> &str {
        &self.category_materialization_slope_digest
    }

    pub fn support_materialization_slope_digest(&self) -> &str {
        &self.support_materialization_slope_digest
    }
}

pub fn forge_query_domain_capability_slope_report(
    representative: &ForgeQueryDomainCapabilityRepresentativeReport,
) -> ForgeQueryDomainCapabilitySlopeReport {
    let counter_snapshot = ForgeQueryDomainCapabilityCertificationCounterSnapshot {
        contribution_width: representative.contribution_width(),
        trace_width: representative.trace_width(),
        category_width: representative.category_width(),
        support_width: representative.support_width(),
        digest: hash_parts(&[
            format!("contribution-width:{}", representative.contribution_width()),
            format!("trace-width:{}", representative.trace_width()),
            format!("category-width:{}", representative.category_width()),
            format!("support-width:{}", representative.support_width()),
        ]),
    };

    ForgeQueryDomainCapabilitySlopeReport {
        contribution_materialization_slope_digest: slope_digest(
            "contribution-materialization",
            representative.contribution_width(),
        ),
        trace_materialization_slope_digest: slope_digest(
            "trace-materialization",
            representative.trace_width(),
        ),
        category_materialization_slope_digest: slope_digest(
            "category-materialization",
            representative.category_width(),
        ),
        support_materialization_slope_digest: slope_digest(
            "support-materialization",
            representative.support_width(),
        ),
        counter_snapshot,
    }
}

fn slope_digest(label: &'static str, width: usize) -> String {
    hash_parts(
        &(1..=3)
            .map(|scale| format!("{label}:{scale}:{}", width * scale))
            .collect::<Vec<_>>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain_capabilities::certification::reports::forge_query_domain_capability_representative_report;

    #[test]
    fn slope_report_reuses_representative_widths() {
        let representative = forge_query_domain_capability_representative_report();
        let slopes = forge_query_domain_capability_slope_report(&representative);

        assert_eq!(
            slopes.counter_snapshot().contribution_width(),
            representative.contribution_width()
        );
        assert_eq!(
            slopes.counter_snapshot().trace_width(),
            representative.trace_width()
        );
        assert!(!slopes.counter_snapshot().digest().is_empty());
        assert!(!slopes
            .contribution_materialization_slope_digest()
            .is_empty());
        assert!(!slopes.trace_materialization_slope_digest().is_empty());
        assert!(!slopes.category_materialization_slope_digest().is_empty());
        assert!(!slopes.support_materialization_slope_digest().is_empty());
    }
}
