use topology::facade::{
    prepare_primitive_construction_query_receipt, TopologyPrimitiveConstructionBirthFamily,
    TopologyPrimitiveConstructionQueryBirthSynopsis,
};
use worth_primitives::{PrimitiveConstructionFamilyContractRegistry, PrimitiveWitnessDescriptor};

use super::super::UnsupportedReplayWorkload;
use super::{canonical_retained_replay_error, MOVEMENT, NEIGHBORHOOD, TOPOLOGY};
use crate::facade::planar_contract_bundle::{
    PlanarBooleanReadinessBundle, PlanarContractBundleValidationContracts,
    PlanarContractBundleValidationReceipt, PlanarContractBundleValidator,
};
use crate::facade::planar_contracts::{
    admit_planar_contract_family, PlanarAdmissionFamily, PlanarAdmissionReceipt,
    PlanarRuntimeConcern,
};
use crate::facade::planar_local_frame::PlanarLocalFrameCertificateReceipt;
use crate::facade::planar_overlap::CoplanarOverlapContractReceipt;
use crate::facade::planar_precision::PlanarPrecisionCertificateReceipt;
use crate::facade::planar_predicate_consumption::PredicateCertificateConsumptionReceipt;
use crate::facade::planar_predicates::PlanarPredicateFactReceipt;
use crate::facade::planar_projection::ProjectPointToCertifiedPlane2DReceipt;
use crate::facade::planar_projection_consumption::{
    ProjectionConsumedPlanarFacts, ProjectionConsumedPlanarFactsContracts,
    ProjectionConsumedPlanarFactsReceipt,
};
use crate::facade::planar_retained_facts::{
    RetainedPlanarFacts, RetainedPlanarFactsContracts, RetainedPlanarFactsReceipt,
};
use crate::facade::planar_segment_segment::CertifiedSegmentSegment2DReceipt;
use crate::facade::planar_signed_area::CertifiedSignedArea2DReceipt;
use crate::facade::planar_topology_contract::{
    PlanarTopologyContractCompleteness, PlanarTopologyContractCompletenessContracts,
    PlanarTopologyContractCompletenessReceipt,
};
use crate::facade::planar_winding::CertifiedPolygonWinding2DReceipt;

pub(crate) struct CanonicalPlanarBundleParts {
    readiness: PlanarContractBundleValidationReceipt,
    topology_contract: PlanarTopologyContractCompletenessReceipt,
    projections: Vec<ProjectPointToCertifiedPlane2DReceipt>,
}

pub(crate) fn canonical_planar_bundle_parts(
    world: &'static str,
) -> Result<CanonicalPlanarBundleParts, UnsupportedReplayWorkload> {
    let admission = admission_receipt()?;
    let topology_contract = topology_contract_receipt(world)?;
    let predicate = predicate_receipt()?;
    let precision = precision_receipt(world, &predicate)?;
    let frame = frame_receipt(world, &precision)?;
    let projections = projected_face_pair(world, &frame)?;
    let left_winding = winding_receipt(world, "face:left", projections[0..4].to_vec())?;
    let right_winding = winding_receipt(world, "face:right", projections[4..8].to_vec())?;
    let signed_area = signed_area_receipt(world, left_winding.clone(), precision.clone())?;
    let right_area = signed_area_receipt(world, right_winding, precision.clone())?;
    let overlap = overlap_receipt(world, signed_area.clone(), right_area)?;
    let segment = segment_receipt(
        world,
        projections[1].clone(),
        projections[2].clone(),
        projections[7].clone(),
        projections[4].clone(),
    )?;
    let segment_predicates = segment_orientation_predicates(&segment)?;
    let predicate_consumption =
        predicate_consumption_receipt(world, segment.clone(), segment_predicates.clone())?;
    let mut predicates = vec![predicate];
    predicates.extend(segment_predicates);
    let readiness = certify_boolean_readiness(
        world,
        admission,
        topology_contract.clone(),
        precision,
        frame,
        projections.clone(),
        predicates,
        vec![segment],
        left_winding,
        signed_area,
        overlap,
        predicate_consumption,
    )?;
    Ok(CanonicalPlanarBundleParts {
        readiness,
        topology_contract,
        projections,
    })
}

pub(crate) fn retained_canonical_planar_facts(
    world: &'static str,
    parts: &CanonicalPlanarBundleParts,
) -> Result<RetainedPlanarFactsReceipt, UnsupportedReplayWorkload> {
    let motion = motion_receipt(world, parts.readiness.clone())?;
    let structural = structural_receipt(world, parts.readiness.clone(), motion.clone())?;
    RetainedPlanarFacts::from_boolean_readiness(parts.readiness.clone())
        .retain_planar_classification()
        .retain_structural_identity(structural)
        .retain_motion_posture(motion)
        .retain_topology_contract(parts.topology_contract.clone())
        .compile(&RetainedPlanarFactsContracts::new(retained_planar_handle(
            world,
        )))
        .map_err(|_| canonical_retained_replay_error("Could not compile retained planar facts."))?
        .retain()
        .map_err(|_| canonical_retained_replay_error("Could not retain planar facts."))
}

pub(crate) fn projection_consumed_canonical_planar_facts(
    world: &'static str,
    parts: &CanonicalPlanarBundleParts,
    retained: &RetainedPlanarFactsReceipt,
) -> Result<ProjectionConsumedPlanarFactsReceipt, UnsupportedReplayWorkload> {
    ProjectionConsumedPlanarFacts::from_retained_planar_facts(retained.clone())
        .consume_bundle_projection_receipts(parts.projections.clone())
        .materialize_as(format!("materialization:canonical-retained:{world}"))
        .compile(&ProjectionConsumedPlanarFactsContracts::new(
            projection_consumption_handle(world),
        ))
        .map_err(|_| {
            canonical_retained_replay_error("Could not compile projection-consumed planar facts.")
        })?
        .consume()
        .map_err(|_| {
            canonical_retained_replay_error("Could not consume projection receipts for replay.")
        })
}

#[allow(clippy::too_many_arguments)]
fn certify_boolean_readiness(
    world: &'static str,
    admission: PlanarAdmissionReceipt,
    topology_contract: PlanarTopologyContractCompletenessReceipt,
    precision: PlanarPrecisionCertificateReceipt,
    frame: PlanarLocalFrameCertificateReceipt,
    projections: Vec<ProjectPointToCertifiedPlane2DReceipt>,
    predicates: Vec<PlanarPredicateFactReceipt>,
    segments: Vec<CertifiedSegmentSegment2DReceipt>,
    winding: CertifiedPolygonWinding2DReceipt,
    signed_area: CertifiedSignedArea2DReceipt,
    overlap: CoplanarOverlapContractReceipt,
    predicate_consumption: PredicateCertificateConsumptionReceipt,
) -> Result<PlanarContractBundleValidationReceipt, UnsupportedReplayWorkload> {
    let bundle = PlanarBooleanReadinessBundle::builder()
        .admission(admission)
        .topology_contract(topology_contract)
        .precision(precision)
        .local_frame(frame)
        .projection_consumption(projections)
        .predicate_authority(predicates)
        .segment_contacts(segments)
        .winding(winding)
        .signed_area(signed_area)
        .coplanar_overlap(overlap)
        .predicate_consumption(predicate_consumption)
        .topology_basis(TOPOLOGY)
        .movement_rotation_posture(MOVEMENT)
        .diagnostic_scope("diagnostics:canonical-retained")
        .build()
        .map_err(|_| canonical_retained_replay_error("Could not build readiness bundle."))?;
    PlanarContractBundleValidator::for_boolean_readiness(bundle)
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&PlanarContractBundleValidationContracts::new(
            bundle_handle(world),
        ))
        .map_err(|_| canonical_retained_replay_error("Could not compile readiness bundle."))?
        .certify()
        .map_err(|_| canonical_retained_replay_error("Could not certify readiness bundle."))
}

fn admission_receipt() -> Result<PlanarAdmissionReceipt, UnsupportedReplayWorkload> {
    admit_planar_contract_family(
        PlanarAdmissionFamily::PlanarContractBundle,
        PlanarRuntimeConcern::BooleanReadinessBundle,
    )
    .ok_or_else(|| canonical_retained_replay_error("Could not admit planar contract bundle."))
}

fn topology_contract_receipt(
    world: &'static str,
) -> Result<PlanarTopologyContractCompletenessReceipt, UnsupportedReplayWorkload> {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::ShellWithHole {
            outer_loop_edge_count: 4,
            hole_loop_edge_counts: vec![4],
        },
    );
    let synopsis = TopologyPrimitiveConstructionQueryBirthSynopsis::new(
        TopologyPrimitiveConstructionBirthFamily::ShellWithHole,
        contract,
        "topology:canonical-retained".to_string(),
        TOPOLOGY.to_string(),
        "planar_shell_with_hole_body".to_string(),
        8,
        8,
        2,
        0,
        1,
        1,
        1,
    );
    let topology_receipt = prepare_primitive_construction_query_receipt(&synopsis)
        .map_err(|_| canonical_retained_replay_error("Could not prepare topology receipt."))?;
    PlanarTopologyContractCompleteness::from_topology_query_receipt(topology_receipt)
        .consume_declared_topology_surfaces("topology.query.declared-surfaces:canonical-retained")
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&PlanarTopologyContractCompletenessContracts::new(
            topology_contract_handle(world),
        ))
        .map_err(|_| canonical_retained_replay_error("Could not compile topology contract."))?
        .certify()
        .map_err(|_| canonical_retained_replay_error("Could not certify topology contract."))
}

use super::planar_receipts::*;
use super::query_handles::*;
