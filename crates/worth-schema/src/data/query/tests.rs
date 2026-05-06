use std::collections::BTreeSet;

use crate::facade::topology_authoring::created_ref;
use crate::facade::{
    admit_worth_query_mutation_batch, worth_query_aspect_path_strings,
    worth_query_aspect_paths_from_set, worth_query_mutation_support_contract,
    RawWorthTopologyIntent, WorthAspect, WorthDiagnosticsAspect, WorthDiagnosticsRelationKind,
    WorthEntityKind, WorthGeometryAspect, WorthGeometryEntityKind, WorthLineageAspect,
    WorthNamingAspect, WorthQueryAspectFamily, WorthQueryAspectPath, WorthQueryCollection,
    WorthQueryComputedDeclarationBuilder, WorthQueryDeclarationError,
    WorthQueryLiveDeclarationBuilder, WorthQueryLiveField, WorthQueryMutationAdmission,
    WorthQueryMutationAdmissionBlocker, WorthQuerySchemaBasis, WorthRelationKind,
    WorthTopologyAspect, WorthTopologyEntityKind, WorthTopologyMutation,
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
    assert_eq!(
        WorthQuerySchemaBasis::TopologyDomainQuery.as_str(),
        "worth.schema.domain.topology_query"
    );
    assert_eq!(WorthQueryCollection::ALL.len(), 9);
    assert_eq!(WorthQuerySchemaBasis::ALL.len(), 11);
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

#[test]
fn worth_live_query_declarations_lower_with_worth_owned_vocabularies() {
    let declaration = WorthQueryLiveDeclarationBuilder::new(
        "worth.topology.entities",
        WorthQueryCollection::TopologyEntity,
        WorthQuerySchemaBasis::TopologyEntityLiveView,
    )
    .grouped_by(WorthQueryAspectPath::TOPOLOGY_BOUNDARY)
    .select([
        WorthQueryAspectPath::TOPOLOGY_STRUCTURE,
        WorthQueryAspectPath::NAMING_PERSISTENT_NAME,
    ])
    .order_by(WorthQueryAspectPath::NAMING_PERSISTENT_NAME)
    .build()
    .expect("worth live declaration should lower into forge-query");

    assert_eq!(declaration.request().target(), "WorthTopologyEntity");
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
            .expect("worth live declaration should preserve ordering")
            .delivered_name(),
        "persistent_name"
    );
    assert!(declaration.schema_view().has_aspect("topology"));
    assert!(declaration.schema_view().has_aspect("naming"));
}

#[test]
fn worth_live_query_declarations_can_carry_topology_runtime_metadata_fields() {
    let declaration = WorthQueryLiveDeclarationBuilder::new(
        "worth.topology.relations",
        WorthQueryCollection::TopologyRelation,
        WorthQuerySchemaBasis::TopologyRelationLiveView,
    )
    .select_fields([
        WorthQueryLiveField::IdentityId,
        WorthQueryLiveField::TopologyKind,
        WorthQueryLiveField::TopologySourceIdentity,
        WorthQueryLiveField::TopologyTargetIdentity,
    ])
    .order_by_field(WorthQueryLiveField::IdentityId)
    .build()
    .expect("worth relation live declaration should lower runtime metadata fields");

    assert_eq!(declaration.request().target(), "WorthTopologyRelation");
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
            .expect("metadata declaration should preserve ordering")
            .delivered_name(),
        "id"
    );
}

#[test]
fn worth_computed_query_declarations_lower_with_worth_owned_aspect_contracts() {
    let declaration = WorthQueryComputedDeclarationBuilder::new("worth.topology.validation")
        .reads([
            WorthQueryAspectPath::TOPOLOGY_STRUCTURE,
            WorthQueryAspectPath::NAMING_PERSISTENT_NAME,
        ])
        .produces([
            WorthQueryAspectPath::DIAGNOSTICS_DECISIONS,
            WorthQueryAspectPath::DIAGNOSTICS_INTERPRETATIONS,
        ])
        .whole_refresh_fallback()
        .build()
        .expect("worth computed declaration should lower into forge-query");

    assert_eq!(declaration.name(), "worth.topology.validation");
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
fn worth_query_declarations_reject_blank_surface_names_early() {
    let error = WorthQueryLiveDeclarationBuilder::new(
        "   ",
        WorthQueryCollection::TopologyEntity,
        WorthQuerySchemaBasis::TopologyEntityLiveView,
    )
    .select([WorthQueryAspectPath::TOPOLOGY_STRUCTURE])
    .build()
    .expect_err("blank worth live surface names must fail early");
    assert!(matches!(
        error,
        WorthQueryDeclarationError::EmptySurfaceName
    ));

    let error = WorthQueryComputedDeclarationBuilder::new("")
        .reads([WorthQueryAspectPath::TOPOLOGY_STRUCTURE])
        .produces([WorthQueryAspectPath::DIAGNOSTICS_DECISIONS])
        .build()
        .expect_err("blank worth computed surface names must fail early");
    assert!(matches!(
        error,
        WorthQueryDeclarationError::EmptySurfaceName
    ));
}

#[test]
fn query_mutation_support_contract_tracks_upstream_authority_closeout() {
    let contract = worth_query_mutation_support_contract()
        .expect("worth query support contract should derive");
    let support_profile = ForgeQueryRuntimeSupportProfile::bridge_backed(
        "worth-query-mutation-support-contract-live",
        "worth-query-mutation-support-contract-preview",
        "worth-query-mutation-support-contract-inspect",
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
    let admission = admit_worth_query_mutation_batch(&RawWorthTopologyIntent::new(
        vec![WorthTopologyMutation::CreateEntity {
            create_key: crate::facade::WorthCreateKey::new("query-ready.model"),
            kind: WorthEntityKind::Topology(WorthTopologyEntityKind::Model),
        }],
        crate::facade::WorthMutationOrigin::Seed,
    ));

    assert!(matches!(admission, WorthQueryMutationAdmission::Admitted));
}

#[test]
fn query_mutation_admission_marks_same_batch_topology_relation_creation_as_ready() {
    let admission = admit_worth_query_mutation_batch(&RawWorthTopologyIntent::new(
        vec![
            WorthTopologyMutation::CreateEntity {
                create_key: crate::facade::WorthCreateKey::new("query-ready.source"),
                kind: WorthEntityKind::Topology(WorthTopologyEntityKind::Vertex),
            },
            WorthTopologyMutation::CreateEntity {
                create_key: crate::facade::WorthCreateKey::new("query-ready.target"),
                kind: WorthEntityKind::Topology(WorthTopologyEntityKind::Vertex),
            },
            WorthTopologyMutation::CreateRelation {
                create_key: crate::facade::WorthCreateKey::new("query-ready.edge"),
                kind: WorthRelationKind::Topology(
                    crate::facade::WorthTopologyRelationKind::HalfEdgeNext,
                ),
                source: created_ref("query-ready.source"),
                target: created_ref("query-ready.target"),
            },
        ],
        crate::facade::WorthMutationOrigin::LocalEdit,
    ));

    assert!(matches!(admission, WorthQueryMutationAdmission::Admitted));
}

#[test]
fn query_mutation_admission_rejects_geometry_and_diagnostics_truth_outside_topology_lane() {
    let admission = admit_worth_query_mutation_batch(&RawWorthTopologyIntent::new(
        vec![
            WorthTopologyMutation::CreateEntity {
                create_key: crate::facade::WorthCreateKey::new("query-gap.geometry"),
                kind: WorthEntityKind::Geometry(WorthGeometryEntityKind::SurfaceBinding),
            },
            WorthTopologyMutation::CreateRelation {
                create_key: crate::facade::WorthCreateKey::new("query-gap.diag-rel"),
                kind: WorthRelationKind::Diagnostics(
                    WorthDiagnosticsRelationKind::WireHasInterpretation,
                ),
                source: created_ref("query-gap.a"),
                target: created_ref("query-gap.b"),
            },
        ],
        crate::facade::WorthMutationOrigin::LocalEdit,
    ));

    let blockers = admission.blockers();
    assert!(blockers.iter().any(|row| {
        row.blocker == WorthQueryMutationAdmissionBlocker::UnsupportedGeometryTruthMutation
    }));
    assert!(blockers.iter().any(|row| {
        row.blocker == WorthQueryMutationAdmissionBlocker::UnsupportedDiagnosticsTruthMutation
    }));
}
