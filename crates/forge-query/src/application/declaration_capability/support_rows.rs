use crate::application::{ForgeQueryDeclarationAspectFit, ForgeQueryDeclarationFamilyTaxonomy};

use super::{
    ForgeQueryDeclarationCapabilityStatus, ForgeQueryDeclarationCapabilityVerb,
    ForgeQueryDeclarationFamilySupportRow,
};

pub(crate) fn row(
    verb: ForgeQueryDeclarationCapabilityVerb,
    status: ForgeQueryDeclarationCapabilityStatus,
    aspect_fit: ForgeQueryDeclarationAspectFit,
) -> ForgeQueryDeclarationFamilySupportRow {
    let reason = match status {
        ForgeQueryDeclarationCapabilityStatus::Admitted => {
            "family capability is admitted for this operating world"
        }
        ForgeQueryDeclarationCapabilityStatus::DeferredDebt => {
            "required family capability remains deferred debt in this Query build"
        }
        ForgeQueryDeclarationCapabilityStatus::Unsupported => {
            "required family capability is unsupported in this operating world"
        }
        ForgeQueryDeclarationCapabilityStatus::InvalidContext => {
            "required family config sections must be enabled before admission"
        }
    };
    ForgeQueryDeclarationFamilySupportRow::new(verb, status, aspect_fit, reason)
}

pub(crate) fn relational_row(
    family_status: ForgeQueryDeclarationCapabilityStatus,
    taxonomy: ForgeQueryDeclarationFamilyTaxonomy,
    admitted_fit: ForgeQueryDeclarationAspectFit,
) -> ForgeQueryDeclarationFamilySupportRow {
    witness_row(
        ForgeQueryDeclarationCapabilityVerb::RelationalTruthWitness,
        family_status,
        admitted_fit,
        taxonomy.primary_authority_family()
            == crate::application::ForgeQueryDeclarationPrimaryAuthorityFamily::RelationalTruth,
        "family is not structurally relational-truth",
    )
}

pub(crate) fn bridge_row(
    family_status: ForgeQueryDeclarationCapabilityStatus,
    taxonomy: ForgeQueryDeclarationFamilyTaxonomy,
    admitted_fit: ForgeQueryDeclarationAspectFit,
) -> ForgeQueryDeclarationFamilySupportRow {
    witness_row(
        ForgeQueryDeclarationCapabilityVerb::BridgeContinuationWitness,
        family_status,
        admitted_fit,
        taxonomy.primary_authority_family()
            == crate::application::ForgeQueryDeclarationPrimaryAuthorityFamily::BridgeContinuation,
        "family is not structurally bridge-continuation",
    )
}

pub(crate) fn signal_row(
    family_status: ForgeQueryDeclarationCapabilityStatus,
    taxonomy: ForgeQueryDeclarationFamilyTaxonomy,
    admitted_fit: ForgeQueryDeclarationAspectFit,
) -> ForgeQueryDeclarationFamilySupportRow {
    match taxonomy.signal_compatibility() {
        crate::application::ForgeQuerySignalCompatibilityPosture::Compatible => witness_row(
            ForgeQueryDeclarationCapabilityVerb::SignalCompatibilityWitness,
            family_status,
            admitted_fit,
            true,
            "",
        ),
        crate::application::ForgeQuerySignalCompatibilityPosture::Deferred => {
            ForgeQueryDeclarationFamilySupportRow::new(
                ForgeQueryDeclarationCapabilityVerb::SignalCompatibilityWitness,
                ForgeQueryDeclarationCapabilityStatus::DeferredDebt,
                admitted_fit,
                "signal compatibility for this family remains explicitly deferred",
            )
        }
        crate::application::ForgeQuerySignalCompatibilityPosture::NotCompatible => {
            ForgeQueryDeclarationFamilySupportRow::new(
                ForgeQueryDeclarationCapabilityVerb::SignalCompatibilityWitness,
                ForgeQueryDeclarationCapabilityStatus::Unsupported,
                ForgeQueryDeclarationAspectFit::MissingRequired,
                "family is not structurally signal-compatible",
            )
        }
    }
}

pub(crate) fn neighborhood_row(
    family_status: ForgeQueryDeclarationCapabilityStatus,
    taxonomy: ForgeQueryDeclarationFamilyTaxonomy,
    admitted_fit: ForgeQueryDeclarationAspectFit,
) -> ForgeQueryDeclarationFamilySupportRow {
    witness_row(
        ForgeQueryDeclarationCapabilityVerb::NeighborhoodGroupingWitness,
        family_status,
        admitted_fit,
        matches!(
            taxonomy.grouped_posture(),
            crate::application::ForgeQueryGroupedDeclarationPosture::NeighborhoodCapable
                | crate::application::ForgeQueryGroupedDeclarationPosture::NeighborhoodAndBatchCapable
        ),
        "family is not structurally neighborhood-capable",
    )
}

pub(crate) fn batch_row(
    family_status: ForgeQueryDeclarationCapabilityStatus,
    taxonomy: ForgeQueryDeclarationFamilyTaxonomy,
    admitted_fit: ForgeQueryDeclarationAspectFit,
) -> ForgeQueryDeclarationFamilySupportRow {
    witness_row(
        ForgeQueryDeclarationCapabilityVerb::BatchGroupingWitness,
        family_status,
        admitted_fit,
        matches!(
            taxonomy.grouped_posture(),
            crate::application::ForgeQueryGroupedDeclarationPosture::BatchCapable
                | crate::application::ForgeQueryGroupedDeclarationPosture::NeighborhoodAndBatchCapable
        ),
        "family is not structurally batch-capable",
    )
}

fn witness_row(
    verb: ForgeQueryDeclarationCapabilityVerb,
    family_status: ForgeQueryDeclarationCapabilityStatus,
    admitted_fit: ForgeQueryDeclarationAspectFit,
    structurally_available: bool,
    unsupported_reason: &'static str,
) -> ForgeQueryDeclarationFamilySupportRow {
    if !structurally_available {
        return ForgeQueryDeclarationFamilySupportRow::new(
            verb,
            ForgeQueryDeclarationCapabilityStatus::Unsupported,
            ForgeQueryDeclarationAspectFit::MissingRequired,
            unsupported_reason,
        );
    }
    row(verb, family_status, admitted_fit)
}
