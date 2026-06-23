use crate::domain_capabilities::identity::{
    compose_counter_snapshot_digest, compose_slope_digest, compose_slope_scale_entry_digest,
};

use super::representative::ForgeQueryDomainCapabilityRepresentativeReport;
use super::scaled::forge_query_domain_capability_scaled_evidence;

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
    _representative: &ForgeQueryDomainCapabilityRepresentativeReport,
) -> ForgeQueryDomainCapabilitySlopeReport {
    let scaled = forge_query_domain_capability_scaled_evidence();
    let full_scale = &scaled[2];
    let counter_snapshot = ForgeQueryDomainCapabilityCertificationCounterSnapshot {
        contribution_width: full_scale.contribution_width(),
        trace_width: full_scale.trace_width(),
        category_width: full_scale.category_width(),
        support_width: full_scale.support_width(),
        digest: compose_counter_snapshot_digest(
            full_scale.contribution_width(),
            full_scale.trace_width(),
            full_scale.category_width(),
            full_scale.support_width(),
        ),
    };

    ForgeQueryDomainCapabilitySlopeReport {
        contribution_materialization_slope_digest: slope_digest(
            "contribution-materialization",
            &scaled,
            |e| (e.contribution_width(), e.contribution_digest()),
        ),
        trace_materialization_slope_digest: slope_digest("trace-materialization", &scaled, |e| {
            (e.trace_width(), e.trace_digest())
        }),
        category_materialization_slope_digest: slope_digest(
            "category-materialization",
            &scaled,
            |e| (e.category_width(), e.category_digest()),
        ),
        support_materialization_slope_digest: slope_digest(
            "support-materialization",
            &scaled,
            |e| (e.support_width(), e.support_digest()),
        ),
        counter_snapshot,
    }
}

fn slope_digest(
    label: &'static str,
    scaled: &[super::scaled::ForgeQueryDomainCapabilityScaledEvidence; 3],
    projector: impl Fn(&super::scaled::ForgeQueryDomainCapabilityScaledEvidence) -> (usize, &str),
) -> String {
    compose_slope_digest(scaled.iter().enumerate().map(|(index, evidence)| {
        let (width, digest) = projector(evidence);
        compose_slope_scale_entry_digest(label, index + 1, width, digest)
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain_capabilities::certification::reports::forge_query_domain_capability_representative_report;

    #[test]
    fn slope_report_reuses_representative_widths() {
        let representative = forge_query_domain_capability_representative_report();
        let slopes = forge_query_domain_capability_slope_report(&representative);

        assert_eq!(slopes.counter_snapshot().contribution_width(), 7);
        assert!(slopes.counter_snapshot().trace_width() >= representative.trace_width());
        assert!(!slopes.counter_snapshot().digest().is_empty());
        assert!(!slopes
            .contribution_materialization_slope_digest()
            .is_empty());
        assert!(!slopes.trace_materialization_slope_digest().is_empty());
        assert!(!slopes.category_materialization_slope_digest().is_empty());
        assert!(!slopes.support_materialization_slope_digest().is_empty());
    }
}
