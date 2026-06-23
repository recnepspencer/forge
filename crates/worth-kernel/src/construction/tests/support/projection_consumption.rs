use forge_query::facade::{
    ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily, ForgeQueryWorkspace,
};
use topology::facade::{
    TopologyConstructionQueryFactProvenance, TopologyConstructionQueryInspectionSurface,
    TopologyConstructionQueryReadSurface,
};
use worth_geom::facade::PrimitiveStabilityClass;

use crate::construction::authoring::{
    require_default_primitive_construction_query_authority, PrimitiveConstructionQueryEntryError,
};
use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::request::PrimitiveConstructionFamily;
use crate::construction::tests::support::evidence_reports::sealed_report_identity;
use crate::construction::tests::support::runtime_truth::{
    prepare_primitive_construction_certification_runtime_truth,
    PrimitiveConstructionCertificationRuntimeTruth,
};

#[derive(Debug)]
pub(crate) enum PrimitiveConstructionQueryProjectionConsumptionParityError {
    QueryEntry(PrimitiveConstructionQueryEntryError),
    QueryRuntime(ForgeQueryRuntimeError),
    RejectedOutcome { reason: String },
    UnsupportedSurface { family: PrimitiveConstructionFamily },
}

impl std::fmt::Display for PrimitiveConstructionQueryProjectionConsumptionParityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::QueryEntry(error) => write!(f, "{error}"),
            Self::QueryRuntime(error) => write!(f, "{error}"),
            Self::RejectedOutcome { reason } => write!(f, "{reason}"),
            Self::UnsupportedSurface { family } => write!(
                f,
                "{} did not retain the sanctioned projection-consumption query surface",
                family.as_str()
            ),
        }
    }
}

impl std::error::Error for PrimitiveConstructionQueryProjectionConsumptionParityError {}

pub(crate) fn prepare_primitive_construction_query_projection_consumption_surface_digest(
    workspace: &mut ForgeQueryWorkspace,
    intent: PrimitiveConstructionIntent,
) -> Result<String, PrimitiveConstructionQueryProjectionConsumptionParityError> {
    let write_contract_digest = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Write)
        .map_err(PrimitiveConstructionQueryProjectionConsumptionParityError::QueryRuntime)?
        .contract_digest()
        .to_string();
    let query_contract_digest = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Inspect)
        .map_err(PrimitiveConstructionQueryProjectionConsumptionParityError::QueryRuntime)?
        .contract_digest()
        .to_string();
    let authority_receipt = require_default_primitive_construction_query_authority(workspace)
        .map_err(PrimitiveConstructionQueryProjectionConsumptionParityError::QueryEntry)?;
    let outcome =
        match prepare_primitive_construction_certification_runtime_truth(intent.into_request()) {
            PrimitiveConstructionCertificationRuntimeTruth::Admitted(outcome) => outcome,
            PrimitiveConstructionCertificationRuntimeTruth::Rejected(rejected) => {
                return Err(
                    PrimitiveConstructionQueryProjectionConsumptionParityError::RejectedOutcome {
                        reason: rejected.reason().to_string(),
                    },
                );
            }
        };

    let sanctioned_surface = outcome.required_query_families()
        == [
            ForgeQueryRuntimeFacadeFamily::Write,
            ForgeQueryRuntimeFacadeFamily::Inspect,
        ]
        && outcome.read_surface()
            == TopologyConstructionQueryReadSurface::ProjectionConsumptionFromInspectionReceipt
        && outcome.inspection_surface()
            == TopologyConstructionQueryInspectionSurface::InspectReceipt
        && outcome.fact_provenance()
            == TopologyConstructionQueryFactProvenance::InspectionBackedProjectionConsumption
        && !outcome.topology_fact_digest().is_empty()
        && !query_contract_digest.is_empty()
        && matches!(
            outcome.stability_class(),
            PrimitiveStabilityClass::StableDirect | PrimitiveStabilityClass::StableAfterEscalation
        );

    if !sanctioned_surface {
        return Err(
            PrimitiveConstructionQueryProjectionConsumptionParityError::UnsupportedSurface {
                family: outcome.family(),
            },
        );
    }

    Ok(sealed_report_identity(
        "worth-kernel.construction.projection-consumption",
        "projection-consumption-surface",
        |report| {
            report
                .shape_participating("family", outcome.family().as_str())?
                .value_participating("write-contract", write_contract_digest)?
                .value_participating("query-contract", query_contract_digest)?
                .value_participating(
                    "query-authority-receipt",
                    authority_receipt.authority_receipt_digest().to_string(),
                )?
                .value_participating(
                    "query-authority-operating-context",
                    authority_receipt
                        .operating_context_identity_digest()
                        .to_string(),
                )?
                .value_participating(
                    "query-authority-basis",
                    authority_receipt.authority_basis_digest().to_string(),
                )?
                .value_participating(
                    "query-authority-configured-handle-support",
                    authority_receipt
                        .configured_handle_support_snapshot_digest()
                        .to_string(),
                )?
                .value_participating(
                    "query-authority-evaluated-support-snapshot",
                    authority_receipt
                        .evaluated_support_snapshot_digest()
                        .to_string(),
                )?
                .value_participating(
                    "query-authority-evaluated-source-matrix",
                    authority_receipt
                        .evaluated_support_source_matrix_digest()
                        .to_string(),
                )?
                .value_participating(
                    "query-authority-support-pin-contract",
                    authority_receipt.support_pin_contract_digest().to_string(),
                )?
                .value_participating(
                    "query-authority-support-pin-report",
                    authority_receipt.support_pin_report_digest().to_string(),
                )?
                .usize_participating(
                    "query-authority-support-pin-findings",
                    authority_receipt.support_pin_finding_count(),
                )?
                .value_sequence_participating(
                    "required-query-families",
                    outcome
                        .required_query_families()
                        .iter()
                        .map(|family| format!("{family:?}")),
                )?
                .shape_participating("read-surface", outcome.read_surface().as_str())?
                .shape_participating("inspection-surface", outcome.inspection_surface().as_str())?
                .shape_participating("fact-provenance", outcome.fact_provenance().as_str())?
                .value_participating("outcome", outcome.outcome_digest().to_string())?
                .bool_participating("sanctioned-surface", true)
        },
    ))
}
