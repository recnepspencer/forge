use super::edge_splitting_split_vertex_identity_support::build_interval_subdivision_schedule_for_metaboss;
use super::metaboss_support::MetabossEventExtractionSubject;
use forge_query::facade::ForgeQueryApplicationFacade;
use std::collections::BTreeSet;
use topology::facade::{EntityId, NamingAttachmentReport, NamingAttachmentRow, PartitionId};
use topology::query_domain::{
    topology_current_head_authoritative_context, topology_query_domain_entry,
};
use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanSplitIdentityEvolutionOutcomeKind, PlanarBooleanSplitNamedArtifactKind,
    PlanarBooleanSplitPersistentNamingInput, PlanarBooleanSplitPersistentNamingQueryBasis,
    PlanarBooleanSplitPersistentNamingReceipt,
};

pub(crate) fn assert_split_persistent_naming_matches_metaboss(
    subject: &MetabossEventExtractionSubject,
) {
    let interval_normalized = build_interval_subdivision_schedule_for_metaboss(subject);
    let split_vertices = interval_normalized
        .mint_split_vertex_identities()
        .expect("metaboss split vertices should mint before persistent naming");
    let fragments = interval_normalized
        .build_split_edge_fragments(&split_vertices)
        .expect("metaboss fragments should build before persistent naming");
    let chains = interval_normalized
        .build_overlap_edge_chains(&fragments)
        .expect("metaboss overlap chains should build before persistent naming");
    let validation = fragments
        .validate_split_edge_chains(&chains)
        .expect("metaboss split-chain validation should certify before persistent naming");
    let naming = PlanarBooleanSplitPersistentNamingReceipt::admit(
        PlanarBooleanSplitPersistentNamingInput::new(
            &validation,
            &fragments,
            &split_vertices,
            &chains,
            typed_topology_query_basis(),
        ),
    )
    .expect("metaboss split persistent naming should admit from Query-native lineage");

    assert!(naming.certifies_query_native_split_persistent_naming());
    assert_eq!(
        naming.split_chain_validation_receipt_identity(),
        validation.receipt_identity()
    );
    assert_eq!(
        naming.split_edge_fragment_set_identity(),
        fragments.fragment_set_identity()
    );
    assert_eq!(
        naming.split_vertex_identity_set_identity(),
        split_vertices.split_vertex_identity_set_identity()
    );
    assert_eq!(
        naming.overlap_edge_chain_set_identity(),
        chains.chain_set_identity()
    );
    assert_query_evolution_is_plural_for_every_source(&naming);
    assert_all_artifact_families_are_named(&naming);
    assert_selector_and_signature_rows_reconcile(&naming);
}

fn assert_query_evolution_is_plural_for_every_source(
    naming: &PlanarBooleanSplitPersistentNamingReceipt,
) {
    assert!(!naming.identity_evolution_rows().is_empty());
    for row in naming.identity_evolution_rows() {
        assert_eq!(
            row.outcome_kind(),
            PlanarBooleanSplitIdentityEvolutionOutcomeKind::PluralSplitSuccessors
        );
        assert_eq!(row.successor_identities().len(), 2);
        assert!(!row.query_digest().is_empty());
        assert!(!row.basis_digest().is_empty());
        assert!(!row.lineage_digest().is_empty());
        assert!(!row.result_digest().is_empty());
    }
}

fn assert_all_artifact_families_are_named(naming: &PlanarBooleanSplitPersistentNamingReceipt) {
    let kinds = naming
        .persistent_name_rows()
        .iter()
        .map(|row| row.artifact_kind())
        .collect::<BTreeSet<_>>();
    assert!(kinds.contains(&PlanarBooleanSplitNamedArtifactKind::SplitFragment));
    assert!(kinds.contains(&PlanarBooleanSplitNamedArtifactKind::SplitVertex));
    assert!(kinds.contains(&PlanarBooleanSplitNamedArtifactKind::OverlapChain));
    assert!(kinds.contains(&PlanarBooleanSplitNamedArtifactKind::RetainedInterval));
    assert!(kinds.contains(&PlanarBooleanSplitNamedArtifactKind::EventCause));
    for row in naming.persistent_name_rows() {
        assert!(!row.persistent_name_identity().is_empty());
        assert!(!row.identity_evolution_query_digest().is_empty());
        assert!(!row.identity_evolution_result_digest().is_empty());
        assert!(!row.subshape_signature_identity().is_empty());
    }
}

fn assert_selector_and_signature_rows_reconcile(
    naming: &PlanarBooleanSplitPersistentNamingReceipt,
) {
    assert_eq!(
        naming.selector_resolution_rows().len(),
        naming.persistent_name_rows().len()
    );
    assert_eq!(
        naming.subshape_signature_rows().len(),
        naming.persistent_name_rows().len()
    );
    assert!(naming
        .subshape_signature_rows()
        .iter()
        .all(|row| row.is_correspondence_only()));
    assert_eq!(naming.counters().geometry_authority_attempts_rejected(), 0);
}

fn typed_topology_query_basis() -> PlanarBooleanSplitPersistentNamingQueryBasis {
    let query = ForgeQueryApplicationFacade::runtime_backed_default();
    let topology_domain_handle = topology_query_domain_entry(&query)
        .with_operating_context(topology_current_head_authoritative_context())
        .validate()
        .expect("metaboss topology Query context should validate")
        .admit()
        .expect("metaboss topology Query context should admit");
    let naming_attachment_report = NamingAttachmentReport {
        fully_named: true,
        orphan_persistent_name_ids: Vec::new(),
        attachments: vec![NamingAttachmentRow {
            topology_entity_id: EntityId::new(PartitionId::new(0), 700, 0),
            topology_kind_name: "Edge".to_string(),
            attached_persistent_name_ids: vec![EntityId::new(PartitionId::new(0), 701, 0)],
        }],
    };
    PlanarBooleanSplitPersistentNamingQueryBasis::from_topology_query_artifacts(
        &topology_domain_handle,
        &naming_attachment_report,
    )
    .expect("metaboss typed topology Query artifacts should build persistent naming basis")
}
