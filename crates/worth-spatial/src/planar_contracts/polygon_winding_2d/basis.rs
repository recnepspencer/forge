use crate::planar_contracts::predicate_authority::PlanarPredicateFactReceipt;
use crate::planar_contracts::projection_2d::ProjectPointToCertifiedPlane2DReceipt;
use crate::planar_contracts::segment_segment_2d::CertifiedSegmentSegment2DReceipt;
use worth_math::sign::TriSign;

use super::containment::CertifiedLoopContainment;
use super::loop_basis::CertifiedTopologyLoopBasis2D;
use super::validation::validate_polygon_winding_basis;
use super::winding::CertifiedLoopWinding;
use super::{CertifiedPolygonWinding2DDenial, CertifiedPolygonWinding2DDenialKind};

#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedPolygonWinding2DBasis {
    primary_loop_identity: String,
    planar_neighborhood_identity: String,
    winding_policy_identity: String,
    loops: Vec<CertifiedLoopWindingSummary>,
    winding_predicate_fact_digests: Vec<String>,
    winding_predicate_signs: Vec<TriSign>,
    segment_contact_fact_digests: Vec<String>,
}

impl CertifiedPolygonWinding2DBasis {
    pub(crate) fn new(
        primary_loop_identity: String,
        planar_neighborhood_identity: String,
        winding_policy_identity: String,
        loops: Vec<CertifiedLoopWindingSummary>,
    ) -> Result<Self, CertifiedPolygonWinding2DDenial> {
        let basis = Self {
            primary_loop_identity,
            planar_neighborhood_identity,
            winding_policy_identity,
            loops,
            winding_predicate_fact_digests: Vec::new(),
            winding_predicate_signs: Vec::new(),
            segment_contact_fact_digests: Vec::new(),
        };
        validate_polygon_winding_basis(&basis)?;
        Ok(basis)
    }

    pub(crate) fn with_certification_evidence(
        mut self,
        predicate_receipts: Vec<PlanarPredicateFactReceipt>,
        segment_receipts: Vec<CertifiedSegmentSegment2DReceipt>,
        containments: Vec<(String, CertifiedLoopContainment)>,
    ) -> Result<Self, CertifiedPolygonWinding2DDenial> {
        self.winding_predicate_fact_digests = predicate_receipts
            .iter()
            .map(|receipt| receipt.fact_digest().to_string())
            .collect();
        self.winding_predicate_fact_digests.sort();
        self.winding_predicate_signs = predicate_receipts
            .iter()
            .map(|receipt| receipt.certified_sign().sign())
            .collect();
        self.segment_contact_fact_digests = segment_receipts
            .iter()
            .map(|receipt| receipt.fact_digest().to_string())
            .collect();
        self.segment_contact_fact_digests.sort();
        let mut predicate_offset = 0;
        for loop_summary in &mut self.loops {
            let predicate_width = loop_summary.vertices().len() - 2;
            let winding = winding_from_predicate_signs(
                &self.winding_predicate_signs[predicate_offset..predicate_offset + predicate_width],
            )?;
            predicate_offset += predicate_width;
            loop_summary.winding = winding;
            if loop_summary.loop_identity != self.primary_loop_identity {
                if let Some((_, containment)) = containments
                    .iter()
                    .find(|(identity, _)| identity == &loop_summary.loop_identity)
                {
                    loop_summary.containment = Some(*containment);
                }
            }
        }
        validate_polygon_winding_basis(&self)?;
        Ok(self)
    }

    pub fn primary_loop_identity(&self) -> &str {
        &self.primary_loop_identity
    }

    pub fn planar_neighborhood_identity(&self) -> &str {
        &self.planar_neighborhood_identity
    }

    pub fn winding_policy_identity(&self) -> &str {
        &self.winding_policy_identity
    }

    pub fn primary_winding(&self) -> CertifiedLoopWinding {
        self.loops[0].winding
    }

    pub fn containment_for(&self, loop_identity: &str) -> Option<CertifiedLoopContainment> {
        self.loops
            .iter()
            .find(|loop_summary| loop_summary.loop_identity() == loop_identity)
            .and_then(|loop_summary| loop_summary.containment)
    }

    pub(crate) fn loop_summaries(&self) -> &[CertifiedLoopWindingSummary] {
        &self.loops
    }

    pub(crate) fn vertices(&self) -> Vec<&ProjectedLoopVertexSnapshot> {
        self.loops
            .iter()
            .flat_map(|loop_summary| loop_summary.vertices.iter())
            .collect()
    }

    pub(crate) fn first_vertex(&self) -> Option<&ProjectedLoopVertexSnapshot> {
        self.loops
            .first()
            .and_then(|loop_summary| loop_summary.vertices.first())
    }

    pub fn frame_identity(&self) -> &str {
        &self.first_vertex().expect("validated basis").frame_identity
    }

    pub fn local_frame_fact_digest(&self) -> &str {
        &self
            .first_vertex()
            .expect("validated basis")
            .local_frame_fact_digest
    }

    pub fn movement_rotation_posture_identity(&self) -> &str {
        &self
            .first_vertex()
            .expect("validated basis")
            .movement_rotation_posture_identity
    }

    pub fn tolerance_policy_identity(&self) -> &str {
        &self
            .first_vertex()
            .expect("validated basis")
            .tolerance_policy_identity
    }

    pub(crate) fn projected_vertex_fact_digests(&self) -> Vec<&str> {
        let mut digests = self
            .vertices()
            .iter()
            .map(|vertex| vertex.projection_fact_digest.as_str())
            .collect::<Vec<_>>();
        digests.sort();
        digests
    }

    pub(crate) fn winding_predicate_fact_digests(&self) -> &[String] {
        &self.winding_predicate_fact_digests
    }

    pub(crate) fn segment_contact_fact_digests(&self) -> &[String] {
        &self.segment_contact_fact_digests
    }

    pub(crate) fn loop_edges_walked(&self) -> usize {
        self.loops
            .iter()
            .map(|loop_summary| loop_summary.vertices().len())
            .sum()
    }

    pub(crate) fn winding_tie_breaks_used(&self) -> usize {
        0
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct CertifiedLoopWindingSummary {
    loop_identity: String,
    topology_basis: CertifiedTopologyLoopBasis2D,
    vertices: Vec<ProjectedLoopVertexSnapshot>,
    winding: CertifiedLoopWinding,
    containment: Option<CertifiedLoopContainment>,
}

impl CertifiedLoopWindingSummary {
    pub(crate) fn new(
        loop_identity: String,
        topology_basis: CertifiedTopologyLoopBasis2D,
        vertices: Vec<ProjectedLoopVertexSnapshot>,
    ) -> Self {
        Self {
            loop_identity,
            topology_basis,
            vertices,
            winding: CertifiedLoopWinding::CounterClockwise,
            containment: None,
        }
    }

    pub(crate) fn loop_identity(&self) -> &str {
        &self.loop_identity
    }

    pub(crate) fn topology_loop_identity(&self) -> &str {
        self.topology_basis.loop_topology_identity()
    }

    pub(crate) fn loop_membership_fact_digest(&self) -> &str {
        self.topology_basis.loop_membership_fact_digest()
    }

    pub(crate) fn topology_to_spatial_contract_digest(&self) -> &str {
        self.topology_basis.topology_to_spatial_contract_digest()
    }

    pub(crate) fn vertices(&self) -> &[ProjectedLoopVertexSnapshot] {
        &self.vertices
    }

    pub(crate) fn canonical_vertices(&self) -> Vec<&ProjectedLoopVertexSnapshot> {
        let start = self
            .vertices
            .iter()
            .enumerate()
            .min_by_key(|(_, vertex)| vertex.projection_fact_digest.as_str())
            .map(|(index, _)| index)
            .unwrap_or(0);
        self.vertices
            .iter()
            .cycle()
            .skip(start)
            .take(self.vertices.len())
            .collect()
    }

    pub(crate) fn winding(&self) -> CertifiedLoopWinding {
        self.winding
    }

    pub(crate) fn containment_identity(&self) -> &'static str {
        self.containment
            .map(|containment| containment.as_str())
            .unwrap_or("primary-loop")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ProjectedLoopVertexSnapshot {
    pub(crate) receipt: ProjectPointToCertifiedPlane2DReceipt,
    pub(crate) source_point_identity: String,
    pub(crate) point_2d: [f64; 2],
    pub(crate) projection_fact_digest: String,
    pub(crate) local_frame_fact_digest: String,
    pub(crate) local_frame_declaration_digest: String,
    pub(crate) local_frame_envelope_digest: String,
    pub(crate) frame_identity: String,
    pub(crate) transform_chain_digest: String,
    pub(crate) movement_rotation_posture_identity: String,
    pub(crate) tolerance_policy_identity: String,
}

impl ProjectedLoopVertexSnapshot {
    pub(crate) fn from_receipt(receipt: &ProjectPointToCertifiedPlane2DReceipt) -> Self {
        Self {
            receipt: receipt.clone(),
            source_point_identity: receipt.source_point_identity().to_string(),
            point_2d: receipt.point_2d(),
            projection_fact_digest: receipt.fact_digest().to_string(),
            local_frame_fact_digest: receipt.local_frame_fact_digest().to_string(),
            local_frame_declaration_digest: receipt
                .basis()
                .local_frame_declaration_digest()
                .to_string(),
            local_frame_envelope_digest: receipt.basis().local_frame_envelope_digest().to_string(),
            frame_identity: receipt.basis().frame_identity().to_string(),
            transform_chain_digest: receipt.basis().transform_chain_digest().to_string(),
            movement_rotation_posture_identity: receipt
                .basis()
                .movement_rotation_posture_identity()
                .to_string(),
            tolerance_policy_identity: receipt.basis().tolerance_policy_identity().to_string(),
        }
    }
}

fn winding_from_predicate_signs(
    signs: &[TriSign],
) -> Result<CertifiedLoopWinding, CertifiedPolygonWinding2DDenial> {
    let has_positive = signs.iter().any(|sign| sign.is_positive());
    let has_negative = signs.iter().any(|sign| sign.is_negative());
    let has_zero = signs.iter().any(|sign| sign.is_zero());
    if has_positive && !has_negative && !has_zero {
        Ok(CertifiedLoopWinding::CounterClockwise)
    } else if has_negative && !has_positive && !has_zero {
        Ok(CertifiedLoopWinding::Clockwise)
    } else {
        Err(CertifiedPolygonWinding2DDenial::new(
            CertifiedPolygonWinding2DDenialKind::AmbiguousWindingPredicateEvidence,
            "mixed or zero winding predicate evidence requires a narrower loop contract",
        ))
    }
}
