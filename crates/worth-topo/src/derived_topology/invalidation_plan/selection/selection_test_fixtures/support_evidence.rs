use super::super::{
    DerivedInvalidationLegalitySupportEvidence, DerivedInvalidationQuerySupportEvidence,
};

pub(crate) fn admitted_query_support() -> DerivedInvalidationQuerySupportEvidence {
    DerivedInvalidationQuerySupportEvidence::from_receipt_digests_for_tests(
        Some("query.projection.consumption.receipt".to_string()),
        Some("query.native.read.receipt".to_string()),
        Some("query.native.write.receipt".to_string()),
    )
}

pub(crate) fn admitted_legality_support() -> DerivedInvalidationLegalitySupportEvidence {
    DerivedInvalidationLegalitySupportEvidence::from_digests_for_tests(
        Some("topology.selected.legality.plan".to_string()),
        Some("topology.selected.validator.receipt".to_string()),
    )
}

#[cfg(test)]
pub(crate) fn query_support_missing_projection_consumption(
) -> DerivedInvalidationQuerySupportEvidence {
    DerivedInvalidationQuerySupportEvidence::from_receipt_digests_for_tests(
        None,
        Some("query.native.read.receipt".to_string()),
        Some("query.native.write.receipt".to_string()),
    )
}

#[cfg(test)]
pub(crate) fn query_support_missing_native_read() -> DerivedInvalidationQuerySupportEvidence {
    DerivedInvalidationQuerySupportEvidence::from_receipt_digests_for_tests(
        Some("query.projection.consumption.receipt".to_string()),
        None,
        Some("query.native.write.receipt".to_string()),
    )
}

#[cfg(test)]
pub(crate) fn query_support_missing_native_write() -> DerivedInvalidationQuerySupportEvidence {
    DerivedInvalidationQuerySupportEvidence::from_receipt_digests_for_tests(
        Some("query.projection.consumption.receipt".to_string()),
        Some("query.native.read.receipt".to_string()),
        None,
    )
}

#[cfg(test)]
pub(crate) fn legality_support_missing_selected_legality_plan(
) -> DerivedInvalidationLegalitySupportEvidence {
    DerivedInvalidationLegalitySupportEvidence::from_digests_for_tests(
        None,
        Some("topology.selected.validator.receipt".to_string()),
    )
}

#[cfg(test)]
pub(crate) fn legality_support_missing_selected_validator_receipt(
) -> DerivedInvalidationLegalitySupportEvidence {
    DerivedInvalidationLegalitySupportEvidence::from_digests_for_tests(
        Some("topology.selected.legality.plan".to_string()),
        None,
    )
}
