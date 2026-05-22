use crate::identity::hash_parts;

use super::super::payloads::{
    ForgeQueryAdmissionContributionPayload, ForgeQueryAftermathContributionPayload,
    ForgeQueryContinuityContributionPayload, ForgeQueryDomainCapabilityCategory,
    ForgeQueryDomainCapabilityPayload, ForgeQueryDomainCapabilitySemanticPosture,
    ForgeQueryExplanationContributionPayload, ForgeQueryInvariantCapabilityContributionPayload,
    ForgeQuerySupportContributionPayload, ForgeQueryWorkflowContributionPayload,
};
use super::super::targets::ForgeQueryDomainCapabilityTargetBinding;

pub struct ForgeQueryCanonicalRuntimeMaterialization<P, T>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    contribution: super::super::ForgeQueryMaterializationReadyDomainCapabilityContribution<P, T>,
    canonical_family: &'static str,
    semantic_identity_digest: String,
    materialization_digest: String,
}

impl<P, T> ForgeQueryCanonicalRuntimeMaterialization<P, T>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    pub(crate) fn new(
        contribution: super::super::ForgeQueryMaterializationReadyDomainCapabilityContribution<
            P,
            T,
        >,
    ) -> Self {
        let category = contribution.payload().payload().category();
        let semantic_posture = contribution.payload().payload().semantic_posture();
        let semantic_code = contribution.payload().payload().semantic_code().to_string();
        let detail = contribution.payload().payload().detail().to_string();
        let target_kind = contribution.payload().target().kind();
        let target_digest = contribution.payload().target().target_digest().to_string();
        let binding_digest = contribution.payload().target().binding_digest().to_string();
        let request_digest = contribution.payload().request_digest().to_string();
        let semantic_identity_digest = hash_parts(&[
            "forge_query_domain_capability_semantic_identity_v1".to_string(),
            format!("category:{}", category.as_str()),
            format!("posture:{}", semantic_posture.as_str()),
            format!("code:{semantic_code}"),
            format!("detail:{detail}"),
        ]);
        let materialization_digest = hash_parts(&[
            "forge_query_domain_capability_canonical_runtime_materialization_v1".to_string(),
            format!("family:{}", canonical_family_for(category)),
            format!("target-kind:{}", target_kind.as_str()),
            format!("target:{target_digest}"),
            format!("binding:{binding_digest}"),
            format!("request:{request_digest}"),
            format!("semantic:{semantic_identity_digest}"),
        ]);
        Self {
            contribution,
            canonical_family: canonical_family_for(category),
            semantic_identity_digest,
            materialization_digest,
        }
    }

    pub fn contribution(
        &self,
    ) -> &super::super::ForgeQueryMaterializationReadyDomainCapabilityContribution<P, T> {
        &self.contribution
    }

    pub fn category(&self) -> ForgeQueryDomainCapabilityCategory {
        self.contribution.payload().payload().category()
    }

    pub fn canonical_family(&self) -> &'static str {
        self.canonical_family
    }

    pub fn semantic_posture(&self) -> ForgeQueryDomainCapabilitySemanticPosture {
        self.contribution.payload().payload().semantic_posture()
    }

    pub fn semantic_code(&self) -> &str {
        self.contribution.payload().payload().semantic_code()
    }

    pub fn detail(&self) -> &str {
        self.contribution.payload().payload().detail()
    }

    pub fn semantic_identity_digest(&self) -> &str {
        &self.semantic_identity_digest
    }

    pub fn materialization_digest(&self) -> &str {
        &self.materialization_digest
    }
}

pub fn materialize_domain_capability_canonical_runtime_artifact<P, T>(
    contribution: super::super::ForgeQueryMaterializationReadyDomainCapabilityContribution<P, T>,
) -> ForgeQueryCanonicalRuntimeMaterialization<P, T>
where
    P: ForgeQueryDomainCapabilityPayload,
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    ForgeQueryCanonicalRuntimeMaterialization::new(contribution)
}

pub type ForgeQueryCanonicalAdmissionArtifact<T> =
    ForgeQueryCanonicalRuntimeMaterialization<ForgeQueryAdmissionContributionPayload, T>;
pub type ForgeQueryCanonicalSupportTraceabilityArtifact<T> =
    ForgeQueryCanonicalRuntimeMaterialization<ForgeQuerySupportContributionPayload, T>;
pub type ForgeQueryCanonicalInvariantCapabilityArtifact<T> =
    ForgeQueryCanonicalRuntimeMaterialization<ForgeQueryInvariantCapabilityContributionPayload, T>;
pub type ForgeQueryCanonicalWorkflowArtifact<T> =
    ForgeQueryCanonicalRuntimeMaterialization<ForgeQueryWorkflowContributionPayload, T>;
pub type ForgeQueryCanonicalContinuityArtifact<T> =
    ForgeQueryCanonicalRuntimeMaterialization<ForgeQueryContinuityContributionPayload, T>;
pub type ForgeQueryCanonicalAftermathArtifact<T> =
    ForgeQueryCanonicalRuntimeMaterialization<ForgeQueryAftermathContributionPayload, T>;
pub type ForgeQueryCanonicalExplanationArtifact<T> =
    ForgeQueryCanonicalRuntimeMaterialization<ForgeQueryExplanationContributionPayload, T>;

fn canonical_family_for(category: ForgeQueryDomainCapabilityCategory) -> &'static str {
    match category {
        ForgeQueryDomainCapabilityCategory::Admission => "intent-admission",
        ForgeQueryDomainCapabilityCategory::SupportTraceability => "declaration-support",
        ForgeQueryDomainCapabilityCategory::InvariantCapability => "capability-invariant",
        ForgeQueryDomainCapabilityCategory::WorkflowPreview => "preview-workflow",
        ForgeQueryDomainCapabilityCategory::ContinuityLineage => "continuity-lineage",
        ForgeQueryDomainCapabilityCategory::ConsequenceAftermath => "consequence-aftermath",
        ForgeQueryDomainCapabilityCategory::ExplanationInspection => "explanation-inspection",
    }
}
