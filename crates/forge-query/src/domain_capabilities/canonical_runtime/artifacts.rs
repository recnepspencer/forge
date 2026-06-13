use crate::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

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
    semantic_identity: ForgeQueryEvidenceIdentity,
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
        let target_identity = contribution.payload().target().target_identity();
        let binding_identity = contribution.payload().target().binding_identity();
        let request_identity =
            canonical_runtime_request_identity(contribution.payload().request_digest());
        let semantic_identity = canonical_runtime_semantic_identity(
            category,
            semantic_posture,
            &semantic_code,
            &detail,
        );
        let materialization_digest = ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_domain_capability_canonical_runtime_materialization_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("family"), canonical_family_for(category))
        .field_shape(ForgeQueryEvidenceTag::new("target_kind"), target_kind.as_str())
        .field_evidence_identity(ForgeQueryEvidenceTag::new("target"), &target_identity)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("binding"), &binding_identity)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("request"), &request_identity)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("semantic"), &semantic_identity)
        .seal()
        .as_str()
        .to_string();
        Self {
            contribution,
            canonical_family: canonical_family_for(category),
            semantic_identity,
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

    pub fn semantic_identity_for_reporting(&self) -> &str {
        self.semantic_identity.as_str()
    }

    pub fn semantic_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.semantic_identity
    }

    pub fn materialization_digest(&self) -> &str {
        &self.materialization_digest
    }
}

fn canonical_runtime_request_identity(request_digest: &str) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::MutationEvidenceSourceDigest)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_domain_capability_request_v1",
        )
        .field_identity(ForgeQueryEvidenceTag::new("request"), request_digest)
        .seal()
}

fn canonical_runtime_semantic_identity(
    category: ForgeQueryDomainCapabilityCategory,
    semantic_posture: ForgeQueryDomainCapabilitySemanticPosture,
    semantic_code: &str,
    detail: &str,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::MutationEvidenceSourceDigest)
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_domain_capability_semantic_identity_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("category"), category.as_str())
        .field_shape(ForgeQueryEvidenceTag::new("posture"), semantic_posture.as_str())
        .field_shape(ForgeQueryEvidenceTag::new("code"), semantic_code)
        .field_shape(ForgeQueryEvidenceTag::new("detail"), detail)
        .seal()
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
