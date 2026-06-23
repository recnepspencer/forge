use crate::workload_platform::planar_boolean_common_plane::{
    PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt,
    PlanarBooleanCommonPlaneOperandSide,
};
use crate::workload_platform::projection_workload::{ProjectedLoop, ProjectedPlanarWorkload};

use super::carrier::{
    PlanarBooleanLoopRole, PlanarBooleanSegmentCarrier, PlanarBooleanSegmentCarrierInput,
};
use super::denial::{
    PlanarBooleanSegmentCarrierSetDenial, PlanarBooleanSegmentCarrierSetDenialKind,
};
use super::endpoint_facts::PlanarBooleanSegmentCarrierEndpointFacts;
use super::identity::segment_carrier_set_identity;
use super::source_validation::{
    validate_cross_operand_context, validate_operand_projection_source,
    validate_operand_source_slot, validate_precision_basis_identity,
};
use crate::workload_platform::planar_boolean_events::segment_identity::{
    PlanarBooleanCanonicalSegmentSet, PlanarBooleanCanonicalSegmentSetDenial,
};

pub struct PlanarBooleanSegmentCarrierOperandSource<'a> {
    pub(super) operand_side: PlanarBooleanCommonPlaneOperandSide,
    pub(super) projected_workload: &'a ProjectedPlanarWorkload,
    pub(super) projection_receipt: &'a PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt,
    pub(super) precision_basis_identity: &'a str,
}

impl<'a> PlanarBooleanSegmentCarrierOperandSource<'a> {
    pub fn new(
        operand_side: PlanarBooleanCommonPlaneOperandSide,
        projected_workload: &'a ProjectedPlanarWorkload,
        projection_receipt: &'a PlanarBooleanCommonPlaneOperandProjectionConsumptionReceipt,
        precision_basis_identity: &'a str,
    ) -> Self {
        Self {
            operand_side,
            projected_workload,
            projection_receipt,
            precision_basis_identity,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanSegmentCarrierSet {
    left: Vec<PlanarBooleanSegmentCarrier>,
    right: Vec<PlanarBooleanSegmentCarrier>,
    segment_carrier_set_identity: String,
}

impl PlanarBooleanSegmentCarrierSet {
    pub fn from_projected_operands(
        left: PlanarBooleanSegmentCarrierOperandSource<'_>,
        right: PlanarBooleanSegmentCarrierOperandSource<'_>,
    ) -> Result<Self, PlanarBooleanSegmentCarrierSetDenial> {
        validate_operand_source_slot(&left, PlanarBooleanCommonPlaneOperandSide::Left)?;
        validate_operand_source_slot(&right, PlanarBooleanCommonPlaneOperandSide::Right)?;
        validate_precision_basis_identity(&left)?;
        validate_precision_basis_identity(&right)?;
        validate_cross_operand_context(&left, &right)?;

        let left = extract_operand_carriers(&left)?;
        let right = extract_operand_carriers(&right)?;
        Ok(Self::from_carriers(left, right))
    }

    pub fn left(&self) -> &[PlanarBooleanSegmentCarrier] {
        &self.left
    }

    pub fn right(&self) -> &[PlanarBooleanSegmentCarrier] {
        &self.right
    }

    pub fn total_carrier_count(&self) -> usize {
        self.left.len() + self.right.len()
    }

    pub fn segment_carrier_set_identity(&self) -> &str {
        &self.segment_carrier_set_identity
    }

    pub fn canonical_segment_set(
        &self,
    ) -> Result<PlanarBooleanCanonicalSegmentSet, PlanarBooleanCanonicalSegmentSetDenial> {
        PlanarBooleanCanonicalSegmentSet::from_carrier_set(self)
    }
}

impl PlanarBooleanSegmentCarrierSet {
    fn from_carriers(
        left: Vec<PlanarBooleanSegmentCarrier>,
        right: Vec<PlanarBooleanSegmentCarrier>,
    ) -> Self {
        let segment_carrier_set_identity = segment_carrier_set_identity(&left, &right);
        Self {
            left,
            right,
            segment_carrier_set_identity,
        }
    }
}

fn extract_operand_carriers(
    source: &PlanarBooleanSegmentCarrierOperandSource<'_>,
) -> Result<Vec<PlanarBooleanSegmentCarrier>, PlanarBooleanSegmentCarrierSetDenial> {
    validate_operand_projection_source(source)?;

    let mut carriers = Vec::new();
    for projected_loop in source.projected_workload.projected_loops() {
        carriers.extend(extract_loop_carriers(source, projected_loop)?);
    }
    if carriers.is_empty() {
        return Err(PlanarBooleanSegmentCarrierSetDenial::new(
            PlanarBooleanSegmentCarrierSetDenialKind::MissingBoundarySegments,
            "planar boolean segment carrier extraction requires boundary-backed projected loop segments",
        ));
    }
    Ok(carriers)
}

fn extract_loop_carriers(
    source: &PlanarBooleanSegmentCarrierOperandSource<'_>,
    projected_loop: &ProjectedLoop,
) -> Result<Vec<PlanarBooleanSegmentCarrier>, PlanarBooleanSegmentCarrierSetDenial> {
    let boundary = projected_loop.boundary().ok_or_else(|| {
        PlanarBooleanSegmentCarrierSetDenial::new(
            PlanarBooleanSegmentCarrierSetDenialKind::MissingBoundaryLoop,
            "segment carriers require projected loops to preserve source boundary geometry",
        )
    })?;
    if boundary.owning_face_identity().is_empty() {
        return Err(PlanarBooleanSegmentCarrierSetDenial::new(
            PlanarBooleanSegmentCarrierSetDenialKind::MissingSourceFaceIdentity,
            "segment carriers require source face topology provenance",
        ));
    }
    let source_loop_identity = projected_loop.identity().topology_entity_identity();
    if source_loop_identity.is_empty() {
        return Err(PlanarBooleanSegmentCarrierSetDenial::new(
            PlanarBooleanSegmentCarrierSetDenialKind::MissingSourceLoopIdentity,
            "segment carriers require source loop topology provenance",
        ));
    }
    if boundary.outer_segments().is_empty() {
        return Err(PlanarBooleanSegmentCarrierSetDenial::new(
            PlanarBooleanSegmentCarrierSetDenialKind::MissingBoundarySegments,
            "segment carriers require source edge provenance for each projected loop segment",
        ));
    }

    boundary
        .outer_segments()
        .iter()
        .enumerate()
        .map(|(segment_index, segment)| {
            if segment.source_edge_identity().is_empty() {
                return Err(PlanarBooleanSegmentCarrierSetDenial::new(
                    PlanarBooleanSegmentCarrierSetDenialKind::MissingSourceEdgeIdentity,
                    "segment carriers require source edge topology provenance",
                ));
            }
            let start = endpoint_facts(
                source,
                projected_loop,
                segment_index,
                "start",
                segment.start_point(),
            )?;
            let end = endpoint_facts(
                source,
                projected_loop,
                segment_index,
                "end",
                segment.end_point(),
            )?;
            Ok(PlanarBooleanSegmentCarrier::new(
                PlanarBooleanSegmentCarrierInput {
                    operand_side: source.operand_side,
                    source_face_identity: boundary.owning_face_identity().to_string(),
                    source_loop_identity: source_loop_identity.to_string(),
                    source_edge_identity: segment.source_edge_identity().to_string(),
                    loop_role: PlanarBooleanLoopRole::OuterBoundary,
                    start,
                    end,
                    local_frame_identity: source
                        .projection_receipt
                        .local_frame_selection_identity()
                        .to_string(),
                    projection_stage_identity: source
                        .projection_receipt
                        .projection_stage_identity()
                        .to_string(),
                    precision_basis_identity: source.precision_basis_identity.to_string(),
                },
            ))
        })
        .collect()
}

fn endpoint_facts(
    source: &PlanarBooleanSegmentCarrierOperandSource<'_>,
    projected_loop: &ProjectedLoop,
    segment_index: usize,
    endpoint_role: &str,
    point: [f64; 2],
) -> Result<PlanarBooleanSegmentCarrierEndpointFacts, PlanarBooleanSegmentCarrierSetDenial> {
    let projected_loop_identity = projected_loop.identity().projected_fact_identity();
    if projected_loop_identity.is_empty() {
        return Err(PlanarBooleanSegmentCarrierSetDenial::new(
            PlanarBooleanSegmentCarrierSetDenialKind::MissingProjectedEndpointProof,
            "segment carriers require projected endpoint proof anchored to a projected loop fact",
        ));
    }
    Ok(
        PlanarBooleanSegmentCarrierEndpointFacts::from_projected_loop_boundary(
            point,
            format!(
                "{}:{}:{segment_index}:{endpoint_role}",
                source.operand_side.query_key(),
                projected_loop.identity().topology_entity_identity()
            ),
            projected_loop_identity,
            source.projection_receipt.projection_stage_identity(),
            source.projection_receipt.projection_local_basis_identity(),
        ),
    )
}
