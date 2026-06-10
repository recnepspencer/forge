#![allow(dead_code)]

use forge_server::{ForgeServerBinaryDownload, ForgeServerQueryHandoffDenial};

pub(crate) fn assert_download_denial(
    denial: &ForgeServerQueryHandoffDenial,
    expected_code: forge_server::ForgeServerQueryHandoffDenialCode,
    detail_fragment: &str,
) {
    assert_eq!(denial.code(), expected_code);
    assert!(
        denial.detail().contains(detail_fragment),
        "expected denial detail `{}` to contain `{detail_fragment}`",
        denial.detail()
    );
}

pub(crate) fn assert_counter(
    receipt: &forge_server::ForgeServerBinaryEgressPerformanceReceipt,
    name: &str,
    expected: u64,
) {
    assert_eq!(receipt.counter(name), Some(expected));
}

pub(crate) fn assert_metadata_parity(
    left: &ForgeServerBinaryDownload,
    right: &ForgeServerBinaryDownload,
) {
    assert_eq!(
        left.read().canonical_digest(),
        right.read().canonical_digest()
    );
    assert_eq!(
        left.read().direct_context().canonical_digest(),
        right.read().direct_context().canonical_digest()
    );
    assert_eq!(
        left.read().response_envelope().canonical_digest(),
        right.read().response_envelope().canonical_digest()
    );
    assert_eq!(
        left.read().support_posture(),
        right.read().support_posture()
    );
}
