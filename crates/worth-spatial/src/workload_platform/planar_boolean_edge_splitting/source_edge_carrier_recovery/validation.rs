use crate::workload_platform::planar_boolean_events::PlanarBooleanSegmentCarrier;

use super::denial::{
    PlanarBooleanSplitSourceEdgeCarrierRecoveryDenial,
    PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind,
};
use super::input::PlanarBooleanSplitSourceEdgeCarrierRecoveryInput;

pub(crate) fn validate_recovery_input(
    input: &PlanarBooleanSplitSourceEdgeCarrierRecoveryInput<'_>,
) -> Result<(), PlanarBooleanSplitSourceEdgeCarrierRecoveryDenial> {
    let scope = input.scope_admission();
    let ledger = input.event_ledger();
    if scope.event_ledger_identity() != ledger.event_ledger_identity() {
        return Err(denial(
            PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind::ScopeLedgerIdentityMismatch,
            ledger.event_ledger_identity(),
            "split source-edge carrier recovery requires the scoped event ledger",
        ));
    }
    if scope.segment_carrier_set_identity() != ledger.segment_carrier_set_identity() {
        return Err(denial(
            PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind::ScopeCarrierSetIdentityMismatch,
            ledger.segment_carrier_set_identity(),
            "split source-edge carrier recovery requires the scoped carrier set identity",
        ));
    }
    if ledger.segment_carriers().is_empty() {
        return Err(denial(
            PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind::MissingCarrierRows,
            ledger.event_ledger_identity(),
            "split source-edge carrier recovery requires event-ledger carrier rows",
        ));
    }
    Ok(())
}

pub(crate) fn validate_carrier_provenance(
    carrier: &PlanarBooleanSegmentCarrier,
    event_ledger_identity: &str,
) -> Result<(), PlanarBooleanSplitSourceEdgeCarrierRecoveryDenial> {
    require_non_empty(
        carrier.carrier_identity(),
        PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind::MissingCarrierIdentity,
        event_ledger_identity,
        "split carriers require carrier identity",
    )?;
    require_non_empty(
        carrier.source_face_identity(),
        PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind::MissingSourceFaceIdentity,
        carrier.carrier_identity(),
        "split carriers require source face topology provenance",
    )?;
    require_non_empty(
        carrier.source_loop_identity(),
        PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind::MissingSourceLoopIdentity,
        carrier.carrier_identity(),
        "split carriers require source loop topology provenance",
    )?;
    require_non_empty(
        carrier.source_edge_identity(),
        PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind::MissingSourceEdgeIdentity,
        carrier.carrier_identity(),
        "split carriers require source edge topology provenance",
    )?;
    require_non_empty(
        carrier.local_frame_identity(),
        PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind::MissingLocalFrameIdentity,
        carrier.carrier_identity(),
        "split carriers require local-frame provenance",
    )?;
    require_non_empty(
        carrier.projection_stage_identity(),
        PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind::MissingProjectionStageIdentity,
        carrier.carrier_identity(),
        "split carriers require projection-stage provenance",
    )?;
    require_non_empty(
        carrier.precision_basis_identity(),
        PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind::MissingPrecisionBasisIdentity,
        carrier.carrier_identity(),
        "split carriers require precision-basis provenance",
    )?;
    validate_endpoint(
        carrier.start().source_endpoint_identity(),
        carrier.carrier_identity(),
    )?;
    validate_endpoint(
        carrier.end().source_endpoint_identity(),
        carrier.carrier_identity(),
    )?;
    validate_projected_endpoint(
        carrier.start().projected_endpoint_fact_identity(),
        carrier.carrier_identity(),
    )?;
    validate_projected_endpoint(
        carrier.end().projected_endpoint_fact_identity(),
        carrier.carrier_identity(),
    )
}

fn validate_endpoint(
    value: &str,
    carrier_identity: &str,
) -> Result<(), PlanarBooleanSplitSourceEdgeCarrierRecoveryDenial> {
    require_non_empty(
        value,
        PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind::MissingEndpointSourceIdentity,
        carrier_identity,
        "split carriers require endpoint source identity",
    )
}

fn validate_projected_endpoint(
    value: &str,
    carrier_identity: &str,
) -> Result<(), PlanarBooleanSplitSourceEdgeCarrierRecoveryDenial> {
    require_non_empty(
        value,
        PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind::MissingProjectedEndpointFactIdentity,
        carrier_identity,
        "split carriers require projected endpoint fact identity",
    )
}

fn require_non_empty(
    value: &str,
    kind: PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind,
    evidence_identity: &str,
    human_reason: &str,
) -> Result<(), PlanarBooleanSplitSourceEdgeCarrierRecoveryDenial> {
    if value.is_empty() {
        Err(denial(kind, evidence_identity, human_reason))
    } else {
        Ok(())
    }
}

pub(crate) fn denial(
    kind: PlanarBooleanSplitSourceEdgeCarrierRecoveryDenialKind,
    evidence_identity: impl Into<String>,
    human_reason: impl Into<String>,
) -> PlanarBooleanSplitSourceEdgeCarrierRecoveryDenial {
    PlanarBooleanSplitSourceEdgeCarrierRecoveryDenial::new(kind, evidence_identity, human_reason)
}
