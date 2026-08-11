use crate::collection::{CollectionResultFamily, DerivedFieldComputationClass};
use crate::identity::ResultDigest;

pub(super) fn synthetic_rows(
    preflight: &crate::basis::ExecutionPreflightBundle,
    basis_digest: &str,
    requested_path_class: Option<&str>,
    materialization_identity: Option<&str>,
) -> Vec<String> {
    let historical_query_identity = preflight.plan().query().validated_query_digest().as_str();
    let collection = preflight.plan().collection();
    let is_cdc_collection = collection
        .map(|collection| {
            matches!(
                collection.post_read_shaping().result_family(),
                CollectionResultFamily::CdcCollection
            )
        })
        .unwrap_or(false);
    let is_display_label_derived = collection
        .map(|collection| {
            matches!(
                collection
                    .post_read_shaping()
                    .derived_field_plan()
                    .computation_class(),
                DerivedFieldComputationClass::DisplayLabelFromIdentityAndProfile
            )
        })
        .unwrap_or(false);

    (0..preflight.plan().result_shape().binding_count())
        .map(|index| {
            if is_cdc_collection {
                format!(
                    "cdc:{}:{}:{}",
                    historical_query_identity, basis_digest, index
                )
            } else if is_display_label_derived {
                format!(
                    "derived:display_label:{}:{}:{}",
                    historical_query_identity, basis_digest, index
                )
            } else {
                format!(
                    "result:{}:{}:{}:{}:{}",
                    historical_query_identity,
                    basis_digest,
                    requested_path_class.unwrap_or("runtime"),
                    materialization_identity.unwrap_or("none"),
                    index
                )
            }
        })
        .collect()
}

pub(super) fn synthetic_result_digest(
    preflight: &crate::basis::ExecutionPreflightBundle,
    basis_digest: &str,
    rows: &[String],
    requested_path_class: Option<&str>,
    materialization_identity: Option<&str>,
) -> ResultDigest {
    let historical_query_identity = preflight.plan().query().validated_query_digest().as_str();
    let collection = preflight.plan().collection();
    let is_cdc_collection = collection
        .map(|collection| {
            matches!(
                collection.post_read_shaping().result_family(),
                CollectionResultFamily::CdcCollection
            )
        })
        .unwrap_or(false);
    let is_display_label_derived = collection
        .map(|collection| {
            matches!(
                collection
                    .post_read_shaping()
                    .derived_field_plan()
                    .computation_class(),
                DerivedFieldComputationClass::DisplayLabelFromIdentityAndProfile
            )
        })
        .unwrap_or(false);

    ResultDigest::from_parts(
        &rows
            .iter()
            .cloned()
            .chain(std::iter::once(format!(
                "query:{}",
                historical_query_identity
            )))
            .chain(std::iter::once(format!("basis:{basis_digest}")))
            .chain(std::iter::once(format!(
                "requested_path_class:{}",
                requested_path_class.unwrap_or("runtime")
            )))
            .chain(std::iter::once(format!(
                "materialization_identity:{}",
                materialization_identity.unwrap_or("none")
            )))
            .chain(std::iter::once(format!(
                "collection_result_family:{}",
                if is_cdc_collection {
                    "cdc_collection"
                } else {
                    "ordinary_collection"
                }
            )))
            .chain(std::iter::once(
                "aggregate_family:none_admitted_yet".to_string(),
            ))
            .chain(std::iter::once(format!(
                "derived_field_family:{}",
                if is_display_label_derived {
                    "display_label"
                } else {
                    "none_admitted_yet"
                }
            )))
            .collect::<Vec<_>>(),
    )
}
