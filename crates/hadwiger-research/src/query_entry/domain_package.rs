use worth_query::facade::domain::{
    WorthQueryCapabilityFamily, WorthQueryConfigSectionFamily,
    WorthQueryDeclarationEntryContributionCategoryFamily, WorthQueryDomainIdentityDeclaration,
    WorthQueryDomainIdentityName, WorthQueryDomainIdentityNamespace,
    WorthQueryDomainInvariantDefinition, WorthQueryDomainInvariantPredicate,
    WorthQueryDomainPackage, WorthQueryDomainSemanticVersion,
};
use worth_relational::facade::identity::KindId;

use super::HadwigerResearchDomainEntry;
use crate::domain_declarations::hadwiger_declaration_family_definitions;
use crate::research_graph_invariants::runtime_vocabulary as vocab;

pub fn hadwiger_research_domain_package() -> WorthQueryDomainPackage<HadwigerResearchDomainEntry> {
    WorthQueryDomainPackage::declare(
        HadwigerResearchDomainEntry,
        WorthQueryDomainIdentityDeclaration::new(
            WorthQueryDomainIdentityNamespace::new("WORTH.hadwiger")
                .expect("static namespace must admit"),
            WorthQueryDomainIdentityName::new("research").expect("static name must admit"),
            WorthQueryDomainSemanticVersion::new(1, 0),
        ),
    )
    .requires_capability(WorthQueryCapabilityFamily::QueryRead)
    .requires_capability(WorthQueryCapabilityFamily::QueryComposition)
    .requires_capability(WorthQueryCapabilityFamily::WorkflowOrchestration)
    .requires_capability(WorthQueryCapabilityFamily::HistoricalEvaluation)
    .requires_configuration(WorthQueryConfigSectionFamily::Query)
    .requires_configuration(WorthQueryConfigSectionFamily::Relational)
    .invariant(requires_relations(
        "failure_residency",
        vocab::FAILURE.id(),
        [
            vocab::FAILURE_HAS_NEGATIVE_EVIDENCE.id(),
            vocab::FAILURE_AFFECTS_ARTIFACT.id(),
            vocab::FAILURE_HAS_SCOPE.id(),
            vocab::FAILURE_HAS_REACTIVATION_HINT.id(),
        ],
    ))
    .invariant(requires_relations(
        "suppression_relation",
        vocab::EXPERIMENT_PLAN.id(),
        [vocab::PLAN_HAS_SUPPRESSION_PROOF.id()],
    ))
    .invariant(requires_relations(
        "hypothesis_lifecycle",
        vocab::HYPOTHESIS.id(),
        [vocab::HYPOTHESIS_HAS_STATUS.id()],
    ))
    .invariant(requires_relations(
        "branch_promotion",
        vocab::FRONTIER_STATE.id(),
        [vocab::FRONTIER_HAS_AUTHORITY_POSTURE.id()],
    ))
    .invariant(requires_relations(
        "executable_experiment_admission",
        vocab::EXPERIMENT_PLAN.id(),
        [vocab::PLAN_HAS_QUERY_READINESS_COUNTER.id()],
    ))
    .declaration_families(hadwiger_declaration_family_definitions())
    .permits_contribution(WorthQueryDeclarationEntryContributionCategoryFamily::Admission)
    .permits_contribution(WorthQueryDeclarationEntryContributionCategoryFamily::SupportTraceability)
    .permits_contribution(WorthQueryDeclarationEntryContributionCategoryFamily::InvariantCapability)
    .permits_contribution(WorthQueryDeclarationEntryContributionCategoryFamily::WorkflowPreview)
}

fn requires_relations(
    name: &'static str,
    entity_kind: u32,
    relation_kinds: impl IntoIterator<Item = u32>,
) -> WorthQueryDomainInvariantDefinition {
    WorthQueryDomainInvariantDefinition::new(
        WorthQueryDomainIdentityName::new(name).expect("static invariant name must admit"),
        WorthQueryDomainSemanticVersion::new(1, 0),
        WorthQueryDomainInvariantPredicate::requires_outgoing_relations(
            vec![KindId::new(entity_kind)],
            relation_kinds.into_iter().map(KindId::new).collect(),
            2,
        ),
    )
}
