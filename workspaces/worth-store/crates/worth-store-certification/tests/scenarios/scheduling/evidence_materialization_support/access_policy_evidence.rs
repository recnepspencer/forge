use super::super::s6_access_policy_support;

use worth_store_certification::S6AccessPolicyEvidenceRow;
use worth_store_physical_backend::{
    AccessPolicyExecutionObservation, AccessPolicyRequest, AdmittedBackendCapabilityWitness,
};

pub(super) fn access_policy_rows() -> Vec<S6AccessPolicyEvidenceRow> {
    let backend = super::backend_witness();
    access_policy_rows_for_backend(&backend)
}

pub(super) fn access_policy_rows_for_backend(
    backend: &AdmittedBackendCapabilityWitness,
) -> Vec<S6AccessPolicyEvidenceRow> {
    let buffered = s6_access_policy_support::admit(
        backend,
        s6_access_policy_support::base_request(backend, AccessPolicyRequest::buffered_read()),
    );
    let mmap =
        s6_access_policy_support::admit(backend, s6_access_policy_support::mmap_request(backend));
    vec![
        s6_access_policy_support::executed_row(
            buffered,
            s6_access_policy_support::ExpectedExecutionRequest::buffered(),
            AccessPolicyExecutionObservation::completed_without_violation()
                .with_page_cache_visibility_observed()
                .with_security_scope_preserved(),
        ),
        s6_access_policy_support::violation_row(
            mmap,
            s6_access_policy_support::ExpectedExecutionRequest::mmap(),
            AccessPolicyExecutionObservation::mmap_lazy_fault(),
        ),
    ]
}

pub(super) fn access_policy_rows_without_violations() -> Vec<S6AccessPolicyEvidenceRow> {
    let backend = super::backend_witness();
    let buffered = s6_access_policy_support::admit(
        &backend,
        s6_access_policy_support::base_request(&backend, AccessPolicyRequest::buffered_read()),
    );
    vec![s6_access_policy_support::executed_row(
        buffered,
        s6_access_policy_support::ExpectedExecutionRequest::buffered(),
        AccessPolicyExecutionObservation::completed_without_violation()
            .with_page_cache_visibility_observed()
            .with_security_scope_preserved(),
    )]
}
