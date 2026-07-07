use std::collections::BTreeSet;

use forge_query::facade::foundation::{AspectFieldKey, AspectName};
use forge_foundational::facade::AspectKey;
use forge_foundational::facade::{CanonicalFieldPath, FieldKey};
use forge_query::facade::{
    ForgeQueryAspectTouch, ForgeQueryComputedBuilder, ForgeQueryLiveViewBuilder,
};

use crate::facade::platform::aspects::{
    Aspect, DiagnosticsAspect, GeometryAspect, LineageAspect, NamingAspect, TopologyAspect,
};
use crate::facade::{
    query_aspect_path_strings, query_aspect_paths_from_set, QueryAspectFamily, QueryAspectPath,
    QueryCollection, QueryLiveField, QuerySchemaBasis,
};

#[test]
fn query_collections_and_schema_bases_have_stable_names() {
    assert_eq!(QueryCollection::TopologyEntity.as_str(), "TopologyEntity");
    assert_eq!(
        QueryCollection::TopologyEquivalenceContract.as_str(),
        "TopologyEquivalenceContract"
    );
    assert_eq!(
        QuerySchemaBasis::AuthoritativeTopologyTruth.as_str(),
        ".schema.authoritative_topology_truth"
    );
    assert_eq!(
        QuerySchemaBasis::TopologyValidationComputed.as_str(),
        ".schema.computed.topology_validation"
    );
    assert_eq!(
        QuerySchemaBasis::TopologyDomainQuery.as_str(),
        ".schema.domain.topology_query"
    );
    assert_eq!(QueryCollection::ALL.len(), 9);
    assert_eq!(QuerySchemaBasis::ALL.len(), 11);
}

#[test]
fn query_aspect_paths_are_valid_forge_query_aspect_field_paths() {
    for path in QueryAspectPath::ALL {
        let value = path.as_str();
        let (section, field) = value
            .split_once('.')
            .expect("query aspect path must use aspect.field form");
        assert!(!section.is_empty());
        assert!(!field.is_empty());
        assert_eq!(section, path.section());
        assert_eq!(field, path.field());
    }
}

#[test]
fn query_aspect_paths_normalize_legacy_single_segment_aspects() {
    assert_eq!(
        Aspect::Diagnostics(DiagnosticsAspect::Decisions).as_str(),
        "diagnostics"
    );
    assert_eq!(
        QueryAspectPath::from_aspect(Aspect::Diagnostics(DiagnosticsAspect::Decisions)).as_str(),
        "diagnostics.decisions"
    );
    assert_eq!(
        Aspect::Lineage(LineageAspect::Provenance).as_str(),
        "lineage"
    );
    assert_eq!(
        QueryAspectPath::from_aspect(Aspect::Lineage(LineageAspect::Provenance)).as_str(),
        "lineage.provenance"
    );
}

#[test]
fn touched_aspect_sets_convert_to_deterministic_query_paths() {
    let aspects = BTreeSet::from([
        Aspect::Diagnostics(DiagnosticsAspect::Decisions),
        Aspect::Topology(TopologyAspect::Boundary),
        Aspect::Naming(NamingAspect::PersistentName),
    ]);

    let paths = query_aspect_paths_from_set(&aspects);
    assert_eq!(
        paths,
        vec![
            QueryAspectPath::TOPOLOGY_BOUNDARY,
            QueryAspectPath::NAMING_PERSISTENT_NAME,
            QueryAspectPath::DIAGNOSTICS_DECISIONS,
        ]
    );

    let path_strings = query_aspect_path_strings(aspects);
    assert_eq!(
        path_strings,
        vec![
            "topology.boundary".to_string(),
            "naming.persistent_name".to_string(),
            "diagnostics.decisions".to_string(),
        ]
    );
}

#[test]
fn query_aspect_families_preserve_domain_boundaries_without_runtime_behavior() {
    assert_eq!(
        QueryAspectPath::TOPOLOGY_STRUCTURE.family(),
        QueryAspectFamily::Topology
    );
    assert_eq!(
        QueryAspectPath::GEOMETRY_BINDING.family(),
        QueryAspectFamily::Geometry
    );
    assert_eq!(
        QueryAspectPath::from_aspect(Aspect::Geometry(GeometryAspect::Fallback)),
        QueryAspectPath::GEOMETRY_FALLBACK
    );
}

fn query_field_key(path: QueryAspectPath) -> AspectFieldKey {
    AspectFieldKey::from_authoring_parts(path.section(), path.field())
        .expect("worth schema query paths should admit as forge-query field keys")
}

fn live_field_key(field: QueryLiveField) -> AspectFieldKey {
    AspectFieldKey::from_authoring_parts(field.aspect(), field.field())
        .expect("worth schema live fields should admit as forge-query field keys")
}

fn aspect_name(name: &str) -> AspectName {
    AspectName::new(name).expect("aspect name should admit")
}

fn aspect_touch(path: QueryAspectPath) -> ForgeQueryAspectTouch {
    ForgeQueryAspectTouch::aspect_field_path(
        AspectKey::new(path.section()).expect("aspect key should admit"),
        CanonicalFieldPath::single(FieldKey::new(path.field()).expect("field key should admit")),
    )
}

#[test]
fn live_query_declarations_lower_with_owned_vocabularies() {
    let declaration = ForgeQueryLiveViewBuilder::surface(".topology.entities")
        .grouped_by(
            AspectKey::new(QueryAspectPath::TOPOLOGY_BOUNDARY.as_str())
                .expect("worth schema query paths should be valid native aspect keys"),
        )
        .select([
            query_field_key(QueryAspectPath::TOPOLOGY_STRUCTURE),
            query_field_key(QueryAspectPath::NAMING_PERSISTENT_NAME),
        ])
        .order_by(query_field_key(QueryAspectPath::NAMING_PERSISTENT_NAME))
        .from(QueryCollection::TopologyEntity.as_str())
        .schema_basis(QuerySchemaBasis::TopologyEntityLiveView.as_str())
        .build()
        .expect(" live declaration should lower into forge-query");

    assert_eq!(declaration.request().target(), "TopologyEntity");
    assert_eq!(
        declaration.request().view_shape().as_str(),
        "kanban_grouped"
    );
    assert_eq!(declaration.request().projection().len(), 2);
    assert_eq!(
        declaration.request().projection()[0].delivered_name(),
        "topology.structure"
    );
    assert_eq!(
        declaration.request().projection()[1].delivered_name(),
        "naming.persistent_name"
    );
    assert_eq!(
        declaration
            .request()
            .ordering()
            .first()
            .expect("live declaration should preserve ordering")
            .source_field_key()
            .field()
            .as_str(),
        "persistent_name"
    );
    assert!(declaration.schema_view().has_aspect(&aspect_name("topology")));
    assert!(declaration.schema_view().has_aspect(&aspect_name("naming")));
}

#[test]
fn live_query_declarations_can_carry_topology_runtime_metadata_fields() {
    let declaration = ForgeQueryLiveViewBuilder::surface(".topology.relations")
        .select([
            live_field_key(QueryLiveField::IdentityId),
            live_field_key(QueryLiveField::TopologyKind),
            live_field_key(QueryLiveField::TopologySourceIdentity),
            live_field_key(QueryLiveField::TopologyTargetIdentity),
        ])
        .order_by(live_field_key(QueryLiveField::IdentityId))
        .from(QueryCollection::TopologyRelation.as_str())
        .schema_basis(QuerySchemaBasis::TopologyRelationLiveView.as_str())
        .build()
        .expect(" relation live declaration should lower runtime metadata fields");

    assert_eq!(declaration.request().target(), "TopologyRelation");
    assert_eq!(declaration.request().projection().len(), 4);
    assert_eq!(
        declaration.request().projection()[0]
            .source_field_key()
            .aspect()
            .as_str(),
        "identity"
    );
    assert_eq!(
        declaration.request().projection()[0]
            .source_field_key()
            .field()
            .as_str(),
        "id"
    );
    assert_eq!(
        declaration.request().projection()[1].delivered_name(),
        "topology.kind"
    );
    assert!(declaration.schema_view().has_aspect(&aspect_name("identity")));
    assert!(declaration.schema_view().has_aspect(&aspect_name("topology")));
    assert_eq!(
        declaration
            .request()
            .ordering()
            .first()
            .expect("metadata declaration should preserve ordering")
            .source_field_key()
            .field()
            .as_str(),
        "id"
    );
}

#[test]
fn computed_query_declarations_lower_with_owned_aspect_contracts() {
    let declaration = ForgeQueryComputedBuilder::surface(".topology.validation")
        .reads([
            aspect_touch(QueryAspectPath::TOPOLOGY_STRUCTURE),
            aspect_touch(QueryAspectPath::NAMING_PERSISTENT_NAME),
        ])
        .produces([
            aspect_touch(QueryAspectPath::DIAGNOSTICS_DECISIONS),
            aspect_touch(QueryAspectPath::DIAGNOSTICS_INTERPRETATIONS),
        ])
        .whole_refresh_fallback()
        .build()
        .expect(" computed declaration should lower into forge-query");

    assert_eq!(declaration.name(), ".topology.validation");
    assert_eq!(
        declaration
            .dependency_aspect_touches()
            .iter()
            .map(|touch| format!(
                "{}.{}",
                touch.native_aspect_key().as_str(),
                touch.native_field_path()
                    .expect("field-level touch")
                    .fields()[0]
                    .as_str()
            ))
            .collect::<Vec<_>>(),
        vec![
            "topology.structure".to_string(),
            "naming.persistent_name".to_string(),
        ]
    );
    assert_eq!(
        declaration
            .produced_aspect_touches()
            .iter()
            .map(|touch| format!(
                "{}.{}",
                touch.native_aspect_key().as_str(),
                touch.native_field_path()
                    .expect("field-level touch")
                    .fields()[0]
                    .as_str()
            ))
            .collect::<Vec<_>>(),
        vec![
            "diagnostics.decisions".to_string(),
            "diagnostics.interpretations".to_string(),
        ]
    );
    assert!(!declaration.incremental());
}
