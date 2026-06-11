use forge_query::facade::{
    ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily, ForgeQueryWorkspace,
};
use topology::facade::{
    TopologyConstructionQueryFactProvenance, TopologyConstructionQueryInspectionSurface,
    TopologyConstructionQueryReadSurface,
};
use worth_geom::facade::PrimitiveStabilityClass;

use crate::construction::authoring::{
    require_primitive_construction_query_authority, PrimitiveConstructionQueryEntryError,
};
use crate::construction::digest::digest_owned_parts;
use crate::construction::intent::PrimitiveConstructionIntent;
use crate::construction::request::PrimitiveConstructionFamily;
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
    let query_contract_digest = workspace
        .admit_public_api_family(ForgeQueryRuntimeFacadeFamily::Inspect)
        .map_err(PrimitiveConstructionQueryProjectionConsumptionParityError::QueryRuntime)?
        .contract_digest()
        .to_string();
    require_primitive_construction_query_authority(workspace)
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

    Ok(digest_owned_parts(&[
        outcome.family().as_str().to_string(),
        query_contract_digest,
        outcome
            .required_query_families()
            .iter()
            .map(|family| format!("{family:?}"))
            .collect::<Vec<_>>()
            .join("|"),
        outcome.read_surface().as_str().to_string(),
        outcome.inspection_surface().as_str().to_string(),
        outcome.fact_provenance().as_str().to_string(),
        outcome.outcome_digest().to_string(),
        true.to_string(),
    ]))
}
