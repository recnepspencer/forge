use std::sync::Arc;

use forge_query::facade::runtime::{
    CustomInvariantDescriptor, CustomInvariantExecutionContext, CustomInvariantExecutionError,
    CustomInvariantOperationalMetadata, CustomInvariantPreparationError,
    CustomInvariantRegistration, CustomInvariantRegistrationError, CustomInvariantRule,
    CustomInvariantRuleId, CustomInvariantScopePlanner, CustomInvariantSemanticIdentity,
    CustomInvariantSemanticVersion, CustomInvariantVerdict, InvariantCostClass,
    InvariantExecutionPoint, InvariantFailureEffect, InvariantGroup, InvariantGroupSet,
};
use forge_relational::facade::identity::EntityId;

use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, HadwigerArtifactAuthorityOwner, HadwigerArtifactCore,
    HadwigerArtifactKind, HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};
use crate::domain_artifacts::HadwigerCanonicalArtifact;

use super::catalog::{HadwigerResearchInvariantCatalog, ResearchGraphInvariantFamily};
use super::runtime_vocabulary as vocab;

const FAILURE_RESIDENCY_RELATIONS: [u32; 4] = [
    vocab::FAILURE_HAS_NEGATIVE_EVIDENCE.id(),
    vocab::FAILURE_AFFECTS_ARTIFACT.id(),
    vocab::FAILURE_HAS_SCOPE.id(),
    vocab::FAILURE_HAS_REACTIVATION_HINT.id(),
];
const SUPPRESSION_RELATIONS: [u32; 1] = [vocab::PLAN_HAS_SUPPRESSION_PROOF.id()];
const HYPOTHESIS_RELATIONS: [u32; 1] = [vocab::HYPOTHESIS_HAS_STATUS.id()];
const BRANCH_PROMOTION_RELATIONS: [u32; 1] = [vocab::FRONTIER_HAS_AUTHORITY_POSTURE.id()];
const EXECUTABLE_EXPERIMENT_RELATIONS: [u32; 1] = [vocab::PLAN_HAS_QUERY_READINESS_COUNTER.id()];

#[derive(Clone, Debug)]
pub struct HadwigerResearchInvariantRegistrationChecked {
    core: HadwigerArtifactCore,
    descriptors: Vec<CustomInvariantDescriptor>,
    registrations: Vec<CustomInvariantRegistration>,
    registration_digests: Vec<String>,
}

impl HadwigerResearchInvariantRegistrationChecked {
    pub(crate) fn new(
        catalog: &HadwigerResearchInvariantCatalog,
        registrations: Vec<CustomInvariantRegistration>,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        let descriptors = registrations
            .iter()
            .map(|registration| registration.descriptor().clone())
            .collect::<Vec<_>>();
        let registration_digests = descriptors
            .iter()
            .map(descriptor_stable_token)
            .collect::<Vec<_>>();
        let core = artifact_core(
            HadwigerArtifactKind::ResearchGraphInvariantRegistrationChecked,
            HadwigerArtifactAuthorityOwner::HadwigerArtifactBuilder,
            HadwigerArtifactSourceReference::ArtifactConstruction {
                operation: "research_graph_invariant_registration_checked".to_string(),
            },
            vec![catalog.reference()],
            registration_payload(catalog, &descriptors),
        )?;
        Ok(Self {
            core,
            descriptors,
            registrations,
            registration_digests,
        })
    }

    pub fn descriptors(&self) -> &[CustomInvariantDescriptor] {
        &self.descriptors
    }

    pub fn custom_invariant_registrations(&self) -> &[CustomInvariantRegistration] {
        &self.registrations
    }

    pub fn registration_digests(&self) -> &[String] {
        &self.registration_digests
    }

    pub fn registers_query_custom_invariant_authority(&self) -> bool {
        true
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(HadwigerResearchInvariantRegistrationChecked, core);

#[derive(Clone, Debug)]
struct ResearchGraphCustomInvariantRule {
    family: ResearchGraphInvariantFamily,
}

#[derive(Clone, Debug)]
struct ResearchGraphRuleScope {
    relevant_entities: Vec<EntityId>,
    planned_relevant_entity_count: usize,
    planned_relevant_relation_count: usize,
    traversal_exhausted: bool,
}

impl ResearchGraphCustomInvariantRule {
    fn new(family: ResearchGraphInvariantFamily) -> Self {
        Self { family }
    }
}

impl CustomInvariantRule for ResearchGraphCustomInvariantRule {
    type Scope = ResearchGraphRuleScope;

    fn descriptor(&self) -> CustomInvariantDescriptor {
        CustomInvariantDescriptor {
            identity: CustomInvariantSemanticIdentity {
                rule_id: CustomInvariantRuleId::new(self.family.query_invariant_family()),
                semantic_version: CustomInvariantSemanticVersion::new(1, 0),
            },
            display_name: Arc::from(display_name(self.family)),
            operational: CustomInvariantOperationalMetadata {
                execution_point: InvariantExecutionPoint::CommitBoundary,
                groups: InvariantGroupSet::of(InvariantGroup::SchemaCompliance),
                cost_class: InvariantCostClass::Touched,
                failure_effect: InvariantFailureEffect::BlockCommit,
            },
        }
    }

    fn prepare_scope(
        &self,
        planner: &mut CustomInvariantScopePlanner<'_>,
    ) -> Result<Self::Scope, CustomInvariantPreparationError> {
        let touched = planner.touched();
        let relevant_entities = touched
            .visible_entity_ids()
            .iter()
            .copied()
            .filter(|entity_id| {
                planner
                    .relations()
                    .entity_kind(*entity_id)
                    .is_some_and(|kind| self.relevant_entity_kind(kind.as_u32()))
            })
            .collect::<Vec<_>>();
        let traversal = planner
            .traversal()
            .walk_outgoing_from(&relevant_entities, 2)?;
        Ok(ResearchGraphRuleScope {
            relevant_entities,
            planned_relevant_entity_count: touched
                .planned_entity_creates()
                .iter()
                .filter(|create| self.relevant_entity_kind(create.kind_id().as_u32()))
                .count(),
            planned_relevant_relation_count: touched
                .planned_relation_creates()
                .iter()
                .filter(|create| self.relevant_relation_kind(create.kind_id().as_u32()))
                .count(),
            traversal_exhausted: traversal.frontier_exhausted(),
        })
    }

    fn evaluate(
        &self,
        context: &CustomInvariantExecutionContext<'_>,
        scope: &Self::Scope,
    ) -> Result<CustomInvariantVerdict, CustomInvariantExecutionError> {
        if !scope.traversal_exhausted {
            return Ok(CustomInvariantVerdict::Violation);
        }
        if scope.relevant_entities.is_empty()
            && scope.planned_relevant_entity_count == 0
            && scope.planned_relevant_relation_count == 0
        {
            return Ok(CustomInvariantVerdict::Pass);
        }
        if scope
            .relevant_entities
            .iter()
            .all(|entity| self.visible_entity_satisfies(context, *entity))
        {
            Ok(CustomInvariantVerdict::Pass)
        } else {
            Ok(CustomInvariantVerdict::Violation)
        }
    }
}

impl ResearchGraphCustomInvariantRule {
    fn relevant_entity_kind(&self, kind_id: u32) -> bool {
        match self.family {
            ResearchGraphInvariantFamily::FailureResidency => kind_id == vocab::FAILURE.id(),
            ResearchGraphInvariantFamily::SuppressionRelation => {
                kind_id == vocab::EXPERIMENT_PLAN.id()
            }
            ResearchGraphInvariantFamily::HypothesisLifecycle => kind_id == vocab::HYPOTHESIS.id(),
            ResearchGraphInvariantFamily::BranchPromotion => kind_id == vocab::FRONTIER_STATE.id(),
            ResearchGraphInvariantFamily::ExecutableExperimentAdmission => {
                kind_id == vocab::EXPERIMENT_PLAN.id()
            }
        }
    }

    fn relevant_relation_kind(&self, kind_id: u32) -> bool {
        required_relation_kinds(self.family)
            .iter()
            .any(|required| *required == kind_id)
    }

    fn visible_entity_satisfies(
        &self,
        context: &CustomInvariantExecutionContext<'_>,
        entity: EntityId,
    ) -> bool {
        let actual = context
            .relations()
            .outgoing_relations_for_entity(entity)
            .into_iter()
            .filter_map(|relation_id| context.relations().relation(relation_id))
            .map(|relation| relation.kind_id.as_u32())
            .collect::<Vec<_>>();
        required_relation_kinds(self.family)
            .iter()
            .all(|required| actual.contains(required))
    }
}

pub(crate) fn registrations_for_catalog(
    catalog: &HadwigerResearchInvariantCatalog,
) -> Result<Vec<CustomInvariantRegistration>, CustomInvariantRegistrationError> {
    catalog
        .rules()
        .iter()
        .map(|rule| {
            CustomInvariantRegistration::new(ResearchGraphCustomInvariantRule::new(rule.family()))
        })
        .collect()
}

fn required_relation_kinds(family: ResearchGraphInvariantFamily) -> &'static [u32] {
    match family {
        ResearchGraphInvariantFamily::FailureResidency => &FAILURE_RESIDENCY_RELATIONS,
        ResearchGraphInvariantFamily::SuppressionRelation => &SUPPRESSION_RELATIONS,
        ResearchGraphInvariantFamily::HypothesisLifecycle => &HYPOTHESIS_RELATIONS,
        ResearchGraphInvariantFamily::BranchPromotion => &BRANCH_PROMOTION_RELATIONS,
        ResearchGraphInvariantFamily::ExecutableExperimentAdmission => {
            &EXECUTABLE_EXPERIMENT_RELATIONS
        }
    }
}

fn display_name(family: ResearchGraphInvariantFamily) -> &'static str {
    match family {
        ResearchGraphInvariantFamily::FailureResidency => "Hadwiger Failure Residency Invariant",
        ResearchGraphInvariantFamily::SuppressionRelation => {
            "Hadwiger Suppression Relation Invariant"
        }
        ResearchGraphInvariantFamily::HypothesisLifecycle => {
            "Hadwiger Hypothesis Lifecycle Invariant"
        }
        ResearchGraphInvariantFamily::BranchPromotion => "Hadwiger Branch Promotion Invariant",
        ResearchGraphInvariantFamily::ExecutableExperimentAdmission => {
            "Hadwiger Executable Experiment Admission Invariant"
        }
    }
}

fn registration_payload(
    catalog: &HadwigerResearchInvariantCatalog,
    descriptors: &[CustomInvariantDescriptor],
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("posture", "custom_invariant_registrations_ready"),
        HadwigerArtifactPayloadEntry::text("catalog", catalog.artifact_digest().stable_token()),
    ];
    for descriptor in descriptors {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "custom_invariant",
            descriptor_stable_token(descriptor),
        ));
    }
    payload
}

fn descriptor_stable_token(descriptor: &CustomInvariantDescriptor) -> String {
    format!(
        "{}:{}.{:?}:{:?}:{:?}:{:?}",
        descriptor.identity.rule_id.as_str(),
        descriptor.identity.semantic_version.major,
        descriptor.identity.semantic_version.minor,
        descriptor.operational.execution_point,
        descriptor.operational.cost_class,
        descriptor.operational.failure_effect
    )
}
