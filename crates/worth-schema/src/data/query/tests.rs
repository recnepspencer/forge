use std::collections::BTreeSet;

use crate::facade::topology_authoring::created_ref;
use crate::facade::{
    admit_query_mutation_batch, query_aspect_path_strings, query_aspect_paths_from_set,
    query_mutation_support_contract, Aspect, DiagnosticsAspect, DiagnosticsRelationKind,
    EntityKind, GeometryAspect, GeometryEntityKind, LineageAspect, NamingAspect, QueryAspectFamily,
    QueryAspectPath, QueryCollection, QueryComputedDeclarationBuilder, QueryDeclarationError,
    QueryLiveDeclarationBuilder, QueryLiveField, QueryMutationAdmission,
    QueryMutationAdmissionBlocker, QuerySchemaBasis, RawTopologyIntent, RelationKind,
    TopologyAspect, TopologyEntityKind, TopologyMutation,
};
use forge_query::facade::{
    ForgeQueryAuthoritativeMutationEvidenceCloseout,
    ForgeQueryAuthoritativeMutationEvidenceSupport, ForgeQueryMutationSurfaceReport,
    ForgeQueryRuntimeBackendPosture, ForgeQueryRuntimePublicApiContract,
    ForgeQueryRuntimePublicApiNamingContract, ForgeQueryRuntimePublicSupportMatrix,
    ForgeQueryRuntimeSupportProfile,
};
use forge_runtime_bridge::facade::RuntimeBridge;

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
    let declaration = QueryLiveDeclarationBuilder::new(
        ".topology.entities",
        QueryCollection::TopologyEntity,
        QuerySchemaBasis::TopologyEntityLiveView,
    )
    .grouped_by(QueryAspectPath::TOPOLOGY_BOUNDARY)
    .select([
        QueryAspectPath::TOPOLOGY_STRUCTURE,
        QueryAspectPath::NAMING_PERSISTENT_NAME,
    ])
    .order_by(QueryAspectPath::NAMING_PERSISTENT_NAME)
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
    let declaration = QueryLiveDeclarationBuilder::new(
        ".topology.relations",
        QueryCollection::TopologyRelation,
        QuerySchemaBasis::TopologyRelationLiveView,
    )
    .select_fields([
        QueryLiveField::IdentityId,
        QueryLiveField::TopologyKind,
        QueryLiveField::TopologySourceIdentity,
        QueryLiveField::TopologyTargetIdentity,
    ])
    .order_by_field(QueryLiveField::IdentityId)
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
    let declaration = QueryComputedDeclarationBuilder::new(".topology.validation")
        .reads([
            QueryAspectPath::TOPOLOGY_STRUCTURE,
            QueryAspectPath::NAMING_PERSISTENT_NAME,
        ])
        .produces([
            QueryAspectPath::DIAGNOSTICS_DECISIONS,
            QueryAspectPath::DIAGNOSTICS_INTERPRETATIONS,
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

#[test]
fn query_declarations_reject_blank_surface_names_early() {
    let error = QueryLiveDeclarationBuilder::new(
        "   ",
        QueryCollection::TopologyEntity,
        QuerySchemaBasis::TopologyEntityLiveView,
    )
    .select([QueryAspectPath::TOPOLOGY_STRUCTURE])
    .build()
    .expect_err("blank  live surface names must fail early");
    assert!(matches!(error, QueryDeclarationError::EmptySurfaceName));

    let error = QueryComputedDeclarationBuilder::new("")
        .reads([QueryAspectPath::TOPOLOGY_STRUCTURE])
        .produces([QueryAspectPath::DIAGNOSTICS_DECISIONS])
        .build()
        .expect_err("blank  computed surface names must fail early");
    assert!(matches!(error, QueryDeclarationError::EmptySurfaceName));
}

#[test]
fn query_mutation_support_contract_tracks_upstream_authority_closeout() {
    let contract =
        query_mutation_support_contract().expect(" query support contract should derive");
    let support_profile = ForgeQueryRuntimeSupportProfile::bridge_backed(
        "query-mutation-support-contract-live",
        "query-mutation-support-contract-preview",
        "query-mutation-support-contract-inspect",
    );
    let public_api_contract =
        ForgeQueryRuntimePublicApiContract::from_support_profile(&support_profile);
    assert_eq!(
        public_api_contract.backend_posture(),
        ForgeQueryRuntimeBackendPosture::Primary
    );
    let support_matrix =
        ForgeQueryRuntimePublicSupportMatrix::from_public_api_contract(&public_api_contract);
    let naming_contract = ForgeQueryRuntimePublicApiNamingContract::standard();
    let mutation_surface = ForgeQueryMutationSurfaceReport::derive(
        public_api_contract.backend_posture(),
        &support_matrix,
        &naming_contract,
    );
    let query_support = ForgeQueryAuthoritativeMutationEvidenceSupport::derive(&support_profile);
    let bridge_support = RuntimeBridge::public_authoritative_mutation_evidence_support();
    let bridge_closeout = RuntimeBridge::public_authoritative_mutation_evidence_closeout();
    let closeout = ForgeQueryAuthoritativeMutationEvidenceCloseout::derive(
        public_api_contract.backend_posture(),
        &support_matrix,
        &mutation_surface,
        &naming_contract,
        &query_support,
        &bridge_support,
        &bridge_closeout,
    );

    assert!(contract
        .admitted_query_substrate_families
        .iter()
        .any(|family| {
            family == "insert_topology_relation_with_same_batch_symbolic_entity_identity_refs"
        }));
    assert!(contract
        .blocked_until_invariant_complete_workflow
        .iter()
        .any(|family| {
            family
                == "topology_relation_create_workflows_beyond_face_inner_loop_require_invariant_complete_subgraphs"
        }));
    assert!(contract
        .blocked_until_invariant_complete_workflow
        .iter()
        .any(|family| {
            family
                == "topology_shell_or_wire_membership_workflows_beyond_admitted_full_wire_rehome_connected_wire_split_single_face_two_face_shell_split_and_full_shell_face_set_rehome_require_invariant_complete_owner_rehome_or_shell_subgraphs"
        }));
    assert!(contract
        .blocked_until_explicit_lowering
        .iter()
        .any(|family| family == "raw_naming_truth_requires_projected_naming_writeback"));
    assert!(contract
        .admitted_query_substrate_families
        .iter()
        .any(|family| family == "verify_existing_topology_entity_kind"));
    assert!(contract
        .admitted_query_substrate_families
        .iter()
        .any(|family| family == "verify_existing_topology_relation_shape"));
    assert!(contract
        .admitted_query_substrate_families
        .iter()
        .any(|family| family == "update_existing_topology_relation_shape_identity_preserving"));
    assert_eq!(
        contract.query_support_digest,
        query_support.support_digest()
    );
    assert_eq!(contract.query_closeout_digest, closeout.closeout_digest());
}

#[test]
fn query_mutation_admission_marks_simple_topology_creates_as_ready() {
    let admission = admit_query_mutation_batch(&RawTopologyIntent::new(
        vec![TopologyMutation::CreateEntity {
            create_key: crate::facade::CreateKey::new("query-ready.model"),
            kind: EntityKind::Topology(TopologyEntityKind::Model),
        }],
        crate::facade::MutationOrigin::Seed,
    ));

    assert!(matches!(admission, QueryMutationAdmission::Admitted));
}

#[test]
fn query_mutation_admission_marks_same_batch_topology_relation_creation_as_ready() {
    let admission = admit_query_mutation_batch(&RawTopologyIntent::new(
        vec![
            TopologyMutation::CreateEntity {
                create_key: crate::facade::CreateKey::new("query-ready.source"),
                kind: EntityKind::Topology(TopologyEntityKind::Vertex),
            },
            TopologyMutation::CreateEntity {
                create_key: crate::facade::CreateKey::new("query-ready.target"),
                kind: EntityKind::Topology(TopologyEntityKind::Vertex),
            },
            TopologyMutation::CreateRelation {
                create_key: crate::facade::CreateKey::new("query-ready.edge"),
                kind: RelationKind::Topology(crate::facade::TopologyRelationKind::HalfEdgeNext),
                source: created_ref("query-ready.source"),
                target: created_ref("query-ready.target"),
            },
        ],
        crate::facade::MutationOrigin::LocalEdit,
    ));

    assert!(matches!(admission, QueryMutationAdmission::Admitted));
}

#[test]
fn query_mutation_admission_rejects_geometry_and_diagnostics_truth_outside_topology_lane() {
    let admission = admit_query_mutation_batch(&RawTopologyIntent::new(
        vec![
            TopologyMutation::CreateEntity {
                create_key: crate::facade::CreateKey::new("query-gap.geometry"),
                kind: EntityKind::Geometry(GeometryEntityKind::SurfaceBinding),
            },
            TopologyMutation::CreateRelation {
                create_key: crate::facade::CreateKey::new("query-gap.diag-rel"),
                kind: RelationKind::Diagnostics(DiagnosticsRelationKind::WireHasInterpretation),
                source: created_ref("query-gap.a"),
                target: created_ref("query-gap.b"),
            },
        ],
        crate::facade::MutationOrigin::LocalEdit,
    ));

    let blockers = admission.blockers();
    assert!(blockers.iter().any(|row| {
        row.blocker == QueryMutationAdmissionBlocker::UnsupportedGeometryTruthMutation
    }));
    assert!(blockers.iter().any(|row| {
        row.blocker == QueryMutationAdmissionBlocker::UnsupportedDiagnosticsTruthMutation
    }));
}
