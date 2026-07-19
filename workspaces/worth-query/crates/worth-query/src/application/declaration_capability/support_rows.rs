use crate::application::{WorthQueryDeclarationAspectFit, WorthQueryDeclarationFamilyTaxonomy};

use super::{
    WorthQueryDeclarationCapabilityStatus, WorthQueryDeclarationCapabilityVerb,
    WorthQueryDeclarationFamilySupportRow,
};

pub(crate) fn row(
    verb: WorthQueryDeclarationCapabilityVerb,
    status: WorthQueryDeclarationCapabilityStatus,
    aspect_fit: WorthQueryDeclarationAspectFit,
) -> WorthQueryDeclarationFamilySupportRow {
    let reason = match status {
        WorthQueryDeclarationCapabilityStatus::Admitted => {
            "family capability is admitted for this operating world"
        }
        WorthQueryDeclarationCapabilityStatus::NotInstalled => {
            "declaration family is not installed by this domain package"
        }
        WorthQueryDeclarationCapabilityStatus::DeferredDebt => {
            "required family capability remains deferred debt in this Query build"
        }
        WorthQueryDeclarationCapabilityStatus::Unsupported => {
            "required family capability is unsupported in this operating world"
        }
        WorthQueryDeclarationCapabilityStatus::InvalidContext => {
            "required family config sections must be enabled before admission"
        }
    };
    WorthQueryDeclarationFamilySupportRow::new(verb, status, aspect_fit, reason)
}

pub(crate) fn relational_row(
    family_status: WorthQueryDeclarationCapabilityStatus,
    taxonomy: WorthQueryDeclarationFamilyTaxonomy,
    admitted_fit: WorthQueryDeclarationAspectFit,
) -> WorthQueryDeclarationFamilySupportRow {
    witness_row(
        WorthQueryDeclarationCapabilityVerb::RelationalTruthWitness,
        family_status,
        admitted_fit,
        taxonomy.primary_authority_family()
            == crate::application::WorthQueryDeclarationPrimaryAuthorityFamily::RelationalTruth,
        "family is not structurally relational-truth",
    )
}

pub(crate) fn bridge_row(
    family_status: WorthQueryDeclarationCapabilityStatus,
    taxonomy: WorthQueryDeclarationFamilyTaxonomy,
    admitted_fit: WorthQueryDeclarationAspectFit,
) -> WorthQueryDeclarationFamilySupportRow {
    witness_row(
        WorthQueryDeclarationCapabilityVerb::BridgeContinuationWitness,
        family_status,
        admitted_fit,
        taxonomy.primary_authority_family()
            == crate::application::WorthQueryDeclarationPrimaryAuthorityFamily::BridgeContinuation,
        "family is not structurally bridge-continuation",
    )
}

pub(crate) fn signal_row(
    family_status: WorthQueryDeclarationCapabilityStatus,
    taxonomy: WorthQueryDeclarationFamilyTaxonomy,
    admitted_fit: WorthQueryDeclarationAspectFit,
) -> WorthQueryDeclarationFamilySupportRow {
    match taxonomy.signal_compatibility() {
        crate::application::WorthQuerySignalCompatibilityPosture::Compatible => witness_row(
            WorthQueryDeclarationCapabilityVerb::SignalCompatibilityWitness,
            family_status,
            admitted_fit,
            true,
            "",
        ),
        crate::application::WorthQuerySignalCompatibilityPosture::Deferred => {
            WorthQueryDeclarationFamilySupportRow::new(
                WorthQueryDeclarationCapabilityVerb::SignalCompatibilityWitness,
                WorthQueryDeclarationCapabilityStatus::DeferredDebt,
                admitted_fit,
                "signal compatibility for this family remains explicitly deferred",
            )
        }
        crate::application::WorthQuerySignalCompatibilityPosture::NotCompatible => {
            WorthQueryDeclarationFamilySupportRow::new(
                WorthQueryDeclarationCapabilityVerb::SignalCompatibilityWitness,
                WorthQueryDeclarationCapabilityStatus::Unsupported,
                WorthQueryDeclarationAspectFit::MissingRequired,
                "family is not structurally signal-compatible",
            )
        }
    }
}

pub(crate) fn neighborhood_row(
    family_status: WorthQueryDeclarationCapabilityStatus,
    taxonomy: WorthQueryDeclarationFamilyTaxonomy,
    admitted_fit: WorthQueryDeclarationAspectFit,
) -> WorthQueryDeclarationFamilySupportRow {
    witness_row(
        WorthQueryDeclarationCapabilityVerb::NeighborhoodGroupingWitness,
        family_status,
        admitted_fit,
        matches!(
            taxonomy.grouped_posture(),
            crate::application::WorthQueryGroupedDeclarationPosture::NeighborhoodCapable
                | crate::application::WorthQueryGroupedDeclarationPosture::NeighborhoodAndBatchCapable
        ),
        "family is not structurally neighborhood-capable",
    )
}

pub(crate) fn batch_row(
    family_status: WorthQueryDeclarationCapabilityStatus,
    taxonomy: WorthQueryDeclarationFamilyTaxonomy,
    admitted_fit: WorthQueryDeclarationAspectFit,
) -> WorthQueryDeclarationFamilySupportRow {
    witness_row(
        WorthQueryDeclarationCapabilityVerb::BatchGroupingWitness,
        family_status,
        admitted_fit,
        matches!(
            taxonomy.grouped_posture(),
            crate::application::WorthQueryGroupedDeclarationPosture::BatchCapable
                | crate::application::WorthQueryGroupedDeclarationPosture::NeighborhoodAndBatchCapable
        ),
        "family is not structurally batch-capable",
    )
}

fn witness_row(
    verb: WorthQueryDeclarationCapabilityVerb,
    family_status: WorthQueryDeclarationCapabilityStatus,
    admitted_fit: WorthQueryDeclarationAspectFit,
    structurally_available: bool,
    unsupported_reason: &'static str,
) -> WorthQueryDeclarationFamilySupportRow {
    if !structurally_available {
        return WorthQueryDeclarationFamilySupportRow::new(
            verb,
            WorthQueryDeclarationCapabilityStatus::Unsupported,
            WorthQueryDeclarationAspectFit::MissingRequired,
            unsupported_reason,
        );
    }
    row(verb, family_status, admitted_fit)
}
