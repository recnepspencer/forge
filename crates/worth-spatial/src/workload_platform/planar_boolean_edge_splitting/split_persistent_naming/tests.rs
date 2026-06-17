use std::collections::BTreeSet;

use crate::workload_platform::planar_boolean_edge_splitting::duplicate_split_normalization::tests_support::{
    raw_interval_entry, raw_point_entry, raw_schedule, raw_set_from_schedules,
};
use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanMicroIntervalPolicy, PlanarBooleanSplitPersistentNamingDenialKind,
};

use super::{
    PlanarBooleanSplitIdentityEvolutionOutcomeKind, PlanarBooleanSplitNamedArtifactKind,
    PlanarBooleanSplitPersistentNamingInput, PlanarBooleanSplitPersistentNamingQueryBasis,
    PlanarBooleanSplitPersistentNamingReceipt,
};
use forge_query::facade::ForgeQueryApplicationFacade;
use topology::facade::{EntityId, NamingAttachmentReport, NamingAttachmentRow, PartitionId};
use topology::query_domain::{
    topology_current_head_authoritative_context, topology_query_domain_entry,
};

#[test]
fn split_persistent_naming_binds_every_split_artifact_to_query_identity_evolution() {
    let products = prepared_split_products();
    let receipt = persistent_naming_for(&products);

    assert!(receipt.certifies_query_native_split_persistent_naming());
    assert_eq!(receipt.identity_evolution_rows().len(), 1);
    assert_eq!(
        receipt.identity_evolution_rows()[0].outcome_kind(),
        PlanarBooleanSplitIdentityEvolutionOutcomeKind::PluralSplitSuccessors
    );
    assert!(receipt.persistent_name_rows().iter().all(|row| !row
        .identity_evolution_query_digest()
        .is_empty()
        && row.identity_evolution_result_digest()
            == receipt.identity_evolution_rows()[0].result_digest()));
    assert_eq!(receipt.counters().identity_evolution_queries_admitted(), 1);
    assert_eq!(receipt.counters().identity_evolution_queries_executed(), 1);
}

#[test]
fn split_identity_evolution_emits_plural_successors_for_source_edge_fragments() {
    let products = prepared_split_products();
    let receipt = persistent_naming_for(&products);
    let evolution = &receipt.identity_evolution_rows()[0];

    assert_eq!(
        evolution.outcome_kind(),
        PlanarBooleanSplitIdentityEvolutionOutcomeKind::PluralSplitSuccessors
    );
    assert_eq!(evolution.successor_identities().len(), 2);
    assert_eq!(receipt.counters().plural_successors_emitted(), 2);
}

#[test]
fn split_persistent_naming_propagates_source_edge_name_to_all_artifact_kinds() {
    let products = prepared_split_products();
    let receipt = persistent_naming_for(&products);
    let kinds = receipt
        .persistent_name_rows()
        .iter()
        .map(|row| row.artifact_kind())
        .collect::<BTreeSet<_>>();

    assert!(kinds.contains(&PlanarBooleanSplitNamedArtifactKind::SplitFragment));
    assert!(kinds.contains(&PlanarBooleanSplitNamedArtifactKind::SplitVertex));
    assert!(kinds.contains(&PlanarBooleanSplitNamedArtifactKind::OverlapChain));
    assert!(kinds.contains(&PlanarBooleanSplitNamedArtifactKind::RetainedInterval));
    assert!(kinds.contains(&PlanarBooleanSplitNamedArtifactKind::EventCause));
    assert!(receipt
        .persistent_name_rows()
        .iter()
        .all(|row| row.source_edge_identity() == "source edge"));
}

#[test]
fn split_selector_resolution_and_subshape_signatures_are_stable_under_replay_order() {
    let first = persistent_naming_for(&prepared_split_products());
    let replayed = persistent_naming_for(&prepared_replayed_split_products());

    assert_eq!(first.receipt_identity(), replayed.receipt_identity());
    assert_eq!(selector_identities(&first), selector_identities(&replayed));
    assert_eq!(
        subshape_signature_identities(&first),
        subshape_signature_identities(&replayed)
    );
    assert!(first
        .subshape_signature_rows()
        .iter()
        .all(|row| row.is_correspondence_only()));
}

#[test]
fn split_persistent_naming_rejects_foreign_fragment_set() {
    let products = prepared_split_products();
    let foreign = prepared_foreign_split_products();
    let denial = PlanarBooleanSplitPersistentNamingReceipt::admit(
        PlanarBooleanSplitPersistentNamingInput::new(
            &products.validation,
            &foreign.fragments,
            &products.vertices,
            &products.chains,
            query_basis(),
        ),
    )
    .expect_err("foreign fragment products must deny naming");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitPersistentNamingDenialKind::ForeignFragmentSet
    );
}

#[test]
fn split_persistent_naming_requires_certified_chain_validation() {
    let products = prepared_split_products();
    let foreign = prepared_foreign_split_products();
    let denial = PlanarBooleanSplitPersistentNamingReceipt::admit(
        PlanarBooleanSplitPersistentNamingInput::new(
            &products.validation,
            &products.fragments,
            &products.vertices,
            &foreign.chains,
            query_basis(),
        ),
    )
    .expect_err("foreign overlap chains must deny naming");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitPersistentNamingDenialKind::ForeignOverlapChainSet
    );
}

#[test]
fn split_persistent_naming_rejects_geometry_display_string_or_coordinate_authority() {
    let products = prepared_split_products();
    let coordinate_derived_basis = PlanarBooleanSplitPersistentNamingQueryBasis::from_query_runtime(
        "worth.topology/current_head_authoritative",
        "display:persistent-name-live-view",
        "naming-attachment-report:split",
    );

    let denial = PlanarBooleanSplitPersistentNamingReceipt::admit(
        PlanarBooleanSplitPersistentNamingInput::new(
            &products.validation,
            &products.fragments,
            &products.vertices,
            &products.chains,
            coordinate_derived_basis,
        ),
    )
    .expect_err("display-derived Query basis identities must not authorize persistent names");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitPersistentNamingDenialKind::GeometryOrDisplayAuthorityRejected
    );
}

#[test]
fn split_persistent_naming_rejects_unattached_topology_query_naming_basis() {
    let mut report = naming_attachment_report();
    report.fully_named = false;

    let denial = PlanarBooleanSplitPersistentNamingQueryBasis::from_topology_query_artifacts(
        &topology_domain_handle(),
        &report,
    )
    .expect_err("unattached Query persistent-name reports must not authorize naming");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitPersistentNamingDenialKind::DanglingPersistentNameReference
    );
}

#[test]
fn split_persistent_naming_rejects_empty_topology_query_naming_evidence() {
    let mut report = naming_attachment_report();
    report.attachments.clear();

    let denial = PlanarBooleanSplitPersistentNamingQueryBasis::from_topology_query_artifacts(
        &topology_domain_handle(),
        &report,
    )
    .expect_err("empty Query persistent-name attachment reports must not authorize naming");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitPersistentNamingDenialKind::DanglingPersistentNameReference
    );
}

#[test]
fn split_persistent_naming_rejects_claimed_attachment_without_persistent_name() {
    let mut report = naming_attachment_report();
    report.attachments[0].attached_persistent_name_ids.clear();

    let denial = PlanarBooleanSplitPersistentNamingQueryBasis::from_topology_query_artifacts(
        &topology_domain_handle(),
        &report,
    )
    .expect_err("attachment rows without persistent names must not authorize naming");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitPersistentNamingDenialKind::DanglingPersistentNameReference
    );
}

#[test]
fn split_persistent_naming_rejects_claimed_fully_named_report_with_orphans() {
    let mut report = naming_attachment_report();
    report
        .orphan_persistent_name_ids
        .push(EntityId::new(PartitionId::new(0), 99, 0));

    let denial = PlanarBooleanSplitPersistentNamingQueryBasis::from_topology_query_artifacts(
        &topology_domain_handle(),
        &report,
    )
    .expect_err("orphan Query persistent names must not authorize split naming");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitPersistentNamingDenialKind::DanglingPersistentNameReference
    );
}

#[test]
fn split_persistent_naming_rejects_duplicate_name_references() {
    let receipt = persistent_naming_for(&prepared_split_products());
    let duplicate_row = receipt.persistent_name_rows()[0].clone();
    let duplicated_rows = vec![duplicate_row.clone(), duplicate_row];
    let mut counters = super::counters::PlanarBooleanSplitPersistentNamingCounters::default();

    let denial = super::validation::validate_persistent_name_rows(&duplicated_rows, &mut counters)
        .expect_err("duplicate persistent-name rows must deny");

    assert_eq!(
        denial.kind(),
        PlanarBooleanSplitPersistentNamingDenialKind::DuplicatePersistentName
    );
    assert_eq!(counters.duplicate_names_rejected(), 1);
}

fn selector_identities(receipt: &PlanarBooleanSplitPersistentNamingReceipt) -> Vec<String> {
    receipt
        .selector_resolution_rows()
        .iter()
        .map(|row| row.row_identity().to_string())
        .collect()
}

fn subshape_signature_identities(
    receipt: &PlanarBooleanSplitPersistentNamingReceipt,
) -> Vec<String> {
    receipt
        .subshape_signature_rows()
        .iter()
        .map(|row| row.row_identity().to_string())
        .collect()
}

fn persistent_naming_for(
    products: &PreparedSplitProducts,
) -> PlanarBooleanSplitPersistentNamingReceipt {
    PlanarBooleanSplitPersistentNamingReceipt::admit(PlanarBooleanSplitPersistentNamingInput::new(
        &products.validation,
        &products.fragments,
        &products.vertices,
        &products.chains,
        query_basis(),
    ))
    .expect("prepared split products should admit persistent naming")
}

fn query_basis() -> PlanarBooleanSplitPersistentNamingQueryBasis {
    PlanarBooleanSplitPersistentNamingQueryBasis::from_topology_query_artifacts(
        &topology_domain_handle(),
        &naming_attachment_report(),
    )
    .expect("typed topology Query artifacts should produce split naming basis")
}

fn topology_domain_handle() -> topology::query_domain::TopologyCurrentHeadConfiguredDomainHandle {
    let query = ForgeQueryApplicationFacade::runtime_backed_default();
    topology_query_domain_entry(&query)
        .with_operating_context(topology_current_head_authoritative_context())
        .validate()
        .expect("current-head topology context should validate")
        .admit()
        .expect("current-head topology context should admit")
}

fn naming_attachment_report() -> NamingAttachmentReport {
    NamingAttachmentReport {
        fully_named: true,
        orphan_persistent_name_ids: Vec::new(),
        attachments: vec![NamingAttachmentRow {
            topology_entity_id: EntityId::new(PartitionId::new(0), 1, 0),
            topology_kind_name: "Edge".to_string(),
            attached_persistent_name_ids: vec![EntityId::new(PartitionId::new(0), 2, 0)],
        }],
    }
}

struct PreparedSplitProducts {
    vertices: crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanSplitVertexIdentitySet,
    fragments: crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeFragmentSet,
    chains: crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapEdgeChainSet,
    validation: crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanSplitChainValidationReceipt,
}

fn prepared_split_products() -> PreparedSplitProducts {
    build_products(vec![
        raw_point_entry("point a", "source edge", "carrier", "event:a", 0.25),
        raw_interval_entry("interval", "source edge", "carrier", "event:interval", 0.4),
        raw_point_entry("point b", "source edge", "carrier", "event:b", 0.75),
    ])
}

fn prepared_replayed_split_products() -> PreparedSplitProducts {
    build_products(vec![
        raw_point_entry("point b", "source edge", "carrier", "event:b", 0.75),
        raw_interval_entry("interval", "source edge", "carrier", "event:interval", 0.4),
        raw_point_entry("point a", "source edge", "carrier", "event:a", 0.25),
    ])
}

fn prepared_foreign_split_products() -> PreparedSplitProducts {
    build_products(vec![
        raw_point_entry(
            "foreign point a",
            "source edge",
            "carrier",
            "event:foreign-a",
            0.3,
        ),
        raw_interval_entry(
            "foreign interval",
            "source edge",
            "carrier",
            "event:foreign-i",
            0.5,
        ),
        raw_point_entry(
            "foreign point b",
            "source edge",
            "carrier",
            "event:foreign-b",
            0.8,
        ),
    ])
}

fn build_products(
    entries: Vec<crate::workload_platform::planar_boolean_edge_splitting::raw_edge_split_schedule::PlanarBooleanRawEdgeSplitScheduleEntry>,
) -> PreparedSplitProducts {
    let interval_subdivision = raw_set_from_schedules(vec![raw_schedule(
        "raw schedule",
        "source edge",
        "carrier",
        entries,
    )])
    .canonicalize_split_schedule_order()
    .expect("raw schedule should order")
    .collapse_duplicate_split_points()
    .expect("duplicate split points should normalize")
    .normalize_endpoint_boundary_splits()
    .expect("endpoint boundaries should normalize")
    .normalize_overlap_interval_subdivisions(PlanarBooleanMicroIntervalPolicy::DenyBelowTolerance)
    .expect("overlap intervals should normalize");
    let vertices = interval_subdivision
        .mint_split_vertex_identities()
        .expect("split vertices should mint");
    let fragments = interval_subdivision
        .build_split_edge_fragments(&vertices)
        .expect("split fragments should build");
    let chains = interval_subdivision
        .build_overlap_edge_chains(&fragments)
        .expect("overlap chains should build");
    let validation = fragments
        .validate_split_edge_chains(&chains)
        .expect("split chains should validate");
    PreparedSplitProducts {
        vertices,
        fragments,
        chains,
        validation,
    }
}
