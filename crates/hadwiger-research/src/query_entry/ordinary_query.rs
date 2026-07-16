//! Hadwiger vocabulary over Query's runtime-installed domain handle.

use worth_foundational::facade::ScalarAspectType;
use worth_query::facade::{domain, read};

use super::{
    HadwigerResearchDomainEntry, HadwigerResearchHandle, HadwigerResearchOperatingContext,
};

const CANDIDATE_ROOT: &str = "HadwigerCandidate";

pub trait HadwigerResearchQueryExt {
    fn research_declarations(
        &self,
        workspace: &worth_query::facade::runtime::WorthQueryWorkspace,
        context: HadwigerResearchOperatingContext,
    ) -> Result<HadwigerResearchHandle, domain::WorthQueryInstalledDomainDeclarationContextDenial>;

    fn candidate_search(
        &self,
    ) -> Result<
        domain::WorthQueryInstalledDomainReadDeclaration<HadwigerResearchDomainEntry>,
        read::WorthQueryReadDeclarationStop,
    >;

    fn candidate_promotion(
        &self,
        label: domain::WorthQuerySessionLabel,
        candidate: HadwigerCandidateContribution,
    ) -> Result<
        domain::WorthQueryInstalledDomainWorkflowDeclaration<HadwigerResearchDomainEntry>,
        domain::WorthQueryMutationDeclarationStop,
    >;
}

impl HadwigerResearchQueryExt
    for domain::WorthQueryInstalledDomainHandle<HadwigerResearchDomainEntry>
{
    fn research_declarations(
        &self,
        workspace: &worth_query::facade::runtime::WorthQueryWorkspace,
        context: HadwigerResearchOperatingContext,
    ) -> Result<HadwigerResearchHandle, domain::WorthQueryInstalledDomainDeclarationContextDenial>
    {
        self.declarations_in(workspace, context)
            .map(HadwigerResearchHandle::new)
    }

    fn candidate_search(
        &self,
    ) -> Result<
        domain::WorthQueryInstalledDomainReadDeclaration<HadwigerResearchDomainEntry>,
        read::WorthQueryReadDeclarationStop,
    > {
        self.read(|query| {
            query.local_collection(
                CANDIDATE_ROOT,
                candidate_schema(),
                |query| {
                    query
                        .project(identity_selector())
                        .project(chromatic_bound_selector())
                },
                |shape| shape.field(identity_field()).field(chromatic_bound_field()),
            )
        })
    }

    fn candidate_promotion(
        &self,
        label: domain::WorthQuerySessionLabel,
        candidate: HadwigerCandidateContribution,
    ) -> Result<
        domain::WorthQueryInstalledDomainWorkflowDeclaration<HadwigerResearchDomainEntry>,
        domain::WorthQueryMutationDeclarationStop,
    > {
        self.mutation(|mutation| {
            mutation
                .set_aspect(
                    domain::WorthQueryAspectTouch::from_authoring_ingress_text("identity.id")?,
                    domain::WorthQueryAuthoredAspectValue::string(candidate.candidate_identity),
                )
                .build_insert(CANDIDATE_ROOT)
        })
        .map(|mutation| mutation.workflow(label))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HadwigerCandidateContribution {
    candidate_identity: String,
}

impl HadwigerCandidateContribution {
    pub fn new(candidate_identity: impl Into<String>) -> Self {
        Self {
            candidate_identity: candidate_identity.into(),
        }
    }
}

fn candidate_schema() -> read::QuerySchemaView {
    read::QuerySchemaView::new(
        "hadwiger-candidate-query",
        [
            read::SchemaFieldView::new(
                read::AspectName::new("identity").expect("static aspect must admit"),
                read::FieldName::new("id").expect("static field must admit"),
                ScalarAspectType::String,
            ),
            read::SchemaFieldView::new(
                read::AspectName::new("colorability").expect("static aspect must admit"),
                read::FieldName::new("lower_bound").expect("static field must admit"),
                ScalarAspectType::String,
            ),
        ],
        [],
    )
}

fn identity_selector() -> read::AspectFieldSelector {
    read::AspectFieldSelector::new("identity", "id").expect("static selector must admit")
}

fn chromatic_bound_selector() -> read::AspectFieldSelector {
    read::AspectFieldSelector::new("colorability", "lower_bound")
        .expect("static selector must admit")
}

fn identity_field() -> read::AuthoredResultShapeField {
    read::AuthoredResultShapeField::new("identity", "id", "identity.id")
        .expect("static result field must admit")
}

fn chromatic_bound_field() -> read::AuthoredResultShapeField {
    read::AuthoredResultShapeField::new("colorability", "lower_bound", "colorability.lower_bound")
        .expect("static result field must admit")
}
