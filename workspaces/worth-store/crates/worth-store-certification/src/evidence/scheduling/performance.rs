use worth_foundational::{
    FoundationalAuthoritativePerformanceClaim, FoundationalCounterBackedPerformanceReceipt,
};

use crate::courtroom::foundational::foundational_boundary_performance::counter_receipt;

use super::S6CertificationEvidenceSources;

type Receipt =
    FoundationalCounterBackedPerformanceReceipt<FoundationalAuthoritativePerformanceClaim>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct S6FoundationalPerformanceReceipts {
    runtime_execution: Receipt,
    access_policy: Receipt,
    security_scope: Receipt,
    qualification: Receipt,
}

impl S6FoundationalPerformanceReceipts {
    pub(crate) fn from_sources(
        sources: &S6CertificationEvidenceSources,
    ) -> Result<Self, crate::FoundationalBoundaryEvidenceDenial> {
        Ok(Self {
            runtime_execution: counter_receipt(
                "store.s6.certification.runtime-execution",
                &runtime_rows(sources),
            )?,
            access_policy: counter_receipt(
                "store.s6.certification.access-policy",
                &access_policy_rows(sources),
            )?,
            security_scope: counter_receipt(
                "store.s6.certification.security-scope-preservation",
                &security_scope_rows(sources),
            )?,
            qualification: counter_receipt(
                "store.s6.certification.backend-qualification",
                &qualification_rows(sources),
            )?,
        })
    }

    pub const fn runtime_execution_receipt(&self) -> &Receipt {
        &self.runtime_execution
    }

    pub const fn access_policy_receipt(&self) -> &Receipt {
        &self.access_policy
    }

    pub const fn security_scope_receipt(&self) -> &Receipt {
        &self.security_scope
    }

    pub const fn qualification_receipt(&self) -> &Receipt {
        &self.qualification
    }

    pub fn has_required_counter_contracts(&self) -> bool {
        [
            (&self.runtime_execution, "store.s6.queue.submitted"),
            (&self.runtime_execution, "store.s6.flush.rows"),
            (
                &self.access_policy,
                "store.s6.access_policy.security_scope_preservations",
            ),
            (&self.security_scope, "store.s6.security_scope.scope_checks"),
            (&self.qualification, "store.s6.qualification.rows"),
        ]
        .into_iter()
        .all(|(receipt, name)| {
            receipt
                .counter_rows()
                .iter()
                .any(|row| row.name().as_str() == name)
        })
    }
}

fn runtime_rows(sources: &S6CertificationEvidenceSources) -> [(&'static str, u64); 8] {
    let queue = sources.queue_execution().counters();
    let foreground = sources.foreground_reservation().counters();
    let background = sources.background_pacing().counters();
    [
        ("store.s6.queue.submitted", queue.submitted_units()),
        ("store.s6.queue.denied", queue.denied_units()),
        ("store.s6.queue.violations", queue.violation_events()),
        (
            "store.s6.queue.peak_depth",
            u64::from(queue.peak_queue_depth()),
        ),
        (
            "store.s6.foreground.wait",
            foreground.stable_read_wait_count(),
        ),
        ("store.s6.background.yield", background.yield_events()),
        ("store.s6.background.denied", background.denied_events()),
        (
            "store.s6.flush.rows",
            sources.flush_durability().len() as u64,
        ),
    ]
}

fn access_policy_rows(sources: &S6CertificationEvidenceSources) -> [(&'static str, u64); 8] {
    let rows = sources.access_policy_rows();
    [
        ("store.s6.access_policy.rows", rows.len() as u64),
        (
            "store.s6.access_policy.buffered_admissions",
            rows.iter()
                .map(|row| row.counters().buffered_admissions())
                .sum(),
        ),
        (
            "store.s6.access_policy.mmap_admissions",
            rows.iter()
                .map(|row| row.counters().mmap_admissions())
                .sum(),
        ),
        (
            "store.s6.access_policy.direct_io_admissions",
            rows.iter()
                .map(|row| row.counters().direct_io_admissions())
                .sum(),
        ),
        (
            "store.s6.access_policy.mixed_mode_admissions",
            rows.iter()
                .map(|row| row.counters().mixed_mode_admissions())
                .sum(),
        ),
        (
            "store.s6.access_policy.security_scope_preservations",
            rows.iter()
                .map(|row| row.counters().security_scope_preservations())
                .sum(),
        ),
        (
            "store.s6.access_policy.denials",
            rows.iter().map(|row| row.counters().denials()).sum(),
        ),
        (
            "store.s6.access_policy.violations",
            rows.iter().map(|row| row.counters().violations()).sum(),
        ),
    ]
}

fn security_scope_rows(sources: &S6CertificationEvidenceSources) -> [(&'static str, u64); 4] {
    let counters = sources.secure_io_preservation().counters();
    [
        (
            "store.s6.security_scope.scope_checks",
            counters.scope_checks(),
        ),
        (
            "store.s6.security_scope.backend_posture_checks",
            counters.backend_posture_checks(),
        ),
        (
            "store.s6.security_scope.denied_checks",
            counters.denied_checks(),
        ),
        (
            "store.s6.security_scope.post_admission_violations",
            sources
                .post_admission_violations()
                .iter()
                .map(|row| row.observed_violations())
                .sum(),
        ),
    ]
}

fn qualification_rows(sources: &S6CertificationEvidenceSources) -> [(&'static str, u64); 3] {
    [
        (
            "store.s6.qualification.rows",
            sources.qualification_matrix().row_count() as u64,
        ),
        (
            "store.s6.qualification.certified_support",
            sources
                .qualification_matrix()
                .certified_support_rows()
                .len() as u64,
        ),
        (
            "store.s6.harness.real_backend_safety",
            u64::from(sources.harness_closeout().real_backend_safety().is_some()),
        ),
    ]
}
