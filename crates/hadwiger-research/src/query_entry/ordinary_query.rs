//! Hadwiger-specific declarations over Query's ordinary capability facade.

use worth_query::facade::{domain, read};

const CANDIDATE_ROOT: &str = "HadwigerCandidate";

pub fn declare_candidate_search(
) -> Result<read::WorthQueryReadDeclaration, read::WorthQueryReadDeclarationStop> {
    read::declare(|query| {
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

impl domain::WorthQueryDomainWorkflowContribution for HadwigerCandidateContribution {
    type Error = domain::WorthQueryMutationDeclarationStop;

    fn contribute(&self) -> Result<domain::WorthQueryMutationDeclaration, Self::Error> {
        domain::declare_mutation(|mutation| {
            mutation
                .set_aspect(
                    domain::WorthQueryAspectTouch::from_authoring_ingress_text("identity.id")?,
                    domain::WorthQueryAuthoredAspectValue::string(&self.candidate_identity),
                )
                .build_insert(CANDIDATE_ROOT)
        })
    }
}

pub fn declare_candidate_promotion(
    label: domain::WorthQuerySessionLabel,
    candidate_identity: impl Into<String>,
) -> Result<domain::WorthQueryDomainWorkflowDeclaration, domain::WorthQueryMutationDeclarationStop>
{
    domain::declare(
        label,
        HadwigerCandidateContribution::new(candidate_identity),
    )
}

fn candidate_schema() -> read::QuerySchemaView {
    read::QuerySchemaView::new(
        "hadwiger-candidate-query",
        [
            read::SchemaFieldView::new(
                read::AspectName::new("identity").expect("static aspect must admit"),
                read::FieldName::new("id").expect("static field must admit"),
                read::SchemaFieldKind::String,
            ),
            read::SchemaFieldView::new(
                read::AspectName::new("colorability").expect("static aspect must admit"),
                read::FieldName::new("lower_bound").expect("static field must admit"),
                read::SchemaFieldKind::String,
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
