use super::LayoutCourtroomTranscriptIdentity;
use super::{certify_layout_foundational_closeout, LayoutFoundationalCloseoutEvidence};
use crate::courtroom::foundational::AspectNativeBoundaryHandoffVerdict;
use crate::courtroom::layout::owner_coverage::LayoutOwnerCoverageReceipt;
use crate::courtroom::layout::owner_evidence::LayoutOwnerExecutionEvidence;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutEvidenceAssemblyDenial {
    FoundationalEvidenceIncomplete,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LayoutEvidenceBundle {
    transcript_identity: LayoutCourtroomTranscriptIdentity,
    coverage: LayoutOwnerCoverageReceipt,
    foundational: LayoutFoundationalCloseoutEvidence,
    durable: crate::courtroom::layout::owner_scenarios::durable_observation::LayoutDurableObservationSource,
}

pub fn assemble_layout_evidence_bundle(
    owner_evidence: LayoutOwnerExecutionEvidence,
    boundary: AspectNativeBoundaryHandoffVerdict,
) -> Result<LayoutEvidenceBundle, LayoutEvidenceAssemblyDenial> {
    let (coverage, performance, durable) = owner_evidence.into_parts();
    let foundational = certify_layout_foundational_closeout(boundary, performance)
        .map_err(|_| LayoutEvidenceAssemblyDenial::FoundationalEvidenceIncomplete)?;
    Ok(LayoutEvidenceBundle {
        transcript_identity: LayoutCourtroomTranscriptIdentity::issue(),
        coverage,
        foundational,
        durable,
    })
}

impl LayoutEvidenceBundle {
    pub const fn transcript_identity(&self) -> LayoutCourtroomTranscriptIdentity {
        self.transcript_identity
    }

    pub const fn coverage(&self) -> &LayoutOwnerCoverageReceipt {
        &self.coverage
    }

    pub const fn foundational(&self) -> &LayoutFoundationalCloseoutEvidence {
        &self.foundational
    }

    pub(crate) const fn durable(
        &self,
    ) -> &crate::courtroom::layout::owner_scenarios::durable_observation::LayoutDurableObservationSource{
        &self.durable
    }
}
