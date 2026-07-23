use std::sync::Arc;

use crate::basis_lifecycle::BasisOperationLane;

use super::super::{WorthQueryBoundDomainOperation, WorthQueryDomainRebindReceipt};
use super::denial::{
    WorthQueryCompatibilityCounters, WorthQueryCompatibilityDenial,
    WorthQueryCompatibilityDenialKind,
};

pub(super) fn require_current_same_installation_after_runtime<D, O, F, L: BasisOperationLane>(
    subject: &WorthQueryBoundDomainOperation<D, O, F, L>,
    candidate: &WorthQueryBoundDomainOperation<D, O, F, L>,
    counters: &mut WorthQueryCompatibilityCounters,
) -> Result<(), WorthQueryCompatibilityDenial> {
    counters.retained_authority_checks += 1;
    if !subject.installation_is_current() {
        return Err(plain(
            WorthQueryCompatibilityDenialKind::InstallationFreshness,
            "both capabilities must retain current installation authority",
            counters,
        ));
    }
    counters.retained_authority_checks += 1;
    if !candidate.installation_is_current() {
        return Err(plain(
            WorthQueryCompatibilityDenialKind::InstallationFreshness,
            "both capabilities must retain current installation authority",
            counters,
        ));
    }
    counters.retained_authority_checks += 1;
    if subject.operation().installation_generation()
        != candidate.operation().installation_generation()
    {
        return Err(plain(
            WorthQueryCompatibilityDenialKind::InstallationGeneration,
            "capabilities belong to different installation generations",
            counters,
        ));
    }
    counters.retained_authority_checks += 1;
    if !Arc::ptr_eq(
        subject.operation().domain_authority(),
        candidate.operation().domain_authority(),
    ) {
        return Err(plain(
            WorthQueryCompatibilityDenialKind::DomainInstallation,
            "capabilities do not retain the same installed operation authority",
            counters,
        ));
    }
    counters.retained_authority_checks += 1;
    if !Arc::ptr_eq(
        subject.operation().operation_authority(),
        candidate.operation().operation_authority(),
    ) {
        return Err(plain(
            WorthQueryCompatibilityDenialKind::DomainInstallation,
            "capabilities do not retain the same installed operation authority",
            counters,
        ));
    }
    require_exact_bound_authorities(subject, candidate, counters)
}

pub(super) fn require_rebind_successor<D, O, F, L: BasisOperationLane>(
    subject: &WorthQueryBoundDomainOperation<D, O, F, L>,
    candidate: &WorthQueryBoundDomainOperation<D, O, F, L>,
    receipt: &WorthQueryDomainRebindReceipt,
    required_domain_receipts: &[WorthQueryDomainRebindReceipt],
    counters: &mut WorthQueryCompatibilityCounters,
) -> Result<(), WorthQueryCompatibilityDenial> {
    counters.retained_authority_checks += 1;
    if !receipt.binds_authorities(
        subject.operation().domain_authority(),
        candidate.operation().domain_authority(),
    ) {
        return Err(plain(
            WorthQueryCompatibilityDenialKind::DomainRebindAuthority,
            "domain rebind receipt does not bind the exact prior and current authorities",
            counters,
        ));
    }
    counters.retained_authority_checks += 1;
    if subject.installation_is_current() {
        return Err(plain(
            WorthQueryCompatibilityDenialKind::InstallationFreshness,
            "rebind requires a stale subject and a current candidate",
            counters,
        ));
    }
    counters.retained_authority_checks += 1;
    if !candidate.installation_is_current() {
        return Err(plain(
            WorthQueryCompatibilityDenialKind::InstallationFreshness,
            "rebind requires a stale subject and a current candidate",
            counters,
        ));
    }
    require_rebind_bound_authorities(subject, candidate, required_domain_receipts, counters)
}

pub(super) fn require_same_runtime<D, O, F, L: BasisOperationLane>(
    subject: &WorthQueryBoundDomainOperation<D, O, F, L>,
    candidate: &WorthQueryBoundDomainOperation<D, O, F, L>,
    counters: &mut WorthQueryCompatibilityCounters,
) -> Result<(), WorthQueryCompatibilityDenial> {
    counters.retained_authority_checks += 1;
    if subject.operation().domain_authority().runtime_authority()
        != candidate.operation().domain_authority().runtime_authority()
    {
        return Err(plain(
            WorthQueryCompatibilityDenialKind::RuntimeAuthority,
            "capabilities belong to different Query runtimes",
            counters,
        ));
    }
    Ok(())
}

fn require_exact_bound_authorities<D, O, F, L: BasisOperationLane>(
    subject: &WorthQueryBoundDomainOperation<D, O, F, L>,
    candidate: &WorthQueryBoundDomainOperation<D, O, F, L>,
    counters: &mut WorthQueryCompatibilityCounters,
) -> Result<(), WorthQueryCompatibilityDenial> {
    counters.retained_authority_checks += 1;
    let subject_graph_count = subject.graph_participations().len();
    counters.retained_authority_checks += 1;
    let candidate_graph_count = candidate.graph_participations().len();
    let graphs_match = subject_graph_count == candidate_graph_count
        && subject
            .graph_participations()
            .iter()
            .zip(candidate.graph_participations())
            .all(|(left, right)| {
                counters.retained_authority_checks += 1;
                left.role == right.role && Arc::ptr_eq(&left.record, &right.record)
            });
    if !graphs_match {
        return Err(plain(
            WorthQueryCompatibilityDenialKind::GraphAuthority,
            "bound graph authority sets differ",
            counters,
        ));
    }
    counters.retained_authority_checks += 1;
    let subject_domain_count = subject.required_domains().len();
    counters.retained_authority_checks += 1;
    let candidate_domain_count = candidate.required_domains().len();
    let domains_match = subject_domain_count == candidate_domain_count
        && subject
            .required_domains()
            .iter()
            .zip(candidate.required_domains())
            .all(|(left, right)| {
                counters.retained_authority_checks += 1;
                left.role == right.role && Arc::ptr_eq(&left.authority, &right.authority)
            });
    if !domains_match {
        return Err(plain(
            WorthQueryCompatibilityDenialKind::RequiredDomainAuthority,
            "bound required-domain authority sets differ",
            counters,
        ));
    }
    Ok(())
}

fn require_rebind_bound_authorities<D, O, F, L: BasisOperationLane>(
    subject: &WorthQueryBoundDomainOperation<D, O, F, L>,
    candidate: &WorthQueryBoundDomainOperation<D, O, F, L>,
    required_domain_receipts: &[WorthQueryDomainRebindReceipt],
    counters: &mut WorthQueryCompatibilityCounters,
) -> Result<(), WorthQueryCompatibilityDenial> {
    counters.retained_authority_checks += 1;
    let subject_graph_count = subject.graph_participations().len();
    counters.retained_authority_checks += 1;
    let candidate_graph_count = candidate.graph_participations().len();
    let graphs_match = subject_graph_count == candidate_graph_count
        && subject
            .graph_participations()
            .iter()
            .zip(candidate.graph_participations())
            .all(|(left, right)| {
                counters.retained_authority_checks += 1;
                left.role == right.role && Arc::ptr_eq(&left.record, &right.record)
            });
    if !graphs_match {
        return Err(plain(
            WorthQueryCompatibilityDenialKind::GraphAuthority,
            "rebind changed an installed graph provider authority",
            counters,
        ));
    }
    counters.retained_authority_checks += 1;
    let subject_domain_count = subject.required_domains().len();
    counters.retained_authority_checks += 1;
    let candidate_domain_count = candidate.required_domains().len();
    let domains_match = subject_domain_count == candidate_domain_count
        && subject
            .required_domains()
            .iter()
            .zip(candidate.required_domains())
            .all(|(left, right)| {
                counters.retained_authority_checks += 1;
                if left.role != right.role {
                    return false;
                }
                if Arc::ptr_eq(&left.authority, &right.authority) {
                    return true;
                }
                required_domain_receipts.iter().any(|receipt| {
                    counters.required_domain_rebind_receipts_inspected += 1;
                    receipt.binds_authorities(&left.authority, &right.authority)
                })
            });
    let exact_receipt_closure = required_domain_receipts.iter().all(|receipt| {
        subject
            .required_domains()
            .iter()
            .zip(candidate.required_domains())
            .any(|(left, right)| {
                counters.required_domain_rebind_receipts_inspected += 1;
                !Arc::ptr_eq(&left.authority, &right.authority)
                    && receipt.binds_authorities(&left.authority, &right.authority)
            })
    });
    if !domains_match || !exact_receipt_closure {
        return Err(plain(
            WorthQueryCompatibilityDenialKind::RequiredDomainAuthority,
            "each changed required-domain authority requires its exact owner-issued rebind receipt",
            counters,
        ));
    }
    Ok(())
}

fn plain(
    kind: WorthQueryCompatibilityDenialKind,
    detail: &'static str,
    counters: &WorthQueryCompatibilityCounters,
) -> WorthQueryCompatibilityDenial {
    WorthQueryCompatibilityDenial::plain(kind, detail, *counters)
}
