use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
};

use crate::bindings::query_native_planar_predicate::PlanarPredicateAuthorityQueryDomain;
use crate::bindings::query_native_planar_segment_segment::{
    CertifiedSegmentSegment2DContracts, CertifiedSegmentSegment2DQueryDomain,
};
use crate::bindings::query_native_planar_winding::authoring::{
    certified_polygon_winding_2d_entry, CertifiedPolygonWinding2DCase,
    CertifiedPolygonWinding2DEntry,
};
use crate::bindings::query_native_planar_winding::domain::CertifiedPolygonWinding2DQueryDomain;
use crate::bindings::query_native_planar_winding::facts::{
    certified_polygon_winding_2d_facts, CertifiedPolygonWinding2DFactError,
};
use crate::planar_contracts::polygon_winding_2d::{
    CertifiedLoopWindingSummary, CertifiedPolygonWinding2DBasis, CertifiedPolygonWinding2DDenial,
    CertifiedPolygonWinding2DDenialKind, CertifiedPolygonWinding2DReceipt,
    CertifiedTopologyLoopBasis2D, ProjectedLoopVertexSnapshot,
};
use crate::planar_contracts::projection_2d::ProjectPointToCertifiedPlane2DReceipt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindingPolicy {
    DenySelfIntersectionAndAmbiguousTouch,
}

impl WindingPolicy {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DenySelfIntersectionAndAmbiguousTouch => {
                "deny-self-intersection-and-ambiguous-touch"
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedProjectedLoop2D {
    loop_identity: String,
    topology_basis: CertifiedTopologyLoopBasis2D,
    vertices: Vec<ProjectPointToCertifiedPlane2DReceipt>,
}

impl CertifiedProjectedLoop2D {
    pub fn from_projected_vertices<I>(
        loop_identity: impl Into<String>,
        topology_basis: CertifiedTopologyLoopBasis2D,
        vertices: I,
    ) -> Result<Self, CertifiedPolygonWinding2DDenial>
    where
        I: IntoIterator<Item = ProjectPointToCertifiedPlane2DReceipt>,
    {
        let loop_identity = loop_identity.into();
        if loop_identity.is_empty() {
            return Err(CertifiedPolygonWinding2DDenial::new(
                CertifiedPolygonWinding2DDenialKind::MissingPrimaryLoopIdentity,
                "certified projected loops require stable loop identity",
            ));
        }
        Ok(Self {
            loop_identity,
            topology_basis,
            vertices: vertices.into_iter().collect(),
        })
    }

    fn into_summary(self) -> CertifiedLoopWindingSummary {
        let vertices = self
            .vertices
            .iter()
            .map(ProjectedLoopVertexSnapshot::from_receipt)
            .collect();
        CertifiedLoopWindingSummary::new(self.loop_identity, self.topology_basis, vertices)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedPolygonWinding2D {
    primary_loop: CertifiedProjectedLoop2D,
    containment_candidates: Vec<CertifiedProjectedLoop2D>,
    planar_neighborhood_identity: String,
    policy: WindingPolicy,
}

impl CertifiedPolygonWinding2D {
    pub fn certify(primary_loop: CertifiedProjectedLoop2D) -> Self {
        Self {
            primary_loop,
            containment_candidates: Vec::new(),
            planar_neighborhood_identity: String::new(),
            policy: WindingPolicy::DenySelfIntersectionAndAmbiguousTouch,
        }
    }

    pub fn with_containment_candidate(mut self, candidate: CertifiedProjectedLoop2D) -> Self {
        self.containment_candidates.push(candidate);
        self
    }

    pub fn within_planar_neighborhood(mut self, identity: impl Into<String>) -> Self {
        self.planar_neighborhood_identity = identity.into();
        self
    }

    pub fn with_policy(mut self, policy: WindingPolicy) -> Self {
        self.policy = policy;
        self
    }

    pub fn compile<'a, WC, SC, PC>(
        self,
        contracts: &'a CertifiedPolygonWinding2DContracts<WC, SC, PC>,
    ) -> Result<CertifiedPolygonWinding2DPlan<'a, WC, SC, PC>, CertifiedPolygonWinding2DDenial>
    where
        WC: ForgeQueryDomainOperatingContext<CertifiedPolygonWinding2DQueryDomain>,
        SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
        PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
    {
        let primary_loop_identity = self.primary_loop.loop_identity.clone();
        let mut loops = vec![self.primary_loop.into_summary()];
        loops.extend(
            self.containment_candidates
                .into_iter()
                .map(CertifiedProjectedLoop2D::into_summary),
        );
        let basis = CertifiedPolygonWinding2DBasis::new(
            primary_loop_identity,
            self.planar_neighborhood_identity,
            self.policy.as_str().to_string(),
            loops,
        )?;
        let entry = certified_polygon_winding_2d_entry(
            CertifiedPolygonWinding2DCase::from_projected_loops(basis),
        );
        Ok(CertifiedPolygonWinding2DPlan { entry, contracts })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedPolygonWinding2DContracts<WC, SC, PC>
where
    WC: ForgeQueryDomainOperatingContext<CertifiedPolygonWinding2DQueryDomain>,
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
{
    winding_handle:
        ForgeQueryAdmittedConfiguredDomainHandle<CertifiedPolygonWinding2DQueryDomain, WC>,
    segment_contracts: CertifiedSegmentSegment2DContracts<SC, PC>,
    predicate_handle:
        ForgeQueryAdmittedConfiguredDomainHandle<PlanarPredicateAuthorityQueryDomain, PC>,
}

impl<WC, SC, PC> CertifiedPolygonWinding2DContracts<WC, SC, PC>
where
    WC: ForgeQueryDomainOperatingContext<CertifiedPolygonWinding2DQueryDomain>,
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
{
    pub fn new(
        winding_handle: ForgeQueryAdmittedConfiguredDomainHandle<
            CertifiedPolygonWinding2DQueryDomain,
            WC,
        >,
        segment_contracts: CertifiedSegmentSegment2DContracts<SC, PC>,
        predicate_handle: ForgeQueryAdmittedConfiguredDomainHandle<
            PlanarPredicateAuthorityQueryDomain,
            PC,
        >,
    ) -> Self {
        Self {
            winding_handle,
            segment_contracts,
            predicate_handle,
        }
    }
}

pub struct CertifiedPolygonWinding2DPlan<'a, WC, SC, PC>
where
    WC: ForgeQueryDomainOperatingContext<CertifiedPolygonWinding2DQueryDomain>,
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
{
    entry: CertifiedPolygonWinding2DEntry,
    contracts: &'a CertifiedPolygonWinding2DContracts<WC, SC, PC>,
}

impl<WC, SC, PC> CertifiedPolygonWinding2DPlan<'_, WC, SC, PC>
where
    WC: ForgeQueryDomainOperatingContext<CertifiedPolygonWinding2DQueryDomain>,
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
{
    pub fn loop_count(&self) -> usize {
        self.entry.case().basis().loop_summaries().len()
    }

    pub fn projected_vertex_count(&self) -> usize {
        self.entry.case().basis().vertices().len()
    }

    pub fn segment_contact_pairs_required(&self) -> usize {
        let loops = self.entry.case().basis().loop_summaries();
        loops
            .iter()
            .skip(1)
            .map(|candidate| loops[0].vertices().len() * candidate.vertices().len())
            .sum()
    }

    pub fn certify(
        self,
    ) -> Result<CertifiedPolygonWinding2DReceipt, CertifiedPolygonWinding2DFactError> {
        certified_polygon_winding_2d_facts(
            &self.entry,
            &self.contracts.winding_handle,
            &self.contracts.segment_contracts,
            &self.contracts.predicate_handle,
        )
    }
}
