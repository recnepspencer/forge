use super::denial::MixedSurfaceKillBoxDenial;
use crate::workload_platform::surface_support::{
    CertifiedSurfaceSupport, SurfaceFamily, UnsupportedSurfaceSupport,
    UnsupportedSurfaceSupportReasonCode,
};
use crate::workload_platform::user_response::{
    WorthUserOutcome, WorthUserResponseSource, WorthUserResponseWorkload,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MixedSurfaceFamilyRunStatus {
    AdmittedPlane,
    Unsupported,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MixedSurfaceFamilyRun {
    family: SurfaceFamily,
    status: MixedSurfaceFamilyRunStatus,
    support_evidence_digest: String,
    human_reason: String,
    unsupported_reason_code: Option<UnsupportedSurfaceSupportReasonCode>,
    user_outcome: WorthUserOutcome,
    upstream_geometry_binding_identity: String,
    upstream_geometry_carriers: usize,
}

impl MixedSurfaceFamilyRun {
    pub(crate) fn from_certified_plane(
        support: CertifiedSurfaceSupport,
        declaration: &str,
    ) -> Self {
        let support_evidence_digest = support
            .receipts()
            .stage_identity()
            .receipt_identity()
            .to_string();
        let upstream_geometry_binding_identity = support
            .receipts()
            .upstream_geometry_binding_identity()
            .to_string();
        let upstream_geometry_carriers = support.receipts().counters().upstream_geometry_carriers();
        let user_outcome = respond(
            WorthUserResponseSource::from_mixed_surface_plane_support(support.receipts()),
            declaration,
        );
        Self {
            family: SurfaceFamily::Plane,
            status: MixedSurfaceFamilyRunStatus::AdmittedPlane,
            support_evidence_digest,
            human_reason:
                "plane surface support is admitted and remains acceptable as pre-boolean input"
                    .to_string(),
            unsupported_reason_code: None,
            user_outcome,
            upstream_geometry_binding_identity,
            upstream_geometry_carriers,
        }
    }

    pub(crate) fn from_unsupported(
        unsupported: UnsupportedSurfaceSupport,
        declaration: &str,
    ) -> Result<Self, MixedSurfaceKillBoxDenial> {
        let family = unsupported.family().ok_or(
            MixedSurfaceKillBoxDenial::MissingSurfaceSupportEvidence {
                family: SurfaceFamily::Unknown,
            },
        )?;
        let receipt = unsupported
            .receipt()
            .ok_or(MixedSurfaceKillBoxDenial::MissingSurfaceSupportEvidence { family })?;
        let support_evidence_digest = receipt.stage_identity().receipt_identity();
        let upstream_geometry_binding_identity = unsupported
            .upstream_geometry_binding_identity()
            .unwrap_or("missing-geometry-binding")
            .to_string();
        let upstream_geometry_carriers = receipt.counters().upstream_geometry_carriers();
        let user_outcome = respond(
            WorthUserResponseSource::from_unsupported_surface_support(&unsupported),
            declaration,
        );
        Ok(Self {
            family,
            status: MixedSurfaceFamilyRunStatus::Unsupported,
            support_evidence_digest,
            human_reason: unsupported.human_reason().to_string(),
            unsupported_reason_code: Some(unsupported.reason_code()),
            user_outcome,
            upstream_geometry_binding_identity,
            upstream_geometry_carriers,
        })
    }

    pub fn family(&self) -> SurfaceFamily {
        self.family
    }

    pub fn status(&self) -> MixedSurfaceFamilyRunStatus {
        self.status
    }

    pub fn support_evidence_digest(&self) -> &str {
        &self.support_evidence_digest
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }

    pub fn unsupported_reason_code(&self) -> Option<UnsupportedSurfaceSupportReasonCode> {
        self.unsupported_reason_code
    }

    pub fn user_outcome(&self) -> &WorthUserOutcome {
        &self.user_outcome
    }

    pub fn user_response_digest(&self) -> &str {
        self.user_outcome.evidence().digest()
    }

    pub fn upstream_geometry_binding_identity(&self) -> &str {
        &self.upstream_geometry_binding_identity
    }

    pub fn upstream_geometry_carriers(&self) -> usize {
        self.upstream_geometry_carriers
    }

    pub fn is_acceptable_m7_input(&self) -> bool {
        self.status == MixedSurfaceFamilyRunStatus::AdmittedPlane
            && self.family == SurfaceFamily::Plane
    }

    pub fn attempt_readiness(&self) -> Result<(), MixedSurfaceKillBoxDenial> {
        if self.is_acceptable_m7_input() {
            Ok(())
        } else {
            Err(
                MixedSurfaceKillBoxDenial::UnsupportedFamilyReadinessAttempt {
                    family: self.family,
                },
            )
        }
    }

    pub fn attempt_with_plane_support_receipt(
        &self,
        plane_run: &MixedSurfaceFamilyRun,
    ) -> Result<(), MixedSurfaceKillBoxDenial> {
        if self.family == SurfaceFamily::Plane
            && plane_run.family == SurfaceFamily::Plane
            && self.support_evidence_digest == plane_run.support_evidence_digest
        {
            return Ok(());
        }
        Err(MixedSurfaceKillBoxDenial::SurfaceFamilyReceiptMismatch {
            target_family: self.family,
            receipt_family: plane_run.family,
        })
    }

    pub fn attempt_with_user_response(
        &self,
        response_run: &MixedSurfaceFamilyRun,
    ) -> Result<(), MixedSurfaceKillBoxDenial> {
        if self.family == response_run.family
            && self.user_response_digest() == response_run.user_response_digest()
        {
            return Ok(());
        }
        Err(MixedSurfaceKillBoxDenial::WrongFamilyUserResponse {
            target_family: self.family,
            response_family: response_run.family,
        })
    }
}

fn respond(source: WorthUserResponseSource, declaration: &str) -> WorthUserOutcome {
    WorthUserResponseWorkload::from_source(source)
        .declared(declaration)
        .respond()
        .expect("mixed surface user response must certify")
        .outcome()
        .clone()
}
