use worth_store_security::admitted_store_internal_security_scope_for_io_qos_test;

pub(super) fn io_qos_security_scope_admission() -> crate::IoSchedulerSecurityScopeAdmission {
    io_qos_security_scope_admission_from(admitted_store_internal_security_scope_for_io_qos_test())
}

fn io_qos_security_scope_admission_from(
    scope: worth_store_security::StoreAdmittedSecurityScope,
) -> crate::IoSchedulerSecurityScopeAdmission {
    crate::admit_security_scope_for_scheduler(&scope)
        .expect("test security scope should admit for scheduler use")
}
