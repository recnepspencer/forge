use worth_store_physical_integrity::PreDecodePhysicalDenial;

pub(crate) fn assert_localized_pre_decode_denial(denial: PreDecodePhysicalDenial) {
    assert!(denial.locality().is_some());
    assert_pre_decode_denial(denial);
}

pub(crate) fn assert_pre_decode_denial_counters(
    denial: PreDecodePhysicalDenial,
    expected_checked_byte_count: u64,
    expected_checksum_executions: u32,
) {
    assert_eq!(denial.protected_byte_count(), expected_checked_byte_count);
    assert_eq!(
        denial.counters().checked_byte_count(),
        expected_checked_byte_count
    );
    assert_eq!(
        denial.counters().checksum_execution_count(),
        expected_checksum_executions
    );
    assert_pre_decode_denial(denial);
}

pub(crate) fn assert_localized_pre_decode_denial_counters(
    denial: PreDecodePhysicalDenial,
    expected_checked_byte_count: u64,
    expected_checksum_executions: u32,
) {
    assert!(denial.locality().is_some());
    assert_pre_decode_denial_counters(
        denial,
        expected_checked_byte_count,
        expected_checksum_executions,
    );
}

fn assert_pre_decode_denial(denial: PreDecodePhysicalDenial) {
    assert_eq!(
        denial.counters().skipped_logical_decode().skipped_count(),
        1
    );
    assert_eq!(
        denial
            .counters()
            .semantic_decoder_invocations()
            .invocation_count(),
        0
    );
}
