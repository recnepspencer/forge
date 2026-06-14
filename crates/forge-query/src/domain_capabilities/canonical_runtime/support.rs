use forge_proof::TransitionOutcome;

use crate::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

use crate::domain_capabilities::payloads::ForgeQuerySupportContributionPosture;
use crate::domain_capabilities::targets::{
    ForgeQueryAdmittedPlanBoundContributionTarget, ForgeQueryDeclarationBoundContributionTarget,
    ForgeQueryDomainCapabilityTargetBinding, ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
};
use crate::domain_capabilities::{
    ForgeQueryCanonicalSupportTraceabilityArtifact, ForgeQueryDomainCapabilityTransitionOutcome,
    ForgeQueryMaterializationReadySupportContribution,
};
use crate::intent_admission::{
    ForgeQueryIntentAdmissionSupportTraceabilityReport,
    ForgeQueryIntentAdmissionSupportTraceabilityRow,
};

pub fn materialize_canonical_support_traceability_artifact<T>(
    contribution: ForgeQueryMaterializationReadySupportContribution<T>,
) -> ForgeQueryCanonicalSupportTraceabilityArtifact<T>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    super::artifacts::materialize_domain_capability_canonical_runtime_artifact(contribution)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentDeclarationSupportTraceabilityArtifact {
    lane: &'static str,
    support_detail: String,
    intent_name: String,
    strategy_name: String,
    strategy_version: String,
    input_contract: String,
    source_lane: crate::runtime::ForgeQueryIntentSourceLane,
    target_lane: crate::runtime::ForgeQueryAuthorityLane,
    target_binding_identity: ForgeQueryEvidenceIdentity,
    request_identity: ForgeQueryEvidenceIdentity,
    materialization_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryIntentDeclarationSupportTraceabilityArtifact {
    pub fn lane(&self) -> &'static str {
        self.lane
    }

    pub fn support_detail(&self) -> &str {
        &self.support_detail
    }

    pub fn intent_name(&self) -> &str {
        &self.intent_name
    }

    pub fn strategy_name(&self) -> &str {
        &self.strategy_name
    }

    pub fn strategy_version(&self) -> &str {
        &self.strategy_version
    }

    pub fn input_contract(&self) -> &str {
        &self.input_contract
    }

    pub fn source_lane(&self) -> crate::runtime::ForgeQueryIntentSourceLane {
        self.source_lane
    }

    pub fn target_lane(&self) -> crate::runtime::ForgeQueryAuthorityLane {
        self.target_lane
    }

    pub fn target_binding_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.target_binding_identity
    }

    pub fn target_binding_for_reporting(&self) -> &str {
        self.target_binding_identity.as_str()
    }

    pub fn request_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.request_identity
    }

    pub fn request_for_reporting(&self) -> &str {
        self.request_identity.as_str()
    }

    pub fn materialization_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.materialization_identity
    }

    pub fn materialization_digest(&self) -> &str {
        self.materialization_identity.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeBoundarySupportTraceabilityArtifact {
    lane: &'static str,
    support_detail: String,
    seam_key: crate::lower_runtime_routing::ForgeQueryLowerRuntimeSeamKey,
    capability_label: &'static str,
    crossing_classification:
        crate::lower_runtime_routing::ForgeQueryLowerRuntimeCrossingClassification,
    route_kind: crate::lower_runtime_routing::ForgeQueryLowerRuntimeRouteKind,
    support_posture: crate::lower_runtime_routing::ForgeQueryLowerRuntimeSupportPosture,
    envelope_identity: ForgeQueryEvidenceIdentity,
    target_binding_identity: ForgeQueryEvidenceIdentity,
    request_identity: ForgeQueryEvidenceIdentity,
    materialization_identity: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryLowerRuntimeBoundarySupportTraceabilityArtifact {
    pub fn lane(&self) -> &'static str {
        self.lane
    }

    pub fn support_detail(&self) -> &str {
        &self.support_detail
    }

    pub fn seam_key(&self) -> crate::lower_runtime_routing::ForgeQueryLowerRuntimeSeamKey {
        self.seam_key
    }

    pub fn capability_label(&self) -> &'static str {
        self.capability_label
    }

    pub fn crossing_classification(
        &self,
    ) -> crate::lower_runtime_routing::ForgeQueryLowerRuntimeCrossingClassification {
        self.crossing_classification
    }

    pub fn route_kind(&self) -> crate::lower_runtime_routing::ForgeQueryLowerRuntimeRouteKind {
        self.route_kind
    }

    pub fn support_posture(
        &self,
    ) -> crate::lower_runtime_routing::ForgeQueryLowerRuntimeSupportPosture {
        self.support_posture
    }

    pub fn envelope_for_reporting(&self) -> &str {
        self.envelope_identity.as_str()
    }

    pub fn target_binding_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.target_binding_identity
    }

    pub fn target_binding_for_reporting(&self) -> &str {
        self.target_binding_identity.as_str()
    }

    pub fn request_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.request_identity
    }

    pub fn request_for_reporting(&self) -> &str {
        self.request_identity.as_str()
    }

    pub fn materialization_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.materialization_identity
    }

    pub fn materialization_digest(&self) -> &str {
        self.materialization_identity.as_str()
    }
}

pub fn materialize_intent_declaration_support_traceability_artifact(
    contribution: ForgeQueryMaterializationReadySupportContribution<
        ForgeQueryDeclarationBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<
    ForgeQueryIntentDeclarationSupportTraceabilityArtifact,
> {
    let domain_contribution = contribution.payload();
    let payload = domain_contribution.payload();
    let Some((name, strategy_name, strategy_version, input_contract, source_lane, target_lane)) =
        domain_contribution
            .target()
            .semantics()
            .intent_declaration()
    else {
        unreachable!("declaration-bound target should preserve declaration semantics");
    };
    TransitionOutcome::Success(ForgeQueryIntentDeclarationSupportTraceabilityArtifact {
        lane: support_lane(payload.posture()),
        support_detail: support_detail(payload.semantic_code(), payload.detail()),
        intent_name: name.to_string(),
        strategy_name: strategy_name.to_string(),
        strategy_version: strategy_version.to_string(),
        input_contract: input_contract.to_string(),
        source_lane,
        target_lane,
        target_binding_identity: domain_contribution.target().binding_identity(),
        request_identity: domain_contribution.request_identity().clone(),
        materialization_identity: ForgeQueryEvidenceIdentity::compose(
            ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest,
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("identity_family"),
            "forge_query_intent_declaration_support_traceability_artifact_v1",
        )
        .field_shape(ForgeQueryEvidenceTag::new("lane"), support_lane(payload.posture()))
        .field_shape(
            ForgeQueryEvidenceTag::new("detail"),
            support_detail(payload.semantic_code(), payload.detail()),
        )
        .field_shape(ForgeQueryEvidenceTag::new("intent"), name)
        .field_shape(ForgeQueryEvidenceTag::new("strategy"), strategy_name)
        .field_shape(ForgeQueryEvidenceTag::new("strategy_version"), strategy_version)
        .field_shape(ForgeQueryEvidenceTag::new("input_contract"), input_contract)
        .field_shape(ForgeQueryEvidenceTag::new("source_lane"), format!("{source_lane:?}"))
        .field_shape(ForgeQueryEvidenceTag::new("target_lane"), format!("{target_lane:?}"))
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("binding"),
            &domain_contribution.target().binding_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("request"),
            domain_contribution.request_identity(),
        )
        .seal(),
    })
}

pub fn materialize_lower_runtime_support_traceability_artifact(
    contribution: ForgeQueryMaterializationReadySupportContribution<
        ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<
    ForgeQueryLowerRuntimeBoundarySupportTraceabilityArtifact,
> {
    let domain_contribution = contribution.payload();
    let payload = domain_contribution.payload();
    let Some((
        seam_key,
        capability_label,
        crossing_classification,
        route_kind,
        support_posture,
        _envelope_digest,
    )) = domain_contribution
        .target()
        .semantics()
        .lower_runtime_boundary()
    else {
        unreachable!("lower-runtime target should preserve lower-runtime semantics");
    };
    let envelope_identity = domain_contribution.target().binding_identity();
    let materialization_identity = ForgeQueryEvidenceIdentity::compose(
        ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest,
    )
    .field_shape(
        ForgeQueryEvidenceTag::new("identity_family"),
        "forge_query_lower_runtime_support_traceability_artifact_v1",
    )
    .field_shape(ForgeQueryEvidenceTag::new("lane"), support_lane(payload.posture()))
    .field_shape(
        ForgeQueryEvidenceTag::new("detail"),
        support_detail(payload.semantic_code(), payload.detail()),
    )
    .field_shape(ForgeQueryEvidenceTag::new("seam_key"), format!("{seam_key:?}"))
    .field_shape(ForgeQueryEvidenceTag::new("capability"), capability_label)
    .field_shape(
        ForgeQueryEvidenceTag::new("crossing"),
        format!("{crossing_classification:?}"),
    )
    .field_shape(ForgeQueryEvidenceTag::new("route"), format!("{route_kind:?}"))
    .field_shape(
        ForgeQueryEvidenceTag::new("support_posture"),
        format!("{support_posture:?}"),
    )
    .field_evidence_identity(
        ForgeQueryEvidenceTag::new("envelope"),
        &envelope_identity,
    )
    .field_evidence_identity(
        ForgeQueryEvidenceTag::new("binding"),
        &domain_contribution.target().binding_identity(),
    )
    .field_evidence_identity(
        ForgeQueryEvidenceTag::new("request"),
        domain_contribution.request_identity(),
    )
    .seal();
    TransitionOutcome::Success(ForgeQueryLowerRuntimeBoundarySupportTraceabilityArtifact {
        lane: support_lane(payload.posture()),
        support_detail: support_detail(payload.semantic_code(), payload.detail()),
        seam_key,
        capability_label,
        crossing_classification,
        route_kind,
        support_posture,
        envelope_identity,
        target_binding_identity: domain_contribution.target().binding_identity(),
        request_identity: domain_contribution.request_identity().clone(),
        materialization_identity,
    })
}

pub fn materialize_intent_admission_support_traceability_report(
    contribution: ForgeQueryMaterializationReadySupportContribution<
        ForgeQueryAdmittedPlanBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<ForgeQueryIntentAdmissionSupportTraceabilityReport>
{
    match support_traceability_row(&contribution) {
        TransitionOutcome::Success(row) => TransitionOutcome::Success(
            ForgeQueryIntentAdmissionSupportTraceabilityReport::from_rows(vec![row]),
        ),
        TransitionOutcome::Denied(denial) => TransitionOutcome::Denied(denial),
        TransitionOutcome::Stale(stale) => TransitionOutcome::Stale(stale),
        TransitionOutcome::RebindRequired(rebind) => TransitionOutcome::RebindRequired(rebind),
        TransitionOutcome::Failed(failure) => TransitionOutcome::Failed(failure),
        TransitionOutcome::Deferred(never) => match never {},
    }
}

pub fn materialize_intent_admission_support_traceability_row(
    contribution: ForgeQueryMaterializationReadySupportContribution<
        ForgeQueryAdmittedPlanBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<ForgeQueryIntentAdmissionSupportTraceabilityRow> {
    support_traceability_row(&contribution)
}

fn support_traceability_row(
    contribution: &ForgeQueryMaterializationReadySupportContribution<
        ForgeQueryAdmittedPlanBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<ForgeQueryIntentAdmissionSupportTraceabilityRow> {
    let domain_contribution = contribution.payload();
    let payload = domain_contribution.payload();
    let Some((family, entrypoint, ..)) = domain_contribution
        .target()
        .semantics()
        .admitted_intent_plan()
    else {
        unreachable!("admitted-plan target should preserve admitted-plan semantics");
    };
    let Some((_, _, request_digest, eligibility_digest, decision_digest)) = domain_contribution
        .target()
        .semantics()
        .admitted_intent_plan()
    else {
        unreachable!("admitted-plan target should preserve admitted-plan semantics");
    };
    TransitionOutcome::Success(
        ForgeQueryIntentAdmissionSupportTraceabilityRow::new_domain_scoped(
            support_lane(payload.posture()),
            family.as_str(),
            entrypoint.as_str(),
            format!("{}:{}", payload.semantic_code(), payload.detail()),
            Some(domain_contribution.target().binding_digest().to_string()),
            Some(request_digest.to_string()),
            Some(eligibility_digest.to_string()),
            Some(decision_digest.to_string()),
        ),
    )
}

fn support_lane(posture: ForgeQuerySupportContributionPosture) -> &'static str {
    match posture {
        ForgeQuerySupportContributionPosture::DeclarationSupport => "domain_support",
        ForgeQuerySupportContributionPosture::DeclarationTraceability => "domain_traceability",
        ForgeQuerySupportContributionPosture::NarrowedSupport => "domain_narrowed_support",
    }
}

fn support_detail(semantic_code: &str, detail: &str) -> String {
    format!("{semantic_code}:{detail}")
}
