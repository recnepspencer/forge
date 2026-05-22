use crate::spatial_intent::arbitration::{
    analyze_spatial_intent_conflict_with_capabilities_and_profile, SpatialArbitrationPreviewHint,
    SpatialAuthoredActKind, SpatialBlockedCapability, SpatialIntentArbitrationAnalysis,
    SpatialIntentCandidate, SpatialIntentCapabilitySet, SpatialObservedRelationFact,
};
use crate::spatial_intent::policy::{SpatialIntentPolicyProfile, SpatialPreviewRichness};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialIntentPreviewCommitDisposition {
    WouldAutoResolve(SpatialIntentCandidate),
    WouldPreserveCandidates,
    WouldRequireClarification,
    WouldBlockOnCapability(SpatialBlockedCapability),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpatialIntentPreviewWarning {
    ClarificationRequired,
    PreservedCandidateSet,
    BlockedFutureCandidate(SpatialBlockedCapability),
    ProfileDrivenAutoResolve(SpatialIntentCandidate),
    HighFidelityPreview,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialIntentPreview {
    policy_profile: SpatialIntentPolicyProfile,
    analysis: SpatialIntentArbitrationAnalysis,
    preview_richness: SpatialPreviewRichness,
    commit_disposition: SpatialIntentPreviewCommitDisposition,
    warnings: Vec<SpatialIntentPreviewWarning>,
}

impl SpatialIntentPreview {
    pub fn policy_profile(&self) -> SpatialIntentPolicyProfile {
        self.policy_profile
    }

    pub fn analysis(&self) -> &SpatialIntentArbitrationAnalysis {
        &self.analysis
    }

    pub fn preview_richness(&self) -> SpatialPreviewRichness {
        self.preview_richness
    }

    pub fn commit_disposition(&self) -> SpatialIntentPreviewCommitDisposition {
        self.commit_disposition
    }

    pub fn warnings(&self) -> &[SpatialIntentPreviewWarning] {
        &self.warnings
    }
}

pub fn prepare_spatial_intent_preview(
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: &[SpatialObservedRelationFact],
) -> SpatialIntentPreview {
    prepare_spatial_intent_preview_with_capabilities_and_profile(
        authored_act,
        observed_relation_facts,
        SpatialIntentCapabilitySet::blocked_defaults(),
        SpatialIntentPolicyProfile::conservative_exact_modeling(),
    )
}

pub fn prepare_spatial_intent_preview_with_capabilities(
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: &[SpatialObservedRelationFact],
    capabilities: SpatialIntentCapabilitySet,
) -> SpatialIntentPreview {
    prepare_spatial_intent_preview_with_capabilities_and_profile(
        authored_act,
        observed_relation_facts,
        capabilities,
        SpatialIntentPolicyProfile::conservative_exact_modeling(),
    )
}

pub fn prepare_spatial_intent_preview_with_profile(
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: &[SpatialObservedRelationFact],
    profile: SpatialIntentPolicyProfile,
) -> SpatialIntentPreview {
    prepare_spatial_intent_preview_with_capabilities_and_profile(
        authored_act,
        observed_relation_facts,
        SpatialIntentCapabilitySet::blocked_defaults(),
        profile,
    )
}

pub fn prepare_spatial_intent_preview_with_capabilities_and_profile(
    authored_act: SpatialAuthoredActKind,
    observed_relation_facts: &[SpatialObservedRelationFact],
    capabilities: SpatialIntentCapabilitySet,
    profile: SpatialIntentPolicyProfile,
) -> SpatialIntentPreview {
    let analysis = analyze_spatial_intent_conflict_with_capabilities_and_profile(
        authored_act,
        observed_relation_facts,
        capabilities,
        profile,
    );
    let commit_disposition = match analysis.preview_hint() {
        SpatialArbitrationPreviewHint::AutoResolve(candidate) => {
            SpatialIntentPreviewCommitDisposition::WouldAutoResolve(candidate)
        }
        SpatialArbitrationPreviewHint::PreserveCandidates => {
            SpatialIntentPreviewCommitDisposition::WouldPreserveCandidates
        }
        SpatialArbitrationPreviewHint::ClarificationRequired => {
            SpatialIntentPreviewCommitDisposition::WouldRequireClarification
        }
        SpatialArbitrationPreviewHint::BlockedByCapability(capability) => {
            SpatialIntentPreviewCommitDisposition::WouldBlockOnCapability(capability)
        }
    };
    let mut warnings = Vec::new();
    match commit_disposition {
        SpatialIntentPreviewCommitDisposition::WouldRequireClarification => {
            warnings.push(SpatialIntentPreviewWarning::ClarificationRequired)
        }
        SpatialIntentPreviewCommitDisposition::WouldPreserveCandidates => {
            warnings.push(SpatialIntentPreviewWarning::PreservedCandidateSet)
        }
        SpatialIntentPreviewCommitDisposition::WouldBlockOnCapability(capability) => warnings.push(
            SpatialIntentPreviewWarning::BlockedFutureCandidate(capability),
        ),
        SpatialIntentPreviewCommitDisposition::WouldAutoResolve(candidate)
            if candidate != SpatialIntentCandidate::baseline_for(authored_act) =>
        {
            warnings.push(SpatialIntentPreviewWarning::ProfileDrivenAutoResolve(
                candidate,
            ))
        }
        SpatialIntentPreviewCommitDisposition::WouldAutoResolve(_) => {}
    }
    if profile.preview_richness() == SpatialPreviewRichness::HighFidelity {
        warnings.push(SpatialIntentPreviewWarning::HighFidelityPreview);
    }
    SpatialIntentPreview {
        policy_profile: profile,
        analysis,
        preview_richness: profile.preview_richness(),
        commit_disposition,
        warnings,
    }
}
