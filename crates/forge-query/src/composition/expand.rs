use crate::authoring::{
    AuthoredBundleError, AuthoredResultShape, AuthoredQuery, AuthoredQueryBundleRequest,
    CollectionAuthoredQuery, CollectionAuthoredResultShape, DetailAuthoredQuery,
    DetailAuthoredResultShape, QueryAuthoringFamily, ResultShapeAuthoringFamily,
};
use crate::binding::QueryBindingDescriptor;
use crate::canonicalization::{canonicalize_request, CanonicalQueryBundle, QueryCanonicalizationError};

use super::digests::CompositionDigest;
use super::errors::QueryCompositionError;
use super::families::QueryCompositionFamily;
use super::report::CompositionReport;
use super::scopes::{
    expand_scopes, validate_basis_evidence_for_canonical_query, ExpandedScopeArtifact,
    QueryScopeDescriptor,
};
use super::templates::{
    instantiate_template, QueryTemplateDescriptor, TemplateBindingSet, TemplateInstantiationArtifact,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExpandedComposedIntent {
    request: AuthoredQueryBundleRequest,
    report: CompositionReport,
}

impl ExpandedComposedIntent {
    pub fn report(&self) -> &CompositionReport {
        &self.report
    }

    pub fn family(&self) -> QueryCompositionFamily {
        self.report.family()
    }

    pub(crate) fn into_authored_request(self) -> AuthoredQueryBundleRequest {
        self.request
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComposedCanonicalQueryBundle {
    canonical: CanonicalQueryBundle,
    composition: CompositionReport,
}

impl ComposedCanonicalQueryBundle {
    pub fn canonical(&self) -> &CanonicalQueryBundle {
        &self.canonical
    }

    pub fn composition(&self) -> &CompositionReport {
        &self.composition
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GuidedCompositionPath;

impl GuidedCompositionPath {
    pub fn expand_detail_scopes(
        query: DetailAuthoredQuery,
        result_shape: DetailAuthoredResultShape,
        scopes: impl IntoIterator<Item = QueryScopeDescriptor>,
    ) -> Result<(ExpandedScopeArtifact, ExpandedComposedIntent), QueryCompositionError> {
        expand_scoped(query, result_shape, QueryBindingDescriptor::default(), scopes)
    }

    pub fn expand_detail_scopes_with_bindings(
        query: DetailAuthoredQuery,
        result_shape: DetailAuthoredResultShape,
        bindings: QueryBindingDescriptor,
        scopes: impl IntoIterator<Item = QueryScopeDescriptor>,
    ) -> Result<(ExpandedScopeArtifact, ExpandedComposedIntent), QueryCompositionError> {
        expand_scoped(query, result_shape, bindings, scopes)
    }

    pub fn expand_collection_scopes(
        query: CollectionAuthoredQuery,
        result_shape: CollectionAuthoredResultShape,
        scopes: impl IntoIterator<Item = QueryScopeDescriptor>,
    ) -> Result<(ExpandedScopeArtifact, ExpandedComposedIntent), QueryCompositionError> {
        expand_scoped(query, result_shape, QueryBindingDescriptor::default(), scopes)
    }

    pub fn expand_collection_scopes_with_bindings(
        query: CollectionAuthoredQuery,
        result_shape: CollectionAuthoredResultShape,
        bindings: QueryBindingDescriptor,
        scopes: impl IntoIterator<Item = QueryScopeDescriptor>,
    ) -> Result<(ExpandedScopeArtifact, ExpandedComposedIntent), QueryCompositionError> {
        expand_scoped(query, result_shape, bindings, scopes)
    }

    pub fn instantiate_detail_template(
        template: QueryTemplateDescriptor<crate::authoring::DetailFamily, crate::authoring::DetailResultShapeFamily>,
        bindings: TemplateBindingSet,
    ) -> Result<(TemplateInstantiationArtifact, ExpandedComposedIntent), QueryCompositionError> {
        instantiate_typed_template(template, bindings, QueryBindingDescriptor::default())
    }

    pub fn instantiate_detail_template_with_query_bindings(
        template: QueryTemplateDescriptor<crate::authoring::DetailFamily, crate::authoring::DetailResultShapeFamily>,
        bindings: TemplateBindingSet,
        query_bindings: QueryBindingDescriptor,
    ) -> Result<(TemplateInstantiationArtifact, ExpandedComposedIntent), QueryCompositionError> {
        instantiate_typed_template(template, bindings, query_bindings)
    }

    pub fn instantiate_collection_template(
        template: QueryTemplateDescriptor<
            crate::authoring::CollectionFamily,
            crate::authoring::CollectionResultShapeFamily,
        >,
        bindings: TemplateBindingSet,
    ) -> Result<(TemplateInstantiationArtifact, ExpandedComposedIntent), QueryCompositionError> {
        instantiate_typed_template(template, bindings, QueryBindingDescriptor::default())
    }

    pub fn instantiate_collection_template_with_query_bindings(
        template: QueryTemplateDescriptor<
            crate::authoring::CollectionFamily,
            crate::authoring::CollectionResultShapeFamily,
        >,
        bindings: TemplateBindingSet,
        query_bindings: QueryBindingDescriptor,
    ) -> Result<(TemplateInstantiationArtifact, ExpandedComposedIntent), QueryCompositionError> {
        instantiate_typed_template(template, bindings, query_bindings)
    }

    pub fn canonicalize_expanded(
        expanded: ExpandedComposedIntent,
    ) -> Result<ComposedCanonicalQueryBundle, QueryCanonicalizationError> {
        let report = expanded.report.clone();
        let canonical = canonicalize_request(expanded.into_authored_request())?;
        Ok(ComposedCanonicalQueryBundle {
            canonical,
            composition: report,
        })
    }
}

fn expand_scoped<Q, S>(
    query: AuthoredQuery<Q>,
    result_shape: AuthoredResultShape<S>,
    bindings: QueryBindingDescriptor,
    scopes: impl IntoIterator<Item = QueryScopeDescriptor>,
) -> Result<(ExpandedScopeArtifact, ExpandedComposedIntent), QueryCompositionError>
where
    Q: QueryAuthoringFamily,
    S: ResultShapeAuthoringFamily,
{
    let scopes = scopes.into_iter().collect::<Vec<_>>();
    let result = expand_scopes(query.into_raw(), result_shape.into_raw(), bindings, &scopes)?;
    let mut digest_parts = vec![
        format!("family:{}", QueryCompositionFamily::NamedScopeExpansion.as_str()),
        format!("scope_lineage:{}", result.artifact.scope_lineage_digest().as_str()),
    ];
    if let Some(evidence) = result.artifact.basis_evidence() {
        digest_parts.push(format!("basis:{}", evidence.basis_digest()));
    }
    let composition_digest = CompositionDigest::from_parts(&digest_parts);
    let report = CompositionReport::scope_expansion(
        composition_digest,
        result.artifact.scope_lineage_digest().clone(),
        result
            .artifact
            .basis_evidence()
            .map(|evidence| evidence.basis_digest().to_string()),
        result
            .artifact
            .basis_evidence()
            .map(|evidence| evidence.basis_family().clone()),
        result.artifact.counters().clone(),
    );
    let request = lower_to_authored_request(
        result.query,
        result.result_shape,
        result.bindings,
        result.artifact.counters().clone(),
    )?;
    validate_basis_evidence_for_request(
        result.artifact.basis_evidence(),
        &request,
        super::families::ScopeFamily::BasisAwareScope,
        result.artifact.counters().clone(),
    )?;
    let artifact = result.artifact;
    Ok((
        artifact,
        ExpandedComposedIntent {
            request,
            report,
        },
    ))
}

fn instantiate_typed_template<Q, S>(
    template: QueryTemplateDescriptor<Q, S>,
    bindings: TemplateBindingSet,
    query_bindings: QueryBindingDescriptor,
) -> Result<(TemplateInstantiationArtifact, ExpandedComposedIntent), QueryCompositionError>
where
    Q: QueryAuthoringFamily,
    S: ResultShapeAuthoringFamily,
{
    let (family, query, result_shape, slots, basis_evidence) = template.into_parts();
    let instantiated = instantiate_template(
        family,
        query.into_raw(),
        result_shape.into_raw(),
        &slots,
        &bindings,
        basis_evidence,
    )?;
    let mut digest_parts = vec![
        format!("family:{}", QueryCompositionFamily::TemplateInstantiation.as_str()),
        format!("binding:{}", instantiated.artifact.binding_digest().as_str()),
    ];
    if let Some(evidence) = instantiated.artifact.basis_evidence() {
        digest_parts.push(format!("basis:{}", evidence.basis_digest()));
    }
    let composition_digest = CompositionDigest::from_parts(&digest_parts);
    let report = CompositionReport::template_instantiation(
        composition_digest,
        instantiated.artifact.binding_digest().clone(),
        instantiated
            .artifact
            .basis_evidence()
            .map(|evidence| evidence.basis_digest().to_string()),
        instantiated
            .artifact
            .basis_evidence()
            .map(|evidence| evidence.basis_family().clone()),
        instantiated.artifact.counters().clone(),
    );
    let request = lower_to_authored_request(
        instantiated.query,
        instantiated.result_shape,
        query_bindings,
        instantiated.artifact.counters().clone(),
    )?;
    validate_basis_evidence_for_request(
        instantiated.artifact.basis_evidence(),
        &request,
        super::families::ScopeFamily::BasisAwareScope,
        instantiated.artifact.counters().clone(),
    )?;
    let artifact = instantiated.artifact;
    Ok((
        artifact,
        ExpandedComposedIntent {
            request,
            report,
        },
    ))
}

fn lower_to_authored_request(
    query: crate::authoring::RawAuthoredQuery,
    result_shape: crate::authoring::RawAuthoredResultShape,
    bindings: QueryBindingDescriptor,
    counters: super::counters::CompositionCounters,
) -> Result<AuthoredQueryBundleRequest, QueryCompositionError> {
    AuthoredQueryBundleRequest::new(query, result_shape, bindings)
        .map_err(|error| map_authored_bundle_error(error, counters))
}

fn map_authored_bundle_error(
    error: AuthoredBundleError,
    counters: super::counters::CompositionCounters,
) -> QueryCompositionError {
    match error {
        AuthoredBundleError::QueryShapeFamilyMismatch {
            query_family,
            result_shape_family,
        } => QueryCompositionError::lowered_authored_boundary_rejected(
            counters,
            format!(
                "composition lowered a query/result-shape family mismatch: query '{query_family:?}', result shape '{result_shape_family:?}'"
            ),
        ),
        AuthoredBundleError::UnprojectedShapeField {
            source_aspect,
            source_field,
            delivered_name,
        } => QueryCompositionError::lowered_authored_boundary_rejected(
            counters,
            format!(
                "composition lowered an unprojected result-shape field '{}.{}' as '{}'",
                source_aspect, source_field, delivered_name
            ),
        ),
    }
}

fn validate_basis_evidence_for_request(
    basis_evidence: Option<&super::scopes::BasisScopeEvidence>,
    request: &AuthoredQueryBundleRequest,
    family: super::families::ScopeFamily,
    counters: super::counters::CompositionCounters,
) -> Result<(), QueryCompositionError> {
    let Some(evidence) = basis_evidence else {
        return Ok(());
    };
    let canonical = canonicalize_request(request.clone()).map_err(|error| {
        QueryCompositionError::lowered_authored_boundary_rejected(
            counters.clone(),
            format!("composition preview canonicalization failed: {:?}", error),
        )
    })?;
    validate_basis_evidence_for_canonical_query(Some(evidence), &canonical, family, counters)
}
