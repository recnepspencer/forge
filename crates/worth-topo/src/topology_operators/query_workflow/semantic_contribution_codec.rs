use forge_query::facade::{
    ForgeQueryContinuityContributionAuthoring, ForgeQueryDeclarationEntryContributionEvidence,
    ForgeQueryExplanationContributionAuthoring,
};

use crate::topology_operators::application::TopologyDeclarationMutationPayload;
use crate::topology_operators::{
    TopologyMutationDerivedFallbackPolicy, TopologyMutationFamily, TopologyMutationNamingOutcome,
    TopologyMutationNamingRow, TopologyMutationNamingScope, TopologyOperatorContributionIntent,
};

const NAMING_ROW_MARKER: &str = ".naming_row.";
const FALLBACK_POLICY_MARKER: &str = ".derived_fallback_policy.";

pub(crate) fn topology_naming_row_contributions<I>(
    declaration: &I,
) -> Vec<TopologyOperatorContributionIntent>
where
    I: TopologyDeclarationMutationPayload,
{
    declaration
        .naming_continuity_matrix()
        .rows
        .into_iter()
        .map(|row| topology_naming_row_contribution::<I>(row))
        .collect()
}

pub(crate) fn topology_fallback_policy_contribution<I>(
    policy: TopologyMutationDerivedFallbackPolicy,
) -> TopologyOperatorContributionIntent
where
    I: TopologyDeclarationMutationPayload,
{
    TopologyOperatorContributionIntent::explanation(
        ForgeQueryExplanationContributionAuthoring::explains_fallback(
            format!(
                "{}{FALLBACK_POLICY_MARKER}{}",
                I::SEMANTIC_FAMILY_KEY,
                policy.as_str()
            ),
            fallback_explanation_detail(policy),
        ),
    )
}

pub(crate) fn topology_naming_row_from_query_evidence(
    evidence: &ForgeQueryDeclarationEntryContributionEvidence,
) -> Option<TopologyMutationNamingRow> {
    let (_, encoded) = evidence.semantic_code().split_once(NAMING_ROW_MARKER)?;
    let mut parts = encoded.split('.');
    let family = family_from_label(parts.next()?)?;
    let scope = scope_from_label(parts.next()?)?;
    let outcome = outcome_from_label(parts.next()?)?;
    if parts.next().is_some() {
        return None;
    }
    Some(TopologyMutationNamingRow {
        family,
        scope,
        outcome,
        reason: evidence.detail().to_string(),
    })
}

pub(crate) fn topology_fallback_policy_from_query_evidence(
    evidence: &ForgeQueryDeclarationEntryContributionEvidence,
) -> Option<TopologyMutationDerivedFallbackPolicy> {
    let (_, label) = evidence
        .semantic_code()
        .split_once(FALLBACK_POLICY_MARKER)?;
    fallback_policy_from_label(label)
}

fn topology_naming_row_contribution<I>(
    row: TopologyMutationNamingRow,
) -> TopologyOperatorContributionIntent
where
    I: TopologyDeclarationMutationPayload,
{
    let semantic_code = format!(
        "{}{NAMING_ROW_MARKER}{}.{}.{}",
        I::SEMANTIC_FAMILY_KEY,
        family_label(row.family),
        scope_label(row.scope),
        outcome_label(row.outcome),
    );
    match row.outcome {
        TopologyMutationNamingOutcome::Preserved => TopologyOperatorContributionIntent::continuity(
            ForgeQueryContinuityContributionAuthoring::preserved(semantic_code, row.reason),
        ),
        TopologyMutationNamingOutcome::Ambiguous | TopologyMutationNamingOutcome::Rejected => {
            TopologyOperatorContributionIntent::explanation(
                ForgeQueryExplanationContributionAuthoring::explains_ambiguity(
                    semantic_code,
                    row.reason,
                ),
            )
        }
    }
}

#[cfg(test)]
fn fallback_policy_label(value: TopologyMutationDerivedFallbackPolicy) -> &'static str {
    value.as_str()
}

fn fallback_policy_from_label(value: &str) -> Option<TopologyMutationDerivedFallbackPolicy> {
    Some(match value {
        "allow_explicit_fallback" => TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback,
        "reject_any_fallback" => TopologyMutationDerivedFallbackPolicy::RejectAnyFallback,
        _ => return None,
    })
}

pub(crate) fn fallback_explanation_detail(
    policy: TopologyMutationDerivedFallbackPolicy,
) -> &'static str {
    match policy {
        TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback => {
            "declared topology mutation allows explicit fallback when runtime reconciliation needs a non-canonical resolution"
        }
        TopologyMutationDerivedFallbackPolicy::RejectAnyFallback => {
            "declared topology mutation rejects fallback and requires canonical continuity if runtime reconciliation would otherwise drift"
        }
    }
}

#[cfg(test)]
mod tests {
    use schema::facade::platform::entities::TopologyEntityKind;

    use super::*;
    use crate::topology_operators::{
        TopologyCreateTopologyEntityDeclaration, TopologyOperatorWorkflowHandleExt,
    };

    #[test]
    fn fallback_policy_semantic_code_round_trips_without_using_detail_text() {
        let facade = forge_query::facade::ForgeQueryApplicationFacade::runtime_backed_default();
        let handle = crate::query_domain::topology_query_domain_entry(&facade)
            .with_operating_context(
                crate::query_domain::topology_current_head_authoritative_context(),
            )
            .validate()
            .expect("current-head context should validate")
            .admit()
            .expect("current-head context should admit");
        let composed = handle
            .orchestrate_topology_operator_with_contributions(
                crate::topology_operators::topology_operator_contribution_workflow(
                    TopologyCreateTopologyEntityDeclaration::new(
                        "fallback-roundtrip.vertex",
                        TopologyEntityKind::Vertex,
                    ),
                ),
            )
            .unwrap_or_else(|_| panic!("topology contribution-composed lane should admit"));
        let expected_code = format!(
            "topology.create_topology_entity{FALLBACK_POLICY_MARKER}{}",
            fallback_policy_label(TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback)
        );
        let evidence = composed
            .contribution_composition()
            .evidence()
            .iter()
            .find(|entry| entry.semantic_code() == expected_code)
            .expect("fallback contribution evidence should be retained");

        assert_eq!(
            topology_fallback_policy_from_query_evidence(evidence),
            Some(TopologyMutationDerivedFallbackPolicy::AllowExplicitFallback)
        );
    }
}
fn family_label(value: TopologyMutationFamily) -> &'static str {
    match value {
        TopologyMutationFamily::CreateTopologyEntity => "create_topology_entity",
        TopologyMutationFamily::RetireTopologyEntity => "retire_topology_entity",
        TopologyMutationFamily::AttachBoundaryMembership => "attach_boundary_membership",
        TopologyMutationFamily::DetachBoundaryMembership => "detach_boundary_membership",
        TopologyMutationFamily::RewireLoopSuccessor => "rewire_loop_successor",
        TopologyMutationFamily::RewireLoopEndpoint => "rewire_loop_endpoint",
        TopologyMutationFamily::AttachShellOrWireMembership => "attach_shell_or_wire_membership",
        TopologyMutationFamily::DetachShellOrWireMembership => "detach_shell_or_wire_membership",
        TopologyMutationFamily::SpliceRadialAdjacency => "splice_radial_adjacency",
        TopologyMutationFamily::DetachRadialAdjacency => "detach_radial_adjacency",
    }
}

fn family_from_label(value: &str) -> Option<TopologyMutationFamily> {
    Some(match value {
        "create_topology_entity" => TopologyMutationFamily::CreateTopologyEntity,
        "retire_topology_entity" => TopologyMutationFamily::RetireTopologyEntity,
        "attach_boundary_membership" => TopologyMutationFamily::AttachBoundaryMembership,
        "detach_boundary_membership" => TopologyMutationFamily::DetachBoundaryMembership,
        "rewire_loop_successor" => TopologyMutationFamily::RewireLoopSuccessor,
        "rewire_loop_endpoint" => TopologyMutationFamily::RewireLoopEndpoint,
        "attach_shell_or_wire_membership" => TopologyMutationFamily::AttachShellOrWireMembership,
        "detach_shell_or_wire_membership" => TopologyMutationFamily::DetachShellOrWireMembership,
        "splice_radial_adjacency" => TopologyMutationFamily::SpliceRadialAdjacency,
        "detach_radial_adjacency" => TopologyMutationFamily::DetachRadialAdjacency,
        _ => return None,
    })
}

fn scope_label(value: TopologyMutationNamingScope) -> &'static str {
    match value {
        TopologyMutationNamingScope::EditedEntityNames => "edited_entity_names",
        TopologyMutationNamingScope::AdjacentEntityNames => "adjacent_entity_names",
    }
}

fn scope_from_label(value: &str) -> Option<TopologyMutationNamingScope> {
    Some(match value {
        "edited_entity_names" => TopologyMutationNamingScope::EditedEntityNames,
        "adjacent_entity_names" => TopologyMutationNamingScope::AdjacentEntityNames,
        _ => return None,
    })
}

fn outcome_label(value: TopologyMutationNamingOutcome) -> &'static str {
    match value {
        TopologyMutationNamingOutcome::Preserved => "preserved",
        TopologyMutationNamingOutcome::Ambiguous => "ambiguous",
        TopologyMutationNamingOutcome::Rejected => "rejected",
    }
}

fn outcome_from_label(value: &str) -> Option<TopologyMutationNamingOutcome> {
    Some(match value {
        "preserved" => TopologyMutationNamingOutcome::Preserved,
        "ambiguous" => TopologyMutationNamingOutcome::Ambiguous,
        "rejected" => TopologyMutationNamingOutcome::Rejected,
        _ => return None,
    })
}
