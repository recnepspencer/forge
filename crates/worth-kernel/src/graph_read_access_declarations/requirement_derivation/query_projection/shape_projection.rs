use crate::graph_read_access_inventory::current_query_graph_read_access_capabilities;

use super::super::super::read_family_catalog::WorthGraphReadDeclarationCatalogRecord;
use super::super::derivation_attempt::WorthGraphReadRequirementDerivationAttempt;
use super::super::derivation_outcome::WorthGraphReadRequirementDerivationOutcome;
use super::super::errors::{
    WorthGraphReadRequirementDerivationError, WorthGraphReadRequirementDerivationErrorKind,
};
use super::capability_gap::WorthGraphReadRequirementDerivationCapabilityGap;

pub(crate) fn derive_query_requirement_outcome_for_catalog_record(
    record: &WorthGraphReadDeclarationCatalogRecord,
) -> Result<WorthGraphReadRequirementDerivationOutcome, WorthGraphReadRequirementDerivationError> {
    let capability_report = current_query_graph_read_access_capabilities();
    if capability_report
        .labels_for_kind(
            crate::graph_read_access_inventory::QueryGraphReadAccessCapabilityKind::RequirementKind,
        )
        .is_empty()
    {
        return Err(WorthGraphReadRequirementDerivationError::new(
            WorthGraphReadRequirementDerivationErrorKind::MissingQueryRequirementCapabilityInventory,
        ));
    }

    Ok(outcome_for_anchor_only_catalog_record(
        record,
        &capability_report,
    ))
}

#[cfg(test)]
pub(crate) fn derive_query_requirement_outcome_for_catalog_record_with_requirement_labels(
    record: &WorthGraphReadDeclarationCatalogRecord,
    requirement_labels: &[&'static str],
) -> Result<WorthGraphReadRequirementDerivationOutcome, WorthGraphReadRequirementDerivationError> {
    if requirement_labels.is_empty() {
        return Err(WorthGraphReadRequirementDerivationError::new(
            WorthGraphReadRequirementDerivationErrorKind::MissingQueryRequirementCapabilityInventory,
        ));
    }
    Ok(
        WorthGraphReadRequirementDerivationOutcome::QueryCapabilityGap(
            WorthGraphReadRequirementDerivationCapabilityGap::missing_query_read_family_artifact(
                record.declaration_identity_digest(),
                record.query_family_anchor().family_digest_seed(),
                requirement_labels,
            ),
        ),
    )
}

pub(crate) fn derivation_attempt_for_catalog_record(
    record: &WorthGraphReadDeclarationCatalogRecord,
) -> WorthGraphReadRequirementDerivationAttempt {
    WorthGraphReadRequirementDerivationAttempt::anchor_only(record)
}

fn outcome_for_anchor_only_catalog_record(
    record: &WorthGraphReadDeclarationCatalogRecord,
    capability_report: &crate::graph_read_access_inventory::QueryGraphReadAccessCapabilityReport,
) -> WorthGraphReadRequirementDerivationOutcome {
    WorthGraphReadRequirementDerivationOutcome::QueryCapabilityGap(
        WorthGraphReadRequirementDerivationCapabilityGap::missing_query_read_family_artifact(
            record.declaration_identity_digest(),
            record.query_family_anchor().family_digest_seed(),
            &capability_report.labels_for_kind(
                crate::graph_read_access_inventory::QueryGraphReadAccessCapabilityKind::RequirementKind,
            ),
        ),
    )
}
