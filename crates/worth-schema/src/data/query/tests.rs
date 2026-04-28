use std::collections::BTreeSet;

use crate::facade::{
    worth_query_aspect_path_strings, worth_query_aspect_paths_from_set, WorthAspect,
    WorthDiagnosticsAspect, WorthGeometryAspect, WorthLineageAspect, WorthNamingAspect,
    WorthQueryAspectFamily, WorthQueryAspectPath, WorthQueryCollection, WorthQuerySchemaBasis,
    WorthTopologyAspect,
};

#[test]
fn query_collections_and_schema_bases_have_stable_worth_names() {
    assert_eq!(
        WorthQueryCollection::TopologyEntity.as_str(),
        "WorthTopologyEntity"
    );
    assert_eq!(
        WorthQueryCollection::TopologyEquivalenceContract.as_str(),
        "WorthTopologyEquivalenceContract"
    );
    assert_eq!(
        WorthQuerySchemaBasis::AuthoritativeTopologyTruth.as_str(),
        "worth.schema.authoritative_topology_truth"
    );
    assert_eq!(
        WorthQuerySchemaBasis::TopologyValidationComputed.as_str(),
        "worth.schema.computed.topology_validation"
    );
    assert_eq!(WorthQueryCollection::ALL.len(), 9);
    assert_eq!(WorthQuerySchemaBasis::ALL.len(), 10);
}

#[test]
fn query_aspect_paths_are_valid_forge_query_aspect_field_paths() {
    for path in WorthQueryAspectPath::ALL {
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
        WorthAspect::Diagnostics(WorthDiagnosticsAspect::Decisions).as_str(),
        "diagnostics"
    );
    assert_eq!(
        WorthQueryAspectPath::from_worth_aspect(WorthAspect::Diagnostics(
            WorthDiagnosticsAspect::Decisions
        ))
        .as_str(),
        "diagnostics.decisions"
    );
    assert_eq!(
        WorthAspect::Lineage(WorthLineageAspect::Provenance).as_str(),
        "lineage"
    );
    assert_eq!(
        WorthQueryAspectPath::from_worth_aspect(WorthAspect::Lineage(
            WorthLineageAspect::Provenance
        ))
        .as_str(),
        "lineage.provenance"
    );
}

#[test]
fn touched_aspect_sets_convert_to_deterministic_query_paths() {
    let aspects = BTreeSet::from([
        WorthAspect::Diagnostics(WorthDiagnosticsAspect::Decisions),
        WorthAspect::Topology(WorthTopologyAspect::Boundary),
        WorthAspect::Naming(WorthNamingAspect::PersistentName),
    ]);

    let paths = worth_query_aspect_paths_from_set(&aspects);
    assert_eq!(
        paths,
        vec![
            WorthQueryAspectPath::TOPOLOGY_BOUNDARY,
            WorthQueryAspectPath::NAMING_PERSISTENT_NAME,
            WorthQueryAspectPath::DIAGNOSTICS_DECISIONS,
        ]
    );

    let path_strings = worth_query_aspect_path_strings(aspects);
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
        WorthQueryAspectPath::TOPOLOGY_STRUCTURE.family(),
        WorthQueryAspectFamily::Topology
    );
    assert_eq!(
        WorthQueryAspectPath::GEOMETRY_BINDING.family(),
        WorthQueryAspectFamily::Geometry
    );
    assert_eq!(
        WorthQueryAspectPath::from_worth_aspect(WorthAspect::Geometry(
            WorthGeometryAspect::Fallback
        )),
        WorthQueryAspectPath::GEOMETRY_FALLBACK
    );
}
