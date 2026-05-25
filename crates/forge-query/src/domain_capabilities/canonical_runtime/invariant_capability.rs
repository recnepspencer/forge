use forge_proof::TransitionOutcome;
use forge_relational::facade::runtime::InvariantCatalog;

use crate::identity::hash_parts;

use crate::domain_capabilities::denials::{
    ForgeQueryDomainCapabilityProgressionDenial, ForgeQueryDomainCapabilityProgressionDenialKind,
};
use crate::domain_capabilities::payloads::{
    ForgeQueryGraphInvariantDenialRuntimeSemantics,
    ForgeQueryInvariantCapabilityContributionPayload,
    ForgeQueryInvariantCapabilityContributionPosture,
};
use crate::domain_capabilities::targets::{
    ForgeQueryDeclarationBoundContributionTarget, ForgeQueryDomainCapabilityTargetBinding,
    ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
};
use crate::domain_capabilities::{
    ForgeQueryCanonicalInvariantCapabilityArtifact, ForgeQueryDomainCapabilityTransitionOutcome,
    ForgeQueryMaterializationReadyInvariantCapabilityContribution,
};
use crate::runtime::ForgeQueryGraphCompositionCapabilitySupportRow;
use crate::runtime::{
    ForgeQueryGraphCompositionDomainInvariantDenial,
    ForgeQueryGraphCompositionDomainInvariantSummary,
};

pub fn materialize_canonical_invariant_capability_artifact<T>(
    contribution: ForgeQueryMaterializationReadyInvariantCapabilityContribution<T>,
) -> ForgeQueryCanonicalInvariantCapabilityArtifact<T>
where
    T: ForgeQueryDomainCapabilityTargetBinding,
{
    super::artifacts::materialize_domain_capability_canonical_runtime_artifact(contribution)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryInvariantCatalogRegistrationArtifact {
    lane: &'static str,
    semantic_code: String,
    detail: String,
    invariant_catalog: InvariantCatalog,
    intent_name: String,
    strategy_name: String,
    strategy_version: String,
    input_contract: String,
    source_lane: crate::runtime::ForgeQueryIntentSourceLane,
    target_lane: crate::runtime::ForgeQueryAuthorityLane,
    target_binding_digest: String,
    request_digest: String,
    materialization_digest: String,
}

impl ForgeQueryInvariantCatalogRegistrationArtifact {
    pub fn lane(&self) -> &'static str {
        self.lane
    }

    pub fn semantic_code(&self) -> &str {
        &self.semantic_code
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn invariant_catalog(&self) -> &InvariantCatalog {
        &self.invariant_catalog
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

    pub fn target_binding_digest(&self) -> &str {
        &self.target_binding_digest
    }

    pub fn request_digest(&self) -> &str {
        &self.request_digest
    }

    pub fn materialization_digest(&self) -> &str {
        &self.materialization_digest
    }
}

pub fn materialize_query_invariant_catalog_registration_artifact(
    contribution: ForgeQueryMaterializationReadyInvariantCapabilityContribution<
        ForgeQueryDeclarationBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<ForgeQueryInvariantCatalogRegistrationArtifact> {
    let domain_contribution = contribution.payload();
    let payload = domain_contribution.payload();
    let Some(invariant_registration) = payload.invariant_registration() else {
        return TransitionOutcome::Denied(missing_runtime_semantics_denial(
            payload,
            domain_contribution.request_digest(),
            domain_contribution.target().kind(),
            "invariant registration",
        ));
    };

    if payload.posture() != ForgeQueryInvariantCapabilityContributionPosture::InvariantRegistration
    {
        return TransitionOutcome::Denied(unsupported_posture_denial(
            payload,
            domain_contribution.request_digest(),
            domain_contribution.target().kind(),
            "invariant registration",
            "invariant-registration",
        ));
    }

    let Some((name, strategy_name, strategy_version, input_contract, source_lane, target_lane)) =
        domain_contribution
            .target()
            .semantics()
            .intent_declaration()
    else {
        unreachable!("declaration-bound target should preserve declaration semantics");
    };
    let invariant_catalog = invariant_registration.canonical_invariant_catalog();

    TransitionOutcome::Success(ForgeQueryInvariantCatalogRegistrationArtifact {
        lane: "query_invariant_catalog_registration",
        semantic_code: payload.semantic_code().to_string(),
        detail: payload.detail().to_string(),
        invariant_catalog: invariant_catalog.clone(),
        intent_name: name.to_string(),
        strategy_name: strategy_name.to_string(),
        strategy_version: strategy_version.to_string(),
        input_contract: input_contract.to_string(),
        source_lane,
        target_lane,
        target_binding_digest: domain_contribution.target().binding_digest().to_string(),
        request_digest: domain_contribution.request_digest().to_string(),
        materialization_digest: hash_parts(&[
            "forge_query_invariant_catalog_registration_artifact_v1".to_string(),
            format!("lane:{}", "query_invariant_catalog_registration"),
            format!("semantic_code:{}", payload.semantic_code()),
            format!("detail:{}", payload.detail()),
            format!("catalog:{}", invariant_registration.registration_digest()),
            format!("intent:{name}"),
            format!("strategy:{strategy_name}"),
            format!("strategy-version:{strategy_version}"),
            format!("input-contract:{input_contract}"),
            format!("source-lane:{:?}", source_lane),
            format!("target-lane:{:?}", target_lane),
            format!("binding:{}", domain_contribution.target().binding_digest()),
            format!("request:{}", domain_contribution.request_digest()),
        ]),
    })
}

pub fn materialize_graph_composition_capability_support_row(
    contribution: ForgeQueryMaterializationReadyInvariantCapabilityContribution<
        ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<ForgeQueryGraphCompositionCapabilitySupportRow> {
    let domain_contribution = contribution.payload();
    let payload = domain_contribution.payload();
    let Some(graph_capability) = payload.graph_capability() else {
        return TransitionOutcome::Denied(missing_runtime_semantics_denial(
            payload,
            domain_contribution.request_digest(),
            domain_contribution.target().kind(),
            "graph capability",
        ));
    };

    match payload.posture() {
        ForgeQueryInvariantCapabilityContributionPosture::CapabilityGap
        | ForgeQueryInvariantCapabilityContributionPosture::SupportSummary => {
            TransitionOutcome::Success(ForgeQueryGraphCompositionCapabilitySupportRow::new(
                graph_capability.capability_family(),
                graph_capability.capability_class(),
            ))
        }
        ForgeQueryInvariantCapabilityContributionPosture::InvariantDenial
        | ForgeQueryInvariantCapabilityContributionPosture::InvariantRegistration => {
            TransitionOutcome::Denied(unsupported_posture_denial(
                payload,
                domain_contribution.request_digest(),
                domain_contribution.target().kind(),
                "graph capability",
                "capability-gap and support-summary",
            ))
        }
    }
}

pub fn materialize_graph_composition_domain_invariant_denial(
    contribution: ForgeQueryMaterializationReadyInvariantCapabilityContribution<
        ForgeQueryLowerRuntimeBoundaryBoundContributionTarget,
    >,
) -> ForgeQueryDomainCapabilityTransitionOutcome<ForgeQueryGraphCompositionDomainInvariantDenial> {
    let domain_contribution = contribution.payload();
    let payload = domain_contribution.payload();
    let Some(graph_invariant_denial) = payload.graph_invariant_denial() else {
        return TransitionOutcome::Denied(missing_runtime_semantics_denial(
            payload,
            domain_contribution.request_digest(),
            domain_contribution.target().kind(),
            "graph invariant denial",
        ));
    };

    match payload.posture() {
        ForgeQueryInvariantCapabilityContributionPosture::InvariantDenial => {
            TransitionOutcome::Success(
                ForgeQueryGraphCompositionDomainInvariantDenial::from_contributed(
                    graph_invariant_denial.invariant_family(),
                    payload.detail(),
                    graph_invariant_summary(graph_invariant_denial),
                ),
            )
        }
        ForgeQueryInvariantCapabilityContributionPosture::CapabilityGap
        | ForgeQueryInvariantCapabilityContributionPosture::SupportSummary
        | ForgeQueryInvariantCapabilityContributionPosture::InvariantRegistration => {
            TransitionOutcome::Denied(unsupported_posture_denial(
                payload,
                domain_contribution.request_digest(),
                domain_contribution.target().kind(),
                "graph invariant denial",
                "invariant-denial",
            ))
        }
    }
}

fn graph_invariant_summary(
    semantics: &ForgeQueryGraphInvariantDenialRuntimeSemantics,
) -> ForgeQueryGraphCompositionDomainInvariantSummary {
    ForgeQueryGraphCompositionDomainInvariantSummary::from_parts(
        semantics.declared_collections().to_vec(),
        semantics.declared_symbols().to_vec(),
        semantics.target_combination_families().to_vec(),
        semantics.lifecycle_families().to_vec(),
        semantics.program_digest().to_string(),
        semantics.breadth_digest().to_string(),
        semantics.counter_snapshot().to_string(),
    )
}

fn missing_runtime_semantics_denial(
    payload: &ForgeQueryInvariantCapabilityContributionPayload,
    request_digest: &str,
    target_kind: crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind,
    runtime_family: &str,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::MissingCanonicalMaterializationSemantics,
        "invariant-capability",
        target_kind,
        request_digest,
        format!(
            "{runtime_family} runtime materialization requires matching runtime semantics for `{}`",
            payload.semantic_code()
        ),
    )
}

fn unsupported_posture_denial(
    payload: &ForgeQueryInvariantCapabilityContributionPayload,
    request_digest: &str,
    target_kind: crate::domain_capabilities::ForgeQueryDomainCapabilityTargetKind,
    runtime_family: &str,
    supported_postures: &str,
) -> ForgeQueryDomainCapabilityProgressionDenial {
    ForgeQueryDomainCapabilityProgressionDenial::new(
        ForgeQueryDomainCapabilityProgressionDenialKind::UnsupportedCanonicalMaterializationPosture,
        "invariant-capability",
        target_kind,
        request_digest,
        format!(
            "{runtime_family} runtime materialization only supports {supported_postures} postures; got `{}`",
            payload.posture().as_str()
        ),
    )
}
