use crate::canonicalization::{
    CanonicalQueryArtifact, CanonicalResultField, CanonicalResultShapeArtifact,
};
use crate::identity::{hash_parts, CanonicalResultShapeDigest};

use super::{
    AuthorizedProjectionCounters, AuthorizedProjectionError, AuthorizedProjectionFailureClass,
    PolicyAspectMask, PolicyInfluencePurpose, PolicyInfluenceSet, ProjectionVisibility,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorizedProjectionIdentity(String);

impl AuthorizedProjectionIdentity {
    pub(crate) fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PolicyFieldInfluenceSet {
    digest: String,
    field_reference_count: usize,
}

impl PolicyFieldInfluenceSet {
    pub(crate) fn new(parts: &[String], field_reference_count: usize) -> Self {
        Self {
            digest: hash_parts(parts),
            field_reference_count,
        }
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn field_reference_count(&self) -> usize {
        self.field_reference_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaskedProjectionArtifact {
    masked_fields: Vec<String>,
    non_disclosing_fields: Vec<String>,
    digest: String,
}

impl MaskedProjectionArtifact {
    pub(crate) fn new(masked_fields: Vec<String>, non_disclosing_fields: Vec<String>) -> Self {
        let mut parts = vec!["masked_projection".to_string()];
        parts.extend(masked_fields.iter().map(|field| format!("masked:{field}")));
        parts.extend(
            non_disclosing_fields
                .iter()
                .map(|field| format!("non_disclosing:{field}")),
        );
        Self {
            masked_fields,
            non_disclosing_fields,
            digest: hash_parts(&parts),
        }
    }

    pub fn masked_fields(&self) -> &[String] {
        &self.masked_fields
    }

    pub fn non_disclosing_fields(&self) -> &[String] {
        &self.non_disclosing_fields
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthorizedProjectionArtifact {
    identity: AuthorizedProjectionIdentity,
    query_digest: String,
    result_shape_digest: String,
    policy_digest: String,
    tenant_schema_basis_digest: String,
    visible_fields: Vec<String>,
    masked_projection: MaskedProjectionArtifact,
    narrowed_result_shape_digest: String,
    influence_set: PolicyFieldInfluenceSet,
    counters: AuthorizedProjectionCounters,
}

impl AuthorizedProjectionArtifact {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        query_digest: &str,
        result_shape_digest: &str,
        policy_digest: &str,
        tenant_schema_basis_digest: &str,
        visible_fields: Vec<String>,
        masked_projection: MaskedProjectionArtifact,
        narrowed_result_shape_digest: String,
        influence_set: PolicyFieldInfluenceSet,
        counters: AuthorizedProjectionCounters,
    ) -> Self {
        let identity = AuthorizedProjectionIdentity::new(hash_parts(&[
            format!("query:{query_digest}"),
            format!("result_shape:{result_shape_digest}"),
            format!("policy:{policy_digest}"),
            format!("tenant_schema:{tenant_schema_basis_digest}"),
            format!("visible:{}", hash_parts(&visible_fields)),
            format!("masked:{}", masked_projection.digest()),
            format!("influence:{}", influence_set.digest()),
        ]));
        Self {
            identity,
            query_digest: query_digest.to_string(),
            result_shape_digest: result_shape_digest.to_string(),
            policy_digest: policy_digest.to_string(),
            tenant_schema_basis_digest: tenant_schema_basis_digest.to_string(),
            visible_fields,
            masked_projection,
            narrowed_result_shape_digest,
            influence_set,
            counters,
        }
    }

    pub fn identity(&self) -> &AuthorizedProjectionIdentity {
        &self.identity
    }

    pub fn query_digest(&self) -> &str {
        &self.query_digest
    }

    pub fn result_shape_digest(&self) -> &str {
        &self.result_shape_digest
    }

    pub fn visible_fields(&self) -> &[String] {
        &self.visible_fields
    }

    pub fn policy_digest(&self) -> &str {
        &self.policy_digest
    }

    pub fn tenant_schema_basis_digest(&self) -> &str {
        &self.tenant_schema_basis_digest
    }

    pub fn masked_projection(&self) -> &MaskedProjectionArtifact {
        &self.masked_projection
    }

    pub fn narrowed_result_shape_digest(&self) -> &str {
        &self.narrowed_result_shape_digest
    }

    pub fn influence_set(&self) -> &PolicyFieldInfluenceSet {
        &self.influence_set
    }

    pub fn counters(&self) -> &AuthorizedProjectionCounters {
        &self.counters
    }
}

pub(crate) fn derive_authorized_projection(
    query: &CanonicalQueryArtifact,
    result_shape: &CanonicalResultShapeArtifact,
    policy_digest: &str,
    tenant_schema_basis_digest: &str,
    mask: &PolicyAspectMask,
    influence: &PolicyInfluenceSet,
    max_projected_fields: usize,
    max_masked_fields: usize,
) -> Result<AuthorizedProjectionArtifact, AuthorizedProjectionError> {
    let mut counters = AuthorizedProjectionCounters::default();
    let mut visible_fields = Vec::new();
    let mut masked_fields = Vec::new();
    let mut non_disclosing_fields = Vec::new();
    let mut influence_parts = Vec::new();

    if query.projection().len() > max_projected_fields {
        return Err(AuthorizedProjectionError::new(
            AuthorizedProjectionFailureClass::ProjectionBudgetExceeded,
            "projection width exceeds policy narrowing budget",
            counters,
        ));
    }

    if mask.masked_entry_count() > max_masked_fields {
        return Err(AuthorizedProjectionError::new(
            AuthorizedProjectionFailureClass::MaskBudgetExceeded,
            "policy mask width exceeds policy narrowing budget",
            counters,
        ));
    }

    for projection in query.projection() {
        counters.inspect_field_reference();
        let field_key = field_key(projection.aspect.as_str(), projection.field.as_str());
        influence_parts.push(format!("projection:{field_key}"));
        match mask.visibility_for_parts(projection.aspect.as_str(), projection.field.as_str()) {
            ProjectionVisibility::Visible => visible_fields.push(field_key),
            ProjectionVisibility::Masked | ProjectionVisibility::DeniedHiddenInfluence => {
                masked_fields.push(field_key)
            }
            ProjectionVisibility::NonDisclosingUseOnly => non_disclosing_fields.push(field_key),
        }
    }

    for field in result_shape.fields() {
        counters.inspect_field_reference();
        let key = result_field_key(field);
        influence_parts.push(format!("result:{key}"));
        match mask.visibility_for_parts(field.source_aspect.as_str(), field.source_field.as_str()) {
            ProjectionVisibility::Visible => {}
            ProjectionVisibility::Masked | ProjectionVisibility::DeniedHiddenInfluence => {
                counters.deny_post_read_redaction();
                return Err(AuthorizedProjectionError::new(
                    AuthorizedProjectionFailureClass::MaskedProjectionRequested,
                    "masked result field cannot be emitted and redacted after read",
                    counters,
                ));
            }
            ProjectionVisibility::NonDisclosingUseOnly => {
                counters.deny_post_read_redaction();
                return Err(AuthorizedProjectionError::new(
                    AuthorizedProjectionFailureClass::NonDisclosingUseWouldBeEmitted,
                    "non-disclosing policy field cannot be emitted",
                    counters,
                ));
            }
        }
    }

    for predicate in query.predicates() {
        counters.inspect_field_reference();
        let key = field_key(predicate.aspect.as_str(), predicate.field.as_str());
        influence_parts.push(format!("predicate:{key}"));
        match mask.visibility_for_parts(predicate.aspect.as_str(), predicate.field.as_str()) {
            ProjectionVisibility::Visible | ProjectionVisibility::NonDisclosingUseOnly => {}
            ProjectionVisibility::Masked | ProjectionVisibility::DeniedHiddenInfluence => {
                counters.deny_hidden_predicate();
                return Err(AuthorizedProjectionError::new(
                    AuthorizedProjectionFailureClass::MaskedPredicateInfluence,
                    "masked predicate influence would leak hidden truth",
                    counters,
                ));
            }
        }
    }

    for ordering in query.ordering() {
        counters.inspect_field_reference();
        let key = field_key(ordering.aspect.as_str(), ordering.field.as_str());
        influence_parts.push(format!("ordering:{key}"));
        match mask.visibility_for_parts(ordering.aspect.as_str(), ordering.field.as_str()) {
            ProjectionVisibility::Visible => {}
            ProjectionVisibility::Masked
            | ProjectionVisibility::DeniedHiddenInfluence
            | ProjectionVisibility::NonDisclosingUseOnly => {
                counters.deny_hidden_ordering();
                return Err(AuthorizedProjectionError::new(
                    AuthorizedProjectionFailureClass::MaskedOrderingInfluence,
                    "masked ordering influence would leak hidden truth",
                    counters,
                ));
            }
        }
    }

    for entry in influence.entries() {
        counters.inspect_field_reference();
        let field = entry.field();
        let key = field_key(field.aspect().as_str(), field.field().as_str());
        influence_parts.push(format!("{}:{key}", entry.purpose().as_str()));
        let visibility = mask.visibility_for(field);
        match entry.purpose() {
            PolicyInfluencePurpose::Grouping => match visibility {
                ProjectionVisibility::Visible => {}
                ProjectionVisibility::Masked
                | ProjectionVisibility::DeniedHiddenInfluence
                | ProjectionVisibility::NonDisclosingUseOnly => {
                    counters.deny_hidden_grouping();
                    return Err(AuthorizedProjectionError::new(
                        AuthorizedProjectionFailureClass::MaskedGroupingInfluence,
                        "masked grouping influence would leak hidden truth",
                        counters,
                    ));
                }
            },
            PolicyInfluencePurpose::DerivedResultField => match visibility {
                ProjectionVisibility::Visible => {}
                ProjectionVisibility::Masked
                | ProjectionVisibility::DeniedHiddenInfluence
                | ProjectionVisibility::NonDisclosingUseOnly => {
                    counters.deny_hidden_derived_field();
                    return Err(AuthorizedProjectionError::new(
                        AuthorizedProjectionFailureClass::MaskedDerivedFieldInfluence,
                        "masked derived result influence would leak hidden truth",
                        counters,
                    ));
                }
            },
            PolicyInfluencePurpose::TemplatePredicate => match visibility {
                ProjectionVisibility::Visible | ProjectionVisibility::NonDisclosingUseOnly => {}
                ProjectionVisibility::Masked | ProjectionVisibility::DeniedHiddenInfluence => {
                    counters.deny_hidden_predicate();
                    return Err(AuthorizedProjectionError::new(
                        AuthorizedProjectionFailureClass::MaskedPredicateInfluence,
                        "masked template predicate influence would leak hidden truth",
                        counters,
                    ));
                }
            },
            PolicyInfluencePurpose::Aggregation => match visibility {
                ProjectionVisibility::Visible => {}
                ProjectionVisibility::Masked
                | ProjectionVisibility::DeniedHiddenInfluence
                | ProjectionVisibility::NonDisclosingUseOnly => {
                    counters.deny_hidden_aggregation();
                    return Err(AuthorizedProjectionError::new(
                        AuthorizedProjectionFailureClass::MaskedAggregationInfluence,
                        "masked aggregation influence would leak hidden truth",
                        counters,
                    ));
                }
            },
            PolicyInfluencePurpose::Cursor => match visibility {
                ProjectionVisibility::Visible => {}
                ProjectionVisibility::Masked
                | ProjectionVisibility::DeniedHiddenInfluence
                | ProjectionVisibility::NonDisclosingUseOnly => {
                    counters.deny_hidden_cursor();
                    return Err(AuthorizedProjectionError::new(
                        AuthorizedProjectionFailureClass::MaskedCursorInfluence,
                        "masked cursor influence would leak hidden truth",
                        counters,
                    ));
                }
            },
            PolicyInfluencePurpose::ViewMembership => match visibility {
                ProjectionVisibility::Visible => {}
                ProjectionVisibility::Masked
                | ProjectionVisibility::DeniedHiddenInfluence
                | ProjectionVisibility::NonDisclosingUseOnly => {
                    counters.deny_hidden_view_membership();
                    return Err(AuthorizedProjectionError::new(
                        AuthorizedProjectionFailureClass::MaskedViewMembershipInfluence,
                        "masked view-membership influence would leak hidden truth",
                        counters,
                    ));
                }
            },
        }
    }

    counters.set_authorized_projection_width(visible_fields.len());
    counters.set_masked_projection_entry_count(masked_fields.len() + non_disclosing_fields.len());
    let narrowed_result_shape_digest = narrowed_result_shape_digest(result_shape.fields(), mask)
        .map_err(|failure| {
            AuthorizedProjectionError::new(
                failure,
                "result shape contains hidden influence",
                counters.clone(),
            )
        })?;
    let influence_set =
        PolicyFieldInfluenceSet::new(&influence_parts, counters.inspected_field_reference_count());
    let masked_projection = MaskedProjectionArtifact::new(masked_fields, non_disclosing_fields);
    Ok(AuthorizedProjectionArtifact::new(
        query.digest().as_str(),
        result_shape.digest().as_str(),
        policy_digest,
        tenant_schema_basis_digest,
        visible_fields,
        masked_projection,
        narrowed_result_shape_digest,
        influence_set,
        counters,
    ))
}

fn narrowed_result_shape_digest(
    fields: &[CanonicalResultField],
    mask: &PolicyAspectMask,
) -> Result<String, AuthorizedProjectionFailureClass> {
    let mut parts = vec!["narrowed_result_shape".to_string()];
    for field in fields {
        match mask.visibility_for_parts(field.source_aspect.as_str(), field.source_field.as_str()) {
            ProjectionVisibility::Visible => parts.push(field.digest_part()),
            ProjectionVisibility::Masked | ProjectionVisibility::DeniedHiddenInfluence => {
                return Err(AuthorizedProjectionFailureClass::MaskedProjectionRequested);
            }
            ProjectionVisibility::NonDisclosingUseOnly => {
                return Err(AuthorizedProjectionFailureClass::NonDisclosingUseWouldBeEmitted);
            }
        }
    }
    Ok(CanonicalResultShapeDigest::from_parts(&parts)
        .as_str()
        .to_string())
}

fn result_field_key(field: &CanonicalResultField) -> String {
    field_key(field.source_aspect.as_str(), field.source_field.as_str())
}

fn field_key(aspect: &str, field: &str) -> String {
    format!("{aspect}.{field}")
}
