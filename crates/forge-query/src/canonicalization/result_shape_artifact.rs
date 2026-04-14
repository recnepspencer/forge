use std::collections::{BTreeMap, BTreeSet};

use crate::authoring::{AuthoredResultShapeField, ResultShapeFamily};
use crate::diagnostics::{CanonicalizationCounters, CanonicalizationWarning, NormalizationEvent};
use crate::identity::CanonicalResultShapeDigest;
use crate::result_shape::canonical_result_shape_family_digest_part;

use super::artifacts::{CanonicalResultField, CanonicalResultShapeArtifact};
use super::errors::QueryCanonicalizationError;

pub(super) fn build_result_shape_artifact(
    family: ResultShapeFamily,
    fields: &[AuthoredResultShapeField],
    projection_field_set: &BTreeSet<(String, String)>,
    warnings: &mut Vec<CanonicalizationWarning>,
    events: &mut Vec<NormalizationEvent>,
    counters: &mut CanonicalizationCounters,
) -> Result<CanonicalResultShapeArtifact, QueryCanonicalizationError> {
    let mut seen = BTreeSet::new();
    let mut delivered_name_sources = BTreeMap::<String, (String, String)>::new();
    let mut ordered = Vec::new();
    let mut duplicate_result_fields = Vec::new();

    for field in fields {
        let canonical = CanonicalResultField {
            source_aspect: field.source_aspect().to_string(),
            source_field: field.source_field().to_string(),
            delivered_name: field.delivered_name().to_string(),
        };
        if !projection_field_set.contains(&canonical.source_projection_key()) {
            return Err(QueryCanonicalizationError::UnprojectedShapeField {
                source_aspect: canonical.source_aspect.clone(),
                source_field: canonical.source_field.clone(),
                delivered_name: canonical.delivered_name.clone(),
            });
        }

        match delivered_name_sources.get(&canonical.delivered_name) {
            Some((source_aspect, source_field))
                if source_aspect != &canonical.source_aspect
                    || source_field != &canonical.source_field =>
            {
                return Err(QueryCanonicalizationError::AmbiguousShapeAliasIdentity {
                    delivered_name: canonical.delivered_name.clone(),
                    first_source_aspect: source_aspect.clone(),
                    first_source_field: source_field.clone(),
                    second_source_aspect: canonical.source_aspect.clone(),
                    second_source_field: canonical.source_field.clone(),
                });
            }
            Some(_) => {}
            None => {
                delivered_name_sources.insert(
                    canonical.delivered_name.clone(),
                    (
                        canonical.source_aspect.clone(),
                        canonical.source_field.clone(),
                    ),
                );
            }
        }

        if !seen.insert(canonical.clone()) {
            duplicate_result_fields.push(canonical.delivered_name.clone());
            counters.result_shape_deduplication_count += 1;
            continue;
        }
        ordered.push(canonical);
    }

    ordered.sort();
    events.extend(
        ordered
            .iter()
            .map(|canonical| NormalizationEvent::ResultFieldRetained {
                source_aspect: canonical.source_aspect.clone(),
                source_field: canonical.source_field.clone(),
                delivered_name: canonical.delivered_name.clone(),
            }),
    );
    duplicate_result_fields.sort();
    for delivered_name in duplicate_result_fields {
        warnings.push(CanonicalizationWarning::DuplicateResultFieldCollapsed {
            delivered_name: delivered_name.clone(),
        });
        events.push(NormalizationEvent::ResultFieldCollapsedDuplicate { delivered_name });
    }
    let mut digest_parts = vec![canonical_result_shape_family_digest_part(&family)];
    digest_parts.extend(ordered.iter().map(CanonicalResultField::digest_part));

    Ok(CanonicalResultShapeArtifact {
        digest: CanonicalResultShapeDigest::from_parts(&digest_parts),
        family,
        fields: ordered,
    })
}
