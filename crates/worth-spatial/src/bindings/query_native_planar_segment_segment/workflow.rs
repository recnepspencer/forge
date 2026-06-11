use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
};

use crate::bindings::query_native_planar_predicate::PlanarPredicateAuthorityQueryDomain;
use crate::bindings::query_native_planar_segment_segment::authoring::{
    certified_segment_segment_2d_entry, CertifiedSegmentSegment2DCase,
    CertifiedSegmentSegment2DEntry,
};
use crate::bindings::query_native_planar_segment_segment::domain::CertifiedSegmentSegment2DQueryDomain;
use crate::bindings::query_native_planar_segment_segment::facts::{
    certified_segment_segment_2d_facts, CertifiedSegmentSegment2DFactError,
};
use crate::planar_contracts::projection_2d::ProjectPointToCertifiedPlane2DReceipt;
use crate::planar_contracts::segment_segment_2d::{
    CertifiedSegmentSegment2DBasis, CertifiedSegmentSegment2DDenial,
    CertifiedSegmentSegment2DDenialKind, CertifiedSegmentSegment2DReceipt,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SegmentContactPolicy {
    CertifyContactsDenyImprintRequired,
    RequireImprintForCollinearOverlap,
}

impl SegmentContactPolicy {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifyContactsDenyImprintRequired => "certify-contacts-deny-imprint-required",
            Self::RequireImprintForCollinearOverlap => "require-imprint-for-collinear-overlap",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedProjectedSegment2D {
    segment_identity: String,
    start: ProjectPointToCertifiedPlane2DReceipt,
    end: ProjectPointToCertifiedPlane2DReceipt,
}

impl CertifiedProjectedSegment2D {
    pub fn from_projected_endpoints(
        segment_identity: impl Into<String>,
        start: ProjectPointToCertifiedPlane2DReceipt,
        end: ProjectPointToCertifiedPlane2DReceipt,
    ) -> Result<Self, CertifiedSegmentSegment2DDenial> {
        let segment = Self {
            segment_identity: segment_identity.into(),
            start,
            end,
        };
        if segment.segment_identity.is_empty() {
            return Err(CertifiedSegmentSegment2DDenial::new(
                CertifiedSegmentSegment2DDenialKind::MissingFirstSegmentIdentity,
                "certified projected segments require a stable segment identity",
            ));
        }
        Ok(segment)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CertifiedSegmentSegment2D {
    first: CertifiedProjectedSegment2D,
    second: CertifiedProjectedSegment2D,
    topology_basis_identity: String,
    policy: SegmentContactPolicy,
}

impl CertifiedSegmentSegment2D {
    pub fn classify(
        first: CertifiedProjectedSegment2D,
        second: CertifiedProjectedSegment2D,
    ) -> Self {
        Self {
            first,
            second,
            topology_basis_identity: String::new(),
            policy: SegmentContactPolicy::CertifyContactsDenyImprintRequired,
        }
    }

    pub fn within_topology_basis(mut self, identity: impl Into<String>) -> Self {
        self.topology_basis_identity = identity.into();
        self
    }

    pub fn with_policy(mut self, policy: SegmentContactPolicy) -> Self {
        self.policy = policy;
        self
    }

    pub fn compile<'a, SC, PC>(
        self,
        contracts: &'a CertifiedSegmentSegment2DContracts<SC, PC>,
    ) -> Result<CertifiedSegmentSegment2DPlan<'a, SC, PC>, CertifiedSegmentSegment2DDenial>
    where
        SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
        PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
    {
        let basis = CertifiedSegmentSegment2DBasis::builder()
            .first_segment_identity(self.first.segment_identity)
            .second_segment_identity(self.second.segment_identity)
            .topology_basis_identity(self.topology_basis_identity)
            .contact_policy_identity(self.policy.as_str())
            .first_segment_endpoints(&self.first.start, &self.first.end)
            .second_segment_endpoints(&self.second.start, &self.second.end)
            .build()?;
        let entry = certified_segment_segment_2d_entry(
            CertifiedSegmentSegment2DCase::from_projected_segments(basis),
        );
        Ok(CertifiedSegmentSegment2DPlan {
            entry,
            contracts,
            policy: self.policy,
        })
    }
}

pub struct CertifiedSegmentSegment2DContracts<SC, PC>
where
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
{
    segment_handle:
        ForgeQueryAdmittedConfiguredDomainHandle<CertifiedSegmentSegment2DQueryDomain, SC>,
    predicate_handle:
        ForgeQueryAdmittedConfiguredDomainHandle<PlanarPredicateAuthorityQueryDomain, PC>,
}

impl<SC, PC> CertifiedSegmentSegment2DContracts<SC, PC>
where
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
{
    pub fn new(
        segment_handle: ForgeQueryAdmittedConfiguredDomainHandle<
            CertifiedSegmentSegment2DQueryDomain,
            SC,
        >,
        predicate_handle: ForgeQueryAdmittedConfiguredDomainHandle<
            PlanarPredicateAuthorityQueryDomain,
            PC,
        >,
    ) -> Self {
        Self {
            segment_handle,
            predicate_handle,
        }
    }
}

pub struct CertifiedSegmentSegment2DPlan<'a, SC, PC>
where
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
{
    entry: CertifiedSegmentSegment2DEntry,
    contracts: &'a CertifiedSegmentSegment2DContracts<SC, PC>,
    policy: SegmentContactPolicy,
}

impl<SC, PC> CertifiedSegmentSegment2DPlan<'_, SC, PC>
where
    SC: ForgeQueryDomainOperatingContext<CertifiedSegmentSegment2DQueryDomain>,
    PC: ForgeQueryDomainOperatingContext<PlanarPredicateAuthorityQueryDomain>,
{
    pub fn required_predicate_count(&self) -> usize {
        4
    }

    pub fn projection_receipt_count(&self) -> usize {
        4
    }

    pub fn policy(&self) -> SegmentContactPolicy {
        self.policy
    }

    pub fn certify(
        self,
    ) -> Result<CertifiedSegmentSegment2DReceipt, CertifiedSegmentSegment2DFactError> {
        certified_segment_segment_2d_facts(
            &self.entry,
            &self.contracts.segment_handle,
            &self.contracts.predicate_handle,
        )
    }
}
