use super::test_support::{
    backend_with_all_access_modes, base_request, direct_io_alignment, page_cache_policy,
    pinned_lifecycle, test_reference,
};
use super::{AccessPolicyAdmission, AccessPolicyRequest, StoreAccessMode};

#[test]
fn equivalent_buffered_declarations_admit_equivalent_policy_and_counters() {
    let first_backend = backend_with_all_access_modes();
    let second_backend = backend_with_all_access_modes();

    let first = AccessPolicyAdmission::for_backend(&first_backend)
        .admit(base_request(
            &first_backend,
            AccessPolicyRequest::buffered_read(),
        ))
        .expect("first buffered policy admits");
    let second = AccessPolicyAdmission::for_backend(&second_backend)
        .admit(base_request(
            &second_backend,
            AccessPolicyRequest::buffered_read(),
        ))
        .expect("second buffered policy admits");

    assert_eq!(first.mode(), StoreAccessMode::Buffered);
    assert_eq!(first.mode(), second.mode());
    assert_eq!(first.profile(), second.profile());
    assert_eq!(first.evidence_class(), second.evidence_class());
    assert!(first.security_scope().is_some());
    assert!(second.security_scope().is_some());
    assert_eq!(first.counters(), second.counters());
}

#[test]
fn equivalent_direct_io_declarations_admit_equivalent_alignment_and_counters() {
    let first_backend = backend_with_all_access_modes();
    let second_backend = backend_with_all_access_modes();
    let reference = test_reference();
    let lifecycle = pinned_lifecycle();

    let first_request = base_request(
        &first_backend,
        AccessPolicyRequest::direct_io_read().with_alignment_requirement(direct_io_alignment(
            &first_backend,
            reference,
            lifecycle,
            4096,
        )),
    );
    let second_request = base_request(
        &second_backend,
        AccessPolicyRequest::direct_io_read().with_alignment_requirement(direct_io_alignment(
            &second_backend,
            reference,
            lifecycle,
            4096,
        )),
    );

    let first = AccessPolicyAdmission::for_backend(&first_backend)
        .admit(first_request)
        .expect("first direct I/O policy admits");
    let second = AccessPolicyAdmission::for_backend(&second_backend)
        .admit(second_request)
        .expect("second direct I/O policy admits");

    assert_eq!(first.mode(), StoreAccessMode::DirectIo);
    assert_eq!(first.request().alignment(), second.request().alignment());
    assert_eq!(first.profile(), second.profile());
    assert_eq!(first.evidence_class(), second.evidence_class());
    assert_eq!(first.counters(), second.counters());
}

#[test]
fn equivalent_unaligned_direct_io_declarations_deny_with_same_topology() {
    let first_backend = backend_with_all_access_modes();
    let second_backend = backend_with_all_access_modes();

    let first = AccessPolicyAdmission::for_backend(&first_backend)
        .admit(
            base_request(&first_backend, AccessPolicyRequest::direct_io_read())
                .with_page_cache_policy(page_cache_policy(&first_backend)),
        )
        .expect_err("first direct I/O policy denies without alignment");
    let second = AccessPolicyAdmission::for_backend(&second_backend)
        .admit(
            base_request(&second_backend, AccessPolicyRequest::direct_io_read())
                .with_page_cache_policy(page_cache_policy(&second_backend)),
        )
        .expect_err("second direct I/O policy denies without alignment");

    assert_eq!(first.kind(), second.kind());
    assert_eq!(first.counters(), second.counters());
}
