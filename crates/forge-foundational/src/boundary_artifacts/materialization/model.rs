use super::vocabulary::{
    FoundationalBoundaryAttachmentPoint, FoundationalBoundaryDecisionCause,
    FoundationalBoundaryDecisionSubject, FoundationalBoundaryMaterializationSeam,
    FoundationalBoundaryMaterializationSource, FoundationalBoundaryPlanningDenial,
    FoundationalBoundarySurfaceDisposition,
};
use crate::boundary_artifacts::{
    FoundationalBoundaryArtifactCategory, FoundationalBoundaryArtifactRole,
};
use crate::profiles::MaterializedFoundationalProfileSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalBoundaryMaterializationCost {
    attachment_count: u32,
    included_attachment_count: u32,
    decision_row_count: u32,
}

impl FoundationalBoundaryMaterializationCost {
    pub(crate) const fn new(
        attachment_count: u32,
        included_attachment_count: u32,
        decision_row_count: u32,
    ) -> Self {
        Self {
            attachment_count,
            included_attachment_count,
            decision_row_count,
        }
    }

    pub const fn attachment_count(&self) -> u32 {
        self.attachment_count
    }

    pub const fn included_attachment_count(&self) -> u32 {
        self.included_attachment_count
    }

    pub const fn decision_row_count(&self) -> u32 {
        self.decision_row_count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalBoundaryMaterializationAttachment {
    point: FoundationalBoundaryAttachmentPoint,
    included: bool,
}

impl FoundationalBoundaryMaterializationAttachment {
    pub(crate) const fn included(point: FoundationalBoundaryAttachmentPoint) -> Self {
        Self {
            point,
            included: true,
        }
    }

    pub(crate) const fn omitted(point: FoundationalBoundaryAttachmentPoint) -> Self {
        Self {
            point,
            included: false,
        }
    }

    pub const fn point(&self) -> FoundationalBoundaryAttachmentPoint {
        self.point
    }

    pub const fn is_included(&self) -> bool {
        self.included
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FoundationalBoundaryMaterializationDecisionRow {
    category: Option<FoundationalBoundaryArtifactCategory>,
    subject: FoundationalBoundaryDecisionSubject,
    cause: FoundationalBoundaryDecisionCause,
    seam: FoundationalBoundaryMaterializationSeam,
    attachment_point: Option<FoundationalBoundaryAttachmentPoint>,
}

impl FoundationalBoundaryMaterializationDecisionRow {
    pub(crate) const fn new(
        category: Option<FoundationalBoundaryArtifactCategory>,
        subject: FoundationalBoundaryDecisionSubject,
        cause: FoundationalBoundaryDecisionCause,
        seam: FoundationalBoundaryMaterializationSeam,
        attachment_point: Option<FoundationalBoundaryAttachmentPoint>,
    ) -> Self {
        Self {
            category,
            subject,
            cause,
            seam,
            attachment_point,
        }
    }

    pub const fn category(&self) -> Option<FoundationalBoundaryArtifactCategory> {
        self.category
    }

    pub const fn subject(&self) -> FoundationalBoundaryDecisionSubject {
        self.subject
    }

    pub const fn cause(&self) -> FoundationalBoundaryDecisionCause {
        self.cause
    }

    pub const fn seam(&self) -> FoundationalBoundaryMaterializationSeam {
        self.seam
    }

    pub const fn attachment_point(&self) -> Option<FoundationalBoundaryAttachmentPoint> {
        self.attachment_point
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryMaterializationInput<Surface> {
    pub(crate) surface: Surface,
    category: FoundationalBoundaryArtifactCategory,
    role: FoundationalBoundaryArtifactRole,
    source: FoundationalBoundaryMaterializationSource,
    authority_claim: bool,
}

impl<Surface> FoundationalBoundaryMaterializationInput<Surface> {
    pub(crate) fn new(
        surface: Surface,
        category: FoundationalBoundaryArtifactCategory,
        role: FoundationalBoundaryArtifactRole,
        source: FoundationalBoundaryMaterializationSource,
        authority_claim: bool,
    ) -> Self {
        Self {
            surface,
            category,
            role,
            source,
            authority_claim,
        }
    }

    pub const fn category(&self) -> FoundationalBoundaryArtifactCategory {
        self.category
    }

    pub const fn role(&self) -> FoundationalBoundaryArtifactRole {
        self.role
    }

    pub const fn source(&self) -> FoundationalBoundaryMaterializationSource {
        self.source
    }

    pub const fn is_authority_claim(&self) -> bool {
        self.authority_claim
    }

    pub const fn surface(&self) -> &Surface {
        &self.surface
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalBoundaryMaterializationPlan<Surface> {
    input: FoundationalBoundaryMaterializationInput<Surface>,
    seam: FoundationalBoundaryMaterializationSeam,
    disposition: FoundationalBoundarySurfaceDisposition,
    attachments: Vec<FoundationalBoundaryMaterializationAttachment>,
    decision_rows: Vec<FoundationalBoundaryMaterializationDecisionRow>,
    cost: FoundationalBoundaryMaterializationCost,
    profile: MaterializedFoundationalProfileSet,
}

impl<Surface> FoundationalBoundaryMaterializationPlan<Surface> {
    pub(crate) fn new(
        input: FoundationalBoundaryMaterializationInput<Surface>,
        seam: FoundationalBoundaryMaterializationSeam,
        disposition: FoundationalBoundarySurfaceDisposition,
        attachments: Vec<FoundationalBoundaryMaterializationAttachment>,
        decision_rows: Vec<FoundationalBoundaryMaterializationDecisionRow>,
        profile: MaterializedFoundationalProfileSet,
    ) -> Self {
        let included_attachment_count = attachments.iter().filter(|row| row.is_included()).count();
        let cost = FoundationalBoundaryMaterializationCost::new(
            attachments.len() as u32,
            included_attachment_count as u32,
            decision_rows.len() as u32,
        );

        Self {
            input,
            seam,
            disposition,
            attachments,
            decision_rows,
            cost,
            profile,
        }
    }

    pub const fn category(&self) -> FoundationalBoundaryArtifactCategory {
        self.input.category()
    }

    pub const fn role(&self) -> FoundationalBoundaryArtifactRole {
        self.input.role()
    }

    pub const fn source(&self) -> FoundationalBoundaryMaterializationSource {
        self.input.source()
    }

    pub const fn seam(&self) -> FoundationalBoundaryMaterializationSeam {
        self.seam
    }

    pub const fn is_authority_claim(&self) -> bool {
        self.input.is_authority_claim()
    }

    pub const fn disposition(&self) -> FoundationalBoundarySurfaceDisposition {
        self.disposition
    }

    pub fn attachments(&self) -> &[FoundationalBoundaryMaterializationAttachment] {
        &self.attachments
    }

    pub fn decision_rows(&self) -> &[FoundationalBoundaryMaterializationDecisionRow] {
        &self.decision_rows
    }

    pub const fn cost(&self) -> FoundationalBoundaryMaterializationCost {
        self.cost
    }

    pub const fn profile(&self) -> &MaterializedFoundationalProfileSet {
        &self.profile
    }

    pub fn materialize(
        self,
    ) -> Result<
        FoundationalMaterializedBoundaryArtifact<Surface>,
        FoundationalBoundaryMaterializationDenial,
    > {
        let FoundationalBoundaryMaterializationPlan {
            input,
            seam,
            disposition,
            attachments,
            decision_rows,
            cost,
            profile,
        } = self;
        let FoundationalBoundaryMaterializationInput {
            surface,
            category,
            role,
            source,
            authority_claim: _,
        } = input;

        match disposition.availability() {
            crate::boundary_artifacts::FoundationalBoundaryAvailability::Present => {}
            crate::boundary_artifacts::FoundationalBoundaryAvailability::Deferred => {
                return Err(FoundationalBoundaryMaterializationDenial::SurfaceDeferred);
            }
            crate::boundary_artifacts::FoundationalBoundaryAvailability::Reconstructable => {
                return Err(FoundationalBoundaryMaterializationDenial::SurfaceReconstructable);
            }
            crate::boundary_artifacts::FoundationalBoundaryAvailability::Unavailable => {
                return Err(FoundationalBoundaryMaterializationDenial::SurfaceUnavailable);
            }
        }

        Ok(FoundationalMaterializedBoundaryArtifact::new(
            surface,
            category,
            role,
            source,
            seam,
            disposition,
            attachments,
            decision_rows,
            cost,
            profile,
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationalMaterializedBoundaryArtifact<Surface> {
    surface: Surface,
    category: FoundationalBoundaryArtifactCategory,
    role: FoundationalBoundaryArtifactRole,
    source: FoundationalBoundaryMaterializationSource,
    seam: FoundationalBoundaryMaterializationSeam,
    disposition: FoundationalBoundarySurfaceDisposition,
    attachments: Vec<FoundationalBoundaryMaterializationAttachment>,
    decision_rows: Vec<FoundationalBoundaryMaterializationDecisionRow>,
    cost: FoundationalBoundaryMaterializationCost,
    profile: MaterializedFoundationalProfileSet,
}

impl<Surface> FoundationalMaterializedBoundaryArtifact<Surface> {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        surface: Surface,
        category: FoundationalBoundaryArtifactCategory,
        role: FoundationalBoundaryArtifactRole,
        source: FoundationalBoundaryMaterializationSource,
        seam: FoundationalBoundaryMaterializationSeam,
        disposition: FoundationalBoundarySurfaceDisposition,
        attachments: Vec<FoundationalBoundaryMaterializationAttachment>,
        decision_rows: Vec<FoundationalBoundaryMaterializationDecisionRow>,
        cost: FoundationalBoundaryMaterializationCost,
        profile: MaterializedFoundationalProfileSet,
    ) -> Self {
        Self {
            surface,
            category,
            role,
            source,
            seam,
            disposition,
            attachments,
            decision_rows,
            cost,
            profile,
        }
    }

    pub const fn surface(&self) -> &Surface {
        &self.surface
    }

    pub const fn category(&self) -> FoundationalBoundaryArtifactCategory {
        self.category
    }

    pub const fn role(&self) -> FoundationalBoundaryArtifactRole {
        self.role
    }

    pub const fn source(&self) -> FoundationalBoundaryMaterializationSource {
        self.source
    }

    pub const fn seam(&self) -> FoundationalBoundaryMaterializationSeam {
        self.seam
    }

    pub const fn disposition(&self) -> FoundationalBoundarySurfaceDisposition {
        self.disposition
    }

    pub fn attachments(&self) -> &[FoundationalBoundaryMaterializationAttachment] {
        &self.attachments
    }

    pub fn decision_rows(&self) -> &[FoundationalBoundaryMaterializationDecisionRow] {
        &self.decision_rows
    }

    pub const fn cost(&self) -> FoundationalBoundaryMaterializationCost {
        self.cost
    }

    pub const fn profile(&self) -> &MaterializedFoundationalProfileSet {
        &self.profile
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationalBoundaryMaterializationDenial {
    Planning(FoundationalBoundaryPlanningDenial),
    SurfaceDeferred,
    SurfaceReconstructable,
    SurfaceUnavailable,
}
