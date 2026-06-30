use crate::workload_platform::evidence_lookup_family_catalog::EvidenceLookupFamilyCatalogCloseout;

use super::admitted_input::{EvidenceLookupAdmittedInput, EvidenceLookupAdmittedInputParts};
use super::counters::EvidenceLookupInputAdmissionCounters;
use super::error::{EvidenceLookupInputAdmissionError, EvidenceLookupInputAdmissionErrorKind};
use super::query_support::EvidenceLookupQueryAdmissionSupport;
use super::request::EvidenceLookupInputAdmissionRequest;
use super::topology_support::EvidenceLookupTopologyAdmissionSupport;

pub fn admit_evidence_lookup_input(
    catalog: &EvidenceLookupFamilyCatalogCloseout,
    request: EvidenceLookupInputAdmissionRequest<'_>,
) -> Result<EvidenceLookupAdmittedInput, EvidenceLookupInputAdmissionError> {
    let Some(stage) = request.stage() else {
        return Err(EvidenceLookupInputAdmissionError::new(
            EvidenceLookupInputAdmissionErrorKind::MissingStageReceiptIdentity,
            "lookup input admission requires explicit stage and receipt family identity",
        )
        .with_counters(EvidenceLookupInputAdmissionCounters::default()));
    };
    let Some(receipt_family) = request.receipt_family() else {
        return Err(EvidenceLookupInputAdmissionError::new(
            EvidenceLookupInputAdmissionErrorKind::MissingStageReceiptIdentity,
            "lookup input admission requires explicit stage and receipt family identity",
        )
        .with_counters(EvidenceLookupInputAdmissionCounters::default()));
    };
    if request.spatial_touch_authority().evidence_stage() != stage {
        return Err(EvidenceLookupInputAdmissionError::new(
            EvidenceLookupInputAdmissionErrorKind::SpatialTouchStageMismatch,
            format!(
                "spatial touch authority stage {:?} cannot admit lookup stage {:?}",
                request.spatial_touch_authority().evidence_stage(),
                stage
            ),
        )
        .with_counters(EvidenceLookupInputAdmissionCounters::default()));
    }
    if request.stage_receipt_spatial_touch_digest()
        != Some(request.spatial_touch_authority().digest().as_str())
    {
        return Err(EvidenceLookupInputAdmissionError::new(
            EvidenceLookupInputAdmissionErrorKind::StageReceiptAuthorityMismatch,
            "stage receipt identity was not admitted from the supplied spatial touch authority",
        )
        .with_counters(EvidenceLookupInputAdmissionCounters::default()));
    }

    let family_selection = catalog.families_for_stage(stage, &receipt_family);
    if family_selection.family_count() == 0 {
        return Err(EvidenceLookupInputAdmissionError::new(
            EvidenceLookupInputAdmissionErrorKind::NoFamilyForStageReceiptIdentity,
            format!(
                "no evidence lookup family matches stage {:?} and receipt family {}",
                stage,
                receipt_family.as_str()
            ),
        )
        .with_counters(EvidenceLookupInputAdmissionCounters::from_selection(
            family_selection.counters(),
        )));
    }

    let mut counters =
        EvidenceLookupInputAdmissionCounters::from_selection(family_selection.counters());
    let mut topology_support = Vec::new();
    let mut query_support = Vec::new();

    for family_identity in family_selection.family_identities() {
        let Some(family) = catalog.family_by_identity(family_identity) else {
            continue;
        };
        let topology = if family.topology_input_posture().requires_topology_receipt() {
            counters.count_topology_required();
            let support = EvidenceLookupTopologyAdmissionSupport::from_required_posture(
                family_identity,
                family.topology_input_posture(),
                request.topology_seed(),
            )
            .map_err(|error| error.with_counters(counters))?;
            counters.count_topology_satisfied();
            support
        } else {
            EvidenceLookupTopologyAdmissionSupport::not_required(family_identity)
        };
        topology_support.push(topology);

        let query = if family.query_posture().requires_query_evidence() {
            counters.count_query_required();
            let support = EvidenceLookupQueryAdmissionSupport::from_catalog_posture(
                family_identity,
                family.query_posture(),
                request.query_evidence(),
            )
            .map_err(|error| error.with_counters(counters))?;
            counters.count_query_satisfied();
            support
        } else {
            EvidenceLookupQueryAdmissionSupport::not_required(family_identity)
        };
        query_support.push(query);
    }

    Ok(EvidenceLookupAdmittedInput::from_parts(
        EvidenceLookupAdmittedInputParts {
            catalog_digest: catalog.catalog_digest().to_string(),
            spatial_touch_digest: request
                .spatial_touch_authority()
                .digest()
                .as_str()
                .to_string(),
            stage_receipt_digest: request
                .stage_receipt_digest()
                .unwrap_or_else(|| request.spatial_touch_authority().evidence_identity())
                .to_string(),
            stage,
            receipt_family,
            family_selection,
            topology_support,
            query_support,
            counters,
        },
    ))
}
