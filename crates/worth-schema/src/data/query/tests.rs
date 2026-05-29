use std::collections::BTreeSet;

use forge_query::facade::{ForgeQueryComputedBuilder, ForgeQueryLiveViewBuilder};

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

#[test]
fn live_query_declarations_lower_with_owned_vocabularies() {
    let declaration = ForgeQueryLiveViewBuilder::surface(".topology.entities")
        .grouped_by(QueryAspectPath::TOPOLOGY_BOUNDARY.as_str())
        .select([
            QueryAspectPath::TOPOLOGY_STRUCTURE.as_str(),
            QueryAspectPath::NAMING_PERSISTENT_NAME.as_str(),
        ])
        .order_by(QueryAspectPath::NAMING_PERSISTENT_NAME.as_str())
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
            .field(),
        "persistent_name"
    );
    assert!(declaration.schema_view().has_aspect("topology"));
    assert!(declaration.schema_view().has_aspect("naming"));
}

#[test]
fn live_query_declarations_can_carry_topology_runtime_metadata_fields() {
    let declaration = ForgeQueryLiveViewBuilder::surface(".topology.relations")
        .select([
            QueryLiveField::IdentityId.delivered_name(),
            QueryLiveField::TopologyKind.delivered_name(),
            QueryLiveField::TopologySourceIdentity.delivered_name(),
            QueryLiveField::TopologyTargetIdentity.delivered_name(),
        ])
        .order_by(QueryLiveField::IdentityId.delivered_name())
        .from(QueryCollection::TopologyRelation.as_str())
        .schema_basis(QuerySchemaBasis::TopologyRelationLiveView.as_str())
        .build()
        .expect(" relation live declaration should lower runtime metadata fields");

    assert_eq!(declaration.request().target(), "TopologyRelation");
    assert_eq!(declaration.request().projection().len(), 4);
    assert_eq!(declaration.request().projection()[0].aspect(), "identity");
    assert_eq!(declaration.request().projection()[0].field(), "id");
    assert_eq!(
        declaration.request().projection()[1].delivered_name(),
        "topology.kind"
    );
    assert!(declaration.schema_view().has_aspect("identity"));
    assert!(declaration.schema_view().has_aspect("topology"));
    assert_eq!(
        declaration
            .request()
            .ordering()
            .first()
            .expect("metadata declaration should preserve ordering")
            .field(),
        "id"
    );
}

#[test]
fn computed_query_declarations_lower_with_owned_aspect_contracts() {
    let declaration = ForgeQueryComputedBuilder::surface(".topology.validation")
        .reads([
            QueryAspectPath::TOPOLOGY_STRUCTURE.as_str(),
            QueryAspectPath::NAMING_PERSISTENT_NAME.as_str(),
        ])
        .produces([
            QueryAspectPath::DIAGNOSTICS_DECISIONS.as_str(),
            QueryAspectPath::DIAGNOSTICS_INTERPRETATIONS.as_str(),
        ])
        .whole_refresh_fallback()
        .build()
        .expect(" computed declaration should lower into forge-query");

    assert_eq!(declaration.name(), ".topology.validation");
    assert_eq!(
        declaration.dependency_aspects(),
        &[
            "topology.structure".to_string(),
            "naming.persistent_name".to_string(),
        ]
    );
    assert_eq!(
        declaration.produced_aspects(),
        &[
            "diagnostics.decisions".to_string(),
            "diagnostics.interpretations".to_string(),
        ]
    );
    assert!(!declaration.incremental());
}
