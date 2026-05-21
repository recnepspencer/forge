use std::collections::BTreeSet;
use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_geom::facade::PrimitiveRealizationExhaustionWitnessKind;
use worth_kernel::facade::{
    prepare_primitive_construction_compound_exhaustion_witness_parity_report,
    prepare_primitive_construction_compound_ordering_parity_report,
    prepare_primitive_construction_compound_parity_report,
};

fn sorted_ids(ids: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut ids = ids.into_iter().collect::<Vec<_>>();
    ids.sort();
    ids
}

#[test]
fn kernel_public_facade_exports_compound_ordering_and_exhaustion_parity_artifacts() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.public-compound-parity".to_string(),
    )
    .expect("workspace");
    let ordering = prepare_primitive_construction_compound_ordering_parity_report(&mut workspace)
        .expect("ordering parity report");
    let exhaustion =
        prepare_primitive_construction_compound_exhaustion_witness_parity_report(&mut workspace)
            .expect("exhaustion parity report");
    let compound = prepare_primitive_construction_compound_parity_report(&mut workspace)
        .expect("compound parity report");

    assert!(ordering.parity_verified());
    assert_eq!(
        sorted_ids(
            ordering
                .authoring_order_rows()
                .iter()
                .map(|row| row.lane_name().to_string()),
        ),
        vec![
            "canonical".to_string(),
            "escalation_clustered".to_string(),
            "family_clustered".to_string(),
            "rejected_first".to_string(),
            "reversed".to_string(),
        ]
    );
    assert_eq!(ordering.lane_reports().len(), 5);
    assert!(ordering
        .scenario_row_for("wire_open_endpoint_graze")
        .expect("wire graze scenario row")
        .stable_across_orders());
    assert!(ordering
        .scenario_row_for("pyramid_semantic_exhaustion")
        .expect("pyramid exhaustion scenario row")
        .exhaustion_reason_stable());
    assert_eq!(
        ordering
            .authoring_order_rows()
            .iter()
            .map(|row| row.lane_digest())
            .collect::<BTreeSet<_>>()
            .len(),
        ordering.authoring_order_rows().len()
    );
    assert!(exhaustion.parity_verified());
    assert_eq!(
        exhaustion
            .row_for("pyramid_semantic_exhaustion")
            .expect("pyramid exhaustion row")
            .witness_kind(),
        PrimitiveRealizationExhaustionWitnessKind::ZeroRadiusPyramidSupportCollapse
    );
    assert_eq!(
        exhaustion
            .rows()
            .iter()
            .map(|row| row.witness_row_digest())
            .collect::<BTreeSet<_>>()
            .len(),
        exhaustion.rows().len()
    );
    assert_eq!(compound.exhaustion().rows().len(), 2);
    assert!(compound.parity_verified());
    assert_eq!(compound.motion().rows().len(), 3);
    assert_eq!(compound.grazing().rows().len(), 2);
    assert_ne!(
        compound.ordering().report_digest(),
        compound.motion().report_digest()
    );
}
